/*
 MIT License

 Copyright (c) 2023 Nami Reghbati

 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:

 The above copyright notice and this permission notice shall be included in all
 copies or substantial portions of the Software.

 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NON INFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 SOFTWARE.
*/

extern crate gl;

use std::fmt::Debug;
use std::mem::size_of;

pub use gl::types::{GLboolean, GLchar, GLenum, GLint, GLsizei, GLuint, GLvoid};
use gl::types::{GLDEBUGPROC, GLsizeiptr};

use crate::{check_gl_call, log};
use crate::wave::assets::renderable_assets::GlREntity;
use crate::wave::graphics::renderer::EnumApi::OpenGL;
use crate::wave::window::GlfwWindow;

static mut S_STATE: EnumState = EnumState::Ok;
static S_ERROR_CALLBACK: GLDEBUGPROC = Some(gl_error_callback);
static mut S_STATS: Stats = Stats {
  m_entities_sent_count: 0,
  m_shader_bound_count: 0,
  m_vao_bound_count: 0,
  m_ibo_bound_count: 0,
  m_texture_bound_count: 0,
};
static mut S_VAO: GLuint = 0;
static mut S_VBO_ARRAY: GLuint = 0;

#[macro_export]
macro_rules! check_gl_call {
    () => {};
    ($name:literal, $gl_function:expr) => {
      unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
      unsafe {
        $gl_function;
        let error = gl::GetError();
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t Error when executing gl call! \
          Code => 0x{1:x}", $name, error);
          return Err(EnumErrors::GlError(error));
        }
      }
    };
    ($name:literal, let mut $var:ident: $var_type:ty = $gl_function:expr) => {
            unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
            let mut $var:$var_type = unsafe { $gl_function };
            unsafe {
              let error = gl::GetError();
              if error != gl::NO_ERROR {
                log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t Error when executing gl call! \
                Code => 0x{1:x}", $name, error);
                return Err(EnumErrors::GlError(error));
              }
            }
          };
    ($name:literal, let $var:ident: $var_type:ty = $gl_function:expr) => {
          unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
          let $var:$var_type = unsafe { $gl_function };
          unsafe {
            let error = gl::GetError();
            if error != gl::NO_ERROR {
              log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t Error when executing gl call! \
              Code => 0x{1:x}", $name, error);
              return Err(EnumErrors::GlError(error));
            }
          }
        };
    ($name:literal, $var:ident = $gl_function:expr) => {
        unsafe { while gl::GetError() != gl::NO_ERROR {} };
        unsafe { $var = $gl_function; }
        unsafe {
          let error = gl::GetError();
          if error != gl::NO_ERROR {
            log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t Error when executing gl call! \
            Code => 0x{1:x}", $name, error);
            return Err(EnumErrors::GlError(error));
          }
        }
      };
}

#[derive(Debug)]
pub enum EnumApi {
  OpenGL(String),
  Vulkan(String),
  DirectX(String),
}

