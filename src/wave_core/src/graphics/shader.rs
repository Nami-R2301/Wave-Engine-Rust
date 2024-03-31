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

use crate::Engine;
use crate::graphics::open_gl;
use crate::graphics::open_gl::shader::GlShader;
use crate::graphics::renderer::{EnumRendererApi, EnumRendererState, Renderer};
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan;
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan::shader::VkShader;
use crate::utils::macros::logger::*;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumShaderState {
  NotCreated,
  Created,
  Sourced,
  Compiled,
  Sent,
  Freed,
  Deleted,
}

#[derive(Debug, PartialEq)]
pub enum EnumShaderError {
  NoStagesProvided,
  StageNotFound,
  StageAlreadyProvided,
  NoActiveRendererError,
  InvalidApi,
  UnsupportedApiFunction,
  FileNotFound,
  PathError,
  VersionNotFound,
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
  VulkanShaderError(vulkan::shader::EnumSpirVError),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumShaderLanguage {
  Glsl,
  GlslSpirV,
  SpirV,
  Binary,
}

impl Default for EnumShaderLanguage {
  fn default() -> Self {
    return EnumShaderLanguage::Glsl;
  }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum EnumShaderStageType {
  Vertex = gl::VERTEX_SHADER,
  Fragment = gl::FRAGMENT_SHADER,
  Geometry = gl::GEOMETRY_SHADER,
  Compute = gl::COMPUTE_SHADER,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash)]
pub enum EnumShaderSource {
  FromFile(String),
  FromStr(String),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum EnumShaderProfile {
  AnyProfile,
  CoreProfile,
  CompatibilityProfile,
}

impl Default for EnumShaderProfile {
  fn default() -> Self {
    return EnumShaderProfile::CoreProfile;
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumShaderHint {
  Api(EnumRendererApi),
  Profile(EnumShaderProfile),
  GlslVersion(u32),
}

impl EnumShaderHint {
  pub fn is(&self, other: &Self) -> bool {
    return std::mem::discriminant(self) == std::mem::discriminant(other);
  }
}

impl Display for EnumShaderError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Shader] -->\t Error encountered with shader(s) : {:?}", self)
  }
}

impl EnumShaderSource {
  pub fn is_file(&self) -> bool {
    return match self {
      EnumShaderSource::FromFile(_) => true,
      EnumShaderSource::FromStr(_) => false
    };
  }
  
  pub fn get_file_path(&self) -> Option<&str> {
    return match self {
      EnumShaderSource::FromFile(file_path) => Some(file_path.as_str()),
      EnumShaderSource::FromStr(_) => None
    };
  }
}

impl From<std::io::Error> for EnumShaderError {
  fn from(err: std::io::Error) -> Self {
    return EnumShaderError::IoError(err.kind());
  }
}

impl From<open_gl::renderer::EnumOpenGLError> for EnumShaderError {
  fn from(_value: open_gl::renderer::EnumOpenGLError) -> Self {
    return EnumShaderError::OpenGLShaderError(open_gl::shader::EnumError::OpenGLApiError);
  }
}

impl From<open_gl::shader::EnumError> for EnumShaderError {
  fn from(value: open_gl::shader::EnumError) -> Self {
    return EnumShaderError::OpenGLShaderError(value);
  }
}

#[cfg(feature = "vulkan")]
impl From<vulkan::shader::EnumSpirVError> for EnumShaderError {
  fn from(value: vulkan::shader::EnumSpirVError) -> Self {
    return EnumShaderError::VulkanShaderError(value);
  }
}

impl std::error::Error for EnumShaderError {}

impl Display for EnumShaderStageType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EnumShaderStageType::Vertex => write!(f, "Vertex"),
      EnumShaderStageType::Fragment => write!(f, "Fragment"),
      EnumShaderStageType::Geometry => write!(f, "Geometry"),
      EnumShaderStageType::Compute => write!(f, "Compute")
    }
  }
}

