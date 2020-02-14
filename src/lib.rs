#[macro_use]
pub extern crate log;
extern crate gl;
extern crate glam;
extern crate glfw;

// CONSIDER: expose publically only what example needs
mod app;
mod renderer;
pub mod shader;
// pub mod texture;
pub mod layer;
pub mod timer;
pub mod window; // CONSIDER: removing this from pub

// CONSIDER: rename of core.rs
pub use app::App; // This needed crate for some reason

pub use log::Level::{Error, Info, Warn};
pub use log::{error, info, warn};
pub use timer::Timer;

pub fn setup_logger(level: log::Level) {
  match simple_logger::init_with_level(level) {
    Ok(()) => info!("Logger setup OK"),
    Err(e) => error!("Error setting up logger: {:?}", e),
  };
}

// #[cfg(test)]
// mod tests {
//   use super::v6_core::App;
//   #[test]
//   fn lib_works() {
//     let mut _app = App::new("The app".to_string());
//     _app.run();
//     assert_eq!(_app.app_name, "The app");
//     assert_eq!(2 + 2, 4);
//   }
// #[test]
// fn fail_test() {
//   panic!("Intentional Fail")
// }
// }
