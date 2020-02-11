// use v6::shader::Shader;

pub struct ExampleLayer {
  debug_name: String,
  shader: v6::shader::Shader,
}
impl ExampleLayer {
  pub fn new() -> Box<Self> {
    let filepath = "./tests/assets/shaders/simple-shader.glsl";
    let shader = v6::shader::Shader::new("tri shader", filepath);
    Box::new(Self {
      debug_name: "Example Layer".to_string(),
      shader,
    })
  }
}
impl v6::layer::Layer for ExampleLayer {
  fn get_name(&self) -> &String {
    &self.debug_name
  }
  fn on_attach(&self) {
    println!(
      "Fancy override on_attach called for layer: {}",
      "Example Layer"
    )
  }
}
