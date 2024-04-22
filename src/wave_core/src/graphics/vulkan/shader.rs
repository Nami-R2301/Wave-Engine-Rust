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

/*
///////////////////////////////////   Vulkan shader    ///////////////////////////////////
///////////////////////////////////                    ///////////////////////////////////
///////////////////////////////////                    ///////////////////////////////////
 */

#[cfg(feature = "vulkan")]
use std::any::Any;

#[cfg(feature = "vulkan")]
use crate::utils::macros::logger::*;
#[cfg(feature = "vulkan")]
use crate::Engine;
#[cfg(feature = "vulkan")]
use crate::graphics::renderer::{EnumRendererApi};
#[cfg(feature = "vulkan")]
use crate::graphics::shader::{self, EnumShaderSource, EnumShaderStageType, Shader, ShaderStage, TraitShader};
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan::renderer::VkContext;
#[cfg(feature = "vulkan")]
use crate::dependencies::ash::vk;

#[cfg(feature = "vulkan")]
#[derive(Debug, PartialEq)]
pub enum EnumSpirVError {
  SpirVCompilationError(shaderc::Error),
  ShaderModuleError,
  InvalidPushConstant,
  ShaderPipelineCreationError,
}

#[cfg(feature = "vulkan")]
impl From<EnumShaderStageType> for shaderc::ShaderKind {
  fn from(value: EnumShaderStageType) -> Self {
    return match value {
      EnumShaderStageType::Vertex => shaderc::ShaderKind::Vertex,
      EnumShaderStageType::Fragment => shaderc::ShaderKind::Fragment,
      EnumShaderStageType::Geometry => shaderc::ShaderKind::Geometry,
      EnumShaderStageType::Compute => shaderc::ShaderKind::Compute,
    };
  }
}

#[cfg(feature = "vulkan")]
impl From<EnumShaderStageType> for vk::ShaderStageFlags {
  fn from(value: EnumShaderStageType) -> Self {
    return match value {
      EnumShaderStageType::Vertex => vk::ShaderStageFlags::VERTEX,
      EnumShaderStageType::Fragment => vk::ShaderStageFlags::FRAGMENT,
      EnumShaderStageType::Geometry => vk::ShaderStageFlags::GEOMETRY,
      EnumShaderStageType::Compute => vk::ShaderStageFlags::COMPUTE,
    };
  }
}

#[cfg(feature = "vulkan")]
#[derive(Debug, Clone)]
pub struct VkShader {
  m_id: u32,
  m_shader_stages: Vec<ShaderStage>,
  m_vk_shader_modules: Vec<vk::ShaderModule>,
}

#[cfg(feature = "vulkan")]
impl TraitShader for VkShader {
  fn new(shader_stages: Vec<ShaderStage>) -> Self {
    let shader_stages_len = shader_stages.len();
    return VkShader {
      m_id: 0,
      m_shader_stages: shader_stages,
      m_vk_shader_modules: Vec::with_capacity(shader_stages_len),
    };
  }
  
  fn from(_other_shader: Self) -> Self where Self: Sized {
    todo!()
  }
  
  fn get_name(&self) -> EnumRendererApi {
    return EnumRendererApi::Vulkan;
  }
  
  fn source(&mut self) -> Result<(), shader::EnumShaderError> {
    return Ok(());
  }
  
