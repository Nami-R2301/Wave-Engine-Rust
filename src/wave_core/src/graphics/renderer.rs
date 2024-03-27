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
use once_cell::sync::Lazy;

#[cfg(feature = "debug")]
use crate::Engine;
use crate::utils::macros::logger::*;
use crate::assets::asset_loader;
use crate::assets::renderable_assets::{REntity};
use crate::events;
use crate::graphics::{open_gl};
use crate::graphics::open_gl::renderer::GlContext;
use crate::graphics::shader::Shader;
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan;
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan::renderer::VkContext;
use crate::math::{Mat4};
use crate::window::Window;

static mut S_ENTITIES_ID_CACHE: Lazy<HashSet<u64>> = Lazy::new(|| HashSet::new());

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumRendererCallCheckingType {
  None,
  Async,
  Sync,
  SyncAndAsync,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumRendererState {
  NotCreated,
  Created,
  Submitted,
  Deleted,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererApi {
  OpenGL,
  Vulkan,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererMode {
  Runtime,
  Editor,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererCullingMode {
  Front,
  Back,
  FrontAndBack,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererPrimitiveMode {
  Point,
  Filled,
  Wireframe,
  SolidWireframe
}

impl Display for EnumRendererCullingMode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return match self {
      EnumRendererCullingMode::Front => write!(f, "Front face culling"),
      EnumRendererCullingMode::Back => write!(f, "Back face culling"),
      EnumRendererCullingMode::FrontAndBack => write!(f, "Front and back face culling")
    }
  }
}

impl Display for EnumRendererPrimitiveMode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return match self {
      EnumRendererPrimitiveMode::Filled => write!(f, "Solid"),
      EnumRendererPrimitiveMode::Point => write!(f, "Point"),
      EnumRendererPrimitiveMode::Wireframe => write!(f, "Wireframe"),
      EnumRendererPrimitiveMode::SolidWireframe => write!(f, "Solid wireframe")
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererOption {
  /// Combine primitives with the same material type into a single buffer if possible, both for the vbo and ibo.
  /// ### Argument:
  /// - *true*: Enables the batching of similar materials from the same shader by appending their vertices in the same vbo,
  /// as well as appending the indices in the same ibo and joining the indices for all the those same matched primitives by
  /// offsetting the indices of each primitive in the buffer by the base one (Similar to how OpenGL's **glDrawElementsBaseVertex(...)**
  /// does it). Effectively, reducing the number of draw commands sent to the renderer, folding it into one per shader.
  ///
  ///   - This is useful in a runtime environment where performance is key, and you would want to cut back on as many draw calls
  /// as possible in an environment where there is no need to keep track of sub-meshes and to uniquely identify primitives.
  ///
  /// - *false*: Disables the batching of similar materials and sends a different draw command to the renderer for every primitive
  /// (or sub-primitive).
  ///
  ///   - This comes in handy when rendering for editor environments where every primitive and sub-primitive needs to be uniquely
  /// identified through ray-casting for example or when wanting to take apart a primitive by hiding or showing selected sub_primitives.
  BatchSameMaterials(bool),
  
  /// Track internal api calls for potential errors and warnings when making api calls in the renderer.
  /// ### Argument:
  /// Four possible values can be provided:
  /// - [EnumRendererCallCheckingType::None]: Disable api call checking altogether.
  ///
  ///   - Note that this setting is most likely what you
  /// would want in a performance-hungry build as it doesn't waste resources for logging states in between api calls. However, you
  /// run the risk of ignoring potential errors and warnings and/or causing a crash down the line if fatal errors do occur.
  ///
  ///
  /// - [EnumRendererCallCheckingType::Sync]: Make synchronous checks in between every call to monitor internal state for warnings and errors.
  ///
  ///   - This setting is what you would want ideally in a debug build as the error and warning reports are instantaneous, ideal for logging.
  /// However, performance-wise, it is the slowest. Also, the error codes are not as descriptive, since it only reports the
  /// error code and the call itself. For more descriptive and clear error handling, see [EnumRendererCallCheckingType::Async] or
  /// [EnumRendererCallCheckingType::SyncAndAsync].
  ///
  ///
  /// - [EnumRendererCallCheckingType::Async]: Let the api asynchronously deal with its own warning and error reporting.
  ///
  ///   - This option is a nice
  /// balance between performance and verbosity and thus is the *default* option if this option isn't toggled manually. However,
  /// the only downside of this approach is that the origin of the error reported might be difficult to track down, due to its async
  /// nature. If this is an issue, try [EnumRendererCallCheckingType::Sync] or [EnumRendererCallCheckingType::SyncAndAsync] for better error messages
  /// while tracking down the errors.
  ///
  ///
  /// - [EnumRendererCallCheckingType::SyncAndAsync]: This setting combines both the synchronous and the asynchronous natures for error
  /// checking.
  ///
  ///   - This setting should be selected if you are on a dev build and require the utmost verbosity and timing for your api
  /// error handling in order to prevent error propagation, while having as many details as possible regarding the error.
  /// Sometimes necessary when dealing with obscure bugs and crashes.
  ///
  ApiCallChecking(EnumRendererCallCheckingType),
  
  /// Enable depth testing to avoid artefacts or overlapping geometry incorrectly displayed onto the screen.
  DepthTest(bool),
  /// Enable culling for a specific face to avoid rendering it when unneeded save on fragment shader calls when rendering.
  /// ### Argument:
  /// Four possible values can be provided:
  /// - [None]: Applies no culling when rendering primitives.
  ///
  /// - Some([EnumRendererCullingMode::Back]) **Default**: Cull only back faces of primitives.
  ///
  /// - Some([EnumRendererCullingMode::Front]): Cull only front faces of primitives. Note that if the winding order is clock-wise,
  /// this essentially is equivalent to [EnumRendererCullingMode::Back] with counter clock-wise winding order.
  ///
  /// - Some([EnumRendererCullingMode::FrontAndBack]): Cull both front and back faces of primitives.
  CullFacing(Option<EnumRendererCullingMode>),
  
  /// Change how the rasterizer processes meshes or sprites when rendered on screen.
  /// ### Argument:
  /// Three possible values can be provided:
  /// - [EnumRendererPrimitiveMode::Filled] **Default**: Show primitives' surfaces as filled triangles.
  /// - [EnumRendererPrimitiveMode::Point]: Show primitives' surface as points instead of filled triangles.
  /// - [EnumRendererPrimitiveMode::Wireframe]: Show the contour of primitives only without filling in the surface area in triangles.
  /// - [EnumRendererPrimitiveMode::SolidWireframe]: Similar to [EnumRendererPrimitiveMode::Wireframe], but the wireframe is displayed
  /// as an additional layer on top of the primitive's solid surface filled in, to better display the depth of each underlying part of the
  /// primitive's polygons.
  ///   - Useful when you want to visualize the wireframe of a 3D model, but there are overlapping vertices due to different layers of depth,
  /// which makes visualizing the line layouts difficult.
  PrimitiveMode(EnumRendererPrimitiveMode),
  MSAA(Option<u8>),
  SRGB(bool),
  Blending(bool, Option<(i64, i64)>),
}

#[derive(Debug, PartialEq)]
pub enum EnumRendererError {
  Init,
  NoApi,
  NoActiveRenderer,
  InvalidApi,
  UnsupportedApi,
  NotImplemented,
  ContextError,
  InvalidAssetSource(asset_loader::EnumAssetError),
  InvalidEntity,
  EntityNotFound,
  ShaderNotFound,
  UboNotFound,
  CError,
  #[cfg(feature = "vulkan")]
  VulkanError(vulkan::renderer::EnumVkContextError),
  OpenGLError(open_gl::renderer::EnumOpenGLError),
  OpenGLInvalidBufferOperation(open_gl::buffer::EnumGlBufferError),
  #[cfg(feature = "vulkan")]
  VulkanInvalidBufferOperation(vulkan::buffer::EnumVulkanBufferError),
}

impl From<asset_loader::EnumAssetError> for EnumRendererError {
  fn from(value: asset_loader::EnumAssetError) -> Self {
    return EnumRendererError::InvalidAssetSource(value);
  }
}

impl From<open_gl::renderer::EnumOpenGLError> for EnumRendererError {
  fn from(value: open_gl::renderer::EnumOpenGLError) -> Self {
    return EnumRendererError::OpenGLError(value);
  }
}

#[cfg(feature = "vulkan")]
impl From<vulkan::renderer::EnumVkContextError> for EnumRendererError {
  fn from(value: vulkan::renderer::EnumVkContextError) -> Self {
    return EnumRendererError::VulkanError(value);
  }
}

impl Display for EnumRendererError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Renderer] -->\t Error encountered with renderer : {:?}", self)
  }
}

impl std::error::Error for EnumRendererError {}

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
  fn on_new(window: &mut Window, renderer_options: HashSet<EnumRendererOption>) -> Result<Self, EnumRendererError> where Self: Sized;
  fn get_api_handle(&mut self) -> &mut dyn Any;
  fn get_api_version(&self) -> f32;
  fn get_max_shader_version_available(&self) -> u16;
  fn check_extension(&self, desired_extension: &str) -> bool;
  fn on_event(&mut self, event: &events::EnumEvent) -> Result<bool, EnumRendererError>;
  fn on_render(&mut self) -> Result<(), EnumRendererError>;
  fn apply(&mut self, window: &mut Window) -> Result<(), EnumRendererError>;
  fn toggle_visibility_of(&mut self, entity_uuid: u64, sub_primitive_offset: Option<usize>, visible: bool);
  fn toggle_primitive_mode(&mut self, mode: EnumRendererPrimitiveMode) -> Result<(), EnumRendererError>;
  fn get_max_msaa_count(&self) -> Result<u8, EnumRendererError>;
  fn to_string(&self) -> String;
  fn toggle_options(&mut self) -> Result<(), EnumRendererError>;
  fn flush(&mut self) -> Result<(), EnumRendererError>;
  fn enqueue(&mut self, entity: &REntity, shader_associated: &mut Shader) -> Result<(), EnumRendererError>;
  fn dequeue(&mut self, id: u64) -> Result<(), EnumRendererError>;
  fn update_ubo_camera(&mut self, view: Mat4, projection: Mat4) -> Result<(), EnumRendererError>;
  fn update_ubo_model(&mut self, model_transform: Mat4, instance_offset: usize) -> Result<(), EnumRendererError>;
  fn free(&mut self) -> Result<(), EnumRendererError>;
}

