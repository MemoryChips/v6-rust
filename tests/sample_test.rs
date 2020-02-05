#[cfg(test)]
mod tests {
  use v6::core::App;
  #[test]
  fn sample_test() {
    let title = "English 日本語 русский язык 官話";
    let w_props = v6::window::WindowProps {
      title: title.to_string(),
      w: 800,
      h: 600,
    };
    let mut app = App::new(title, w_props, 1);
    app.run();
    assert_eq!(app.app_name, title);
    assert!(app.is_running());
  }
}
