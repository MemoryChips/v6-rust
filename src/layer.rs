pub struct LayerStack {
  pub layers: Vec<Box<dyn Layer>>,
  layer_insert_index: usize,
}

impl LayerStack {
  pub fn new() -> Self {
    Self {
      layers: Vec::new(),
      layer_insert_index: 0,
    }
  }
  pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
    layer.on_attach();
    self.layers.insert(self.layer_insert_index, layer);
    self.layer_insert_index += 1;
  }
  pub fn push_overlay(&mut self, layer: Box<dyn Layer>) {
    layer.on_attach();
    self.layers.push(layer);
  }
  //   void popLayer(Layer *layer);
  //   void popOverlay(Layer *overlay);
}
impl Drop for LayerStack {
  fn drop(&mut self) {
    for v in self.layers.iter() {
      v.on_detach();
    }
  }
}

pub struct BasicLayer {
  pub debug_name: String,
}

impl BasicLayer {
  pub fn new(debug_name: &str) -> Box<Self> {
    Box::new(Self {
      debug_name: debug_name.to_string(),
    })
  }
}

pub trait Layer {
  fn get_name(&self) -> &String;
  fn on_attach(&self) {
    println!("default on_attach called for layer: {}", self.get_name());
  }
  fn on_detach(&self) {
    println!("default on_detach called for layer: {}", self.get_name());
  }
  // virtual void onUpdate([[maybe_unused]] Timestep tf) {}
  // virtual void onImGuiRender() {}
  // virtual void onEvent([[maybe_unused]] Event &event) {}
}

impl Layer for BasicLayer {
  fn get_name(&self) -> &String {
    &self.debug_name
  }
}

pub struct FancyLayer {
  pub debug_name: String,
}

impl FancyLayer {
  pub fn new(debug_name: &str) -> Box<Self> {
    Box::new(Self {
      debug_name: debug_name.to_string() + " - Fancy",
    })
  }
}

impl Layer for FancyLayer {
  fn get_name(&self) -> &String {
    // &String::new("Fancy")
    &self.debug_name
  }
  fn on_attach(&self) {
    println!(
      "Fancy override on_attach called for layer: {}",
      self.debug_name
    )
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn layer_insert_test() {
    // use super::Layer;
    let test_layer_1 = super::BasicLayer::new("Test Layer 1");
    let test_layer_2 = super::BasicLayer::new("Test Layer 2");
    let test_fancy_overlayer_a = super::FancyLayer::new("Test Overlay A");
    let test_overlayer_b = super::BasicLayer::new("Test Overlay B");
    let mut test_layer_stack = super::LayerStack::new();
    test_layer_stack.push_layer(test_layer_1);
    test_layer_stack.push_overlay(test_fancy_overlayer_a);
    test_layer_stack.push_layer(test_layer_2);
    test_layer_stack.push_overlay(test_overlayer_b);
    for v in test_layer_stack.layers.iter() {
      println!("{}", v.get_name());
    }
    assert_eq!(2, test_layer_stack.layer_insert_index);
    assert_eq!("Test Layer 2", test_layer_stack.layers[1].get_name());
    assert!(false);
  }
  // impl super::BasicLayer {
  //   pub fn on_attach(&self) {
  //     println!("override on_attach called for layer: {}", self.debug_name)
  //   }
  // }
}
