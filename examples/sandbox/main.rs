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
  //   Ok(n) => println!("Logger setup OK"),
  //   Err(e) => println!("Error setting up logger: {:?}", e),
  // };

  let _t = timer::Timer::new(&format!("Main Example, {}, {}", file!(), line!()));
  let w_props = v6::v6_core::WindowProps {
    title: "English 日本語 русский язык 官話".to_string(),
    w: 400,
    h: 400,
  };
  let mut app = App::new(w_props, 2);
  app.run();
  info!("App is now running: {}", app.is_running());
  // let one_sec = std::time::Duration::from_millis(1000);
  // std::thread::sleep(one_sec);
  // app.stop();
}
