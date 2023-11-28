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

use crate::{check_gl_call};
use crate::wave::graphics::buffer::{GLboolean, GLfloat, GLchar, GLenum, GLint, GLuint};
use crate::wave::math::Mat4;

use crate::log;
use crate::wave::graphics::renderer::{EnumApi, EnumState, Renderer};

#[derive(Debug, PartialEq)]
pub enum EnumErrors {
  ProgramCreationError,
  ShaderFileError,
  ShaderSyntaxError,
  ShaderTypeError,
  ShaderSourcing,
  ShaderCompilation,
  ShaderLinkage,
  UniformNotFound,
  GlError(GLenum),
  AnyConversionError,
}

pub trait TraitShader {
  fn new(vertex_file_path: &'static str, fragment_file_path: &'static str) -> Result<Self, EnumErrors> where Self: Sized;
  fn from(other_shader: Self) -> Self where Self: Sized;
  fn compile(&self, shader_id: u32, shader_type: &dyn std::any::Any) -> Result<(), EnumErrors>;
  fn send(&mut self) -> Result<(), EnumErrors>;
  fn upload_data(&mut self, uniform_name: &'static str, uniform: &dyn std::any::Any) -> Result<(), EnumErrors>;
  fn get_id(&self) -> u32;
}

pub struct GlslShader<T: TraitShader> {
  m_api_data: T,
  // For debug purposes.
  m_uniform_cache: std::collections::HashMap<&'static str, i8>,
}

impl<T: TraitShader> GlslShader<T> {
  pub fn new(vertex_file_path: &'static str, fragment_file_path: &'static str) -> Result<Self, EnumErrors> {
    let shader = T::new(vertex_file_path, fragment_file_path)?;
    
    return Ok(GlslShader {
      m_api_data: shader,
      m_uniform_cache: std::collections::HashMap::new(),
    });
  }
  
  pub fn compile(&self, shader_id: u32, shader_type: &dyn std::any::Any) -> Result<(), EnumErrors> {
    return self.m_api_data.compile(shader_id, shader_type);
  }
  pub fn send(&mut self) -> Result<(), EnumErrors> {
    return self.m_api_data.send();
  }
  pub fn upload_data(&mut self, uniform_name: &'static str, uniform: &dyn std::any::Any) -> Result<(), EnumErrors> {
    return self.m_api_data.upload_data(uniform_name, uniform);
  }
  
  pub fn get_api_data(&self) -> &T {
    return &self.m_api_data;
  }
  
  pub fn get_id(&self) -> u32 {
    return self.m_api_data.get_id();
  }
}

/*
///////////////////////////////////   Vulkan    ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
 */

#[derive(Debug, Clone, PartialEq)]
pub struct VkShader {
  pub m_id: u32,
  pub m_vertex_str: String,
  // For debug purposes.
  pub m_fragment_str: String,
  m_uniform_cache: std::collections::HashMap<&'static str, GLint>,
}

impl TraitShader for VkShader {
  fn new(_vertex_file_path: &'static str, _fragment_file_path: &'static str) -> Result<Self, EnumErrors> where Self: Sized {
    return Ok(VkShader {
      m_id: 0,
      m_vertex_str: "Empty".to_string(),
      m_fragment_str: "Empty".to_string(),
      m_uniform_cache: std::collections::HashMap::new()
    });
  }
  
  fn from(_other_shader: Self) -> Self where Self: Sized {
    todo!()
  }
  
  fn compile(&self, _shader_id: u32, _shader_type: &dyn std::any::Any) -> Result<(), EnumErrors> {
    todo!()
  }
  
  fn send(&mut self) -> Result<(), EnumErrors> {
    return Ok(());
  }
  
  fn upload_data(&mut self, _uniform_name: &'static str, _uniform: &dyn std::any::Any) -> Result<(), EnumErrors> {
    return Ok(());
  }
  
  fn get_id(&self) -> u32 {
    return self.m_id;
  }
}


/*
///////////////////////////////////   OpenGL    ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
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
  fn new(vertex_file_path: &'static str, fragment_file_path: &'static str) -> Result<Self, EnumErrors> {
    let vertex_file_str = std::fs::read_to_string(vertex_file_path);
    let fragment_file_str = std::fs::read_to_string(fragment_file_path);
    
    if vertex_file_str.is_err() || fragment_file_str.is_err() {
      return Err(EnumErrors::ShaderFileError);
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
  
  fn compile(&self, shader_id: u32, shader_type: &dyn std::any::Any) -> Result<(), EnumErrors> {
    // Compile and link.
    check_gl_call!("Shader", gl::CompileShader(shader_id));
    
    if !shader_type.is::<GLenum>() || !shader_type.is::<u32>() {
      return Err(EnumErrors::ShaderFileError);
    }
    
    let conversion = shader_type.downcast_ref::<GLenum>();
    
    if conversion.is_none() {
      return Err(EnumErrors::AnyConversionError);
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
          return Err(EnumErrors::ShaderTypeError);
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
      return Err(EnumErrors::ShaderSyntaxError);
    }
    
    return Ok(());
  }
  
  fn send(&mut self) -> Result<(), EnumErrors> {
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
    // Compile our shaders.
    match (self.compile(vertex_shader, &gl::VERTEX_SHADER),
      self.compile(fragment_shader, &gl::FRAGMENT_SHADER)) {
      (Ok(_), Ok(_)) => {}
      _ => {
        return Err(EnumErrors::ShaderCompilation);
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
      return Err(EnumErrors::ProgramCreationError);
    }
    
    // Delete shaders CPU-side, since we uploaded it to the GPU VRAM.
    check_gl_call!("Shader", gl::DeleteShader(vertex_shader));
    check_gl_call!("Shader", gl::DeleteShader(fragment_shader));
    
    // Validate program.
    check_gl_call!("Shader", gl::ValidateProgram(self.m_id));
    return Ok(());
  }
  
  fn upload_data(&mut self, uniform_name: &'static str, uniform: &dyn std::any::Any) -> Result<(), EnumErrors> {
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
        return Err(EnumErrors::UniformNotFound);
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
        log!(EnumLogColor::Yellow, "ERROR", "[Shader] -->\t Uniform '{0}' has an unsupported type for glsl! \
       Not uploading it...", uniform_name);
      }
    }
    return Ok(());
  }
  
  fn get_id(&self) -> u32 {
    return self.m_id;
  }
}

impl GlShader {
  pub fn bind(&self) -> Result<(), EnumErrors> {
    check_gl_call!("Shader", gl::UseProgram(self.m_id));
    return Ok(());
  }
  
  fn source(&self, shader_id: GLuint, shader_str: &String) -> Result<(), EnumErrors> {
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
      
      if renderer.m_api.get_type() == EnumApi::OpenGL && renderer.m_api.get_state() != EnumState::Shutdown {
        gl::UseProgram(0);
        gl::DeleteProgram(self.m_id);
      }
    }
  }
}
