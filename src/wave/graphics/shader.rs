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

use std::fmt::{Display, Formatter};
use crate::log;

use crate::wave::EnumApi;
use crate::wave::graphics::open_gl::shader::GlShader;
use crate::wave::graphics::open_gl::renderer::EnumOpenGLErrors;
use crate::wave::graphics::renderer::Renderer;

#[cfg(feature = "Vulkan")]
use crate::wave::graphics::vulkan::shader::VkShader;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumError {
  InvalidApi,
  ProgramCreationError,
  ShaderFileError,
  ShaderSyntaxError,
  ShaderTypeError,
  ShaderSourcing,
  ShaderCompilation,
  ShaderLinkage,
  UnsupportedUniformType,
  UniformNotFound,
  AnyConversionError,
  OpenGLError(EnumOpenGLErrors)
}

impl Display for EnumError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Shader] -->\t Error encountered with shader(s) : {:?}", self)
  }
}

impl std::error::Error for EnumError {

}

pub trait TraitShader {
  fn new(vertex_file_path: &'static str, fragment_file_path: &'static str) -> Result<Self, EnumError> where Self: Sized;
  fn from(other_shader: Self) -> Self where Self: Sized;
  fn compile(&self, shader_id: u32, shader_type: &dyn std::any::Any) -> Result<(), EnumError>;
  fn send(&mut self) -> Result<(), EnumError>;
  fn upload_data(&mut self, uniform_name: &'static str, uniform: &dyn std::any::Any) -> Result<(), EnumError>;
  fn get_id(&self) -> u32;
  fn to_string(&self) -> String;
}

pub struct Shader {
  m_api_data: Box<dyn TraitShader>,
  // For debug purposes.
  #[allow(unused)]
  m_uniform_cache: std::collections::HashMap<&'static str, i8>,
}

impl Shader {
  pub fn new(vertex_file_path: &'static str, fragment_file_path: &'static str) -> Result<Self, EnumError> {
    let renderer = Renderer::get();
    if renderer.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot create shader : No active renderer!");
      return Err(EnumError::InvalidApi);
    }
    
    return match renderer.as_ref().unwrap().get_type() {
      EnumApi::OpenGL => {
        Ok(Shader {
          m_api_data: Box::new(GlShader::new(vertex_file_path, fragment_file_path)?),
          m_uniform_cache: std::collections::HashMap::new(),
        })
      }
      EnumApi::Vulkan => {
        #[cfg(not(feature = "Vulkan"))]
        {
          log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot create shader : Vulkan not supported!");
          return Err(EnumError::InvalidApi);
        }
        
        #[cfg(feature = "Vulkan")]
        Ok(Shader {
          m_api_data: Box::new(VkShader::new(vertex_file_path, fragment_file_path)?),
          m_uniform_cache: std::collections::HashMap::new(),
        })
      }
    }
  }
  
  pub fn compile(&self, shader_id: u32, shader_type: &dyn std::any::Any) -> Result<(), EnumError> {
    return self.m_api_data.compile(shader_id, shader_type);
  }
  pub fn send(&mut self) -> Result<(), EnumError> {
    return self.m_api_data.send();
  }
  pub fn upload_data(&mut self, uniform_name: &'static str, uniform: &dyn std::any::Any) -> Result<(), EnumError> {
    return self.m_api_data.upload_data(uniform_name, uniform);
  }
  
  pub fn get_api(&self) -> &dyn TraitShader {
    return self.m_api_data.as_ref();
  }
  
  pub fn get_id(&self) -> u32 {
    return self.m_api_data.get_id();
  }
  
  pub fn to_string(&self) -> String {
    return format!("{0}", self.m_api_data.to_string())
  }
}
