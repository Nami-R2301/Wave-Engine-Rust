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

use crate::wave::graphics::open_gl::renderer::{EnumOpenGLErrors};
use crate::wave::graphics::open_gl::buffer::{GLboolean, GLchar, GLenum, GLint, GLuint, GLfloat};
use crate::wave::graphics::renderer::{EnumState, Renderer};
use crate::wave::graphics::shader::{EnumError, TraitShader};
use crate::wave::math::Mat4;
use crate::wave::EnumApi;

use crate::{check_gl_call, log};

/*
///////////////////////////////////   OpenGL shader    ///////////////////////////////////
///////////////////////////////////                    ///////////////////////////////////
///////////////////////////////////                    ///////////////////////////////////
 */

#[derive(Debug, Clone, PartialEq)]
pub struct GlShader {
  pub m_id: u32,
  pub m_vertex_str: String,
  // For debug purposes.
  pub m_fragment_str: String,
  m_uniform_cache: std::collections::HashMap<&'static str, GLint>,
}

impl TraitShader for GlShader {
  fn new(vertex_file_path: &'static str, fragment_file_path: &'static str) -> Result<Self, EnumError> {
    let vertex_file_str = std::fs::read_to_string(vertex_file_path);
    let fragment_file_str = std::fs::read_to_string(fragment_file_path);
    
    if vertex_file_str.is_err() || fragment_file_str.is_err() {
      return Err(EnumError::ShaderFileError);
    }
    
    return Ok(GlShader {
      m_id: 0,
      m_vertex_str: vertex_file_str.unwrap_or("Empty".to_string()),
      m_fragment_str: fragment_file_str.unwrap_or("Empty".to_string()),
      m_uniform_cache: std::collections::HashMap::new(),
    });
  }
  
  fn from(_other_shader: Self) -> Self where Self: Sized {
    todo!()
  }
  
  fn compile(&self, shader_id: u32, shader_type: &dyn std::any::Any) -> Result<(), EnumError> {
    // Compile and link.
    check_gl_call!("Shader", gl::CompileShader(shader_id));
    
    if !shader_type.is::<GLenum>() || !shader_type.is::<u32>() {
      return Err(EnumError::ShaderFileError);
    }
    
    let conversion = shader_type.downcast_ref::<GLenum>();
    
    if conversion.is_none() {
      return Err(EnumError::AnyConversionError);
    }
    
    // Error checking.
    let mut compiled_successfully: GLint = 0;
    let mut buffer_length: GLint = 0;
    
    unsafe { gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compiled_successfully) };
    if compiled_successfully as GLboolean == gl::FALSE {
      #[allow(unused)]
        let mut shader_type_str: String = "Undefined".to_string();
      // For debug purposes.
      match *conversion.unwrap() {
        gl::VERTEX_SHADER => {
          shader_type_str = "vertex shader".to_string();
        }
        gl::FRAGMENT_SHADER => {
          shader_type_str = "fragment shader".to_string();
        }
        gl::COMPUTE_SHADER => {
          shader_type_str = "compute shader".to_string();
        }
        type_ => {
          log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Unknown type of shader '{:#?}'!", type_);
          return Err(EnumError::ShaderTypeError);
        }
      }
      
      // Get info length.
      unsafe {
        gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH,
          &mut buffer_length as *mut _)
      };
      let mut buffer: Vec<GLchar> = Vec::with_capacity(buffer_length as usize);
      
      unsafe {
        gl::GetShaderInfoLog(shader_id, buffer_length, &mut buffer_length,
          buffer.as_mut_ptr())
      };
      
