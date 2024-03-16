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
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use crate::{log};
use crate::wave_core::assets::asset_loader;
use crate::wave_core::assets::renderable_assets::{REntity};
use crate::wave_core::camera::{Camera};
use crate::wave_core::events;
use crate::wave_core::graphics::{open_gl};
use crate::wave_core::graphics::open_gl::renderer::GlContext;
use crate::wave_core::graphics::shader::Shader;
#[cfg(feature = "vulkan")]
use crate::wave_core::graphics::{vulkan};

#[cfg(feature = "vulkan")]
use crate::wave_core::graphics::vulkan::renderer::VkContext;
use crate::wave_core::math::{Mat4};
use crate::wave_core::window::Window;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumCallCheckingType {
  None,
  Async,
  Sync,
  SyncAndAsync,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumState {
  NotCreated,
  Created,
  Deleted,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumApi {
  OpenGL,
  Vulkan,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererOption {
  ApiCallChecking(EnumCallCheckingType),
  DepthTest(bool),
  CullFacing(Option<i64>),
  Wireframe(bool),
  MSAA(Option<u8>),
  SRGB(bool),
  Blending(bool, i64, i64),
}

#[derive(Debug, PartialEq)]
pub enum EnumError {
  Init,
  NoApi,
  NoActiveRenderer,
  InvalidApi,
  UnsupportedApi,
  NotImplemented,
  ContextError,
  InvalidAssetSource(asset_loader::EnumError),
  InvalidEntity,
  EntityNotFound,
  ShaderNotFound,
  UboNotFound,
  CError,
  #[cfg(feature = "vulkan")]
  VulkanError(vulkan::renderer::EnumError),
  OpenGLError(open_gl::renderer::EnumError),
  OpenGLInvalidBufferOperation(open_gl::buffer::EnumError),
  #[cfg(feature = "vulkan")]
  VulkanInvalidBufferOperation(vulkan::buffer::EnumError),
}

impl From<asset_loader::EnumError> for EnumError {
  fn from(value: asset_loader::EnumError) -> Self {
    return EnumError::InvalidAssetSource(value);
  }
}

impl From<open_gl::renderer::EnumError> for EnumError {
  fn from(value: open_gl::renderer::EnumError) -> Self {
    return EnumError::OpenGLError(value);
  }
}

#[cfg(feature = "vulkan")]
impl From<vulkan::renderer::EnumError> for EnumError {
  fn from(value: vulkan::renderer::EnumError) -> Self {
    return EnumError::VulkanError(value);
  }
}

impl Display for EnumError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Renderer] -->\t Error encountered with renderer : {:?}", self)
  }
}

impl std::error::Error for EnumError {}

pub(crate) struct Stats {
  m_entities_sent_count: u32,
  m_shader_bound_count: u32,
  m_vao_bound_count: u32,
  m_ibo_bound_count: u32,
  m_texture_bound_count: u32,
}

impl Stats {
  #[allow(unused)]
  pub(crate) fn new() -> Self {
    return Stats {
      m_entities_sent_count: 0,
      m_shader_bound_count: 0,
      m_vao_bound_count: 0,
      m_ibo_bound_count: 0,
      m_texture_bound_count: 0,
    };
  }
  
  #[allow(unused)]
  pub(crate) fn reset(&mut self) {
    self.m_ibo_bound_count = 0;
    self.m_shader_bound_count = 0;
    self.m_entities_sent_count = 0;
    self.m_vao_bound_count = 0;
    self.m_texture_bound_count = 0;
  }
}

pub(crate) trait TraitContext {
  fn default() -> Self where Self: Sized;
  fn on_new(window: &mut Window) -> Result<Self, EnumError> where Self: Sized;
  fn get_api_handle(&mut self) -> &mut dyn Any;
  fn get_api_version(&self) -> f32;
  fn get_max_shader_version_available(&self) -> u16;
  fn check_extension(&self, desired_extension: &str) -> bool;
  fn on_event(&mut self, event: &events::EnumEvent) -> Result<bool, EnumError>;
  fn on_render(&mut self) -> Result<(), EnumError>;
  fn submit(&mut self, window: &mut Window, features: &HashSet<EnumRendererOption>) -> Result<(), EnumError>;
  fn get_max_msaa_count(&self) -> Result<u8, EnumError>;
  fn to_string(&self) -> String;
  fn toggle(&mut self, option: EnumRendererOption) -> Result<(), EnumError>;
  fn setup_camera(&mut self, camera: &Camera) -> Result<(), EnumError>;
  fn flush(&mut self) -> Result<(), EnumError>;
  fn enqueue(&mut self, entity: &REntity, shader_associated: &mut Shader) -> Result<(), EnumError>;
  fn dequeue(&mut self, id: u64) -> Result<(), EnumError>;
  fn update(&mut self, shader_associated: &mut Shader, transform: Mat4) -> Result<(), EnumError>;
  fn on_delete(&mut self) -> Result<(), EnumError>;
}

pub struct Renderer {
  pub m_type: EnumApi,
  pub m_state: EnumState,
  pub m_options: HashSet<EnumRendererOption>,
  m_api: Box<dyn TraitContext>,
}

