use once_cell::sync::Lazy;
use std::sync::RwLock;

pub(crate) mod js_helpers;
pub(crate) mod log;

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::util::log::log(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! add_event_listener {
  ($event_name:expr, $cb:expr) => {
    let cb = Closure::wrap(Box::new($cb) as Box<dyn FnMut(_)>);

    window()
      .add_event_listener_with_callback($event_name, cb.as_ref().unchecked_ref())
      .unwrap();

    cb.forget();
  };
}

pub fn next_id() -> u32 {
  static NEXT_ID: Lazy<RwLock<u32>> = Lazy::new(|| RwLock::new(0));

  if let Ok(mut id) = NEXT_ID.write() {
    if *id < u32::MAX {
      *id += 1;
    } else {
      *id = 0;
    }

    return *id;
  }

  0
}

pub fn rand() -> f64 {
  if let Ok(mut randy) = RANDY.write() {
    return randy.next();
  }

  0.0
}

static RANDY: Lazy<RwLock<Randy>> = Lazy::new(|| RwLock::new(Randy::new(0)));

struct Randy {
  seed: u32,
}

impl Randy {
  fn new(seed: u32) -> Randy {
    Randy { seed }
  }

  fn next(&mut self) -> f64 {
    let x = self.seed as f64;
    let x = x.sin() * 10000.0;
    let x = x - x.floor();
    self.seed += 1;
    x
  }
}

pub struct MRand {
  seed: f64,
}

impl MRand {
  pub fn new(seed: f64) -> MRand {
    let mut seed = seed.floor();

    if seed % 2147483647.0 <= 0.0 {
      seed += 2147483646.0
    }

    MRand { seed }
  }

  fn next_int(&mut self) -> u32 {
    self.seed = (self.seed * 48271.0) % 2147483647.0;

    self.seed as u32
  }

  pub fn next(&mut self) -> f64 {
    ((self.next_int() as f64) - 1.0) / 2147483646.0
  }
}

#[cfg(test)]
mod tests {
  use crate::util::MRand;

  #[test]
  fn test_m_rand() {
    let mut r = MRand::new(0.0);

    assert_eq!(r.next(), 0.9999775220639794);
    assert_eq!(r.next(), 0.9149675508169155);
    assert_eq!(r.next(), 0.39864739440255553);
  }
}
