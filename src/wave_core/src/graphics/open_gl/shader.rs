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
use std::collections::{HashMap, HashSet};

use gl::types::{GLenum, GLsizei};

use crate::check_gl_call;
#[cfg(feature = "debug")]
use crate::Engine;
use crate::graphics::open_gl::buffer::{GLboolean, GLchar, GLfloat, GLint, GLuint};
use crate::graphics::open_gl::renderer::S_GL_4_6;
use crate::graphics::renderer::{EnumRendererApi};
use crate::graphics::shader::{self, EnumShaderSource, EnumShaderStageType, ShaderStage, TraitShader};
use crate::math::Mat4;
use crate::S_ENGINE;
use crate::utils::macros::logger::*;

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
  ShaderBinaryCompilationError,
  NoBinaryFormatsError,
  UnsupportedUniformType,
  UniformNotFound,
  OpenGLApiError,
}

#[derive(Debug, Clone)]
pub struct GlShader {
  pub(crate) m_program_id: u32,
  m_shader_ids: HashMap<EnumShaderStageType, GLuint>,
  m_shader_stages: HashSet<ShaderStage>,
  m_uniform_cache: HashMap<&'static str, GLint>,
}

impl TraitShader for GlShader {
  fn new(shader_stages: Vec<ShaderStage>) -> Self {
    return GlShader {
      m_program_id: 0,
      m_shader_ids: HashMap::with_capacity(shader_stages.len()),
      m_shader_stages: HashSet::from_iter(shader_stages.into_iter()),
      m_uniform_cache: Default::default(),
    };
  }
  
  fn from(_other_shader: Self) -> Self where Self: Sized {
    todo!()
  }
  
  fn get_name(&self) -> EnumRendererApi {
    return EnumRendererApi::OpenGL;
  }
  
  fn source(&mut self) -> Result<(), shader::EnumShaderError> {
    for shader_stage in self.m_shader_stages.iter() {
      check_gl_call!("GlShader", let shader_id: GLuint = gl::CreateShader(shader_stage.m_stage as GLenum));
      if shader_id == 0 {
        log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Cannot create shader of type {0} : Error \
        => ShaderType is not an accepted value!", shader_stage.m_stage);
        return Err(shader::EnumShaderError::from(EnumError::ShaderSourcingError));
      }
      let shader_source: Vec<u8>;
      
      match &shader_stage.m_source {
        EnumShaderSource::FromFile(file_path_str) => {
          let file_path = std::path::Path::new(file_path_str);
          if shader_stage.cache_status() {
            self.m_shader_ids.insert(shader_stage.m_stage, shader_id);
            continue;
          }
          shader_source = std::fs::read(file_path)?;
        }
        EnumShaderSource::FromStr(literal_source) => {
          shader_source = literal_source.as_bytes().to_vec();
        }
      }
      
      let c_str: std::ffi::CString = std::ffi::CString::new(shader_source)
        .expect("[GlShader] -->\t Could not convert shader string in GlShader::source() from Vec<u8> \
       to CString!");
      
      check_gl_call!("GlShader", gl::ShaderSource(shader_id, 1, &(c_str.as_ptr()), std::ptr::null()));
      self.m_shader_ids.insert(shader_stage.m_stage, shader_id);
    }
    
    return Ok(());
  }
  
