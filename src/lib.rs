#[macro_use]
pub extern crate log;
extern crate gl;
extern crate glam;
extern crate glfw;

// CONSIDER: expose publically only what example needs
mod app;
// mod renderer;
pub mod shader;
// pub mod texture;
pub mod layer;
pub mod timer;
// pub mod window; // CONSIDER: removing this from pub

pub use app::App;

pub use log::Level::{Error, Info, Warn};
pub use log::{error, info, warn};
pub use timer::Timer;

pub fn setup_logger(level: log::Level) {
  match simple_logger::init_with_level(level) {
    Ok(()) => info!("Logger setup OK"),
    Err(e) => error!("Error setting up logger: {:?}", e),
  };
}
