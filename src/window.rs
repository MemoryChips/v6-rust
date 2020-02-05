// TODO: add set viewport as done C++ version

type WindowEventRcvr = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;

pub struct WindowProps {
  pub title: String,
  pub w: u32,
  pub h: u32,
}

#[allow(dead_code)] // TODO Remove when possible
pub struct Window {
  pub props: WindowProps,
  pub window: glfw::Window,
  minimized: bool,
  pub events: WindowEventRcvr,
  pub glfw: glfw::Glfw,
}

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
    // TODO: unable to terminate glfw on window drop
    // unsafe {
    //   glfw::ffi::glfwTerminate();
    // }
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

    // This is a function in the C++ version
    window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

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
