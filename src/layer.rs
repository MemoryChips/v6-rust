use crate::render_command::RenderCommand;

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
  pub fn push_layer(&mut self, layer: Layer) -> usize {
    self.layers.insert(self.layer_insert_index, layer);
    self.layer_insert_index += 1;
    self.layer_insert_index - 1
  }
  pub fn push_overlay(&mut self, layer: Layer) -> usize {
    self.layers.push(layer);
    self.layers.len()
  }
  //   void popLayer(Layer *layer);
  //   void popOverlay(Layer *overlay);
}
impl Drop for LayerStack {
  fn drop(&mut self) {
    for l in self.layers.iter() {
      l.on_detach();
    }
  }
}

pub struct Layer {
  pub name: String,
  pub command_list: Vec<RenderCommand>,
}
impl Layer {
  pub fn new() -> Self {
    Self {
      name: "Layer Name Here".to_string(),
      command_list: Vec::new(),
    }
  }
  pub fn new_named(name: &str) -> Self {
    Self {
      name: name.to_string(),
      command_list: Vec::new(),
    }
  }
  pub fn on_detach(&self) {
    println!("Layer detached: {}", self.name)
  }
}

#[cfg(test)]
mod tests {
  use super::{Layer, LayerStack};
  #[test]
  fn layer_insert_test() {
    let test_layer_1 = Layer::new_named("Test Layer 1");
    let test_layer_2 = Layer::new_named("Test Layer 2");
    let test_layer_3 = Layer::new_named("Test Layer 3");
    let test_layer_overlay = Layer::new_named("Test Layer Overlay");
    let mut test_layer_stack = LayerStack::new();
    test_layer_stack.push_layer(test_layer_1);
    test_layer_stack.push_layer(test_layer_2);
    test_layer_stack.push_overlay(test_layer_overlay);
    test_layer_stack.push_layer(test_layer_3);
    test_layer_stack
      .layers
      .iter()
      .for_each(|v| println!("{}", v.name));
    assert_eq!(3, test_layer_stack.layer_insert_index);
    assert_eq!(4, test_layer_stack.layers.len());
    assert_eq!("Test Layer 2", test_layer_stack.layers[1].name);
    assert_eq!("Test Layer 3", test_layer_stack.layers[2].name);
    assert_eq!("Test Layer Overlay", test_layer_stack.layers[3].name);
  }
}