  fn compile(&mut self) -> Result<(), shader::EnumShaderError> {
    let entry_point = "main";
    let compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.add_macro_definition("Vulkan", None);
    options.set_optimization_level(shaderc::OptimizationLevel::Performance);
    
    // Force converting unbound uniforms to SPIR-V compatible uniforms (aka bound ones).
    options.set_auto_bind_uniforms(true);
    
    #[cfg(feature = "debug")]
    options.set_generate_debug_info();
    
    // Switch from left-handed coordinates (OpenGL) to right-handed (Vulkan, DirectX).
    options.set_invert_y(true);
    options.set_warnings_as_errors();
    
    for shader_stage in self.m_shader_stages.iter() {
      let shader_binary: Vec<u8>;
      
      if shader_stage.cache_status() {
        log!(EnumLogColor::Blue, "INFO", "[VkShader] -->\t Cached shader {0} found, \
         skipping compilation.", shader_stage.m_source);
        continue;
      }
      log!(EnumLogColor::Yellow, "WARN", "[VkShader] -->\t Cached shader {0} not found, \
            compiling it.", shader_stage.m_source);
      
      match &shader_stage.m_source {
        EnumShaderSource::FromFile(file_path_str) => {
          let file_path = std::path::Path::new(file_path_str);
          let file_contents = std::fs::read_to_string(file_path_str)?;
          
          match compiler.compile_into_spirv(file_contents.as_str(),
            shaderc::ShaderKind::from(shader_stage.m_stage), file_path_str, entry_point,
            Some(&options)) {
            Ok(compiled_file) => {
              Shader::cache(file_path, compiled_file.as_binary_u8().to_vec())?;
              shader_binary = compiled_file.as_binary_u8().to_vec();
            }
            Err(err) => {
              log!(EnumLogColor::Red, "ERROR", "[VkShader] -->\t Cannot compile {0} shader into \
                  SPIR-V : Error => \n{err}", shader_stage.m_stage);
              return Err(shader::EnumShaderError::from(EnumSpirVError::SpirVCompilationError(err)));
            }
          };
        }
        EnumShaderSource::FromStr(_) => todo!()
      };
      let shader_module = VkShader::create_vk_shader(&shader_binary)?;
      self.m_vk_shader_modules.push(shader_module);
    }
    return Ok(());
  }
  
  fn apply(&mut self) -> Result<(), shader::EnumShaderError> {
    self.source()?;
    self.compile()?;
    return Ok(());
  }
  
  fn to_string(&self) -> String {
    let mut format: String = String::from("");
    
    for shader_stage in self.m_shader_stages.iter() {
      format += format!("\n{0:117}[Vulkan] |{1}| ({2}, {3})",
        "", shader_stage.m_stage, shader_stage.m_source, shader_stage.cache_status()).as_str();
    }
    return format;
  }
  
  fn upload_data(&mut self, _uniform_name: &'static str, _uniform: &dyn std::any::Any) -> Result<(), shader::EnumShaderError> {
    return Ok(());
  }
  
  fn get_id(&self) -> u32 {
    return self.m_id;
  }
  
  fn get_api_handle(&self) -> &dyn Any {
    return self;
  }
  
  fn free(&mut self) -> Result<(), shader::EnumShaderError> {
    unsafe {
      let vk_context =
        Engine::get_active_renderer().get_api_handle()
          .downcast_mut::<VkContext>()
          .expect("[VkShader] -->\t Cannot retrieve Vulkan instance : Renderer not Vulkan!");
      for shader_module in self.m_vk_shader_modules.iter() {
        vk_context.get_handle().destroy_shader_module(*shader_module, None);
      }
    }
    return Ok(());
  }
}

#[cfg(feature = "vulkan")]
impl VkShader {
  pub fn get_vk_shaders(&self) -> &Vec<vk::ShaderModule> {
    return &self.m_vk_shader_modules;
  }
  
  fn create_vk_shader(shader_binary: &Vec<u8>) -> Result<vk::ShaderModule, shader::EnumShaderError> {
    let mut shader_module_create_info: vk::ShaderModuleCreateInfo = vk::ShaderModuleCreateInfo::default();
    shader_module_create_info.code_size = shader_binary.len();
    shader_module_create_info.p_code = shader_binary.as_ptr() as *const u32;
    
    let renderer =  Engine::get_active_renderer();
    let vk_context =
      renderer.get_api_handle()
        .downcast_mut::<VkContext>()
        .expect("[VkShader] -->\t Cannot retrieve Vulkan instance : Renderer not Vulkan!");
    return match unsafe { vk_context.get_handle().create_shader_module(&shader_module_create_info, None) } {
      Ok(shader_module) => Ok(shader_module),
      #[allow(unused)]
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[VkShader] -->\t Cannot create Vulkan shader module : \
          Vulkan returned with error => {err:#?}");
        Err(shader::EnumShaderError::from(EnumSpirVError::ShaderModuleError))
      }
    };
  }
}