  fn compile(&mut self) -> Result<(), shader::EnumShaderError> {
    let mut cached_shader_array: Vec<ShaderStage> = Vec::with_capacity(self.m_shader_stages.len());
    
    for shader_stage in self.m_shader_stages.iter() {
      let shader_id = self.m_shader_ids.get(&shader_stage.m_stage).unwrap();
      if shader_stage.cache_status() {
        log!(EnumLogColor::Blue, "WARN", "[GlShader] -->\t Cached shader {0} found, not compiling it...", shader_stage.m_source);
        cached_shader_array.push(shader_stage.clone());
        continue;
      } else {
        log!(EnumLogColor::Yellow, "WARN", "[GlShader] -->\t Shader {0} not cached or modified, compiling it...",
          shader_stage.m_source);
        
        // Compile and link.
        check_gl_call!("GlShader", gl::CompileShader(*shader_id));
      }
      
      // Error checking.
      let mut compiled_successfully: GLint = 0;
      let mut buffer_length: GLint = 0;
      
      unsafe { gl::GetShaderiv(*shader_id, gl::COMPILE_STATUS, &mut compiled_successfully) };
      if compiled_successfully as GLboolean == gl::FALSE {
        #[allow(unused)]
          let _shader_type_str: String;
        // For debug purposes.
        match shader_stage.m_stage {
          EnumShaderStageType::Vertex => {
            _shader_type_str = "vertex shader".to_string();
          }
          EnumShaderStageType::Fragment => {
            _shader_type_str = "fragment shader".to_string();
          }
          EnumShaderStageType::Geometry => {
            _shader_type_str = "geometry shader".to_string();
          }
          EnumShaderStageType::Compute => {
            _shader_type_str = "compute shader".to_string();
          }
        }
        
        // Get info length.
        unsafe {
          gl::GetShaderiv(*shader_id, gl::INFO_LOG_LENGTH,
            &mut buffer_length as *mut _)
        };
        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_length as usize);
        
        unsafe {
          gl::GetShaderInfoLog(*shader_id, buffer_length, &mut buffer_length, buffer.as_mut_ptr().cast())
        };
        
        log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Error, could not compile {0}!\n \
        Info => {1}", _shader_type_str, unsafe {
        std::ffi::CStr::from_ptr(buffer.as_ptr().cast()).to_str()
          .expect("[GlShader] -->\t Cannot convert shader info log to Rust str in compile!")
      });
        return Err(shader::EnumShaderError::from(EnumError::ShaderSyntaxError));
      }
    }
    
    if !cached_shader_array.is_empty() {
      self.compile_binary(cached_shader_array)?;
    }
    
    return Ok(());
  }
  
  fn apply(&mut self) -> Result<(), shader::EnumShaderError> {
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
          self.m_program_id, unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()).to_str()
          .expect("[GlShader] -->\t Cannot convert shader info log to Rust str in apply!")
        });
      return Err(shader::EnumShaderError::from(EnumError::ProgramCreationError));
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
      format += format!("\n{0:117}[GlShader] |{1}| ({2}, {3})",
        "", shader_stage.m_stage, shader_stage.m_source, shader_stage.cache_status()).as_str();
    }
    return format;
  }
  
  fn upload_data(&mut self, uniform_name: &'static str, uniform: &dyn Any) -> Result<(), shader::EnumShaderError> {
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
        return Err(shader::EnumShaderError::from(EnumError::UniformNotFound));
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
      } else if uniform.is::<bool>() {
        let value_ptr = uniform.downcast_ref::<bool>().unwrap();
        check_gl_call!("GlShader", gl::Uniform1i(*self.m_uniform_cache.get(uniform_name).unwrap(), *value_ptr as i32));
      } else {
        log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Type of uniform '{0}' is unsupported for glsl!",
          uniform_name);
        return Err(shader::EnumShaderError::from(EnumError::UnsupportedUniformType));
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
  
  fn free(&mut self) -> Result<(), shader::EnumShaderError> {
    if gl::UseProgram::is_loaded() {
      check_gl_call!("GlShader", gl::UseProgram(0));
      check_gl_call!("GlShader", gl::DeleteProgram(self.m_program_id));
    }
    return Ok(());
  }
}

impl GlShader {
  pub fn bind(&self) -> Result<(), shader::EnumShaderError> {
    check_gl_call!("GlShader", gl::UseProgram(self.m_program_id));
    return Ok(());
  }
  
