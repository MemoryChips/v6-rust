use v6::v6_core::App;
mod timer;
#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
  simple_logger::init_with_level(log::Level::Info).unwrap();
  // match simple_logger::init_with_level(log::Level::Info) {
  //   // Ok(n) => n,
  //   // Err(e) => 0,
  //   Ok(n) => println!("Logger setup OK code n: {:?}", n),
  //   Err(e) => println!("Error setting up logger: {:?}", e),
  // };

  warn!("warn from main example");
  info!("info from main example");
  let _t = timer::Timer::new("Main Example");
  let mut app = App::new("The app".to_string());
  info!("App is running: {}", app.is_running());
  app.run();
  info!("App is now running: {}", app.is_running());
}
