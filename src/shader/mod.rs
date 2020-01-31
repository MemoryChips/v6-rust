// #[macro_use]
// extern crate log;
// extern crate glfw;
// use glfw::Glfw;
// use glfw::{Action, Context, Key};
extern crate gl;

use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

pub struct Shader {
  pub name: String, // TODO: SHould this be public?
  pub renderer_id: u32,
}

impl Shader {
  pub fn new(name: &str, vertex_src: &str, fragment_src: &str) -> Self {
    let vs = compile_shader(vertex_src, gl::VERTEX_SHADER);
    let fs = compile_shader(fragment_src, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);
    let mut vao = 0;
    let mut vbo = 0;
    const VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5]; // FIXME: pass this into new
    unsafe {
      // Create Vertex Array Object
      gl::GenVertexArrays(1, &mut vao);
      gl::BindVertexArray(vao);
      // Create a Vertex Buffer Object and copy the vertex data to it
      gl::GenBuffers(1, &mut vbo);
      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        mem::transmute(&VERTEX_DATA[0]),
        gl::STATIC_DRAW,
      );
      // Use shader program
      gl::UseProgram(program);
      gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());
      // Specify the layout of the vertex data
      let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
      gl::EnableVertexAttribArray(pos_attr as GLuint);
      gl::VertexAttribPointer(
        pos_attr as GLuint,
        2,
        gl::FLOAT,
        gl::FALSE as GLboolean,
        0,
        ptr::null(),
      );
    }
    let s = Shader {
      name: name.to_string(),
      renderer_id: program,
    };

    s
  }
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
  let shader_int;
  unsafe {
    shader_int = gl::CreateShader(ty);
    // Attempt to compile the shader
    let c_str = CString::new(src.as_bytes()).unwrap();
    gl::ShaderSource(shader_int, 1, &c_str.as_ptr(), ptr::null());
    gl::CompileShader(shader_int);
    // Get the compile status
    let mut status = gl::FALSE as GLint;
    gl::GetShaderiv(shader_int, gl::COMPILE_STATUS, &mut status);
    // Fail on error
    if status != (gl::TRUE as GLint) {
      let mut len = 0;
      gl::GetShaderiv(shader_int, gl::INFO_LOG_LENGTH, &mut len);
      let mut buf = Vec::with_capacity(len as usize);
      buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
      gl::GetShaderInfoLog(
        shader_int,
        len,
        ptr::null_mut(),
        buf.as_mut_ptr() as *mut GLchar,
      );
      panic!(
        "{}",
        str::from_utf8(&buf)
          .ok()
          .expect("ShaderInfoLog not valid utf8")
      );
    }
  }
  shader_int
}
pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
  unsafe {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    // Get the link status
    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
    // Fail on error
    if status != (gl::TRUE as GLint) {
      let mut len: GLint = 0;
      gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
      let mut buf = Vec::with_capacity(len as usize);
      buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
      gl::GetProgramInfoLog(
        program,
        len,
        ptr::null_mut(),
        buf.as_mut_ptr() as *mut GLchar,
      );
      panic!(
        "{}",
        str::from_utf8(&buf)
          .ok()
          .expect("ProgramInfoLog not valid utf8")
      );
    }
    program
  }
}
