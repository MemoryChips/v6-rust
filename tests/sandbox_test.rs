mod test_layers;
// use test_layers::ExampleLayer;

#[test]
fn sandbox_test() {
  let title = "English 日本語 русский язык 官話";
  let w = 800;
  let h = 600;
  let mut app = v6::App::new(title, w, h, 1);

  // let layer_one = ExampleLayer::new();
  // app.layer_stack.push_layer(layer_one);

  app.run();
  assert_eq!(app.app_name, title);
  assert!(app.is_running());
  // assert!(false);
}
