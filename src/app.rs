use crate::layer::LayerStack;
// use crate::timer::Timer;
use glam::vec4;
use std::str;

mod renderer;
mod window;
use window::*;

pub struct App {
  pub app_name: String,
  // imGuiLayer: ImGuiLayer,
  running: bool,
  last_frame_time_sec: f64,
  time_step: f64,
  duration_secs: u64, // Eventually remove when app runs in its own thread OR stop runs in its own thread
  pub layer_stack: LayerStack,
  pub window: Window,
}
impl App {
  pub fn run(&mut self) {
    // self.running = true;

    use std::time::Instant;
    let duration = std::time::Duration::from_secs(self.duration_secs);
    let stop_time = Instant::now() + duration;
    let mut count_down = 0;

    while self.running {
      let time_step: f64;
      unsafe {
        let time = glfw::ffi::glfwGetTime();
        time_step = time - self.last_frame_time_sec;
        self.last_frame_time_sec = time;
      }
      if count_down == 0 {
        info!("Frame time_step: {}", time_step);
        count_down = 20;
      }
      count_down -= 1;
      if self.duration_secs != 0 && stop_time < Instant::now() {
        self.running = false;
        // break;
      }
      self.run_loop(0.0);
      // unsafe {
      //   renderer::api::set_clear_color(&vec4(0.3, 0.7, 0.3, 1.0));
      //   renderer::api::clear();
      //   gl::DrawArrays(gl::TRIANGLES, 0, 3);
      // }
      // self
      //   .layer_stack
      //   .layers
      //   .iter()
      //   .for_each(|s| println!("{}", s.name));
      // // TODO: Add timestep thing
      // // .for_each(|s| s.on_update(time_step)); // TODO: Add timestep thing
      // self.window.glfw.poll_events(); // CONSIDER: move this when onUpdate is created
      // for (_, event) in glfw::flush_messages(&self.window.events) {
      //   App::handle_window_event(&mut self.window.window, event);
      // }
      // if self.window.window.should_close()
      //   || (self.duration_secs != 0 && stop_time < Instant::now())
      // {
      //   self.running = false;
      //   // break;
      // }
      // // CONSIDER: move this to window.rs? Yes when OnUpdate is added
      // use glfw::Context; // needed for next line
      // self.window.window.swap_buffers();
    }
  }
  pub fn update_time_step(&mut self) {
    unsafe {
      let time = glfw::ffi::glfwGetTime();
      self.time_step = time - self.last_frame_time_sec;
      self.last_frame_time_sec = time;
    }
  }
  pub fn run_loop(&mut self, _time_step: f64) {
    unsafe {
      renderer::api::set_clear_color(&vec4(0.3, 0.7, 0.3, 1.0));
      renderer::api::clear();
      gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
    self
      .layer_stack
      .layers
      .iter()
      .for_each(|s| println!("{}", s.name));
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
  // pub fn is_running(&self) -> bool {
  //   self.running
  // }
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
    };

    unsafe {
      renderer::api::init();
    } // CONSIDER: checking success

    app
  }
}
impl Drop for App {
  fn drop(&mut self) {
    println!("<{}> destructor for App called", self.app_name);
  }
}
