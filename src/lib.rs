pub mod v6_core {
  // #[allow(dead_code)]
  pub struct App {
    pub app_name: String,
  }
  impl App {
    // #[allow(dead_code)]
    pub fn run(&self) {
      println!("App name: {}", self.app_name)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::v6_core::App;
  #[test]
  fn lib_works() {
    let _app = App {
      app_name: "The app".to_string(),
    };
    _app.run();
    assert_eq!(_app.app_name, "The app");
    assert_eq!(2 + 2, 4);
  }
  // #[test]
  // fn fail_test() {
  //   panic!("Intentional Fail")
  // }
}
