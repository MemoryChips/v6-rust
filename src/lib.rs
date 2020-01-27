// use simple_logger::{info, trace, warn};
#[macro_use]
extern crate log;
// use log::info;
// use log::{info, trace, warn};
// extern crate rand;
// use rand::prelude::*;
// extern crate simple_logger;

pub mod v6_core {
  // #[allow(dead_code)]
  pub struct App {
    pub app_name: String,
  }
  impl App {
    // #[allow(dead_code)]
    pub fn run(&self) {
      println!("secret number: {}", rand::random::<f64>());
      info!("{} is running", self.app_name);
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
