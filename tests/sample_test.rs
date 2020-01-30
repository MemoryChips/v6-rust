#[cfg(test)]
mod tests {
  use v6::v6_core::App;
  #[test]
  fn sample_test() {
    let mut _app = App::new("The app".to_string());
    _app.run();
    assert_eq!(_app.app_name, "The app");
    assert!(_app.is_running());
  }
  // #[test]
  // fn fail_test() {
  //   panic!("Intentional Fail")
  // }
}
