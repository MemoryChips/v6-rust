mod example_layer;

const DEMO_VERSION: &str = "1.0.0";

fn main() {
  v6::setup_logger(v6::Info);

  let _t = v6::Timer::new(&format!("Main Example, {}, {}", file!(), line!()));
  let w_props = v6::window::WindowProps {
    title: "English 日本語 русский язык 官話".to_string(),
    w: 400,
    h: 400,
  };
  v6::info!("Demo Version: {}", DEMO_VERSION);

  let layer_one = example_layer::ExampleLayer::new();

  let mut app = v6::App::new("Example App", w_props, 2);
  app.layer_stack.push_layer(layer_one);
  app.run();
}
