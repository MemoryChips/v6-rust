mod test_layers;
// use test_layers::ExampleLayer;

#[test]
fn sandbox_test() {
  let title = "English 日本語 русский язык 官話";
  let w = 800;
  let h = 600;
  let mut app = v6::App::new(title, w, h, 1);

  let mut shader_lib = v6::shader::ShaderLibrary::new();
  shader_lib.add("simple", "./tests/assets/shaders/simple-shader.glsl");
  // app.run();
  let mut count: i32 = 0;

  // TODO: Copy example app here to make a good test

  while app.running && count < 100 {
    count += 1;
    app.run_loop();
  }
  assert_eq!(app.app_name, title);
  // assert!(false);
}
