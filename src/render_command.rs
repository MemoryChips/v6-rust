use glam::Vec4;
// use glam::{mat4, vec4, Mat4, Vec4};

pub enum RenderCommand {
  Clear,
  SetClearColor { color: Vec4 },
  DrawTri,
  // DrawIndexed(vertex_array: VertexArray),
  SetViewport { x: u32, y: u32, w: u32, h: u32 },
}

#[cfg(test)]
mod tests {
  use super::RenderCommand;
  struct FakeRend {
    rend_type: String,
  }
  impl FakeRend {
    fn new() -> Self {
      FakeRend {
        rend_type: "FakeRend".to_string(),
      }
    }
    fn run(&self, c: &RenderCommand) {
      match c {
        RenderCommand::Clear => assert_eq!("FakeRend", self.rend_type),
        RenderCommand::DrawTri => assert_eq!("FakeRend", self.rend_type),
        _ => assert!(false, "Got unknown render command"),
      }
    }
  }

  #[test]
  fn render_command_test() {
    let rend = FakeRend::new();
    let com1 = RenderCommand::Clear;
    let com2 = RenderCommand::DrawTri;
    let command_list: Vec<RenderCommand> = vec![com1, com2];
    command_list.iter().for_each(|c| rend.run(c));
  }
}
