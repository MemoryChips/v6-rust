use v6::v6_core::App;
mod timer;
#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
  let _result = simple_logger::init();
  info!("info from main example");
  let _t = timer::Timer::new("Main Example");
  let app = App {
    app_name: "The app".to_string(),
  };
  app.run();
}
