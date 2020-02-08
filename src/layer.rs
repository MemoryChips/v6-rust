pub struct LayerStack {
  pub layers: Vec<Layer>,
  layer_insert_index: usize,
}

impl LayerStack {
  pub fn new() -> Self {
    Self {
      layers: Vec::new(),
      layer_insert_index: 0,
    }
  }
  pub fn push_layer(&mut self, layer: Layer) {
    layer.on_attach();
    self.layers.insert(self.layer_insert_index, layer);
    self.layer_insert_index += 1;
  }
  pub fn push_overlay(&mut self, layer: Layer) {
    layer.on_attach();
    self.layers.push(layer);
  }
  //   void pushOverlay(Layer *overlay);
  //   void popLayer(Layer *layer);
  //   void popOverlay(Layer *overlay);
}
pub struct Layer {
  pub debug_name: String,
}

impl Layer {
  pub fn new(debug_name: &str) -> Self {
    Layer {
      debug_name: debug_name.to_string(),
    }
  }
  // pub fn on_attach(&self) {
  //   println!("default on_attach called for layer: {}", self.debug_name)
  // }
  // virtual void onDetach() {}
  // virtual void onUpdate([[maybe_unused]] Timestep tf) {}
  // virtual void onImGuiRender() {}
  // virtual void onEvent([[maybe_unused]] Event &event) {}
  // pub fn get_name(&self) -> &String {
  //   &self.debug_name
  // }
}

trait Attachable {
  fn get_name(&self) -> &String;
  fn on_attach(&self) {
    println!("default on_attach called for layer: {}", self.get_name());
  }
}

impl Attachable for Layer {
  fn get_name(&self) -> &String {
    &self.debug_name
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn layer_insert_test() {
    use super::Attachable;
    let test_layer_1 = super::Layer::new("Test Layer 1");
    let test_layer_2 = super::Layer::new("Test Layer 2");
    let test_overlayer_a = super::Layer::new("Test Overlay A");
    let test_overlayer_b = super::Layer::new("Test Overlay B");
    let mut test_layer_stack = super::LayerStack::new();
    test_layer_stack.push_layer(test_layer_1);
    test_layer_stack.push_overlay(test_overlayer_a);
    test_layer_stack.push_layer(test_layer_2);
    test_layer_stack.push_overlay(test_overlayer_b);
    for v in test_layer_stack.layers.iter() {
      println!("{}", v.get_name());
    }
    assert_eq!(2, test_layer_stack.layer_insert_index);
    assert_eq!("Test Layer 2", test_layer_stack.layers[1].get_name());
    assert!(false);
  }
  // impl super::Layer {
  //   pub fn on_attach(&self) {
  //     println!("override on_attach called for layer: {}", self.debug_name)
  //   }
  // }
}
