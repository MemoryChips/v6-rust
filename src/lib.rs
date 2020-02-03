#[macro_use]
extern crate log;
extern crate gl;
extern crate glam;
extern crate glfw;

mod timer;
// use timer::Timer;
pub mod renderer;
mod shader;
pub mod texture;

pub mod v6_core {
  use super::shader;
  use super::timer::Timer;
  use std::str;

  pub struct WindowProps {
    pub title: String,
    pub w: u32,
    pub h: u32,
    // vsync: bool,
  }
  // Shader sources
  static VS_SRC: &'static str = "
#version 150
in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}";

  static FS_SRC: &'static str = "
#version 150
out vec4 out_color;

void main() {
    out_color = vec4(1.0, 1.0, 0.0, 1.0);
}";

  type WindowEventRcvr = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;
  #[allow(dead_code)] // FIXME Remove when possible
  pub struct App {
    pub app_name: String,
    pub window: glfw::Window,
    // imGuiLayer: ImGuiLayer,
    running: bool,
    minimized: bool,
    // layerStack: LayerStack,
    last_frame_time_sec: f64,
    duration_secs: u64, // Eventually remove when app runs in its own thread OR stop runs in its own thread
    events: WindowEventRcvr,
    glfw: glfw::Glfw,
  }
  impl App {
    pub fn get_string(which: gl::types::GLenum) -> String {
      let info = unsafe {
        let data = std::ffi::CStr::from_ptr(gl::GetString(which) as *const _)
          .to_bytes()
          .to_vec();
        String::from_utf8(data).unwrap()
      };
      info
    }
    pub fn run(&mut self) {
      let _t = Timer::new("Run time");
      use crate::glfw::Context; // for make_current function
      self.running = true;
      // info!("secret number: {}", rand::random::<f64>());
      // info!("{} is running: {}", self.app_name, self.is_running());
      info!("App name: {}", self.app_name);
      self.window.set_key_polling(true);
      self.window.make_current();

      gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
      // info!("OpenGL Info:");
      let info = App::get_string(gl::VENDOR);
      // let info = gl::GetString(gl::VENDOR) as *mut char;
      // let error_info = (*info).to_string();
      info!("  Vendor: {:?}", info);
      // info!("  Vendor: {0}", String from gl::GetString(gl::VENDOR));
      // info!("  Renderer: {0}", glGetString(GL_RENDERER));
      // info!("  Version: {0}", glGetString(GL_VERSION));
      let _tri_shader = shader::Shader::new("tri shader", VS_SRC, FS_SRC);
      self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

      use std::time::Instant;
      let duration = std::time::Duration::from_secs(self.duration_secs);
      let stop_time = Instant::now() + duration;

      loop {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
          App::handle_window_event(&mut self.window, event);
        }
        unsafe {
          gl::ClearColor(0.3, 0.0, 0.3, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);
          // Draw a triangle from the 3 vertices
          gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        if self.window.should_close() || (self.duration_secs != 0 && stop_time < Instant::now()) {
          break;
        }
        self.window.swap_buffers();
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
    pub fn stop(&mut self) {
      self.window.set_should_close(true);
    }
    pub fn new(props: WindowProps, duration_secs: u64) -> App {
      let (glfw, window, events) = App::glfw_init(&props);
      let app = App {
        app_name: props.title,
        running: false,
        minimized: false,
        last_frame_time_sec: 0.0,
        duration_secs,
        window,
        events,
        glfw,
      };
      app
    }
    fn glfw_init(props: &WindowProps) -> (glfw::Glfw, glfw::Window, WindowEventRcvr) {
      let glfw: glfw::Glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Unable to initialize glfw");
      let (window, window_events) = glfw
        .create_window(
          props.w,
          props.h,
          &props.title,
          // "English 日本語 русский язык 官話",
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
