/*
///////////////////////////////////   Vulkan shader    ///////////////////////////////////
///////////////////////////////////                    ///////////////////////////////////
///////////////////////////////////                    ///////////////////////////////////
 */

extern crate shaderc;

use crate::log;
use crate::wave::graphics::renderer::EnumApi;
use crate::wave::graphics::shader::{EnumError, EnumShaderSource, EnumShaderType, Shader, ShaderStage, TraitShader};

impl From<EnumShaderType> for shaderc::ShaderKind {
  fn from(value: EnumShaderType) -> Self {
    return match value {
      EnumShaderType::Vertex => shaderc::ShaderKind::Vertex,
      EnumShaderType::Fragment => shaderc::ShaderKind::Fragment,
      EnumShaderType::Compute => shaderc::ShaderKind::Compute,
    };
  }
}

#[derive(Debug, Clone)]
pub struct VkShader {
  m_id: u32,
  m_shader_stages: Vec<ShaderStage>,
}

impl TraitShader for VkShader {
  fn default() -> Self where Self: Sized {
    return Self {
      m_id: 0,
      m_shader_stages: Vec::with_capacity(2),
    };
  }
  
  fn new(shader_stages: Vec<ShaderStage>) -> Result<Self, EnumError> where Self: Sized {
    return Ok(VkShader {
      m_id: 0,
      m_shader_stages: shader_stages,
    });
  }
  
  fn from(_other_shader: Self) -> Self where Self: Sized {
    todo!()
  }
  
  fn get_name(&self) -> EnumApi {
    return EnumApi::Vulkan;
  }
  
  fn source(&mut self) -> Result<(), EnumError> {
    for shader_stage in self.m_shader_stages.iter_mut() {
      match &shader_stage.m_source {
        EnumShaderSource::FromFile(file_path) => {
          let file_path = std::path::Path::new(file_path.as_str());
          match Shader::check_cache(file_path) {
            Ok(_) => {
              if file_path.extension().unwrap() == "spv" {
                shader_stage.m_source = EnumShaderSource::FromFile(format!("cache/{0}",
                  file_path.file_name().unwrap().to_str().unwrap()));
              } else {
                shader_stage.m_source = EnumShaderSource::FromFile(format!("cache/{0}.spv",
                  file_path.file_name().unwrap().to_str().unwrap()));
              }
              shader_stage.m_is_cached = true;
            }
            Err(err) => {
              if err == EnumError::IoError(std::io::ErrorKind::NotFound) {
                shader_stage.m_is_cached = false;
              } else {
                log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Error While reading cached vertex \
                  shader {0} : Error => {1:?}", file_path.to_str().unwrap(), err);
              }
            }
          }
        }
        _ => {}
      }
    }
    return Ok(());
  }
  
  fn compile(&mut self) -> Result<(), EnumError> {
    for shader_stage in self.m_shader_stages.iter() {
      if shader_stage.m_is_cached {
        log!(EnumLogColor::Blue, "INFO", "[Shader] -->\t Cached {0} shader {1} found",
          shader_stage.m_type, shader_stage.m_source);
        continue;
      }
      log!(EnumLogColor::Yellow, "WARN", "[Shader] -->\t Cached {0} shader {1} not found, \
            compiling it...", shader_stage.m_type, shader_stage.m_source);
      
      
      let compiler = shaderc::Compiler::new().unwrap();
      let mut options = shaderc::CompileOptions::new().unwrap();
      #[cfg(not(feature = "debug"))]
      options.set_optimization_level(shaderc::OptimizationLevel::Performance);
      
      // Force converting unbound uniforms to SPIR-V compatible uniforms (aka bound ones).
      options.set_auto_bind_uniforms(true);
      
      #[cfg(feature = "debug")]
      options.set_generate_debug_info();
      
      // Switch from left handed coordinates (OpenGL) to right handed (Vulkan, DirectX).
      options.set_invert_y(true);
      options.set_warnings_as_errors();
      
      match &shader_stage.m_source {
        EnumShaderSource::FromFile(file_path_str) => {
          let file_path = std::path::Path::new(file_path_str.as_str());
          let file_contents = std::fs::read_to_string(file_path_str.as_str())?;
          
          match compiler.compile_into_spirv(file_contents.as_str(),
            shaderc::ShaderKind::from(shader_stage.m_type), file_path_str.as_str(), "main",
            Some(&options)) {
            Ok(compiled_file) => {
              Shader::cache(file_path, compiled_file.as_binary_u8().to_vec())?;
            }
            Err(err) => {
              log!(EnumLogColor::Red, "ERROR", "[Shader] -->\t Cannot compile vertex shader into SPIR-V : \
                    Error => {0:?}", err);
              return Err(EnumError::SpirVError(err));
            }
          };
        }
        EnumShaderSource::FromStr(_) => todo!()
      };
    }
    return Ok(());
  }
  
  fn send(&mut self) -> Result<(), EnumError> {
    self.source()?;
    self.compile()?;
    return Ok(());
  }
  
  fn to_string(&self) -> String {
    todo!()
  }
  
  fn upload_data(&mut self, _uniform_name: &'static str, _uniform: &dyn std::any::Any) -> Result<(), EnumError> {
    return Ok(());
  }
  
  fn get_id(&self) -> u32 {
    return self.m_id;
  }
  
  fn on_delete(&mut self) -> Result<(), EnumError> {
    return Ok(());
  }
}