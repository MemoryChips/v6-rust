#[cfg(test)]
mod tests {
  use v6::v6_core::App;
  #[test]
  fn it_works() {
    let _app = App {
      app_name: "The app".to_string(),
    };
    _app.run();
    assert_eq!(_app.app_name, "The app");
  }
  // #[test]
  // fn fail_test() {
  //   panic!("Intentional Fail")
  // }
}
