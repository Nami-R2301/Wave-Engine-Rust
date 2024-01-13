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
use crate::wave::graphics::open_gl;
use crate::wave::graphics::open_gl::shader::GlShader;
use crate::wave::graphics::renderer::{EnumApi, Renderer};
#[cfg(feature = "Vulkan")]
use crate::wave::graphics::vulkan;
#[cfg(feature = "Vulkan")]
use crate::wave::graphics::vulkan::shader::VkShader;

#[derive(Debug, PartialEq)]
pub enum EnumError {
  InvalidApi,
  UnsupportedApiFunction,
  FileNotFound,
  PathError,
  UnsupportedFileType,
  ShaderNotCached,
  InvalidShaderSource,
  InvalidFileOperation,
  IoError(std::io::ErrorKind),
  OpenGLShaderError(open_gl::shader::EnumError),
  #[cfg(feature = "Vulkan")]
  VulkanShaderError(vulkan::shader::EnumError),
}

impl Display for EnumError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Shader] -->\t Error encountered with shader(s) : {:?}", self)
  }
}

impl From<std::io::Error> for EnumError {
  fn from(err: std::io::Error) -> Self {
    return EnumError::IoError(err.kind());
  }
}

impl From<open_gl::renderer::EnumError> for EnumError {
  fn from(_value: open_gl::renderer::EnumError) -> Self {
    return EnumError::OpenGLShaderError(open_gl::shader::EnumError::OpenGLApiError);
  }
}

impl From<open_gl::shader::EnumError> for EnumError {
  fn from(value: open_gl::shader::EnumError) -> Self {
    return EnumError::OpenGLShaderError(value);
  }
}

#[cfg(feature = "Vulkan")]
impl From<vulkan::shader::EnumError> for EnumError {
  fn from(value: vulkan::shader::EnumError) -> Self {
    return EnumError::VulkanShaderError(value);
  }
}

impl std::error::Error for EnumError {}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum EnumShaderType {
  Vertex = gl::VERTEX_SHADER,
  Fragment = gl::FRAGMENT_SHADER,
  Compute = gl::COMPUTE_SHADER,
}

impl Display for EnumShaderType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EnumShaderType::Vertex => write!(f, "Vertex"),
      EnumShaderType::Fragment => write!(f, "Fragment"),
      EnumShaderType::Compute => write!(f, "Compute")
    }
  }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash)]
pub enum EnumShaderSource {
  FromFile(String),
  FromStr(String),
}

impl Display for EnumShaderSource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EnumShaderSource::FromFile(file_path) => write!(f, "File : {0}", file_path),
      
      EnumShaderSource::FromStr(literal_source) => write!(f, "Literal : {0}", literal_source)
    }
  }
}

impl From<EnumShaderSource> for String {
  fn from(value: EnumShaderSource) -> Self {
    return match value {
      EnumShaderSource::FromFile(file_path_str) => file_path_str,
      EnumShaderSource::FromStr(literal_source_str) => literal_source_str,
    };
  }
}

pub trait TraitShader {
  fn default() -> Self where Self: Sized;
  fn new(shader_module: Vec<ShaderStage>) -> Result<Self, EnumError> where Self: Sized;
  fn from(other_shader: Self) -> Self where Self: Sized;
  fn get_name(&self) -> EnumApi;
  fn source(&mut self) -> Result<(), EnumError>;
  fn compile(&mut self) -> Result<(), EnumError>;
  fn submit(&mut self) -> Result<(), EnumError>;
  fn to_string(&self) -> String;
  fn upload_data(&mut self, uniform_name: &'static str, uniform: &dyn std::any::Any) -> Result<(), EnumError>;
  fn get_id(&self) -> u32;
  fn get_api_handle(&self) -> &dyn std::any::Any;
  fn on_delete(&mut self) -> Result<(), EnumError>;
}

#[derive(Debug, Clone, Hash)]
pub struct ShaderStage {
  pub m_type: EnumShaderType,
  pub m_source: EnumShaderSource,
  m_is_cached: bool,
}

impl ShaderStage {
  pub fn default() -> Self {
    return Self {
      m_type: EnumShaderType::Vertex,
      m_source: EnumShaderSource::FromStr(String::from("")),
      m_is_cached: false,
    };
  }
  
  pub fn new(shader_type: EnumShaderType, shader_source: EnumShaderSource) -> Self {
    // Try loading from cache.
    let mut is_cached = false;
    let mut source = shader_source.clone();
    match &shader_source {
      EnumShaderSource::FromFile(file_path_str) => {
        let file_path = std::path::Path::new(file_path_str);
        if Shader::check_cache(file_path).is_ok() {
          is_cached = true;
          source = EnumShaderSource::FromFile(format!("cache/{0}.spv",
            file_path.file_stem().unwrap().to_str().unwrap()));
        }
      }
      EnumShaderSource::FromStr(_) => {}
    }
    
    return Self {
      m_type: shader_type,
      m_source: source,
      m_is_cached: is_cached,
    };
  }
  
