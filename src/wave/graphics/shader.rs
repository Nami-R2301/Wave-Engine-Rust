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

use crate::{check_gl_call, log};
use crate::wave::graphics::buffer::{GLboolean, GLchar, GLenum, GLfloat, GLint, GLuint};
use crate::wave::math::Mat4;

#[derive(Debug, PartialEq)]
pub enum EnumErrors {
  Ok,
  ProgramCreation,
  InvalidShaderFile,
  InvalidShaderSyntax,
  ShaderSourcing,
  ShaderCompilation,
  ShaderLinkage,
  UniformNotFound,
  GlError(GLenum),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlslShader {
  pub m_program_id: GLuint,
  pub m_vertex_str: String,
  // For debug purposes.
  pub m_fragment_str: String,
  // For debug purposes.
  m_uniform_cache: std::collections::HashMap<&'static str, GLint>,
}

impl GlslShader {
  pub fn new(vertex_file_path: &'static str, fragment_file_path: &'static str) -> Result<Self, EnumErrors> {
    let vertex_file_str = std::fs::read_to_string(vertex_file_path);
    let fragment_file_str = std::fs::read_to_string(fragment_file_path);
    
    if vertex_file_str.is_err() || fragment_file_str.is_err() {
      return Err(EnumErrors::InvalidShaderFile);
    }
    
    return Ok(GlslShader {
      m_program_id: 0,
      m_vertex_str: vertex_file_str.unwrap_or("Empty".to_string()),
      m_fragment_str: fragment_file_str.unwrap_or("Empty".to_string()),
      m_uniform_cache: std::collections::HashMap::new(),
    });
  }
  
  pub fn from(_other_shader: Self) -> Self {
    todo!();
  }
  
  pub fn send(&mut self) -> Result<(), EnumErrors> {
    #[cfg(feature = "OpenGL")]
    {
      check_gl_call!("Shader", self.m_program_id = gl::CreateProgram());
      
      check_gl_call!("Shader", let vertex_shader: GLuint = gl::CreateShader(gl::VERTEX_SHADER));
      check_gl_call!("Shader", let fragment_shader: GLuint = gl::CreateShader(gl::FRAGMENT_SHADER));
      
      // Source our shaders.
      match (self.source(vertex_shader, &self.m_vertex_str),
        self.source(fragment_shader, &self.m_fragment_str)) {
        (Ok(_), Ok(_)) => {}
        (Err(_), Err(_)) => {
          log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Unable to source shaders {0} and {1}!",
          vertex_shader, fragment_shader);
          return Err(EnumErrors::ShaderSourcing);
        }
        (Err(_), Ok(_)) => {
          log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Unable to source vertex shader {0}!",
          vertex_shader);
          return Err(EnumErrors::ShaderSourcing);
        }
        (Ok(_), Err(_)) => {
          log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Unable to source fragment shader {0}!",
          fragment_shader);
          return Err(EnumErrors::ShaderSourcing);
        }
      }
      unsafe {
        // Compile our shaders.
        match (self.compile(vertex_shader, gl::VERTEX_SHADER),
          self.compile(fragment_shader, gl::FRAGMENT_SHADER)) {
          (Ok(_), Ok(_)) => {}
          _ => {
            return Err(EnumErrors::ShaderCompilation);
          }
        }
      }
      
      // Attach shaders to program.
      check_gl_call!("Shader", gl::AttachShader(self.m_program_id, vertex_shader));
      check_gl_call!("Shader", gl::AttachShader(self.m_program_id, fragment_shader));
      
      // Link program.
      check_gl_call!("Shader", gl::LinkProgram(self.m_program_id));
      
