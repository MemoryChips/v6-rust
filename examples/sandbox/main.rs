use v6::layer;
use v6::timer::Timer;

const DEMO_VERSION: &str = "1.0.0";

fn main() {
  v6::setup_logger(v6::Info);

  let _t = v6::Timer::new(&format!("Main Example, {}, {}", file!(), line!()));
  let title = "English 日本語 русский язык 官話";
  let w = 400;
  let h = 400;
  v6::info!("Demo Version: {}", DEMO_VERSION);

  let mut app = v6::App::new(title, w, h, 2);
  v6::info!("App name: {}", app.app_name);

  // CONSIDER: move shader library setup inside app
  let mut shader_lib = v6::shader::ShaderLibrary::new();
  shader_lib.add(
    "simple",
    "./examples/sandbox/assets/shaders/simple-shader.glsl",
  );
  // shader_lib.add("simple", "./tests/assets/shaders/simple-shader.glsl");

  let layer_one = layer::Layer::new();
  app.layer_stack.push_layer(layer_one);
  {
    let _t = Timer::new("Run time");
    app.run();
  }
}
