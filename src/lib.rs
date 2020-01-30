#[macro_use]
extern crate log;
extern crate glfw;
// use glfw::Glfw;
// use glfw::{Action, Context, Key};

pub mod v6_core {
  // use std::sync::mpsc::Receiver<f64, glfw::WindowEvent> as WindowEvent
  // use std::sync::mpsc::Receiver as WindowEventRcvr;
  type WindowEventRcvr = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;
  #[allow(dead_code)] // FIXME Remove when possible
  pub struct App {
    pub app_name: String,
    window: glfw::Window,
    // imGuiLayer: ImGuiLayer,
    running: bool,
    minimized: bool,
    // layerStack: LayerStack,
    last_frame_time_sec: f64,
    events: WindowEventRcvr,
    glfw: glfw::Glfw,
  }
  impl App {
    pub fn run(&mut self) {
      use crate::glfw::Context; // for make_current function
      self.running = true;
      info!("secret number: {}", rand::random::<f64>());
      info!("{} is running: {}", self.app_name, self.is_running());
      info!("App name: {}", self.app_name);
      self.window.set_key_polling(true);
      self.window.make_current();
      self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

      while !self.window.should_close() {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
          App::handle_window_event(&mut self.window, event);
        }
      }
    }
    fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
          window.set_should_close(true)
        }
        _ => {}
      }
    }
    pub fn is_running(&self) -> bool {
      self.running
    }
    pub fn new(name: String) -> App {
      let (glfw, window, events) = App::glfw_init();
      let app = App {
        app_name: name,
        running: false,
        minimized: false,
        last_frame_time_sec: 0.0,
        window,
        events,
        glfw,
      };
      app
    }
    fn glfw_init() -> (glfw::Glfw, glfw::Window, WindowEventRcvr) {
      // let glfw: Glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Unable to initialize glfw");
      let glfw: glfw::Glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Unable to initialize glfw");
      let (window, window_events) = glfw
        .create_window(
          400,
          400,
          "English 日本語 русский язык 官話",
          glfw::WindowMode::Windowed,
        )
        .expect("Unable to create window");
      (glfw, window, window_events)
    }
    // pub fn on_event(&self, &e Event) {}
    // pub fn push_layer(&self, &layer Layer) {}
    // pub fn push_overlay(&self, &layer Layer) {}
    // pub fn get_window(&self) -> &Window {
    //   &self.window
    // }
    // fn onWindowClosed(&self, WindowCloseEvent &e) {}
    // bool onWindowResize(&self, WindowResizeEvent &e) {}
    // bool onKeyReleased([[maybe_unused]] KeyReleasedEvent &e) {}
  }
  impl Drop for App {
    fn drop(&mut self) {
      println!("<{}> destructor for App called", self.app_name);
    }
  }
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
