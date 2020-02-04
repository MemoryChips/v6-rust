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
  events: WindowEventRcvr,
  glfw: glfw::Glfw,
}

// fn glfw_error_callback(err: i32, description: &char) {
//   error!("GLFW Error ({0}): {1}", err, description);
// }

impl Window {
  // pub fn set_event_callback(&mut self, event_callback: WindowEventRcvr) {
  //   self.event_callback = event_callback;
  // }
  pub fn new(props: WindowProps) -> Self {
    info!(
      "Creating window {0} ({1}, {2})",
      props.title, props.w, props.h
    );
    let (mut glfw, window, events) = Window::glfw_init(&props);
    glfw.set_error_callback(glfw::LOG_ERRORS);
    Window {
      props,
      window,
      events,
      glfw,
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
