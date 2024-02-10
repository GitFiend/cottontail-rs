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
