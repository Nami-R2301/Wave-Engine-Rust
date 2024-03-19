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
use crate::wave_core::Engine;
use crate::wave_core::graphics::open_gl;
use crate::wave_core::graphics::open_gl::shader::GlShader;
use crate::wave_core::graphics::renderer::{EnumApi, Renderer};
#[cfg(feature = "vulkan")]
use crate::wave_core::graphics::vulkan;
#[cfg(feature = "vulkan")]
use crate::wave_core::graphics::vulkan::shader::VkShader;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumState {
  NotCreated,
  Created,
  Sourced,
  Compiled,
  Sent,
  Freed,
  Deleted,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumShaderLanguageType {
  Glsl,
  GlslSpirV,
  SpirV,
  Binary,
}

#[derive(Debug, PartialEq)]
pub enum EnumError {
  NoShaderStagesProvided,
  NoActiveRendererError,
  InvalidApi,
  UnsupportedApiFunction,
  FileNotFound,
  PathError,
  UnsupportedFileType,
  UnsupportedFileVersion,
  ShaderNotCached,
  ShaderModified,
  ShaderBinaryError,
  InvalidShaderSource,
  InvalidFileOperation,
  IoError(std::io::ErrorKind),
  OpenGLShaderError(open_gl::shader::EnumError),
  #[cfg(feature = "vulkan")]
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

#[cfg(feature = "vulkan")]
impl From<vulkan::shader::EnumError> for EnumError {
  fn from(value: vulkan::shader::EnumError) -> Self {
    return EnumError::VulkanShaderError(value);
  }
}

impl std::error::Error for EnumError {}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum EnumShaderStage {
  Vertex = gl::VERTEX_SHADER,
  Fragment = gl::FRAGMENT_SHADER,
  Compute = gl::COMPUTE_SHADER,
}

impl Display for EnumShaderStage {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EnumShaderStage::Vertex => write!(f, "Vertex"),
      EnumShaderStage::Fragment => write!(f, "Fragment"),
      EnumShaderStage::Compute => write!(f, "Compute")
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
  fn free(&mut self, active_renderer: *mut Renderer) -> Result<(), EnumError>;
}

#[derive(Debug, Clone, Hash)]
pub struct ShaderStage {
  pub(crate) m_stage: EnumShaderStage,
  pub(crate) m_source: EnumShaderSource,
  m_is_cached: bool,
}

impl ShaderStage {
  pub fn default(stage: EnumShaderStage) -> Self {
    return Self {
      m_stage: stage,
      m_source: (stage == EnumShaderStage::Vertex)
        .then(|| EnumShaderSource::FromFile("res/shaders/glsl_420.vert".to_string()))
        .unwrap_or(EnumShaderSource::FromFile("res/shaders/glsl_420.frag".to_string())),
      m_is_cached: false,
    };
  }
  
  pub fn new(shader_type: EnumShaderStage, shader_source: EnumShaderSource) -> Self {
    return Self {
      m_stage: shader_type,
      m_source: shader_source,
      m_is_cached: false,
    };
  }
  
  pub fn cache_status(&self) -> bool {
    return self.m_is_cached;
  }
}

impl PartialEq<Self> for ShaderStage {
  fn eq(&self, other: &Self) -> bool {
    return self.m_stage == other.m_stage && self.m_source == other.m_source;
  }
}

impl Eq for ShaderStage {}

pub struct Shader {
  m_state: EnumState,
  m_version: u16,
  m_shader_lang: EnumShaderLanguageType,
  m_api_data: Box<dyn TraitShader>,
}

impl Shader {
  pub fn default() -> Self {
    return Self {
      m_state: EnumState::NotCreated,
      m_version: 420,
      m_shader_lang: EnumShaderLanguageType::Glsl,
      m_api_data: Box::new(GlShader::default()),
    };
  }
  
  pub fn new(mut shader_stages: Vec<ShaderStage>) -> Result<Self, EnumError> {
    if shader_stages.is_empty() {
      log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot create shader : No shader stages \
        provided!");
      return Err(EnumError::NoShaderStagesProvided);
    }
    
    let renderer: &mut Renderer = Engine::get_active_renderer();
    let mut shader_program: Shader = Shader::default();
    let mut previous_cache_status: bool = false;
    
    for (position, shader_stage) in shader_stages.iter_mut().enumerate() {
      Self::check_validity(&shader_stage)?;
      shader_program.m_version = Self::check_version_compatibility(&shader_stage)?;
      
      match &shader_stage.m_source {
        EnumShaderSource::FromFile(file_path_str) => {
          let file_path = std::path::Path::new(file_path_str.as_str());
          
          // Try loading from cache.
          if Shader::check_cache(file_path).is_ok() {
            shader_stage.m_is_cached = true;
            
            // If one shader stage is non-binary, make the rest uniform.
            if position != 0 && shader_stage.m_is_cached != previous_cache_status {
              shader_stage.m_is_cached = previous_cache_status;
            }
            previous_cache_status = shader_stage.m_is_cached;
            
            if shader_stage.m_is_cached {
              let mut cache_path_str: String = format!("cache/{0}", file_path.file_name()
                .ok_or(EnumError::InvalidFileOperation)?
                .to_str()
                .ok_or(EnumError::InvalidFileOperation)?);
              if file_path.extension().ok_or(EnumError::InvalidFileOperation)? != "spv" {
                cache_path_str += ".spv";
              }
              
              shader_stage.m_source = EnumShaderSource::FromFile(cache_path_str);
            }
          }
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
    
    shader_program.parse_language(shader_stages.get(0).unwrap())?;
    
    match renderer.m_type {
      EnumApi::OpenGL => {
        shader_program.m_api_data = Box::new(GlShader::new(shader_stages)?);
      }
      #[cfg(feature = "vulkan")]
      EnumApi::Vulkan => {
        shader_program.m_api_data = Box::new(VkShader::new(shader_stages)?);
      }
    }
    
    shader_program.m_state = EnumState::Created;
    return Ok(shader_program);
  }
  
  pub fn check_cache(shader_file_path: &std::path::Path) -> Result<Vec<u8>, EnumError> {
    let renderer: &mut Renderer = Engine::get_active_renderer();
    if renderer.m_type == EnumApi::OpenGL && !renderer.check_extension("GL_ARB_gl_spirv") {
      log!(EnumLogColor::Yellow, "WARN", "[Shader] -->\t Cannot load from cached SPIR-V binary : \
          \n{0:113}Current OpenGL renderer doesn't support the extension required 'GL_ARB_gl_spirv'\
          \n{0:113}Attempting to load from glsl shader directly...", "");
      return Err(EnumError::InvalidApi);
    }
    
    let cache_path: &std::path::Path;
    let mut cache_path_str: String = format!("cache/{0}", shader_file_path.file_name()
      .ok_or(EnumError::InvalidFileOperation)?
      .to_str()
      .ok_or(EnumError::InvalidFileOperation)?);
    if shader_file_path.extension().ok_or(EnumError::InvalidFileOperation)? != "spv" {
      cache_path_str += ".spv";
    }
    
    cache_path = std::path::Path::new(&cache_path_str);
    
    let src_last_time_modified = shader_file_path.metadata()?.modified()?;
    let cache_last_time_modified = cache_path.metadata()?.modified()?;
    if src_last_time_modified > cache_last_time_modified {
      log!(EnumLogColor::Yellow, "WARN", "[Shader] -->\t Shader source modified since last cache, recompiling shader stages...");
      return Err(EnumError::ShaderModified);
    }
    
    let cache_buffer = std::fs::read(cache_path)?;
    return Ok(cache_buffer);
  }
  
  pub fn cache(shader_name: &std::path::Path, binary: Vec<u8>) -> Result<(), EnumError> {
    let cache_path_str: String = format!("cache/{0}.spv", shader_name.file_name().unwrap().to_str().unwrap());
    
    std::fs::write(cache_path_str, binary.as_slice())?;
    return Ok(());
  }
  
  pub fn get_version(&self) -> u16 {
    return self.m_version;
  }
  
  fn check_validity(shader_stage: &ShaderStage) -> Result<(), EnumError> {
    match &shader_stage.m_source {
      EnumShaderSource::FromFile(file_path_str) => {
        let file_path = std::path::Path::new(file_path_str);
        if !file_path.exists() {
          log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot open shader file : File {0:?} does not exist!", file_path);
          return Err(EnumError::FileNotFound);
        }
        
        let shader_extension = file_path.extension().ok_or(EnumError::PathError)?;
        if shader_extension != "vert" && shader_extension != "frag" && shader_extension != "spv" {
          log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Error while opening shader : Unsupported \
      shader file '.{1:?}'!\n{0:113}Supported file types => '.vert', '.spv', '.frag'", "",
        shader_extension);
          return Err(EnumError::UnsupportedFileType);
        }
        
        let source = std::fs::read_to_string(file_path_str)?;
        if !source.contains("#version") || !source.contains("void main()") {
          return Err(EnumError::InvalidShaderSource);
        }
      }
      EnumShaderSource::FromStr(literal_str) => {
        if !literal_str.contains("#version") || !literal_str.contains("void main()") {
          return Err(EnumError::InvalidShaderSource);
        }
      }
    }
    
    return Ok(());
  }
  
  fn check_version_compatibility(shader_stage: &ShaderStage) -> Result<u16, EnumError> {
    let source: String;
    match &shader_stage.m_source {
      EnumShaderSource::FromFile(file_path_str) => {
        source = std::fs::read_to_string(file_path_str)?;
      }
      EnumShaderSource::FromStr(literal_str) => {
        source = literal_str.clone();
      }
    }
    
    let version_number_str = source.split_once("#version")
      .expect("[Shader] -->\t Cannot split GLSL #version and version number in set_language()!");
    let version_number: u16 = version_number_str.1.get(1..4)
      .expect("[Shader] -->\t Cannot get GLSL version number in set_language()!")
      .parse()
      .expect(&format!("[Shader] -->\t Cannot parse GLSL version number {0} in set_language()!",
        version_number_str.1.get(1..4).unwrap()));
    
    
    let renderer: &mut Renderer = Engine::get_active_renderer();
    
    if renderer.m_type == EnumApi::OpenGL {
      let max_version = renderer.get_max_shader_version_available();
      // If the shader source version number is higher than supported.
      if max_version < version_number {
        // Try loading a source file with the appropriate version number.
        log!(EnumLogColor::Yellow, "WARN", "[Shader] -->\t Attempting to load a source file compatible with glsl {0}", max_version);
        std::fs::read_to_string(&source.replace("#version 420", &("#version ".to_owned() + &max_version.to_string())))?;
        return Ok(max_version);
      }
    }
    return Ok(renderer.get_max_shader_version_available());
  }
  
  fn parse_language(&mut self, shader_stage: &ShaderStage) -> Result<(), EnumError> {
    let source: String;
    match &shader_stage.m_source {
      EnumShaderSource::FromFile(file_path_str) => {
        let file_path = std::path::Path::new(&file_path_str);
        
        if !file_path.exists() {
          return Err(EnumError::FileNotFound);
        }
        
        if file_path.extension().unwrap() == "bin" {
          self.m_shader_lang = EnumShaderLanguageType::Binary;
          return Ok(());
        } else if file_path.extension().unwrap() == "spv" {
          self.m_shader_lang = EnumShaderLanguageType::SpirV;
          return Ok(());
        }
        
        source = std::fs::read_to_string(&file_path_str)?;
      }
      EnumShaderSource::FromStr(source_str) => {
        source = source_str.clone();
      }
    };
    
    // If we have a GLSL shader with preprocessor instructions compatible with SPIR-V.
    if source.contains("GL_SPIRV") || source.contains("Vulkan") {
      // Compatible GLSL-SPIR-V shader found, setting the appropriate language.
      self.m_shader_lang = EnumShaderLanguageType::GlslSpirV;
      return Ok(());
    }
    
    self.m_shader_lang = EnumShaderLanguageType::Glsl;
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
  
  pub fn get_lang(&self) -> EnumShaderLanguageType {
    return self.m_shader_lang;
  }
  
  pub fn to_string(&self) -> String {
    return format!("ID: {1}\n{0:117}[Api] |Shader stage| (Source, Cached?) : {2}",
      "", self.get_id(), self.m_api_data.to_string());
  }
  
  pub fn free(&mut self) -> Result<(), EnumError> {
    if self.m_state == EnumState::NotCreated || self.m_state == EnumState::Deleted {
      log!(EnumLogColor::Yellow, "WARN", "[Renderer] -->\t Cannot delete renderer : Renderer not \
      created or already deleted!");
      return Ok(());
    }
    
    let renderer: &mut Renderer = Engine::get_active_renderer();
    self.m_api_data.free(renderer)?;
    self.m_state = EnumState::Deleted;
    return Ok(());
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    log!(EnumLogColor::Purple, "INFO", "[Shader] -->\t Dropping shader...");
    match self.free() {
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

impl Display for Shader {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{0}", self.to_string())
  }
}
