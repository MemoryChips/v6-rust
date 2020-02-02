type WindowEventRcvr = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;

// trait

pub struct WindowProps {
  title: String,
  w: u32,
  h: u32,
  // vsync: bool,
}

pub struct Window {
  props: WindowProps,
  event_callback: WindowEventRcvr,
}

impl Window {
  pub fn set_event_callback(&mut self, event_callback: WindowEventRcvr) {
    self.event_callback = event_callback;
  }
  pub new(props: WindowProps)->Self {
    let glfw: glfw::Glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Unable to initialize glfw");
Window{props}
  }
}