impl Display for EnumShaderSource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EnumShaderSource::FromFile(file_path) => write!(f, "'{0}'", file_path),
      
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
  fn new(shader_module: Vec<ShaderStage>) -> Result<Self, EnumShaderError> where Self: Sized;
  fn from(other_shader: Self) -> Self where Self: Sized;
  fn get_name(&self) -> EnumRendererApi;
  fn source(&mut self) -> Result<(), EnumShaderError>;
  fn compile(&mut self) -> Result<(), EnumShaderError>;
  fn apply(&mut self) -> Result<(), EnumShaderError>;
  fn to_string(&self) -> String;
  fn upload_data(&mut self, uniform_name: &'static str, uniform: &dyn std::any::Any) -> Result<(), EnumShaderError>;
  fn get_id(&self) -> u32;
  fn get_api_handle(&self) -> &dyn std::any::Any;
  fn free(&mut self) -> Result<(), EnumShaderError>;
}

#[derive(Debug, Clone, Hash)]
pub struct ShaderStage {
  pub(crate) m_stage: EnumShaderStageType,
  pub(crate) m_source: EnumShaderSource,
  pub(crate) m_lang: EnumShaderLanguage,
  m_is_cached: bool,
}

impl ShaderStage {
  pub fn default(stage: EnumShaderStageType) -> Self {
    let stage_: EnumShaderStageType;
    let source: EnumShaderSource;
    
    match stage {
      EnumShaderStageType::Vertex => {
        stage_ = EnumShaderStageType::Vertex;
        source = EnumShaderSource::FromFile("glsl_420.vert".to_string());
      }
      EnumShaderStageType::Fragment => {
        stage_ = EnumShaderStageType::Fragment;
        source = EnumShaderSource::FromFile("glsl_420.frag".to_string());
      }
      EnumShaderStageType::Geometry => {
        stage_ = EnumShaderStageType::Geometry;
        source = EnumShaderSource::FromFile("glsl_420.gs".to_string());
      }
      EnumShaderStageType::Compute => {
        stage_ = EnumShaderStageType::Compute;
        source = EnumShaderSource::FromFile("glsl_420.cs".to_string());
      }
    }
    
    return Self {
      m_stage: stage_,
      m_source: source,
      m_lang: EnumShaderLanguage::Glsl,
      m_is_cached: false,
    };
  }
  
