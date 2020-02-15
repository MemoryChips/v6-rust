mod example_layer;

const DEMO_VERSION: &str = "1.0.0";

fn main() {
  v6::setup_logger(v6::Info);

  let _t = v6::Timer::new(&format!("Main Example, {}, {}", file!(), line!()));
  let title = "English 日本語 русский язык 官話";
  let w = 400;
  let h = 400;
  v6::info!("Demo Version: {}", DEMO_VERSION);

  let mut app = v6::App::new(title, w, h, 2);

  let layer_one = example_layer::ExampleLayer::new();
  app.layer_stack.push_layer(layer_one);

  app.run();
}
