type WindowEventRcvr = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;

// trait

pub struct WindowProps {
  title: String,
  w: u32,
  h: u32,
  // vsync: bool,
}

#[allow(dead_code)] // FIXME Remove when possible
pub struct Window {
  props: WindowProps,
  window: glfw::Window,
  minimized: bool,
  events: WindowEventRcvr,
  glfw: glfw::Glfw,
}

// fn glfw_error_callback(err: i32, description: &char) {
//   error!("GLFW Error ({0}): {1}", err, description);
// }

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

impl Drop for Window {
  fn drop(&mut self) {
    info!("Window dropped.");
    unsafe {
      glfw::ffi::glfwTerminate();
    }
  }
}

impl Window {
  // pub fn set_event_callback(&mut self, event_callback: WindowEventRcvr) {
  //   self.event_callback = event_callback;
  // }
  pub fn new(props: WindowProps) -> Self {
    info!(
      "Creating window {0} ({1}, {2})",
      props.title, props.w, props.h
    );
    let (mut glfw, mut window, events) = Window::glfw_init(&props);
    glfw.set_error_callback(glfw::LOG_ERRORS);

    use glfw::Context; // for make_current function
    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    info!("OpenGL Info:");
    let info = get_string(gl::VENDOR);
    info!("  Vendor: {0}", info);
    info!("  Renderer: {0}", get_string(gl::RENDERER));
    info!("  Version: {0}", get_string(gl::VERSION));

    Window {
      props,
      window,
      events,
      glfw,
      minimized: false,
    }
  }
  fn glfw_init(props: &WindowProps) -> (glfw::Glfw, glfw::Window, WindowEventRcvr) {
    // glfw::ffi::
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
}