  pub fn new(shader_type: EnumShaderStageType, shader_source: EnumShaderSource) -> Self {
    return Self {
      m_stage: shader_type,
      m_source: shader_source,
      m_lang: EnumShaderLanguage::Glsl,
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
  m_state: EnumShaderState,
  m_version: u16,
  m_shader_lang: EnumShaderLanguage,
  m_api_data: Box<dyn TraitShader>,
  m_hints: Vec<EnumShaderHint>,
  m_stages: Vec<ShaderStage>,
}

impl Shader {
  pub fn default() -> Self {
    let mut hints = Vec::with_capacity(2);
    hints.push(EnumShaderHint::Api(EnumRendererApi::default()));
    hints.push(EnumShaderHint::GlslVersion(420));
    
    return Self {
      m_state: EnumShaderState::Created,
      m_version: 420,
      m_shader_lang: EnumShaderLanguage::Glsl,
      m_api_data: Box::new(GlShader::default()),
      m_hints: hints,
      m_stages: Vec::with_capacity(3),
    };
  }
  
  pub fn new() -> Self {
    #[cfg(not(feature = "vulkan"))]
    return Self {
      m_state: EnumShaderState::Created,
      m_version: 330,
      m_shader_lang: EnumShaderLanguage::Glsl,
      m_api_data: Box::new(GlShader::default()),
      m_hints: Vec::with_capacity(3),
      m_stages: Vec::with_capacity(3),
    };
    
    #[cfg(feature = "vulkan")]
    return Self {
      m_state: EnumShaderState::Created,
      m_version: 330,
      m_shader_lang: EnumShaderLanguage::Glsl,
      m_api_data: Box::new(VkShader::default()),
      m_hints: Vec::with_capacity(3),
      m_stages: Vec::with_capacity(3),
    };
  }
  
  pub fn shader_hint(&mut self, hint: EnumShaderHint) {
    if let Some(position) = self.m_hints.iter().position(|h| h.is(&hint)) {
      self.m_hints.remove(position);
    }
    
    self.m_hints.push(hint);
  }
  
  pub fn clear_hints(&mut self) {
    self.m_hints.clear();
  }
  
  pub fn push_stage(&mut self, shader_stage: ShaderStage) -> Result<(), EnumShaderError> {
    if self.m_stages.contains(&shader_stage) {
      return Err(EnumShaderError::StageAlreadyProvided);
    }
    self.m_stages.push(shader_stage);
    return Ok(());
  }
  
  pub fn remove_stage(&mut self, shader_stage_type: EnumShaderStageType) -> Result<(), EnumShaderError> {
    if self.m_stages.is_empty() {
      return Err(EnumShaderError::NoStagesProvided);
    }
    
    if let Some(stage) = self.m_stages.iter().position(|stage| stage.m_stage == shader_stage_type) {
      self.m_stages.remove(stage);
      return Ok(());
    }
    return Err(EnumShaderError::StageNotFound);
  }
  
  pub fn pop_stage(&mut self) -> Result<ShaderStage, EnumShaderError> {
    if self.m_stages.is_empty() {
      return Err(EnumShaderError::NoStagesProvided);
    }
    
    return Ok(self.m_stages.pop().unwrap());
  }
  
  pub fn apply(&mut self) -> Result<(), EnumShaderError> {
    if self.m_stages.is_empty() {
      log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot create shader : No shader stages \
        provided!");
      return Err(EnumShaderError::NoStagesProvided);
    }
    
    for hint in self.m_hints.iter() {
      match hint {
        EnumShaderHint::Profile(profile) => Self::load_profile(&mut self.m_stages, *profile)?,
        EnumShaderHint::GlslVersion(version) => Self::target_version(&mut self.m_stages, *version)?,
        _ => {}
      }
    }
    
    for shader_stage in self.m_stages.iter_mut() {
      Self::check_validity(&shader_stage)?;
      self.m_version = Self::check_version_compatibility(&shader_stage)?;
      
      match &shader_stage.m_source {
        EnumShaderSource::FromFile(file_path_str) => {
          let file_path = std::path::Path::new(file_path_str.as_str());
          
          // Try loading from cache.
          if Self::check_cache(file_path).is_ok() {
            shader_stage.m_is_cached = true;
            
            if shader_stage.m_is_cached {
              let mut cache_path_str: String = format!("cache/{0}", file_path.file_name()
                .ok_or(EnumShaderError::InvalidFileOperation)?
                .to_str()
                .ok_or(EnumShaderError::InvalidFileOperation)?);
              if file_path.extension().ok_or(EnumShaderError::InvalidFileOperation)? != "spv" {
                cache_path_str += ".spv";
              }
              
              shader_stage.m_source = EnumShaderSource::FromFile(cache_path_str);
            }
            continue;
          }
          shader_stage.m_is_cached = false;
        }
        EnumShaderSource::FromStr(literal_string) => {
          if literal_string.is_empty() {
            log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot create shader : Empty source \
          string given for shader source!");
            return Err(EnumShaderError::InvalidShaderSource);
          }
        }
      }
    }
    
    if self.m_stages.iter().any(|stage| !stage.m_is_cached) {
      // If one shader stage is uncached or the differs from its cached version, make the rest uncached.
      self.m_stages.iter_mut().for_each(|stage: &mut ShaderStage| {
        stage.m_is_cached = false;
        
        match &stage.m_source {
          EnumShaderSource::FromFile(file_str) => {
            let cached_path = std::path::Path::new(&file_str);
            
            if cached_path.extension().expect("Cannot retrieve extension from cached shader path!") != "spv" {
              return;
            }
            
            let uncached_path_str: String = format!("res/shaders/{0}", cached_path.file_stem()
              .ok_or(EnumShaderError::InvalidFileOperation).expect(&format!("Cannot get filename out of shader source: {0}",
              file_str))
              .to_str()
              .ok_or(EnumShaderError::InvalidFileOperation).expect(&format!("Cannot convert to str out of shader source: {0}",
              file_str)));
            
            stage.m_source = EnumShaderSource::FromFile(uncached_path_str);
          }
          _ => {}
        }
      });
    }
    
    self.parse_language()?;
    
    if self.m_hints.contains(&EnumShaderHint::Api(EnumRendererApi::OpenGL)) {
      self.m_api_data = Box::new(GlShader::new(self.m_stages.clone())?);
      self.m_api_data.apply()?;
      self.m_state = EnumShaderState::Sent;
      return Ok(());
    }
    
    #[cfg(not(feature = "vulkan"))]
    {
      log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot create SPirV shader, vulkan feature not enabled!");
      return Err(EnumShaderError::InvalidApi);
    }
    
    #[cfg(feature = "vulkan")]
    {
      self.m_api_data = Box::new(VkShader::new(self.m_stages.clone())?);
      self.m_api_data.apply()?;
      self.m_state = EnumShaderState::Sent;
      return Ok(());
    }
  }
  
  pub fn target_version(shader_stages: &mut Vec<ShaderStage>, version_requested: u32) -> Result<(), EnumShaderError> {
    // Try loading the exact target version specified first.
    let shader_folder = std::path::Path::new("res/shaders");
    if !shader_folder.exists() {
      log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot load version {0}, shaders root directory (res/shaders/) \
      not found!", version_requested);
      return Err(EnumShaderError::FileNotFound);
    }
    
    for shader_stage in shader_stages {
      match &shader_stage.m_source {
        EnumShaderSource::FromStr(literal) => {
          if !literal.contains(version_requested.to_string().as_str()) {
            log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot load shader version {0}, source version does not match \
            version number requested!", version_requested);
            return Err(EnumShaderError::VersionNotFound);
          }
        }
        EnumShaderSource::FromFile(file_path) => {
          let file_path_str = &format!("res/shaders/{0}", file_path);
          let glsl_source = std::path::Path::new(file_path_str);
          
          if !glsl_source.exists() || !glsl_source.is_file() {
            log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot load shader version {0}, source file '{1}' not found!",
              version_requested, glsl_source.to_str().unwrap());
            return Err(EnumShaderError::FileNotFound);
          }
          
          let mut file_path_str: String;
          match shader_stage.m_stage {
            EnumShaderStageType::Vertex => {
              file_path_str = "res/shaders/glsl_420.vert".to_string();
            }
            EnumShaderStageType::Fragment => {
              file_path_str = "res/shaders/glsl_420.frag".to_string();
            }
            EnumShaderStageType::Geometry => {
              file_path_str = "res/shaders/glsl_420.gs".to_string();
            }
            EnumShaderStageType::Compute => {
              file_path_str = "res/shaders/glsl_420.cp".to_string();
            }
          }
          
          let mut glsl_other_source = std::path::Path::new(&file_path_str);
          let mut new_version = 460;
          let version_pattern = format!("#version {0}", version_requested);
          let mut file_contents = std::fs::read_to_string(glsl_other_source);
          
          // Keep loading from a different source supporting the requested version until we find one, if we can't
          // simply throw VersionNotFound error.
          while new_version > 330 && (!file_contents.is_ok() || !file_contents?.contains(version_pattern.as_str())) {
            log!(EnumLogColor::Yellow, "WARN", "[Shader] -->\t '{0}' not targeting glsl #v{1}, attempting fallbacks...",
              glsl_other_source.to_str().unwrap(), version_requested);
            
            new_version -= 10;  // Decrement glsl version.
            match shader_stage.m_stage {
              EnumShaderStageType::Vertex => {
                file_path_str = format!("res/shaders/glsl_{0}.vert", new_version);
              }
              EnumShaderStageType::Fragment => {
                file_path_str = format!("res/shaders/glsl_{0}.frag", new_version);
              }
              EnumShaderStageType::Geometry => {
                file_path_str = format!("res/shaders/glsl_{0}.gs", new_version);
              }
              EnumShaderStageType::Compute => {
                file_path_str = format!("res/shaders/glsl_{0}.cp", new_version);
              }
            }
            glsl_other_source = std::path::Path::new(&file_path_str);
            file_contents = std::fs::read_to_string(glsl_other_source);
          }
          
          // If despite all glsl fallbacks, we can't find the right version, we abort.
          if !glsl_other_source.exists() {
            log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot load shader version {0}, source version does not match \
            version number requested!", version_requested);
            return Err(EnumShaderError::VersionNotFound);
          }
          
          log!(EnumLogColor::Green, "INFO", "[Shader] -->\t Shader source compatible found for #version {0}: {1}!",
              version_requested, file_path_str);
          shader_stage.m_source = EnumShaderSource::FromFile(String::from(glsl_other_source.to_str()
            .expect("Cannot convert path to &str!")));
        }
      }
    }
    return Ok(());
  }
  
  pub fn load_profile(_shader_stage: &mut Vec<ShaderStage>, _profile_requested: EnumShaderProfile) -> Result<(), EnumShaderError> {
    todo!()
  }
  
  pub fn check_cache(shader_file_path: &std::path::Path) -> Result<Vec<u8>, EnumShaderError> {
    let renderer: &mut Renderer = Engine::get_active_renderer();
    if renderer.m_type == EnumRendererApi::OpenGL && !renderer.check_extension("GL_ARB_gl_spirv") {
      log!(EnumLogColor::Yellow, "WARN", "[Shader] -->\t Cannot load from cached SPIR-V binary : \
          \n{0:113}Current OpenGL renderer doesn't support the extension required 'GL_ARB_gl_spirv'\
          \n{0:113}Attempting to load from glsl shader directly...", "");
      return Err(EnumShaderError::InvalidApi);
    }
    
    let cache_path: &std::path::Path;
    let mut cache_path_str: String = format!("cache/{0}", shader_file_path.file_name()
      .ok_or(EnumShaderError::InvalidFileOperation)?
      .to_str()
      .ok_or(EnumShaderError::InvalidFileOperation)?);
    if shader_file_path.extension().ok_or(EnumShaderError::InvalidFileOperation)? != "spv" {
      cache_path_str += ".spv";
    }
    
    cache_path = std::path::Path::new(&cache_path_str);
    
    if !cache_path.exists() {
      log!(EnumLogColor::Yellow, "WARN", "[Shader] -->\t Shader stage cache not found, recompiling all shader stages...");
      return Err(EnumShaderError::ShaderModified);
    }
    
    let src_last_time_modified = shader_file_path.metadata()?.modified()?;
    let cache_last_time_modified = cache_path.metadata()?.modified()?;
    if src_last_time_modified > cache_last_time_modified {
      log!(EnumLogColor::Yellow, "WARN", "[Shader] -->\t Shader source modified since last cache, recompiling shader stages...");
      return Err(EnumShaderError::ShaderModified);
    }
    
    let cache_buffer = std::fs::read(cache_path)?;
    return Ok(cache_buffer);
  }
  
  pub fn cache(shader_name: &std::path::Path, binary: Vec<u8>) -> Result<(), EnumShaderError> {
    let cache_path_str: String = format!("cache/{0}.spv", shader_name.file_name().unwrap().to_str().unwrap());
    
    std::fs::write(cache_path_str, binary.as_slice())?;
    return Ok(());
  }
  
  pub fn get_version(&self) -> u16 {
    return self.m_version;
  }
  
  fn check_validity(shader_stage: &ShaderStage) -> Result<(), EnumShaderError> {
    match &shader_stage.m_source {
      EnumShaderSource::FromFile(file_path_str) => {
        let file_path = std::path::Path::new(file_path_str);
        if !file_path.exists() {
          log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot open shader file : File {0:?} does not exist!", file_path);
          return Err(EnumShaderError::FileNotFound);
        }
        
        let source = std::fs::read_to_string(file_path)?;
        if !source.contains("#version") || !source.contains("void main()") {
          return Err(EnumShaderError::InvalidShaderSource);
        }
      }
      EnumShaderSource::FromStr(literal_str) => {
        if !literal_str.contains("#version") || !literal_str.contains("void main()") {
          return Err(EnumShaderError::InvalidShaderSource);
        }
      }
    }
    
    return Ok(());
  }
  
  fn check_version_compatibility(shader_stage: &ShaderStage) -> Result<u16, EnumShaderError> {
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
    
    let max_version = renderer.get_max_shader_version_available();
      // If the shader source version number is higher than supported.
    if max_version < version_number {
        // Try loading a source file with the appropriate version number.
        log!(EnumLogColor::Yellow, "WARN", "[Shader] -->\t Attempting to load a source file compatible with glsl {0}", max_version);
        std::fs::read_to_string(&source.replace("#version 420", &("#version ".to_owned() + &max_version.to_string())))?;
        return Ok(max_version);
    }
    return Ok(max_version);
  }
  
  fn parse_language(&mut self) -> Result<(), EnumShaderError> {
    for stage in self.m_stages.iter_mut() {
      let source: String;
      match &stage.m_source {
        EnumShaderSource::FromFile(file_path_str) => {
          let file_path = std::path::Path::new(&file_path_str);
          
          if !file_path.exists() {
            return Err(EnumShaderError::FileNotFound);
          }
          
          if file_path.extension().unwrap() == "bin" {
            stage.m_lang = EnumShaderLanguage::Binary;
            continue;
          }
          if file_path.extension().unwrap() == "spv" {
            stage.m_lang = EnumShaderLanguage::SpirV;
            continue;
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
        stage.m_lang = EnumShaderLanguage::GlslSpirV;
        continue;
      }
      stage.m_lang = EnumShaderLanguage::Glsl;
    }
    
    if self.m_stages.len() == 1 && self.m_stages.iter().find(|stage| stage.m_lang == EnumShaderLanguage::Binary).is_some() {
      self.m_shader_lang = EnumShaderLanguage::Binary;
      return Ok(());
    }
    
    if self.m_stages.iter().all(|stage| stage.m_lang == EnumShaderLanguage::GlslSpirV) {
      self.m_shader_lang = EnumShaderLanguage::GlslSpirV;
      return Ok(());
    }
    
    if self.m_stages.iter().all(|stage| stage.m_lang == EnumShaderLanguage::SpirV) {
      self.m_shader_lang = EnumShaderLanguage::SpirV;
      return Ok(());
    }
    
    self.m_shader_lang = EnumShaderLanguage::Glsl;
    return Ok(());
  }
  
  pub fn upload_data(&mut self, uniform_name: &'static str, uniform: &dyn std::any::Any) -> Result<(), EnumShaderError> {
    return self.m_api_data.upload_data(uniform_name, uniform);
  }
  
  pub fn get_api(&self) -> &dyn TraitShader {
    return self.m_api_data.as_ref();
  }
  
  pub fn get_id(&self) -> u32 {
    return self.m_api_data.get_id();
  }
  
  pub fn get_lang(&self) -> EnumShaderLanguage {
    return self.m_shader_lang;
  }
  
  pub fn to_string(&self) -> String {
    return format!("ID: {1}\n{0:117}[Api] |Shader stage| (Source, Cached?) : {2}",
      "", self.get_id(), self.m_api_data.to_string());
  }
  
  pub fn free(&mut self) -> Result<(), EnumShaderError> {
    if self.m_state != EnumShaderState::Sent {
      return Ok(());
    }
    
    let renderer: &mut Renderer = Engine::get_active_renderer();
    if renderer.m_state != EnumRendererState::Deleted || renderer.m_state != EnumRendererState::NotCreated {
      self.m_api_data.free()?;
    }
    self.m_state = EnumShaderState::Deleted;
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