  pub fn cache_status(&self) -> &bool {
    return &self.m_is_cached;
  }
}

impl PartialEq<Self> for ShaderStage {
  fn eq(&self, other: &Self) -> bool {
    return self.m_type == other.m_type && self.m_source == other.m_source;
  }
}

impl Eq for ShaderStage {}

pub struct Shader {
  m_api_data: Box<dyn TraitShader>,
}

impl Shader {
  pub fn default() -> Self {
    return Self {
      m_api_data: Box::new(GlShader::default()),
    };
  }
  
  pub fn new(mut shader_stages: Vec<ShaderStage>) -> Result<Self, EnumError> {
    for shader_stage in shader_stages.iter_mut() {
      match &shader_stage.m_source {
        EnumShaderSource::FromFile(file_path_str) => {
          if file_path_str.is_empty() {
            log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot create shader : Invalid file \
            path given for shader source!");
            return Err(EnumError::InvalidShaderSource);
          }
          
          // Check if file exists and is a supported format.
          let file_path = std::path::Path::new(file_path_str.as_str());
          Shader::check_file_validity(file_path)?;
        }
        EnumShaderSource::FromStr(literal_string) => {
          if literal_string.is_empty() {
            log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot create shader : Empty source \
          string given for shader source!");
            return Err(EnumError::InvalidShaderSource);
          }
        }
      }
    }
    
    let renderer = Renderer::get().expect("Cannot retrieve active renderer!");
    let mut shader: Shader = Shader::default();
    
    match unsafe { (*renderer).m_type } {
      EnumApi::OpenGL => {
        shader.m_api_data = Box::new(GlShader::new(shader_stages)?);
      }
      #[cfg(feature = "Vulkan")]
      EnumApi::Vulkan => {
        shader.m_api_data = Box::new(VkShader::new(shader_stages)?);
      }
      #[cfg(not(feature = "Vulkan"))]
      EnumApi::Vulkan => {}
    }
    
    return Ok(shader);
  }
  
  pub fn check_cache(shader_file_path: &std::path::Path) -> Result<Vec<u8>, EnumError> {
    let renderer = Renderer::get().expect("Cannot retrieve active renderer!");
    unsafe {
      let renderer_api_version = (*renderer).get_max_shader_version_available();
      if (*renderer).m_type == EnumApi::OpenGL && renderer_api_version < 4.6 {
        log!(EnumLogColor::Yellow, "WARN", "[Shader] -->\t Cannot load from cached SPIR-V binary : \
          Current OpenGL renderer doesn't support the extension required 'ARB_gl_spirv', found \
          starting OpenGL version 4.6 and higher, current available version : {1}!\n{0:113}\
          Attempting to load from glsl shader directly...", "", renderer_api_version);
        return Err(EnumError::InvalidApi);
      }
    }
    
    let cache_path_str: String = format!("cache/{0}", shader_file_path.file_stem().unwrap()
      .to_str().unwrap());
    let buffer = std::fs::read(cache_path_str)?;
    
    return Ok(buffer);
  }
  
  pub fn cache(shader_name: &std::path::Path, binary: Vec<u8>) -> Result<(), EnumError> {
    let cache_path_str: String = format!("cache/{0}.spv", shader_name.file_name().unwrap().to_str().unwrap());
    
    std::fs::write(cache_path_str, binary.as_slice())?;
    return Ok(());
  }
  
  fn check_file_validity(file_path: &std::path::Path) -> Result<(), EnumError> {
    if !file_path.exists() {
      log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot open shader file : File {0:?} \
      does not exist!", file_path);
      return Err(EnumError::FileNotFound);
    }
    
    let shader_extension = file_path.extension().ok_or(EnumError::PathError)?;
    
    if shader_extension != "vert" && shader_extension != "frag" && shader_extension != "spv" {
      log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Error while opening shader : Unsupported \
      shader file '.{1:?}'!\n{0:113}Supported file types => '.vert', '.spv', '.frag'", "",
        shader_extension);
      return Err(EnumError::UnsupportedFileType);
    }
    return Ok(());
  }
  
  pub fn submit(&mut self) -> Result<(), EnumError> {
    return self.m_api_data.submit();
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
    return self.m_api_data.to_string();
  }
  
  pub fn on_delete(&mut self) -> Result<(), EnumError> {
    self.m_api_data.on_delete()?;
    return Ok(());
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    let renderer = Renderer::get();
    if renderer.is_some() {
      log!(EnumLogColor::Purple, "INFO", "[Shader] -->\t Dropping shader...");
      match self.on_delete() {
        Ok(_) => {
          log!(EnumLogColor::Green, "INFO", "[Shader] -->\t Dropped shader successfully...");
        }
        #[allow(unused)]
        Err(err) => {
          log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Error while dropping shader : \
        Error => {:?}", err);
          log!(EnumLogColor::Red, "INFO", "[Shader] -->\t Dropped shader unsuccessfully...");
        }
      }
    }
  }
}

impl Display for Shader {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Shader] -->\t Program ID => {1}\n{0:113}[Api] |Shader stage| (Source, Cached?) : {2}",
      "", self.m_api_data.get_id(), self.to_string())
  }
}
