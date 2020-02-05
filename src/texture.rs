extern crate log;
// use log::{error, info};
extern crate gl;

use gl::types::*;
// use std::ffi::CString;
// use std::mem;
// use std::ptr;
// use std::str;

// pub fn create_texture_from_file(&path: str) {}

#[allow(dead_code)] // TODO Remove when possible
pub struct Texture2D {
  // path: String,
  w: GLsizei,
  h: GLsizei,
  internal_format: GLenum,
  data_format: GLenum,
  renderer_id: u32,
}

impl Texture2D {
  pub fn new(w: i32, h: i32) -> Texture2D {
    let mut renderer_id: u32 = 0;
    let internal_format = gl::RGBA8;
    unsafe {
      // TODO Create test window to test texture setup in opengl
      gl::CreateTextures(gl::TEXTURE_2D, 1, &mut renderer_id);
      gl::TextureStorage2D(renderer_id, 1, internal_format, w, h);
    }
    Texture2D {
      w,
      h,
      internal_format,
      data_format: gl::RGBA,
      renderer_id,
    }
  }
  pub fn get_w(&self) -> i32 {
    self.w
  }
  pub fn get_h(&self) -> i32 {
    self.h
  }
  pub fn area(&self) -> i32 {
    self.w * self.h
  }
}

// #[cfg(test)]
// mod tests {
//   // use v6::v6_core::App;
//   // use v6::texture;
//   // use texture::texture;
//   use super::Texture2D;
//   #[test]
//   fn texture_2d_test() {
//     // gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
//     let tt = Texture2D::new(5, 10);
//     assert_eq!(tt.area(), 50);
//     // assert!(_app.is_running());
//   }
// }
