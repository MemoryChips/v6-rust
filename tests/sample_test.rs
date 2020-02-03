#[cfg(test)]
mod tests {
  use v6::v6_core::App;
  #[test]
  fn sample_test() {
    let title = "English 日本語 русский язык 官話";
    let w_props = v6::v6_core::WindowProps {
      title: title.to_string(),
      w: 800,
      h: 600,
    };
    let mut _app = App::new(w_props, 1);
    _app.run();
    assert_eq!(_app.app_name, title);
    assert!(_app.is_running());
  }
  // use v6::renderer::SceneData;
  // #[test]
  // fn renderer_test() {
  //   let s = SceneData { d: 26 };
  //   assert_eq!(s.d, 26);
  // }
}
