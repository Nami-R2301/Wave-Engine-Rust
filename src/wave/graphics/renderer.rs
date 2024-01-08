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

use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use crate::log;

use crate::wave::assets::renderable_assets::REntity;
use crate::wave::graphics::open_gl::renderer::{EnumOpenGLErrors, GlContext};
use crate::wave::graphics::shader::Shader;
#[cfg(feature = "Vulkan")]
use crate::wave::graphics::vulkan::renderer::{EnumVulkanErrors, VkContext};
use crate::wave::window::Window;

pub static mut S_RENDERER: Option<*mut Renderer> = None;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumApi {
  OpenGL,
  Vulkan,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumFeature {
  Debug(bool),
  DepthTest(bool),
  CullFacing(Option<i64>),
  Wireframe(bool),
  MSAA(Option<u8>),
  SRGB(bool),
  Blending(bool, i64, i64),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumError {
  Init,
  NoApi,
  UnsupportedApi,
  NotImplemented,
  ContextError,
  InvalidEntity,
  EntityNotFound,
  CError,
  #[cfg(feature = "Vulkan")]
  VulkanError(EnumVulkanErrors),
  OpenGLError(EnumOpenGLErrors),
  MSAAError,
  ShaderError,
  InvalidBufferSize,
  InvalidVertexAttribute,
  InvalidAttributeDivisor,
}

impl Display for EnumError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Renderer] -->\t Error encountered with renderer : {:?}", self)
  }
}

impl std::error::Error for EnumError {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumState {
  Ok,
  Error,
  CriticalError,
  Shutdown,
}

pub struct Stats {
  m_entities_sent_count: u32,
  m_shader_bound_count: u32,
  m_vao_bound_count: u32,
  m_ibo_bound_count: u32,
  m_texture_bound_count: u32,
}

impl Stats {
  pub fn new() -> Self {
    return Stats {
      m_entities_sent_count: 0,
      m_shader_bound_count: 0,
      m_vao_bound_count: 0,
      m_ibo_bound_count: 0,
      m_texture_bound_count: 0,
    };
  }
  
  pub fn reset(&mut self) {
    self.m_ibo_bound_count = 0;
    self.m_shader_bound_count = 0;
    self.m_entities_sent_count = 0;
    self.m_vao_bound_count = 0;
    self.m_texture_bound_count = 0;
  }
}

pub trait TraitContext {
  fn on_new(window: &mut Window) -> Result<Self, EnumError> where Self: Sized;
  fn get_api_version(&self) -> f32;
  fn on_events(&mut self, window_event: glfw::WindowEvent) -> Result<bool, EnumError>;
  fn on_render(&mut self) -> Result<(), EnumError>;
  fn on_delete(&mut self) -> Result<(), EnumError>;
  fn submit(&mut self, features: &HashSet<EnumFeature>) -> Result<(), EnumError>;
  fn get_max_msaa_count(&self) -> Result<u8, EnumError>;
  fn to_string(&self) -> String;
  fn toggle(&mut self, feature: EnumFeature) -> Result<(), EnumError>;
  fn begin(&mut self);
  fn end(&mut self);
  fn batch(&mut self);
  fn flush(&mut self);
  fn enqueue(&mut self, entity: &REntity, shader_associated: &mut Shader) -> Result<(), EnumError>;
  fn dequeue(&mut self, id: &u64) -> Result<(), EnumError>;
}

pub struct Renderer {
  pub m_type: EnumApi,
  pub m_state: EnumState,
  pub m_features: HashSet<EnumFeature>,
  m_api: Box<dyn TraitContext>,
}

impl Renderer {
  pub fn new(api_preference: Option<EnumApi>, window: &mut Window) -> Result<Renderer, EnumError> {
    // If user has not chosen an api, choose accordingly.
    if api_preference.is_none() {
      #[cfg(feature = "Vulkan")]
      return Ok(Renderer {
        m_type: EnumApi::Vulkan,
        m_state: EnumState::Ok,
        m_features: HashSet::new(),
        m_api: Box::new(VkContext::on_new(window)?),
      });
      
      #[cfg(not(feature = "Vulkan"))]
      return Ok(Renderer {
        m_type: EnumApi::OpenGL,
        m_state: EnumState::Ok,
        m_features: HashSet::new(),
        m_api: Box::new(GlContext::on_new(window)?),
      });
    }
    
    return match api_preference.unwrap() {
      EnumApi::OpenGL => {
        Ok(Renderer {
          m_type: EnumApi::OpenGL,
          m_state: EnumState::Ok,
          m_features: HashSet::new(),
          m_api: Box::new(GlContext::on_new(window)?),
        })
      }
      EnumApi::Vulkan => {
        #[cfg(not(feature = "Vulkan"))]
        {
          log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot create renderer : Vulkan feature \
        not enabled!\nMake sure to turn on Vulkan rendering by enabling it in the Cargo.toml \
        file under features!");
          return Err(EnumError::UnsupportedApi);
        }
        
        #[cfg(feature = "Vulkan")]
        Ok(Renderer {
          m_type: EnumApi::Vulkan,
          m_state: EnumState::Ok,
          m_features: HashSet::new(),
          m_api: Box::new(VkContext::on_new(window)?),
        })
      }
    };
  }
  
