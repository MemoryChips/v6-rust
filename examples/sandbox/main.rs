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

#[cfg(test)]
mod tests {
  #[test]
  fn sandbox_layer_test() {
    // // use super::Layer;
    // let test_example_layer = ExampleLayer::new();
    // let test_layer_1 = super::BasicLayer::new("Test Layer 1");
    // let test_layer_2 = super::BasicLayer::new("Test Layer 2");
    // let test_fancy_overlayer_a = FancyLayer::new("Test Overlay A");
    // let test_overlayer_b = super::BasicLayer::new("Test Overlay B");
    // let mut test_layer_stack = super::LayerStack::new();
    // test_layer_stack.push_layer(test_layer_1);
    // test_layer_stack.push_overlay(test_fancy_overlayer_a);
    // test_layer_stack.push_layer(test_layer_2);
    // test_layer_stack.push_overlay(test_overlayer_b);
    // test_layer_stack.push_layer(test_example_layer);
    // test_layer_stack
    //   .layers
    //   .iter()
    //   .for_each(|v| println!("{}", v.get_name()));
    // for v in test_layer_stack.layers.iter() {
    //   println!("{}", v.get_name());
    // }
    // assert_eq!(3, test_layer_stack.layer_insert_index);
    // assert_eq!("Test Layer 2", test_layer_stack.layers[1].get_name());
    // assert_eq!("Example Layer", test_layer_stack.layers[2].get_name());
    assert!(false);
  }
  // impl super::BasicLayer {
  //   pub fn on_attach(&self) {
  //     println!("override on_attach called for layer: {}", self.debug_name)
  //   }
  // }
}
