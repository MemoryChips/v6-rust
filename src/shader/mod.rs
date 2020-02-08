// #[macro_use]
extern crate log;
use log::{error, info};
extern crate gl;
extern crate regex;
use regex::Regex;

use gl::types::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use std::ptr;
use std::str;

fn create_whitespace_cstring_with_len(len: usize) -> CString {
  // allocate buffer of correct size
  let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
  // fill it with len spaces
  buffer.extend([b' '].iter().cycle().take(len));
  // convert buffer to CString
  unsafe { CString::from_vec_unchecked(buffer) }
}

// CONSIDER: Move this inside Shader Impl
fn shader_type(s_type: &str) -> GLenum {
  match s_type {
    "vertex" => return gl::VERTEX_SHADER,
    "fragment" | "pixel" => return gl::FRAGMENT_SHADER,
    _ => return 0,
  }
}

type ShaderSources = HashMap<GLenum, String>;

pub struct Shader {
  name: String,
  renderer_id: u32,
}

impl Drop for Shader {
  fn drop(&mut self) {
    Shader::delete_program(self.renderer_id);
    info!(
      "[{}] Shader struct dropped and program deleted: {} {}",
      line!(),
      self.name,
      self.renderer_id
    );
  }
}

impl Shader {
  fn delete_program(renderer_id: u32) {
    unsafe {
      gl::DeleteProgram(renderer_id);
    }
  }
  fn delete_shaders(program: u32, shader_ids: Vec<u32>) {
    unsafe {
      for shader_id in shader_ids {
        gl::DetachShader(program, shader_id);
        gl::DeleteShader(shader_id);
        info!("Deleting shader id: {}", shader_id);
      }
    }
  }
  fn read_shaders_from_file(filename: &str) -> ShaderSources {
    println!("filepath: {}", filename);
    let mut shader_sources_map: ShaderSources = HashMap::with_capacity(4);
    match File::open(filename) {
      Ok(f) => {
        let reader = BufReader::new(f);
        let re = Regex::new(r"\#type (?P<type>.+)").unwrap();
        let mut shader_source = String::with_capacity(512);
        let mut current_shader_type: GLenum = 0;
        let mut working_on_type = false;
        for line in reader.lines() {
          match line {
            Ok(l) => {
              match re.captures(&l).and_then(|cap| cap.name("type")) {
                Some(t) => {
                  if working_on_type {
                    // println!("{}", shader_source);
                    shader_sources_map.insert(current_shader_type, shader_source);
                    shader_source = "".to_string();
                  }
                  current_shader_type = shader_type(t.as_str());
                  working_on_type = true;
                  // println!("GLenum type: {} : {} ", t.as_str(), current_shader_type);
                }
                _ => {
                  if working_on_type {
                    shader_source += &l;
                    shader_source += &"\n";
                  }
                }
              };
            }
            Err(_e) => error!("Io error accessing file {}: {}", filename, _e),
          }
        }
        if working_on_type {
          // println!("{}", shader_source);
          shader_sources_map.insert(current_shader_type, shader_source);
        }
      }
      Err(_e) => error!("Shader file error: {}", _e),
    }
    shader_sources_map
  }
  pub fn new(name: &str, filepath: &str) -> Self {
    let shaders = Shader::read_shaders_from_file(filepath);
    Shader::new_from_sources(name, shaders)
  }
  // CONSIDER dropping this version of new
  pub fn _new(name: &str, vertex_src: &str, fragment_src: &str) -> Self {
    let mut sources: ShaderSources = HashMap::with_capacity(4);
    sources.insert(gl::VERTEX_SHADER, vertex_src.to_string());
    sources.insert(gl::FRAGMENT_SHADER, fragment_src.to_string());
    Shader::new_from_sources(name, sources)
  }
  fn new_from_sources(name: &str, sources: ShaderSources) -> Self {
    // let mut sources: ShaderSources = HashMap::with_capacity(4);
    // sources.insert(gl::VERTEX_SHADER, vertex_src.to_string());
    // sources.insert(gl::FRAGMENT_SHADER, fragment_src.to_string());
    let program = link_program(sources);
    let mut vao = 0;
    let mut vbo = 0;
    const VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5]; // CONSIDER: pass this into new
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
pub fn link_program(sources: ShaderSources) -> GLuint {
  unsafe {
    let program = gl::CreateProgram();
    let mut shader_ids: Vec<u32> = Vec::new();
    for (t, s) in &sources {
      // match
      let shader_id = compile_shader(s, *t);
      shader_ids.push(shader_id);
      gl::AttachShader(program, shader_id);
    }
    // gl::AttachShader(program, vs);
    // gl::AttachShader(program, fs);
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
    let mut is_linked: GLint = 0;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut is_linked);
    if is_linked == gl::FALSE as GLint {
      error!("Program link failure");
      let mut max_length: GLint = 0;
      gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut max_length);
      let info_log = create_whitespace_cstring_with_len(max_length as usize);
      gl::GetProgramInfoLog(
        program,
        max_length,
        &mut max_length,
        info_log.as_ptr() as *mut GLchar,
      );
      let error_info = info_log.to_string_lossy().into_owned();
      error!("[{}] Error info: {}", line!(), error_info);
      Shader::delete_program(program);
      Shader::delete_shaders(program, shader_ids);
    } else {
      info!("Program link success: {}", program);
      Shader::delete_shaders(program, shader_ids);
    }

    program
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn load_shader_file_test() {
    let filepath = "./examples/sandbox/assets/shaders/flatcolor.glsl";
    let ss_map = super::Shader::read_shaders_from_file(filepath);
    for k in ss_map.keys() {
      println!("Key: {}", k);
      let valid_key = *k == 35632 || *k == 35633;
      assert!(valid_key);
    }
  }
}
