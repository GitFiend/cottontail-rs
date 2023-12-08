// language=CSS prefix=body{ suffix=}
const OMG: &str = r#"
   border: 1px solid #000;
"#;

macro_rules! style {
  ($s: tt) => {
    // language=CSS prefix=body{ suffix=}
    r#"$s"#
  };
}

/*


div()
  .style().position(Absolute)
  .children([
    div().into(),
    span().into(),
  ]).into()


 */

// fn test() {
//   let s = style! {
//     height: 100px;
//   };
// }
