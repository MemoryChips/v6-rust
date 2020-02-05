use v6::core::App;
use v6::timer;
#[macro_use]
extern crate log;
extern crate simple_logger;

const DEMO_VERSION: &str = "1.0.0";

fn main() {
  // simple_logger::init_with_level(log::Level::Info).unwrap();
  // FIXME: move this to lib.rs
  match simple_logger::init_with_level(log::Level::Info) {
    Ok(()) => info!("Logger setup OK"),
    Err(e) => error!("Error setting up logger: {:?}", e),
  };

  let _t = timer::Timer::new(&format!("Main Example, {}, {}", file!(), line!()));
  let w_props = v6::window::WindowProps {
    title: "English 日本語 русский язык 官話".to_string(),
    w: 400,
    h: 400,
  };
  info!("Demo Version: {}", DEMO_VERSION);
  let mut app = App::new("Example App", w_props, 4);
  app.run();
}
