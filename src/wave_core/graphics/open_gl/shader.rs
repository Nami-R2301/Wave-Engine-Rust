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

use std::any::Any;
use std::collections::{HashMap};

use gl::types::{GLenum, GLsizei};

use crate::{check_gl_call, log};
use crate::wave_core::graphics::open_gl::buffer::{GLboolean, GLchar, GLfloat, GLint, GLuint};
use crate::wave_core::graphics::open_gl::renderer::{S_GL_4_6};
use crate::wave_core::graphics::renderer::{EnumApi, Renderer, S_RENDERER};
use crate::wave_core::graphics::shader::{self, EnumShaderSource, EnumShaderType, ShaderStage, TraitShader};
use crate::wave_core::math::Mat4;
use crate::wave_core::utils::into_rust_string;

/*
///////////////////////////////////   OpenGL shader    ///////////////////////////////////
///////////////////////////////////                    ///////////////////////////////////
///////////////////////////////////                    ///////////////////////////////////
 */

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum EnumError {
  ProgramCreationError,
  ShaderFileError,
  ShaderCachingError,
  ShaderSyntaxError,
  ShaderTypeError,
  ShaderSourcingError,
  ShaderCompilationError,
  ShaderLinkageError,
  ShaderModuleError,
  ShaderBinaryError,
  UnsupportedUniformType,
  UniformNotFound,
  OpenGLApiError,
}

#[derive(Debug, Clone)]
pub struct GlShader {
  pub m_program_id: u32,
  m_shader_ids: HashMap<EnumShaderType, GLuint>,
  m_shader_stages: Vec<ShaderStage>,
  m_uniform_cache: HashMap<&'static str, GLint>,
}

impl TraitShader for GlShader {
  fn default() -> Self {
    return Self {
      m_program_id: 0,
      m_shader_ids: HashMap::with_capacity(3),
      m_shader_stages: Vec::with_capacity(3),
      m_uniform_cache: Default::default(),
    };
  }
  
