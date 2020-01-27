use std::time::Instant;
pub struct Timer {
  start_time: Instant,
  name: String,
}

impl Timer {
  pub fn new(n: &str) -> Timer {
    Timer {
      start_time: Instant::now(),
      name: n.to_string(),
    }
  }
}

impl Drop for Timer {
  fn drop(&mut self) {
    let d = Instant::now().duration_since(self.start_time);
    println!("<{}> elapsed time uS: {}", self.name, d.as_micros());
  }
}
