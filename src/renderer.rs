use glam::*;

#[allow(dead_code)]
pub struct SceneData {
  // pub d: u32,
  view_projection_matrix: Mat4,
}
pub fn create_mat4_f(f: f32) -> Mat4 {
  let v = vec4(f, f, f, f);
  mat4(v, v, v, v)
}

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
