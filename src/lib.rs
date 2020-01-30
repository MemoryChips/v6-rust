#[macro_use]
extern crate log;
extern crate glfw;
// use glfw::Glfw;
// use glfw::{Action, Context, Key};
extern crate gl;

pub mod v6_core {
  use gl::types::*;
  use std::ffi::CString;
  use std::mem;
  use std::ptr;
  use std::str;
  // use std::sync::mpsc::Receiver<f64, glfw::WindowEvent> as WindowEvent
  // use std::sync::mpsc::Receiver as WindowEventRcvr;

  // Vertex data
  static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

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

  fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
      shader = gl::CreateShader(ty);
      // Attempt to compile the shader
      let c_str = CString::new(src.as_bytes()).unwrap();
      gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
      gl::CompileShader(shader);
      // Get the compile status
      let mut status = gl::FALSE as GLint;
      gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
      // Fail on error
      if status != (gl::TRUE as GLint) {
        let mut len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = Vec::with_capacity(len as usize);
        buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
        gl::GetShaderInfoLog(
          shader,
          len,
          ptr::null_mut(),
          buf.as_mut_ptr() as *mut GLchar,
        );
        panic!(
          "{}",
          str::from_utf8(&buf)
            .ok()
            .expect("ShaderInfoLog not valid utf8")
        );
      }
    }
    shader
  }
  fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
      let program = gl::CreateProgram();
      gl::AttachShader(program, vs);
      gl::AttachShader(program, fs);
      gl::LinkProgram(program);
      // Get the link status
      let mut status = gl::FALSE as GLint;
      gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
      // Fail on error
      if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = Vec::with_capacity(len as usize);
        buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
        gl::GetProgramInfoLog(
          program,
          len,
          ptr::null_mut(),
          buf.as_mut_ptr() as *mut GLchar,
        );
        panic!(
          "{}",
          str::from_utf8(&buf)
            .ok()
            .expect("ProgramInfoLog not valid utf8")
        );
      }
      program
    }
  }

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

      gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);

      // Create GLSL shaders
      let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
      let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
      let program = link_program(vs, fs);
      let mut vao = 0;
      let mut vbo = 0;

      unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
          gl::ARRAY_BUFFER,
          (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
          mem::transmute(&VERTEX_DATA[0]),
          gl::STATIC_DRAW,
        );
        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());
        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
          pos_attr as GLuint,
          2,
          gl::FLOAT,
          gl::FALSE as GLboolean,
          0,
          ptr::null(),
        );
      }

      self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

      while !self.window.should_close() {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
          App::handle_window_event(&mut self.window, event);
        }
        unsafe {
          // Clear the screen to black
          gl::ClearColor(0.3, 0.3, 0.3, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);
          // Draw a triangle from the 3 vertices
          gl::DrawArrays(gl::TRIANGLES, 0, 3);
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