      let mut program_link_status: GLint = 0;
      unsafe { gl::GetProgramiv(self.m_program_id, gl::LINK_STATUS, &mut program_link_status); }
      if program_link_status as GLboolean == gl::FALSE {
        let mut buffer_length: GLint = 0;
        unsafe { gl::GetProgramiv(self.m_program_id, gl::INFO_LOG_LENGTH, &mut buffer_length); }
        let mut buffer: Vec<GLchar> = Vec::with_capacity(buffer_length as usize);
        
        unsafe {
          gl::GetProgramInfoLog(self.m_program_id, buffer_length, &mut buffer_length,
            buffer.as_mut_ptr());
        }
        log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Error linking program {0}! Error => {1}",
          self.m_program_id, unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()).to_str().unwrap() });
        return Err(EnumErrors::ProgramCreation);
      }
      
      // Delete shaders CPU-side, since we uploaded it to the GPU VRAM.
      check_gl_call!("Shader", gl::DeleteShader(vertex_shader));
      check_gl_call!("Shader", gl::DeleteShader(fragment_shader));
      
      // Validate program.
      check_gl_call!("Shader", gl::ValidateProgram(self.m_program_id));
    }
    return Ok(());
  }
  
  pub fn bind(&self) -> Result<(), EnumErrors> {
    #[cfg(feature = "OpenGL")]
    {
      check_gl_call!("Shader", gl::UseProgram(self.m_program_id));
    }
    return Ok(());
  }
  
  fn source(&self, shader_id: GLuint, shader_str: &String) -> Result<(), EnumErrors> {
    #[cfg(feature = "OpenGL")]
    {
      let c_str: std::ffi::CString = std::ffi::CString::new(shader_str.as_str())
        .expect("[Shader] -->\t Could not convert shader string in GlShader::source() from &str \
       to CString!");
      
      check_gl_call!("Shader", gl::ShaderSource(shader_id, 1, &(c_str.as_ptr()), std::ptr::null()));
    }
    return Ok(());
  }
  
  unsafe fn compile(&self, shader_id: GLenum, shader_type: GLenum) -> Result<(), EnumErrors> {
    #[cfg(feature = "OpenGL")]
    {
      // Compile and link.
      check_gl_call!("Shader", gl::CompileShader(shader_id));
      let mut shader_type_str: String = "".to_string();
      
      // For debug purposes.
      match shader_type {
        gl::VERTEX_SHADER => {
          shader_type_str = "vertex shader".to_string();
        }
        gl::FRAGMENT_SHADER => {
          shader_type_str = "fragment shader".to_string();
        }
        gl::COMPUTE_SHADER => {
          shader_type_str = "compute shader".to_string();
        }
        _ => {}
      }
      
      // Error checking.
      let mut compiled_successfully: GLint = 0;
      let mut buffer_length: GLint = 0;
      
      gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compiled_successfully);
      if compiled_successfully as GLboolean == gl::FALSE {
        // Get info length.
        gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut buffer_length as *mut _);
        let mut buffer: Vec<GLchar> = Vec::with_capacity(buffer_length as usize);
        
        gl::GetShaderInfoLog(shader_id, buffer_length, &mut buffer_length, buffer.as_mut_ptr());
        
        log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Error, could not compile {0} shader!\n \
        Info => {1}", shader_type_str, std::ffi::CStr::from_ptr(buffer.as_ptr()).to_str().unwrap());
        return Err(EnumErrors::InvalidShaderSyntax);
      }
    }
    
    return Ok(());
  }
  
  pub fn upload_uniform(&mut self, name: &'static str, uniform: &dyn std::any::Any) -> Result<(), EnumErrors> {
    match self.bind() {
      Ok(_) => {}
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Could not use shader {0}!", self.m_program_id);
        return Err(err);
      }
    }
    
    #[cfg(feature = "OpenGL")]
    {
      if !self.m_uniform_cache.contains_key(name) {
        let c_str: std::ffi::CString = std::ffi::CString::new(name)
          .expect("[Shader] -->\t Error converting str to CString when trying to upload uniform!");
        
        check_gl_call!("Shader", let new_uniform: GLint = gl::GetUniformLocation(self.m_program_id, c_str.as_ptr()));
        if new_uniform == -1 {
          log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Could not upload uniform '{0}'!", name);
          return Err(EnumErrors::UniformNotFound);
        }
        self.m_uniform_cache.insert(name, new_uniform);
        
        
        if uniform.is::<u32>() {
          let value_ptr = uniform.downcast_ref::<u32>().unwrap();
          check_gl_call!("Shader", gl::Uniform1ui(*self.m_uniform_cache.get(name).unwrap(), *value_ptr));
        } else if uniform.is::<i32>() {
          let value_ptr = uniform.downcast_ref::<i32>().unwrap();
          check_gl_call!("Shader", gl::Uniform1i(*self.m_uniform_cache.get(name).unwrap(), *value_ptr));
        } else if uniform.is::<f32>() {
          let value_ptr = uniform.downcast_ref::<f32>().unwrap();
          check_gl_call!("Shader", gl::Uniform1f(*self.m_uniform_cache.get(name).unwrap(), *value_ptr));
        } else if uniform.is::<f64>() {
          let value_ptr = uniform.downcast_ref::<f64>().unwrap();
          check_gl_call!("Shader", gl::Uniform1d(*self.m_uniform_cache.get(name).unwrap(), *value_ptr));
        } else if uniform.is::<Mat4>() {
          let value_ptr = uniform.downcast_ref::<Mat4>().unwrap();
          check_gl_call!("Shader", gl::UniformMatrix4fv(*self.m_uniform_cache.get(name).unwrap(),
          1, gl::FALSE, (value_ptr.as_array().as_ptr()) as *const GLfloat));
        } else {
          log!(EnumLogColor::Yellow, "ERROR", "[Shader] -->\t Uniform '{0}' has an unsupported type for glsl! \
       Not uploading it...", name);
        }
      }
    }
    return Ok(());
  }
}

// Free from the GPU when we are done with the shader program.
impl Drop for GlslShader {
  fn drop(&mut self) {
    #[cfg(feature = "OpenGL")]
    {
      unsafe {
        gl::UseProgram(0);
        gl::DeleteProgram(self.m_program_id);
      }
    }
  }
}
