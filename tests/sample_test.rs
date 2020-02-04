#[cfg(test)]
mod tests {
  use v6::v6_core::App;
  #[test]
  fn sample_test() {
    let title = "English 日本語 русский язык 官話";
    let w_props = v6::window::WindowProps {
      title: title.to_string(),
      w: 800,
      h: 600,
    };
    let mut _app = App::new(w_props, 1);
    _app.run();
    assert_eq!(_app.app_name, "Main App"); // TODO: make this settable
    assert!(_app.is_running());
  }
}
