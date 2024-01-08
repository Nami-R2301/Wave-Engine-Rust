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

use std::collections::HashMap;

use gl::types::{GLenum, GLsizei};

use crate::{check_gl_call, log};
use crate::wave::graphics::open_gl::buffer::{GLboolean, GLchar, GLfloat, GLint, GLuint};
use crate::wave::graphics::open_gl::renderer::{EnumOpenGLErrors, S_GL_4_6};
use crate::wave::graphics::renderer::{EnumApi, EnumState, Renderer};
use crate::wave::graphics::shader::{EnumError, EnumShaderSource, EnumShaderType, Shader, ShaderStage, TraitShader};
use crate::wave::math::Mat4;

/*
///////////////////////////////////   OpenGL shader    ///////////////////////////////////
///////////////////////////////////                    ///////////////////////////////////
///////////////////////////////////                    ///////////////////////////////////
 */

#[derive(Debug, Clone)]
pub struct GlShader {
  pub m_program_id: u32,
  m_shader_ids: Vec<GLuint>,
  m_shader_stages: Vec<ShaderStage>,
  m_uniform_cache: HashMap<&'static str, GLint>,
}

impl TraitShader for GlShader {
  fn default() -> Self {
    return Self {
      m_program_id: 0,
      m_shader_ids: Vec::with_capacity(2),
      m_shader_stages: Vec::with_capacity(2),
      m_uniform_cache: Default::default(),
    };
  }
  
  fn new(shader_stages: Vec<ShaderStage>) -> Result<Self, EnumError> {
    return Ok(GlShader {
      m_program_id: 0,
      m_shader_ids: Vec::with_capacity(2),
      m_shader_stages: shader_stages,
      m_uniform_cache: Default::default(),
    });
  }
  
  fn from(_other_shader: Self) -> Self where Self: Sized {
    todo!()
  }
  
  fn get_name(&self) -> EnumApi {
    return EnumApi::OpenGL;
  }
  
  fn source(&mut self) -> Result<(), EnumError> {
    for shader_stage in self.m_shader_stages.iter_mut() {
      check_gl_call!("Shader", let shader_id: GLuint = gl::CreateShader(shader_stage.m_type as GLenum));
      let shader_source: Vec<u8>;
      
      match &shader_stage.m_source {
        EnumShaderSource::FromFile(file_path_str) => {
          let file_path = std::path::Path::new(file_path_str.as_str());
          if Shader::check_cache(file_path).is_ok() {
            shader_stage.m_is_cached = true;
            self.m_shader_ids.push(shader_id);
            continue;
          }
          shader_stage.m_is_cached = false;
          shader_source = std::fs::read(file_path)?;
        }
        EnumShaderSource::FromStr(literal_source) => {
          shader_source = literal_source.clone().into_bytes();
        }
      }
      
      let c_str: std::ffi::CString = std::ffi::CString::new(shader_source)
        .expect("[Shader] -->\t Could not convert shader string in GlShader::source() from &str \
       to CString!");
      
      check_gl_call!("Shader", gl::ShaderSource(shader_id, 1, &(c_str.as_ptr()), std::ptr::null()));
      self.m_shader_ids.push(shader_id);
    }
    
    return Ok(());
  }
  