  fn new(shader_stages: Vec<ShaderStage>) -> Result<Self, shader::EnumError> {
    return Ok(GlShader {
      m_program_id: 0,
      m_shader_ids: HashMap::with_capacity(shader_stages.len()),
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
  
  fn source(&mut self) -> Result<(), shader::EnumError> {
    for shader_stage in self.m_shader_stages.iter() {
      check_gl_call!("GlShader", let shader_id: GLuint = gl::CreateShader(shader_stage.m_type as GLenum));
      let shader_source: Vec<u8>;
      
      match &shader_stage.m_source {
        EnumShaderSource::FromFile(file_path_str) => {
          let file_path = std::path::Path::new(file_path_str);
          if shader_stage.cache_status() {
            self.m_shader_ids.insert(shader_stage.m_type, shader_id);
            continue;
          }
          shader_source = std::fs::read(file_path)?;
        }
        EnumShaderSource::FromStr(literal_source) => {
          shader_source = literal_source.as_bytes().to_vec();
        }
      }
      
      let c_str: std::ffi::CString = std::ffi::CString::new(shader_source)
        .expect("[GlShader] -->\t Could not convert shader string in GlShader::source() from &str \
       to CString!");
      
      check_gl_call!("GlShader", gl::ShaderSource(shader_id, 1, &(c_str.as_ptr()), std::ptr::null()));
      self.m_shader_ids.insert(shader_stage.m_type, shader_id);
    }
    
    return Ok(());
  }
  
  fn compile(&mut self) -> Result<(), shader::EnumError> {
    let mut cached_shader_array: Vec<ShaderStage> = Vec::with_capacity(self.m_shader_stages.len());
    
    for shader_stage in self.m_shader_stages.iter() {
      let shader_id = self.m_shader_ids.get(&shader_stage.m_type)
        .expect(format!("[GlShader] -->\t Cannot find shader ID for {0}", shader_stage.m_type).as_str());
      if shader_stage.cache_status() {
        log!(EnumLogColor::Blue, "WARN", "[GlShader] -->\t Cached shader {0} found, \
            not compiling it.", shader_stage.m_source);
        cached_shader_array.push(shader_stage.clone());
        continue;
      } else {
        log!(EnumLogColor::Yellow, "WARN", "[GlShader] -->\t Cached shader {0} not found, \
            compiling it.", shader_stage.m_source);
        
        // Compile and link.
        check_gl_call!("GlShader", gl::CompileShader(*shader_id));
      }
      
      // Error checking.
      let mut compiled_successfully: GLint = 0;
      let mut buffer_length: GLint = 0;
      
      unsafe { gl::GetShaderiv(*shader_id, gl::COMPILE_STATUS, &mut compiled_successfully) };
      if compiled_successfully as GLboolean == gl::FALSE {
        #[allow(unused)]
          let shader_type_str: String;
        // For debug purposes.
        match shader_stage.m_type {
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
          gl::GetShaderiv(*shader_id, gl::INFO_LOG_LENGTH,
            &mut buffer_length as *mut _)
        };
        let mut buffer: Vec<GLchar> = Vec::with_capacity(buffer_length as usize);
        
        unsafe {
          gl::GetShaderInfoLog(*shader_id, buffer_length, &mut buffer_length, buffer.as_mut_ptr())
        };
        
        log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Error, could not compile {0}!\n \
        Info => {1}", shader_type_str, unsafe {
        std::ffi::CStr::from_ptr(buffer.as_ptr()).to_str().unwrap()
      });
        return Err(shader::EnumError::from(EnumError::ShaderSyntaxError));
      }
    }
    
    if !cached_shader_array.is_empty() {
      self.compile_binary(cached_shader_array)?;
    }
    
    return Ok(());
  }
  
  fn submit(&mut self) -> Result<(), shader::EnumError> {
    if self.m_shader_stages.is_empty() {
      log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Cannot send shader : No shader stages \
      provided!");
    }
    
    check_gl_call!("GlShader", self.m_program_id = gl::CreateProgram());
    
    self.source()?;
    self.compile()?;
    
    // Attach shaders to program.
    for (_shader_type, id) in self.m_shader_ids.iter() {
      check_gl_call!("GlShader", gl::AttachShader(self.m_program_id, *id));
    }
    
    // Link program.
    check_gl_call!("GlShader", gl::LinkProgram(self.m_program_id));
    
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
      log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Error linking program {0}! Error => {1}",
          self.m_program_id, unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()).to_str().unwrap() });
      return Err(shader::EnumError::from(EnumError::ProgramCreationError));
    }
    
    // Delete shaders CPU-side, since we uploaded it to the GPU VRAM.
    for (_shader_type, id) in self.m_shader_ids.iter() {
      check_gl_call!("GlShader", gl::DeleteShader(*id));
    }
    
    // Validate program.
    check_gl_call!("GlShader", gl::ValidateProgram(self.m_program_id));
    return Ok(());
  }
  
  fn to_string(&self) -> String {
    let mut format: String = String::from("");
    
    for shader_stage in self.m_shader_stages.iter() {
      format += format!("\n{0:115}[GlShader] |{1}| ({2}, {3})",
        "", shader_stage.m_type, shader_stage.m_source, shader_stage.cache_status()).as_str();
    }
    return format;
  }
  
  fn upload_data(&mut self, uniform_name: &'static str, uniform: &dyn std::any::Any) -> Result<(), shader::EnumError> {
    match self.bind() {
      Ok(_) => {}
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Could not use shader {0}!", self.m_program_id);
        return Err(err);
      }
    }
    
    if !self.m_uniform_cache.contains_key(uniform_name) {
      let c_str: std::ffi::CString = std::ffi::CString::new(uniform_name)
        .expect("[GlShader] -->\t Error converting str to CString when trying to upload uniform!");
      
      check_gl_call!("GlShader", let new_uniform: GLint = gl::GetUniformLocation(self.m_program_id, c_str.as_ptr()));
      if new_uniform == -1 {
        log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Could not upload uniform '{0}'!",
        uniform_name);
        return Err(shader::EnumError::from(EnumError::UniformNotFound));
      }
      self.m_uniform_cache.insert(uniform_name, new_uniform);
      
      
      if uniform.is::<u32>() {
        let value_ptr = uniform.downcast_ref::<u32>().unwrap();
        check_gl_call!("GlShader", gl::Uniform1ui(*self.m_uniform_cache.get(uniform_name).unwrap(), *value_ptr));
      } else if uniform.is::<i32>() {
        let value_ptr = uniform.downcast_ref::<i32>().unwrap();
        check_gl_call!("GlShader", gl::Uniform1i(*self.m_uniform_cache.get(uniform_name).unwrap(), *value_ptr));
      } else if uniform.is::<f32>() {
        let value_ptr = uniform.downcast_ref::<f32>().unwrap();
        check_gl_call!("GlShader", gl::Uniform1f(*self.m_uniform_cache.get(uniform_name).unwrap(), *value_ptr));
      } else if uniform.is::<f64>() {
        let value_ptr = uniform.downcast_ref::<f64>().unwrap();
        check_gl_call!("GlShader", gl::Uniform1d(*self.m_uniform_cache.get(uniform_name).unwrap(), *value_ptr));
      } else if uniform.is::<Mat4>() {
        let value_ptr = uniform.downcast_ref::<Mat4>().unwrap();
        check_gl_call!("GlShader", gl::UniformMatrix4fv(*self.m_uniform_cache.get(uniform_name).unwrap(),
          1, gl::FALSE, (value_ptr.as_array().as_ptr()) as *const GLfloat));
      } else {
        log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Type of uniform '{0}' is unsupported for glsl!",
          uniform_name);
        return Err(shader::EnumError::from(EnumError::UnsupportedUniformType));
      }
    }
    return Ok(());
  }
  
  fn get_id(&self) -> u32 {
    return self.m_program_id;
  }
  
  fn get_api_handle(&self) -> &dyn Any {
    return self;
  }
  
  fn on_delete(&mut self, active_renderer: *mut Renderer) -> Result<(), shader::EnumError> {
    unsafe {
      if (*active_renderer).m_type != EnumApi::OpenGL {
        log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Cannot delete shader : Renderer is not OpenGL!");
        return Err(shader::EnumError::InvalidApi);
      }
      
      gl::UseProgram(0);
      gl::DeleteProgram(self.m_program_id);
    }
    return Ok(());
  }
}

