#[macro_use]
extern crate log;

pub mod v6_core {
  // #[allow(dead_code)]
  pub struct App {
    pub app_name: String,
    running: bool,
  }
  impl App {
    pub fn run(&mut self) {
      self.running = true;
      println!("secret number: {}", rand::random::<f64>());
      info!("{} is running: {}", self.app_name, self.is_running());
      println!("App name: {}", self.app_name)
    }
    pub fn is_running(&self) -> bool {
      self.running
    }
    pub fn new(name: String) -> App {
      let app = App {
        app_name: name,
        running: false,
      };
      app
    }
  }
}

#[cfg(test)]
mod tests {
  use super::v6_core::App;
  #[test]
  fn lib_works() {
    let mut _app = App::new("The app".to_string());
    _app.run();
    assert_eq!(_app.app_name, "The app");
    assert_eq!(2 + 2, 4);
  }
  // #[test]
  // fn fail_test() {
  //   panic!("Intentional Fail")
  // }
}
