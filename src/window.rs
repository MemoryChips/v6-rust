// type WindowEventRcvr = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;

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
  // event_callback: WindowEventRcvr,
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
    let mut glfw: glfw::Glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Unable to initialize glfw"); // TODO: use OK
    glfw.set_error_callback(glfw::LOG_ERRORS);
    Window { props }
  }
}
