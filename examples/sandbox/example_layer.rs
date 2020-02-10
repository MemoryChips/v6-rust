pub struct ExampleLayer {
  debug_name: String,
}
impl ExampleLayer {
  pub fn new() -> Box<Self> {
    Box::new(Self {
      debug_name: "Example Layer".to_string(),
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