#[derive(Debug, Copy, Clone)]
pub enum EnumFeature {
  Debug(bool),
  DepthTest(bool),
  CullFacing(bool, GLenum),
  Wireframe(bool),
  MSAA(bool, u8),
  SRGB(bool),
  Blending(bool, GLenum, GLenum),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumErrors {
  Init,
  NotImplemented,
  InvalidEntity,
  EntityNotFound,
  GlError(GLenum),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumState {
  Ok,
  Error,
  CriticalError,
}

pub trait TraitSendableEntity {
  fn send(&mut self) -> Result<(), EnumErrors>;
  fn resend(&mut self) -> Result<(), EnumErrors>;
  fn free(&mut self) -> Result<(), EnumErrors>;
  fn is_sent(&self) -> bool;
}

pub struct Stats {
  m_entities_sent_count: u32,
  m_shader_bound_count: u32,
  m_vao_bound_count: u32,
  m_ibo_bound_count: u32,
  m_texture_bound_count: u32,
}

impl Stats {
  pub fn new() -> Self {
    return Stats {
      m_entities_sent_count: 0,
      m_shader_bound_count: 0,
      m_vao_bound_count: 0,
      m_ibo_bound_count: 0,
      m_texture_bound_count: 0,
    };
  }
  
  pub fn reset(&mut self) {
    self.m_ibo_bound_count = 0;
    self.m_shader_bound_count = 0;
    self.m_entities_sent_count = 0;
    self.m_vao_bound_count = 0;
    self.m_texture_bound_count = 0;
  }
}

pub struct GlRenderer {}

impl GlRenderer {
  pub fn new() -> Result<(), EnumErrors> {
    let result = unsafe { init_api() };
    return match result {
      Ok(_) => { Ok(()) }
      Err(err) => { Err(err) }
    }
  }
  
  pub fn shutdown() -> Result<(), EnumErrors> {
    if unsafe { S_STATE == EnumState::Error } {
      return Err(EnumErrors::NotImplemented);
    }
    return Ok(());
  }
  
  pub fn begin() {}
  
  pub fn end() {}
  
  pub fn flush() {}
  
  pub fn send(sendable_entity: &GlREntity) -> Result<(), EnumErrors> {
    if sendable_entity.is_empty() {
      log!(EnumLogColor::Yellow, "WARN", "[Renderer] --> Entity {0} sent has no \
      vertices! Not sending it...", sendable_entity)
    }
    
    check_gl_call!("Renderer", gl::CreateBuffers(1, &mut S_VBO_ARRAY));
    check_gl_call!("Renderer", gl::CreateVertexArrays(1, &mut S_VAO));
    check_gl_call!("Renderer", gl::BindVertexArray(S_VAO));
    
    let object_size: usize = sendable_entity.size();
    let mut offset: usize = 0;
    
    // Allocate main dynamic vbo to hold all the data provided.
    check_gl_call!("Renderer", gl::BindBuffer(gl::ARRAY_BUFFER, S_VBO_ARRAY));
    check_gl_call!("Renderer", gl::BufferData(gl::ARRAY_BUFFER, object_size as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    // IDs (Vec3).
    check_gl_call!("Renderer", gl::BufferSubData(gl::ARRAY_BUFFER, offset as GLsizeiptr,
      (size_of::<u32>() * sendable_entity.m_data.m_ids.len()) as GLsizeiptr,
      sendable_entity.m_data.m_ids.as_ptr() as *const GLvoid));
    offset += size_of::<u32>() * sendable_entity.m_data.m_ids.len();
    
    // Positions (Vec3s).
    check_gl_call!("Renderer", gl::BufferSubData(gl::ARRAY_BUFFER, offset as GLsizeiptr,
      (size_of::<f32>() * sendable_entity.m_data.m_positions.len()) as GLsizeiptr ,
      sendable_entity.m_data.m_positions.as_ptr() as *const GLvoid));
    offset += size_of::<f32>() * sendable_entity.m_data.m_positions.len();
    
    // Normals (Vec3s).
    check_gl_call!("Renderer", gl::BufferSubData(gl::ARRAY_BUFFER, offset as GLsizeiptr,
      (size_of::<f32>() * sendable_entity.m_data.m_normals.len()) as GLsizeiptr,
      sendable_entity.m_data.m_normals.as_ptr() as *const GLvoid));
    offset += size_of::<f32>() * sendable_entity.m_data.m_normals.len();
    
    // Colors (Colors).
    check_gl_call!("Renderer", gl::BufferSubData(gl::ARRAY_BUFFER, offset as GLsizeiptr,
      (size_of::<f32>() * sendable_entity.m_data.m_colors.len()) as GLsizeiptr,
      sendable_entity.m_data.m_colors.as_ptr() as *const GLvoid));
    offset += size_of::<f32>() * sendable_entity.m_data.m_colors.len();
    
    // Texture coordinates (Vec2s).
    check_gl_call!("Renderer", gl::BufferSubData(gl::ARRAY_BUFFER, offset as GLsizeiptr,
      (size_of::<f32>() * sendable_entity.m_data.m_texture_coords.len()) as GLsizeiptr,
      sendable_entity.m_data.m_texture_coords.as_ptr() as *const GLvoid));
    
    offset = 0;  // Reset offset for vertex attributes.
    
    // Enable vertex attributes.
    check_gl_call!("Renderer", gl::VertexAttribIPointer(0, 1, gl::UNSIGNED_INT, 0, offset as *const GLvoid));
    check_gl_call!("Renderer", gl::EnableVertexAttribArray(0));
    offset += size_of::<u32>() * sendable_entity.m_data.m_ids.len();
    
    check_gl_call!("Renderer", gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, offset as *const GLvoid));
    check_gl_call!("Renderer", gl::EnableVertexAttribArray(1));
    offset += size_of::<f32>() * sendable_entity.m_data.m_positions.len();
    
    check_gl_call!("Renderer", gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 0, offset as *const GLvoid));
    check_gl_call!("Renderer", gl::EnableVertexAttribArray(2));
    offset += size_of::<f32>() * sendable_entity.m_data.m_normals.len();
    
    check_gl_call!("Renderer", gl::VertexAttribPointer(3, 4, gl::FLOAT, gl::FALSE, 0, offset as *const GLvoid));
    check_gl_call!("Renderer", gl::EnableVertexAttribArray(3));
    offset += size_of::<f32>() * sendable_entity.m_data.m_colors.len();
    
    check_gl_call!("Renderer", gl::VertexAttribPointer(4, 2, gl::FLOAT, gl::FALSE, 0, offset as *const GLvoid));
    check_gl_call!("Renderer", gl::EnableVertexAttribArray(4));
    
    return Ok(());
  }
  
  pub fn draw() -> Result<(), EnumErrors> {
    // check_gl_call!("Renderer", gl::BindBuffer(S_VBO_ARRAYS[0]));
    check_gl_call!("Renderer", gl::DrawArrays(gl::TRIANGLES, 0, 36));
    return Ok(());
  }
  
  pub fn free(_entity_sent_id: &u32) -> Result<(), EnumErrors> {
    todo!()
  }
  
  pub fn get_renderer_info() -> String {
    unsafe {
      let renderer_info = std::ffi::CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8)
        .to_str().unwrap_or("Cannot retrieve renderer information!");
      
      let str: String = format!("Renderer Hardware => {0}", renderer_info);
      return str;
    }
  }
  
  pub fn get_api_info() -> EnumApi {
    unsafe {
      let api_vendor = std::ffi::CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8)
        .to_str().unwrap_or("Cannot retrieve api vendor information!");
      let api_version = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
        .to_str().unwrap_or("Cannot retrieve api version information!");
      
      let str: String = format!("{0}, {1}", api_vendor, api_version);
      return OpenGL(str);
    }
  }
  
