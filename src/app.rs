use crate::layer::LayerStack;

use std::str;

use crate::render_command::RenderCommand;

mod renderer;
mod window;
use window::*;

pub struct App {
  pub app_name: String,
  // imGuiLayer: ImGuiLayer,
  pub running: bool,
  last_frame_time_sec: f64,
  pub time_step: f64,
  pub duration_secs: u64, // Eventually remove when app runs in its own thread OR stop runs in its own thread
  pub layer_stack: LayerStack,
  pub window: Window,
  // pub shader_lib: ShaderLibrary,
}
impl App {
  pub fn update_time_step(&mut self) {
    unsafe {
      let time = glfw::ffi::glfwGetTime();
      self.time_step = time - self.last_frame_time_sec;
      self.last_frame_time_sec = time;
    }
  }
  pub fn run_loop(&mut self) {
    self.update_time_step();

    self.layer_stack.layers.iter().for_each(|l| {
      l.command_list.iter().for_each(|c| match c {
        RenderCommand::DrawTri => renderer::api::draw_tri(),
        RenderCommand::Clear => renderer::api::clear(),
        RenderCommand::SetClearColor { color } => renderer::api::set_clear_color(color),
        _ => println!("Got an unknown render command",),
      })
    });

    self.window.glfw.poll_events(); // CONSIDER: move this when onUpdate is created
    for (_, event) in glfw::flush_messages(&self.window.events) {
      App::handle_window_event(&mut self.window.window, event);
    }
    if self.window.window.should_close() {
      self.running = false;
      // break;
    }
    use glfw::Context; // needed for next line
    self.window.window.swap_buffers();
  }

  // CONSIDER: move this to Window.rs? Yes when OnUpdate is added
  fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
      glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
        window.set_should_close(true)
      }
      _ => {}
    }
  }
  pub fn stop(&mut self) {
    self.window.window.set_should_close(true);
  }
  pub fn new(app_name: &str, w: u32, h: u32, duration_secs: u64) -> App {
    let props = WindowProps {
      title: app_name.to_string(),
      w,
      h,
    };
    let window = Window::new(props);
    let app = App {
      app_name: app_name.to_string(),
      running: true,
      last_frame_time_sec: 0.0,
      time_step: 0.0,
      duration_secs,
      window,
      layer_stack: LayerStack::new(),
      // shader_lib: ShaderLibrary::new(),
    };

    unsafe {
      renderer::api::init();
    } // CONSIDER: checking success

    app
  }
}
impl Drop for App {
  fn drop(&mut self) {
    info!("Destructor for App <{}>  called", self.app_name);
  }
}
