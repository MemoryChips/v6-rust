use v6::v6_core::App;
mod timer;
#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
  // simple_logger::init_with_level(log::Level::Warn).unwrap();
  simple_logger::init_with_level(log::Level::Info).unwrap();
  warn!("warn from main example");
  info!("info from main example");
  let _t = timer::Timer::new("Main Example");
  let mut app = App::new("The app".to_string());
  info!("App is running: {}", app.is_running());
  app.run();
  info!("App is now running: {}", app.is_running());
}
