use crate::shader;
use crate::timer::Timer;
use crate::window::*;
// use super::shader;
// use super::timer::Timer;
// use super::window::*;
use std::str;

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

pub struct App {
  pub app_name: String,
  // imGuiLayer: ImGuiLayer,
  running: bool,
  last_frame_time_sec: f64,
  duration_secs: u64, // Eventually remove when app runs in its own thread OR stop runs in its own thread
  // layerStack: LayerStack,
  pub window: Window,
}
impl App {
  // TODO: Move this to a better place
  pub fn get_string(which: gl::types::GLenum) -> String {
    unsafe {
      let data = std::ffi::CStr::from_ptr(gl::GetString(which) as *const _)
        .to_bytes()
        .to_vec();
      match String::from_utf8(data) {
        Ok(info) => info,
        Err(_e) => "".to_string(),
      }
    }
  }
  pub fn run(&mut self) {
    let _t = Timer::new("Run time");
    self.running = true;
    info!("App name: {}", self.app_name);

    // TODO: move this to window.rs?
    use glfw::Context; // for make_current function
    self.window.window.set_key_polling(true);
    self.window.window.make_current();
    gl::load_with(|symbol| self.window.window.get_proc_address(symbol) as *const _);

    info!("OpenGL Info:");
    let info = App::get_string(gl::VENDOR);
    info!("  Vendor: {0}", info);
    info!("  Renderer: {0}", App::get_string(gl::RENDERER));
    info!("  Version: {0}", App::get_string(gl::VERSION));

    let _tri_shader = shader::Shader::new("tri shader", VS_SRC, FS_SRC);
    // TODO: Move to window.rs?
    self
      .window
      .glfw
      .set_swap_interval(glfw::SwapInterval::Sync(1));

    use std::time::Instant;
    let duration = std::time::Duration::from_secs(self.duration_secs);
    let stop_time = Instant::now() + duration;

    loop {
      self.window.glfw.poll_events();
      for (_, event) in glfw::flush_messages(&self.window.events) {
        App::handle_window_event(&mut self.window.window, event);
      }
      unsafe {
        gl::ClearColor(0.3, 0.0, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        // Draw a triangle from the 3 vertices
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
      }
      if self.window.window.should_close()
        || (self.duration_secs != 0 && stop_time < Instant::now())
      {
        break;
      }
      self.window.window.swap_buffers();
    }
  }
  // TODO: move this to Window.rs?
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
    self.window.window.set_should_close(true);
  }
  pub fn new(props: WindowProps, duration_secs: u64) -> App {
    // let (glfw, window, events) = App::glfw_init(&props);
    let window = Window::new(props);
    let app = App {
      app_name: "Main App".to_string(),
      running: false,
      last_frame_time_sec: 0.0,
      duration_secs,
      window,
    };
    app
  }
}
impl Drop for App {
  fn drop(&mut self) {
    println!("<{}> destructor for App called", self.app_name);
  }
}