impl<'a> Renderer {
  pub fn new(api_preference: EnumApi) -> Result<Self, EnumError> {
    return match api_preference {
      EnumApi::OpenGL => {
        Ok(Renderer {
          m_type: EnumApi::OpenGL,
          m_state: EnumState::Created,
          m_options: HashSet::new(),
          m_api: Box::new(GlContext::default()),
        })
      }
      EnumApi::Vulkan => {
        #[cfg(feature = "vulkan")]
        return Ok(Renderer {
          m_type: EnumApi::Vulkan,
          m_state: EnumState::Created,
          m_options: HashSet::new(),
          m_api: Box::new(VkContext::default()),
        });
        
        #[cfg(not(feature = "vulkan"))]
        {
          log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot create renderer : Vulkan feature \
            not enabled!\nMake sure to turn on Vulkan rendering by enabling it in the Cargo.toml \
            file under features!");
          Err(EnumError::UnsupportedApi)
        }
      }
    };
  }
  
  pub fn renderer_hint(&mut self, feature_desired: EnumRendererOption) {
    self.m_options.insert(feature_desired);
  }
  
  pub fn renderer_hints(&mut self, features_desired: HashSet<EnumRendererOption>) {
    self.m_options = features_desired;
  }
  
  pub fn submit_camera(&mut self, camera: &Camera) -> Result<(), EnumError> {
    return self.m_api.setup_camera(camera);
  }
  
  pub fn submit(&mut self, window: &mut Window) -> Result<(), EnumError> {
    match self.m_type {
      EnumApi::OpenGL => {
        self.m_api = Box::new(GlContext::on_new(window)?);
      }
      EnumApi::Vulkan => {
        self.m_api = Box::new(VkContext::on_new(window)?);
      }
    }
    return self.m_api.submit(window, &self.m_options);
  }
  
  pub fn check_extension(&self, desired_extension: &str) -> bool {
    return self.m_api.check_extension(desired_extension);
  }
  
  pub fn on_event(&mut self, event: &events::EnumEvent) -> Result<bool, EnumError> {
    return self.m_api.on_event(event);
  }
  
  pub fn flush(&mut self) -> Result<(), EnumError> {
    log!("INFO", "[Renderer] -->\t User called for manual flushing of rendered assets");
    return self.m_api.flush();
  }
  
  pub fn on_render(&mut self) -> Result<(), EnumError> {
    return self.m_api.on_render();
  }
  
  pub fn toggle(&mut self, feature: EnumRendererOption) -> Result<(), EnumError> {
    return self.m_api.toggle(feature);
  }
  
  pub fn get_api_handle(&mut self) -> &mut dyn Any {
    return self.m_api.get_api_handle();
  }
  
  pub fn on_delete(&mut self) -> Result<(), EnumError> {
    log!(EnumLogColor::Purple, "INFO", "[Renderer] -->\t Freeing resources...");
    if self.m_state == EnumState::NotCreated || self.m_state == EnumState::Deleted {
      log!(EnumLogColor::Yellow, "WARN", "[Renderer] -->\t Cannot delete renderer : Renderer not \
      created or already deleted!");
      return Ok(());
    }
    
    // Free up resources.
    self.m_api.on_delete()?;
    self.m_state = EnumState::Deleted;
    log!(EnumLogColor::Green, "INFO", "[Renderer] -->\t Freed resources successfully");
    return Ok(());
  }
  
  pub fn enqueue(&mut self, r_entity: &REntity, shader_associated: &mut Shader) -> Result<(), EnumError> {
    log!("INFO", "[Renderer] -->\t Enqueuing entity : {0}, associated with shader : {1}", r_entity.get_uuid(), shader_associated.get_id());
    return self.m_api.enqueue(r_entity, shader_associated);
  }
  
  pub fn dequeue(&mut self, id: u64) -> Result<(), EnumError> {
    log!("INFO", "[Renderer] -->\t De-queuing entity : {0}", id);
    return self.m_api.dequeue(id);
  }
  
  pub fn update(&mut self, shader_associated: &mut Shader, transform: Mat4) -> Result<(), EnumError> {
    return self.m_api.update(shader_associated, transform);
  }
  
  pub fn get_version(&self) -> f32 {
    return self.m_api.get_api_version();
  }
  
  pub fn get_max_shader_version_available(&self) -> u16 {
    return self.m_api.get_max_shader_version_available();
  }
}

impl Display for Renderer {
  fn fmt(&self, format: &mut Formatter<'_>) -> std::fmt::Result {
    write!(format, "\nState =>\t\t {0:#?};\n{1}", self.m_state, self.m_api.to_string())
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    if self.m_state != EnumState::Deleted && self.m_state != EnumState::NotCreated {
      log!(EnumLogColor::Purple, "INFO", "[Renderer] -->\t Dropping renderer...");
      match self.on_delete() {
        Ok(_) => {
          log!(EnumLogColor::Green, "INFO", "[Renderer] -->\t Dropped renderer successfully...");
        }
        #[allow(unused)]
        Err(err) => {
          log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Error while dropping renderer : \
        Error => {:?}", err);
          log!(EnumLogColor::Red, "INFO", "[Renderer] -->\t Dropped renderer unsuccessfully...");
        }
      }
    }
  }
}