  pub fn get_shading_info() -> String {
    unsafe {
      let shading_info = std::ffi::CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8)
        .to_str().unwrap_or("Cannot retrieve shading language information!");
      
      let str: String = format!("Shading Language => {0}", shading_info);
      return str;
    }
  }
  
  pub fn get_state() -> EnumState {
    return unsafe { S_STATE };
  }
  
  pub fn get_callback() -> GLDEBUGPROC {
    return S_ERROR_CALLBACK;
  }
  
  pub fn get_stats() -> &'static Stats {
    unsafe { return &S_STATS; }
  }
  
  pub fn toggle_feature(feature: EnumFeature) {
    match feature {
      EnumFeature::Debug(flag) => unsafe {
        if flag {
          gl::Enable(gl::DEBUG_OUTPUT);
          gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
          gl::DebugMessageCallback(S_ERROR_CALLBACK, std::ptr::null());
          gl::DebugMessageControl(gl::DONT_CARE, gl::DEBUG_TYPE_OTHER,
            gl::DONT_CARE, 0, std::ptr::null(), gl::FALSE);
        } else {
          gl::Disable(gl::DEBUG_OUTPUT);
          gl::Disable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        }
      }
      EnumFeature::DepthTest(flag) => unsafe {
        if flag {
          gl::Enable(gl::DEPTH_TEST);
        } else {
          gl::Disable(gl::DEPTH_TEST);
        }
      }
      EnumFeature::MSAA(flag, _) => unsafe {
        if flag {
          gl::Enable(gl::MULTISAMPLE);
        } else {
          gl::Disable(gl::MULTISAMPLE);
        }
      }
      EnumFeature::Blending(flag, s_factor, d_factor) => unsafe {
        if flag {
          gl::Enable(gl::BLEND);
          gl::BlendFunc(s_factor, d_factor);
        } else {
          gl::Disable(gl::BLEND);
        }
      }
      EnumFeature::Wireframe(flag) => unsafe {
        if flag {
          gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        } else {
          gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
      }
      EnumFeature::SRGB(flag) => unsafe {
        if flag {
          gl::Enable(gl::FRAMEBUFFER_SRGB);
        } else {
          gl::Disable(gl::FRAMEBUFFER_SRGB);
        }
      }
      EnumFeature::CullFacing(flag, face) => unsafe {
        if flag {
          gl::Enable(gl::CULL_FACE);
          gl::CullFace(face);
        } else {
          gl::Disable(gl::CULL_FACE);
        }
      }
    }
  }
}