pub struct Renderer {
  pub m_type: EnumRendererApi,
  pub m_state: EnumRendererState,
  pub m_options: HashSet<EnumRendererOption>,
  m_api: Box<dyn TraitContext>,
}

impl<'a> Renderer {
  pub fn new(api_preference: EnumRendererApi) -> Result<Self, EnumRendererError> {
    return match api_preference {
      EnumRendererApi::OpenGL => {
        Ok(Renderer {
          m_type: EnumRendererApi::OpenGL,
          m_state: EnumRendererState::Created,
          m_options: HashSet::new(),
          m_api: Box::new(GlContext::default()),
        })
      }
      EnumRendererApi::Vulkan => {
        #[cfg(feature = "vulkan")]
        return Ok(Renderer {
          m_type: EnumRendererApi::Vulkan,
          m_state: EnumRendererState::Created,
          m_options: HashSet::new(),
          m_api: Box::new(VkContext::default()),
        });
        
        #[cfg(not(feature = "vulkan"))]
        {
          log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot create renderer : Vulkan feature \
            not enabled!\nMake sure to turn on Vulkan rendering by enabling it in the Cargo.toml \
            file under features!");
          Err(EnumRendererError::UnsupportedApi)
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
  
  pub fn hide(&mut self, entity_uuid: u64, sub_primitive_offset: Option<usize>) {
    return self.m_api.toggle_visibility_of(entity_uuid, sub_primitive_offset, false);
  }
  
  pub fn show(&mut self, entity_uuid: u64, sub_primitive_offset: Option<usize>) {
    return self.m_api.toggle_visibility_of(entity_uuid, sub_primitive_offset, true);
  }
  
  pub fn apply(&mut self, window: &mut Window) -> Result<(), EnumRendererError> {
    return match self.m_type {
      EnumRendererApi::OpenGL => {
        self.m_api = Box::new(GlContext::on_new(window, self.m_options.clone())?);
        self.m_api.apply(window)
      }
      EnumRendererApi::Vulkan => {
        #[cfg(not(feature = "vulkan"))]
        {
          log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot set VkContext, vulkan feature not enabled!");
          return Err(EnumRendererError::InvalidApi);
        }
        #[cfg(feature = "vulkan")]
        {
          self.m_api = Box::new(VkContext::on_new(window, self.m_options.clone())?);
          self.m_api.apply(window)
        }
      }
    }
  }
  
  pub fn toggle_primitive_mode(&mut self, mode: EnumRendererPrimitiveMode) -> Result<(), EnumRendererError> {
    return self.m_api.toggle_primitive_mode(mode);
  }
  
  pub fn check_extension(&self, desired_extension: &str) -> bool {
    return self.m_api.check_extension(desired_extension);
  }
  
  pub fn on_event(&mut self, event: &events::EnumEvent) -> Result<bool, EnumRendererError> {
    match event {
      events::EnumEvent::WindowCloseEvent(_time) => {
        self.m_api.free()?;
        self.m_state = EnumRendererState::Deleted;
        return Ok(true);
      }
      _ => {}
    }
    return self.m_api.on_event(event);
  }
  
  pub fn flush(&mut self) -> Result<(), EnumRendererError> {
    log!("INFO", "[Renderer] -->\t User called for manual flushing of rendered assets");
    return self.m_api.flush();
  }
  
  pub fn on_render(&mut self) -> Result<(), EnumRendererError> {
    return self.m_api.on_render();
  }
  
  // pub fn enable(&mut self, feature: EnumRendererOption) -> Result<(), EnumRendererError> {
  //   return self.m_api.enable(feature);
  // }
  
  pub fn get_api_handle(&mut self) -> &mut dyn Any {
    return self.m_api.get_api_handle();
  }
  
  pub fn free(&mut self) -> Result<(), EnumRendererError> {
    log!(EnumLogColor::Purple, "INFO", "[Renderer] -->\t Freeing resources...");
    if self.m_state == EnumRendererState::NotCreated || self.m_state == EnumRendererState::Deleted {
      return Ok(());
    }
    
    // Free up resources.
    self.m_api.free()?;
    self.m_state = EnumRendererState::Deleted;
    log!(EnumLogColor::Green, "INFO", "[Renderer] -->\t Freed resources successfully");
    return Ok(());
  }
  
  pub fn enqueue(&mut self, r_entity: &mut REntity, shader_associated: &mut Shader) -> Result<(), EnumRendererError> {
    let mut new_id = rand::random::<u64>();
    unsafe {
      while S_ENTITIES_ID_CACHE.contains(&new_id) {
        new_id = rand::random();
      }
    }
    r_entity.set_uuid(new_id);
    return self.m_api.enqueue(r_entity, shader_associated);
  }
  
  pub fn dequeue(&mut self, id: u64, _primitive_index_selected: Option<usize>) -> Result<(), EnumRendererError> {
    return self.m_api.dequeue(id);
  }
  
  pub fn update_ubo_camera(&mut self, view: Mat4, projection: Mat4) -> Result<(), EnumRendererError> {
    return self.m_api.update_ubo_camera(view, projection);
  }
  
  pub fn update_ubo_model(&mut self, model_transform: Mat4, instance_offset: usize) -> Result<(), EnumRendererError> {
    return self.m_api.update_ubo_model(model_transform, instance_offset);
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
    if self.m_state != EnumRendererState::Deleted && self.m_state != EnumRendererState::NotCreated {
      log!(EnumLogColor::Purple, "INFO", "[Renderer] -->\t Dropping renderer...");
      match self.free() {
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