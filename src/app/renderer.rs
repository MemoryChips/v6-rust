use glam::{mat4, vec4, Mat4};

#[allow(dead_code)]
pub struct SceneData {
  // pub d: u32,
  view_projection_matrix: Mat4,
}
#[allow(dead_code)]
pub fn create_mat4_f(f: f32) -> Mat4 {
  let v = vec4(f, f, f, f);
  mat4(v, v, v, v)
}

pub mod api {
  use glam::Vec4;
  pub unsafe fn init() {
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    // gl::Enable(gl::DEPTH_TEST);
  }
  #[allow(dead_code)]
  pub unsafe fn set_viewport(x: i32, y: i32, w: i32, h: i32) {
    gl::Viewport(x, y, w, h);
  }
  pub unsafe fn set_clear_color(color: &Vec4) {
    gl::ClearColor(color.x(), color.y(), color.z(), color.w());
  }
  pub unsafe fn clear() {
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
  }
}

// void OpenGLRendererAPI::DrawIndexed(const Ref<VertexArray> &vertexArray) {
// glDrawElements(GL_TRIANGLES, vertexArray->GetIndexBuffer()->GetCount(),
// GL_UNSIGNED_INT, nullptr);
// // glBindTexture(GL_TEXTURE_2D, 0); // unbind texture
// }

#[cfg(test)]
mod tests {
  use super::SceneData;
  #[test]
  fn render_debug_test() {
    let scene_data = SceneData {
      view_projection_matrix: super::create_mat4_f(1.0),
      // view_projection_matrix: glm::mat4(m11: f32, m21: f32, m31: f32, m41: f32, m12: f32, m22: f32, m32: f32, m42: f32, m13: f32, m23: f32, m33: f32, m43: f32, m14: f32, m24: f32, m34: f32, m44: f32),
    };
    assert_eq!(scene_data.view_projection_matrix, super::create_mat4_f(1.0));
  }
}