impl GlShader {
  pub fn bind(&self) -> Result<(), shader::EnumError> {
    check_gl_call!("GlShader", gl::UseProgram(self.m_program_id));
    return Ok(());
  }
  
  fn compile_binary(&mut self, shader_stages: Vec<ShaderStage>) -> Result<(), shader::EnumError> {
    let gl4_6 = unsafe { S_GL_4_6.as_ref().unwrap() };
    let entry_point = "main";
    
    for shader_stage in shader_stages.into_iter() {
      let shader_id = self.m_shader_ids.get(&shader_stage.m_type)
        .expect(format!("[GlShader] -->\t Cannot find shader ID for {0}", shader_stage.m_type).as_str());
      
      match &shader_stage.m_source {
        EnumShaderSource::FromStr(_) => todo!(),
        EnumShaderSource::FromFile(file_path_str) => {
          let buffer: Vec<u8> = std::fs::read(file_path_str)?;
          log!(EnumLogColor::Yellow, "DEBUG", "[GlShader] -->\t Attempting to read cache of {0}!", file_path_str);
          
          check_gl_call!("GlShader", gl::ShaderBinary(1, shader_id,
              gl46::gl_enumerations::GL_SHADER_BINARY_FORMAT_SPIR_V.0,
              buffer.as_ptr() as *mut std::ffi::c_void, buffer.len() as GLsizei));
          
          // Specialize the shader (specify the entry point)
          check_gl_call!("GlShader", gl4_6.SpecializeShader(*shader_id, entry_point.as_ptr(), 0, std::ptr::null(), std::ptr::null()));
          
          let mut buffer_length: GLint = 0;
          let mut info_length: GLint = 0;
          let mut compiled_successfully: GLint = 0;
          check_gl_call!("GlShader", gl::GetShaderiv(*shader_id, gl::COMPILE_STATUS, &mut compiled_successfully));
          
          if compiled_successfully == 0 {
            check_gl_call!("GlShader", gl::GetShaderiv(*shader_id, gl::INFO_LOG_LENGTH, &mut buffer_length));
            let mut buffer_c: Vec<u8> = Vec::with_capacity(buffer_length as usize);
            
            check_gl_call!("GlShader", gl::GetShaderInfoLog(*shader_id, buffer_length,
                  &mut info_length, buffer_c.as_mut_ptr() as *mut GLchar));
            
            unsafe { buffer_c.set_len(info_length as usize) };
            
            let rust_str = into_rust_string(buffer_c.as_slice()).expect("Cannot convert bytes to Rust str");
            
            log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Error, could not compile {0}! \
                \nInfo => {1}", file_path_str, rust_str);
            
            return Err(shader::EnumError::ShaderBinaryError);
          }
        }
      }
    }
    return Ok(());
  }
}