  fn compile(&mut self) -> Result<(), EnumError> {
    for index in 0..self.m_shader_ids.len() {
      if self.m_shader_stages[index].m_is_cached {
        match &self.m_shader_stages[index].m_source {
          EnumShaderSource::FromFile(file_path_str) => {
            let file_path = std::path::Path::new(&file_path_str);
            let mut buffer: Vec<u8> = Shader::check_cache(file_path)?;
            check_gl_call!("Shader", gl::ShaderBinary(1, &self.m_shader_ids[index],
              gl46::gl_enumerations::GL_SHADER_BINARY_FORMAT_SPIR_V.0, buffer.as_mut_ptr() as *mut std::ffi::c_void,
              buffer.len() as GLsizei));
            // Specialize the shader (specify the entry point)
            check_gl_call!("Shader", S_GL_4_6.as_ref().unwrap().SpecializeShader(self.m_shader_ids[index],
              "main".as_ptr(), 0, std::ptr::null(), std::ptr::null()));
            continue;
          }
          EnumShaderSource::FromStr(_) => todo!()
        }
      } else {
        // Compile and link.
        check_gl_call!("Shader", gl::CompileShader(self.m_shader_ids[index]));
      }
      
      // Error checking.
      let mut compiled_successfully: GLint = 0;
      let mut buffer_length: GLint = 0;
      
      unsafe { gl::GetShaderiv(self.m_shader_ids[index], gl::COMPILE_STATUS, &mut compiled_successfully) };
      if compiled_successfully as GLboolean == gl::FALSE {
        let shader_type_str: String;
        // For debug purposes.
        match self.m_shader_stages[index].m_type {
          EnumShaderType::Vertex => {
            shader_type_str = "vertex shader".to_string();
          }
          EnumShaderType::Fragment => {
            shader_type_str = "fragment shader".to_string();
          }
          EnumShaderType::Compute => {
            shader_type_str = "compute shader".to_string();
          }
        }
        
        // Get info length.
        unsafe {
          gl::GetShaderiv(self.m_shader_ids[index], gl::INFO_LOG_LENGTH,
            &mut buffer_length as *mut _)
        };
        let mut buffer: Vec<GLchar> = Vec::with_capacity(buffer_length as usize);
        
        unsafe {
          gl::GetShaderInfoLog(self.m_shader_ids[index], buffer_length, &mut buffer_length,
            buffer.as_mut_ptr())
        };
        
        log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Error, could not compile {0}!\n \
        Info => {1}", shader_type_str, unsafe {
        std::ffi::CStr::from_ptr(buffer.as_ptr()).to_str().unwrap()
      });
        return Err(EnumError::ShaderSyntaxError);
      }
    }
    
    return Ok(());
  }
  
  fn send(&mut self) -> Result<(), EnumError> {
    if self.m_shader_stages.is_empty() {
      log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot send shader : No shader stages \
      provided!");
    }
    
    check_gl_call!("Shader", self.m_program_id = gl::CreateProgram());
    
    self.source()?;
    self.compile()?;
    
    // Attach shaders to program.
    for index in 0..self.m_shader_ids.len() {
      check_gl_call!("Shader", gl::AttachShader(self.m_program_id, self.m_shader_ids[index]));
    }
    
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
      return Err(EnumError::ProgramCreationError);
    }
    
    // Delete shaders CPU-side, since we uploaded it to the GPU VRAM.
    for index in 0..self.m_shader_ids.len() {
      check_gl_call!("Shader", gl::DeleteShader(self.m_shader_ids[index]));
    }
    
    // Validate program.
    check_gl_call!("Shader", gl::ValidateProgram(self.m_program_id));
    return Ok(());
  }
  
  fn to_string(&self) -> String {
    todo!()
  }
  
  fn upload_data(&mut self, uniform_name: &'static str, uniform: &dyn std::any::Any) -> Result<(), EnumError> {
    match self.bind() {
      Ok(_) => {}
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Could not use shader {0}!", self.m_program_id);
        return Err(err);
      }
    }
    
    if !self.m_uniform_cache.contains_key(uniform_name) {
      let c_str: std::ffi::CString = std::ffi::CString::new(uniform_name)
        .expect("[Shader] -->\t Error converting str to CString when trying to upload uniform!");
      
      check_gl_call!("Shader", let new_uniform: GLint = gl::GetUniformLocation(self.m_program_id, c_str.as_ptr()));
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
    return self.m_program_id;
  }
  
  fn on_delete(&mut self) -> Result<(), EnumError> {
    Ok(unsafe {
      let renderer = Renderer::get()
        .expect("[Shader] -->\t Cannot drop GlShader, renderer is None! Exiting...");
      
      if (*renderer).m_type == EnumApi::OpenGL && (*renderer).m_state != EnumState::Shutdown {
        gl::UseProgram(0);
        gl::DeleteProgram(self.m_program_id);
      }
    })
  }
}

impl GlShader {
  pub fn bind(&self) -> Result<(), EnumError> {
    check_gl_call!("Shader", gl::UseProgram(self.m_program_id));
    return Ok(());
  }
}