  fn compile_binary(&mut self, binary_shader_stages: Vec<ShaderStage>) -> Result<(), shader::EnumShaderError> {
    let gl4_6 = unsafe { S_GL_4_6.as_ref().unwrap() };
    let entry_point = std::ffi::CString::new("main").expect("Cannot convert entry point main to C str!");
    
    let mut supported_binary_format_count: GLint = 0;
    check_gl_call!("GlShader", gl::GetIntegerv(gl::NUM_SHADER_BINARY_FORMATS, &mut supported_binary_format_count));
    if supported_binary_format_count == 0 {
      log!(EnumLogColor::Red, "Error", "[GlShader] -->\t Cannot compile shader binary : No binary formats supported!");
      return Err(shader::EnumShaderError::from(EnumError::NoBinaryFormatsError));
    }
    
    let mut supported_binary_format_array: Vec<GLint> = Vec::with_capacity(supported_binary_format_count as usize);
    check_gl_call!("GlShader", gl::GetIntegerv(gl::SHADER_BINARY_FORMATS, supported_binary_format_array.as_mut_ptr()));
    
    unsafe { supported_binary_format_array.set_len(supported_binary_format_count as usize) };
    
    let mut bin_formats_str: String = String::from("[");
    for index in 0..supported_binary_format_array.len() {
      bin_formats_str += &format!("0x{0:X}", supported_binary_format_array[index]);
      if index + 1 != supported_binary_format_array.len() {
        bin_formats_str += ", ";
      }
    }
    bin_formats_str += "]";
    log!(EnumLogColor::Yellow, "WARN", "[GlShader] -->\t Shader binary formats supported => \
              {0}", bin_formats_str);
    
    for shader_stage in binary_shader_stages.into_iter() {
      let shader_id = self.m_shader_ids.get(&shader_stage.m_stage)
        .expect(format!("[GlShader] -->\t Cannot find shader ID for {0}", shader_stage.m_stage).as_str());
      
      match &shader_stage.m_source {
        EnumShaderSource::FromStr(_) => todo!(),
        EnumShaderSource::FromFile(file_path_str) => {
          let buffer: Vec<u8> = std::fs::read(file_path_str)?;
          
          check_gl_call!("GlShader", gl::ShaderBinary(1, shader_id,
              gl46::gl_enumerations::GL_SHADER_BINARY_FORMAT_SPIR_V.0,
              buffer.as_ptr() as *mut std::ffi::c_void, buffer.len() as GLsizei));
          
          let mut loaded_successfully: GLint = 0;
          check_gl_call!("GlShader", gl::GetShaderiv(*shader_id, gl46::gl_enumerations::GL_SPIR_V_BINARY.0, &mut loaded_successfully));
          
          if loaded_successfully == 0 {
            let mut buffer_length: GLint = 0;
            let mut info_length: GLint = 0;
            
            check_gl_call!("GlShader", gl::GetShaderiv(*shader_id, gl::INFO_LOG_LENGTH, &mut buffer_length));
            let mut buffer_c: Vec<u8> = Vec::with_capacity(buffer_length as usize);
            
            check_gl_call!("GlShader", gl::GetShaderInfoLog(*shader_id, buffer_length,
                  &mut info_length, buffer_c.as_mut_ptr() as *mut GLchar));
            
            unsafe { buffer_c.set_len(info_length as usize) };
            
            let _c_string = std::ffi::CString::new(buffer_c.as_slice())
              .expect("[GlShader] -->\t Cannot convert bytes to CString!");
            
            log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Error, could not compile {0}! \
                \nInfo => {1}", file_path_str, _c_string.to_str().unwrap_or("Error"));
            
            return Err(shader::EnumShaderError::ShaderBinaryError);
          }
          
          // Specialize the shader (specify the entry point)
          unsafe {
            gl4_6.SpecializeShader(*shader_id, entry_point.as_ptr().cast(), 0,
              std::ptr::null(), std::ptr::null());
          }
          
          let mut compiled_successfully: GLint = 0;
          check_gl_call!("GlShader", gl::GetShaderiv(*shader_id, gl::COMPILE_STATUS, &mut compiled_successfully));
          
          if compiled_successfully == 0 {
            let mut buffer_length: GLint = 0;
            
            check_gl_call!("GlShader", gl::GetShaderiv(*shader_id, gl::INFO_LOG_LENGTH, &mut buffer_length));
            let mut buffer_c: Vec<u8> = Vec::with_capacity(buffer_length as usize);
            
            check_gl_call!("GlShader", gl::GetShaderInfoLog(*shader_id, buffer_length,
                  &mut buffer_length, buffer_c.as_mut_ptr().cast()));
            
            log!(EnumLogColor::Red, "ERROR", "[GlShader] -->\t Error, could not compile {0}! \
                \nInfo => {1:?}", file_path_str, unsafe {
                std::ffi::CStr::from_ptr(buffer_c.as_ptr().cast()).to_str()
                .expect("[GlShader] -->\t Cannot convert shader info log to Rust str in compile_binary!")
                });
            
            return Err(shader::EnumShaderError::from(EnumError::ShaderBinaryCompilationError));
          }
        }
      }
    }
    return Ok(());
  }
}