  pub fn renderer_hint(&mut self, feature_desired: EnumFeature) {
    self.m_features.insert(feature_desired);
  }
  
  pub fn renderer_hints(&mut self, features_desired: HashSet<EnumFeature>) {
    self.m_features = features_desired;
  }
  
  pub fn submit(&mut self) -> Result<(), EnumError> {
    return Ok(self.m_api.submit(&self.m_features)?);
  }
  
  pub fn on_events(&mut self, window_event: glfw::WindowEvent) -> Result<bool, EnumError> {
    return self.m_api.on_events(window_event);
  }
  
  pub fn on_render(&mut self) -> Result<(), EnumError> {
    return self.m_api.on_render();
  }
  
  pub fn toggle(&mut self, feature: EnumFeature) -> Result<(), EnumError> {
    return self.m_api.toggle(feature);
  }
  
  pub fn on_delete(&mut self) -> Result<(), EnumError> {
    if self.m_state == EnumState::Error {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot delete renderer : No active renderer!");
      return Err(EnumError::NotImplemented);
    }
    if self.m_state == EnumState::Shutdown {
      return Ok(());
    }
    // Free up resources.
    self.m_api.on_delete()?;
    self.m_state = EnumState::Shutdown;
    return Ok(());
  }
  
  pub fn enqueue(&mut self, r_entity: &REntity, associated_shader: &mut Shader) -> Result<(), EnumError> {
    return self.m_api.enqueue(r_entity, associated_shader);
  }
  
  pub fn dequeue(&mut self, id: &u64) -> Result<(), EnumError> {
    return self.m_api.dequeue(id);
  }
  
  pub fn get() -> Option<*mut Renderer> {
    return unsafe { S_RENDERER };
  }
  
  pub fn get_version(&self) -> f32 {
    return self.m_api.get_api_version();
  }
}

impl Display for Renderer {
  fn fmt(&self, format: &mut Formatter<'_>) -> std::fmt::Result {
    write!(format, "State => {0:#?}\n{1:113}{2}", self.m_state, "", self.m_api.to_string())
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    log!(EnumLogColor::Purple, "INFO", "[Renderer] -->\t Dropping renderer...");
    match self.on_delete() {
      Ok(_) => {
        log!(EnumLogColor::Green, "INFO", "[Renderer] -->\t Dropped renderer successfully...");
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Error while dropping renderer : \
        Error => {:?}", err);
        log!(EnumLogColor::Red, "INFO", "[Renderer] -->\t Dropped renderer unsuccessfully...");
      }
    }
  }
}