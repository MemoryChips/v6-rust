// use v6::shader::Shader;

pub struct ExampleLayer {
  debug_name: String,
  // shader: v6::shader::Shader, // CONSIDER: Any reason to keep shader around
}
impl ExampleLayer {
  pub fn new() -> Box<Self> {
    let filepath = "./tests/assets/shaders/simple-shader.glsl";
    let _shader = v6::shader::Shader::new("tri shader", filepath);
    // println!("tri shader / simple loaded: {:?}", &shader);
    Box::new(Self {
      debug_name: "Example Layer".to_string(),
      // shader,
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
  fn on_update(&self, _ts: f64) {
    static mut I: i32 = 0;
    unsafe {
      if I > 10 {
        println!(
          "Example Layer on_update called for layer: {:.1} mS",
          _ts * 1000.0
        );
        I = 0;
      }
      I += 1;
    }
  }
}