unsafe fn init_api() -> Result<(), EnumErrors> {
  gl::load_with(|f_name| GlfwWindow::get_active_window().get_proc_address_raw(f_name));
  
  check_gl_call!("Renderer", gl::Viewport(0, 0, 1920, 1080));
  check_gl_call!("Renderer", gl::ClearColor(0.15, 0.15, 0.15, 1.0));
  
  check_gl_call!("Renderer", gl::FrontFace(gl::CW));
  
  return Ok(());
}

extern "system" fn gl_error_callback(error_code: GLenum, e_type: GLenum, _id: GLuint,
                                     severity: GLenum, _length: GLsizei, error_message: *const GLchar,
                                     _user_param: *mut std::ffi::c_void) {
  let mut final_error_msg: String = "".to_string();
  if error_code != gl::NO_ERROR {
    match error_code {
      _ => { final_error_msg += &format!("Code => 0x{0:x}; ", error_code) }
    }
    
    match e_type {
      gl::DEBUG_TYPE_ERROR => { final_error_msg += "Type => Error; "; }
      gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => { final_error_msg += "Type => Deprecated behavior; "; }
      gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => { final_error_msg += "Type => Undefined behavior; "; }
      gl::DEBUG_TYPE_PORTABILITY => { final_error_msg += "Type => Portability; "; }
      gl::DEBUG_TYPE_PERFORMANCE => { final_error_msg += "Type => Performance; "; }
      gl::DEBUG_TYPE_MARKER => { final_error_msg += "Type => Marker; "; }
      gl::DEBUG_TYPE_PUSH_GROUP => { final_error_msg += "Type => Push group; "; }
      gl::DEBUG_TYPE_POP_GROUP => { final_error_msg += "Type => Pop group; "; }
      gl::DEBUG_TYPE_OTHER => { final_error_msg += "Type => Other; "; }
      _ => { final_error_msg = "Type => Unknown; ".to_string(); }
    }
    
    match severity {
      gl::DEBUG_SEVERITY_HIGH => { final_error_msg += "Severity => Fatal (High); " }
      gl::DEBUG_SEVERITY_MEDIUM => { final_error_msg += "Severity => Fatal (Medium); " }
      gl::DEBUG_SEVERITY_LOW => { final_error_msg += "Severity => Warn (Low); " }
      gl::DEBUG_SEVERITY_NOTIFICATION => { final_error_msg += "Severity => Warn (Info); " }
      _ => { final_error_msg += "Severity => Fatal (Unknown); " }
    }
    
    final_error_msg += unsafe {
      &error_message.as_ref().into_iter()
        .map(|&character| character.to_string())
        .collect::<String>()
    };
    log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t {0}", final_error_msg);
  }
}