#[macro_use]
extern crate log;
extern crate gl;
extern crate glam;
extern crate glfw;

pub mod core;
pub mod renderer;
pub mod shader;
pub mod texture;
pub mod timer;
pub mod window;

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