      log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Error, could not compile {0} shader!\n \
        Info => {1}", shader_type_str, unsafe {
        std::ffi::CStr::from_ptr(buffer.as_ptr()).to_str().unwrap()
      });
      return Err(EnumError::ShaderSyntaxError);
    }
    
    return Ok(());
  }
  
  fn send(&mut self) -> Result<(), EnumError> {
    check_gl_call!("Shader", self.m_id = gl::CreateProgram());
    
    check_gl_call!("Shader", let vertex_shader: GLuint = gl::CreateShader(gl::VERTEX_SHADER));
    check_gl_call!("Shader", let fragment_shader: GLuint = gl::CreateShader(gl::FRAGMENT_SHADER));
    
    // Source our shaders.
    match (self.source(vertex_shader, &self.m_vertex_str),
      self.source(fragment_shader, &self.m_fragment_str)) {
      (Ok(_), Ok(_)) => {}
      (Err(_), Err(_)) => {
        log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Unable to source shaders {0} and {1}!",
          vertex_shader, fragment_shader);
        return Err(EnumError::ShaderSourcing);
      }
      (Err(_), Ok(_)) => {
        log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Unable to source vertex shader {0}!",
          vertex_shader);
        return Err(EnumError::ShaderSourcing);
      }
      (Ok(_), Err(_)) => {
        log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Unable to source fragment shader {0}!",
          fragment_shader);
        return Err(EnumError::ShaderSourcing);
      }
    }
    // Compile our shaders.
    match (self.compile(vertex_shader, &gl::VERTEX_SHADER),
      self.compile(fragment_shader, &gl::FRAGMENT_SHADER)) {
      (Ok(_), Ok(_)) => {}
      _ => {
        return Err(EnumError::ShaderCompilation);
      }
    }
    
    // Attach shaders to program.
    check_gl_call!("Shader", gl::AttachShader(self.m_id, vertex_shader));
    check_gl_call!("Shader", gl::AttachShader(self.m_id, fragment_shader));
    
    // Link program.
    check_gl_call!("Shader", gl::LinkProgram(self.m_id));
    
    let mut program_link_status: GLint = 0;
    unsafe { gl::GetProgramiv(self.m_id, gl::LINK_STATUS, &mut program_link_status); }
    if program_link_status as GLboolean == gl::FALSE {
      let mut buffer_length: GLint = 0;
      unsafe { gl::GetProgramiv(self.m_id, gl::INFO_LOG_LENGTH, &mut buffer_length); }
      let mut buffer: Vec<GLchar> = Vec::with_capacity(buffer_length as usize);
      
      unsafe {
        gl::GetProgramInfoLog(self.m_id, buffer_length, &mut buffer_length,
          buffer.as_mut_ptr());
      }
      log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Error linking program {0}! Error => {1}",
          self.m_id, unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()).to_str().unwrap() });
      return Err(EnumError::ProgramCreationError);
    }
    
    // Delete shaders CPU-side, since we uploaded it to the GPU VRAM.
    check_gl_call!("Shader", gl::DeleteShader(vertex_shader));
    check_gl_call!("Shader", gl::DeleteShader(fragment_shader));
    
    // Validate program.
    check_gl_call!("Shader", gl::ValidateProgram(self.m_id));
    return Ok(());
  }
  
  fn upload_data(&mut self, uniform_name: &'static str, uniform: &dyn std::any::Any) -> Result<(), EnumError> {
    match self.bind() {
      Ok(_) => {}
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Could not use shader {0}!", self.m_id);
        return Err(err);
      }
    }
    
    if !self.m_uniform_cache.contains_key(uniform_name) {
      let c_str: std::ffi::CString = std::ffi::CString::new(uniform_name)
        .expect("[Shader] -->\t Error converting str to CString when trying to upload uniform!");
      
      check_gl_call!("Shader", let new_uniform: GLint = gl::GetUniformLocation(self.m_id, c_str.as_ptr()));
      if new_uniform == -1 {
        log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Could not upload uniform '{0}'!",
        uniform_name);
        return Err(EnumError::UniformNotFound);
      }
      self.m_uniform_cache.insert(uniform_name, new_uniform);
      
      
      if uniform.is::<u32>() {
        let value_ptr = uniform.downcast_ref::<u32>().unwrap();
        check_gl_call!("Shader", gl::Uniform1ui(*self.m_uniform_cache.get(uniform_name).unwrap(), *value_ptr));
      } else if uniform.is::<i32>() {
        let value_ptr = uniform.downcast_ref::<i32>().unwrap();
        check_gl_call!("Shader", gl::Uniform1i(*self.m_uniform_cache.get(uniform_name).unwrap(), *value_ptr));
      } else if uniform.is::<f32>() {
        let value_ptr = uniform.downcast_ref::<f32>().unwrap();
        check_gl_call!("Shader", gl::Uniform1f(*self.m_uniform_cache.get(uniform_name).unwrap(), *value_ptr));
      } else if uniform.is::<f64>() {
        let value_ptr = uniform.downcast_ref::<f64>().unwrap();
        check_gl_call!("Shader", gl::Uniform1d(*self.m_uniform_cache.get(uniform_name).unwrap(), *value_ptr));
      } else if uniform.is::<Mat4>() {
        let value_ptr = uniform.downcast_ref::<Mat4>().unwrap();
        check_gl_call!("Shader", gl::UniformMatrix4fv(*self.m_uniform_cache.get(uniform_name).unwrap(),
          1, gl::FALSE, (value_ptr.as_array().as_ptr()) as *const GLfloat));
      } else {
        log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Type of uniform '{0}' is unsupported for glsl!",
          uniform_name);
        return Err(EnumError::UnsupportedUniformType);
      }
    }
    return Ok(());
  }
  
  fn get_id(&self) -> u32 {
    return self.m_id;
  }
  
  fn to_string(&self) -> String {
    return format!("Vertex shader :\n{0}\nFragment shader : \n{1}", self.m_vertex_str, self.m_fragment_str);
  }
}

impl GlShader {
  pub fn bind(&self) -> Result<(), EnumError> {
    check_gl_call!("Shader", gl::UseProgram(self.m_id));
    return Ok(());
  }
  
  fn source(&self, shader_id: GLuint, shader_str: &String) -> Result<(), EnumError> {
    let c_str: std::ffi::CString = std::ffi::CString::new(shader_str.as_str())
      .expect("[Shader] -->\t Could not convert shader string in GlShader::source() from &str \
       to CString!");
    
    check_gl_call!("Shader", gl::ShaderSource(shader_id, 1, &(c_str.as_ptr()), std::ptr::null()));
    return Ok(());
  }
}

// Free from the GPU when we are done with the shader program.
impl Drop for GlShader {
  fn drop(&mut self) {
    unsafe {
      let renderer = Renderer::get().as_ref()
        .expect("[Shader] -->\t Cannot drop GlShader, renderer is None! Exiting...");
      
      if renderer.get_type() == EnumApi::OpenGL && renderer.get_state() != EnumState::Shutdown {
        gl::UseProgram(0);
        gl::DeleteProgram(self.m_id);
      }
    }
  }
}