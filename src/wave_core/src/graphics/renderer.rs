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
use std::fmt::{Display, Formatter};

use crate::Engine;
use crate::utils::macros::logger::*;
use crate::assets::asset_loader;
use crate::assets::r_assets::{REntity};
use crate::{events, TraitApply, TraitFree, TraitHint};
use crate::graphics::{open_gl, texture};
use crate::graphics::open_gl::renderer::GlContext;
use crate::graphics::shader::{Shader};
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan;
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan::renderer::VkContext;
use crate::math::{Mat4};
use crate::window::Window;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumRendererState {
  NotCreated,
  Created,
  Submitted,
  Deleted,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumRendererCallCheckingMode {
  None,
  Async,
  Sync,
  SyncAndAsync,
}

impl Default for EnumRendererCallCheckingMode {
  fn default() -> Self {
    return EnumRendererCallCheckingMode::Async;
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererApi {
  OpenGL,
  Vulkan,
}

impl Default for EnumRendererApi {
  fn default() -> Self {
    return EnumRendererApi::OpenGL;
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererTarget {
  Runtime,
  Editor,
}

impl Default for EnumRendererTarget {
  fn default() -> Self {
    return EnumRendererTarget::Runtime;
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererCull {
  Front,
  Back,
  FrontAndBack,
}

impl Default for EnumRendererCull {
  fn default() -> Self {
    return EnumRendererCull::Back;
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererBlendingFactor {
  Zero,
  One,
  SrcColor,
  OneMinusSrcColor,
  DstColor,
  OneMinusDstColor,
  SrcAlpha,
  OneMinusSrcAlpha,
  DstAlpha,
  OneMinusDstAlpha,
  ConstantColor,
  OneMinusConstantColor,
  ConstantAlpha,
  OneMinusConstantAlpha
}

impl Default for EnumRendererBlendingFactor {
  fn default() -> Self {
    return EnumRendererBlendingFactor::OneMinusSrcAlpha;
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererRenderPrimitiveAs {
  Points,
  Filled,
  Wireframe,
  SolidWireframe,
}

impl Default for EnumRendererRenderPrimitiveAs {
  fn default() -> Self {
    return EnumRendererRenderPrimitiveAs::SolidWireframe;
  }
}

impl Display for EnumRendererBlendingFactor {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return match self {
      EnumRendererBlendingFactor::Zero => write!(f, "Zero"),
      EnumRendererBlendingFactor::One => write!(f, "One"),
      EnumRendererBlendingFactor::SrcColor => write!(f, "Source color"),
      EnumRendererBlendingFactor::OneMinusSrcColor => write!(f, "One minus source color"),
      EnumRendererBlendingFactor::DstColor => write!(f, "Destination color"),
      EnumRendererBlendingFactor::OneMinusDstColor => write!(f, "One minus dst color"),
      EnumRendererBlendingFactor::SrcAlpha => write!(f, "Source alpha"),
      EnumRendererBlendingFactor::OneMinusSrcAlpha => write!(f, "One minus src alpha"),
      EnumRendererBlendingFactor::DstAlpha => write!(f, "Destination alpha"),
      EnumRendererBlendingFactor::OneMinusDstAlpha => write!(f, "One minus dst alpha"),
      EnumRendererBlendingFactor::ConstantColor => write!(f, "Constant color"),
      EnumRendererBlendingFactor::OneMinusConstantColor => write!(f, "One minus constant color"),
      EnumRendererBlendingFactor::ConstantAlpha => write!(f, "Constant alpha"),
      EnumRendererBlendingFactor::OneMinusConstantAlpha => write!(f, "One minus constant alpha"),
    }
  }
}

impl Display for EnumRendererCull {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return match self {
      EnumRendererCull::Front => write!(f, "Front face culling"),
      EnumRendererCull::Back => write!(f, "Back face culling"),
      EnumRendererCull::FrontAndBack => write!(f, "Front and back face culling")
    };
  }
}

impl Display for EnumRendererRenderPrimitiveAs {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return match self {
      EnumRendererRenderPrimitiveAs::Filled => write!(f, "Filled"),
      EnumRendererRenderPrimitiveAs::Points => write!(f, "Point"),
      EnumRendererRenderPrimitiveAs::Wireframe => write!(f, "Wireframe"),
      EnumRendererRenderPrimitiveAs::SolidWireframe => write!(f, "Solid wireframe")
    };
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererOptimizationMode {
  NoOptimizations,
  MinimizeDrawCalls
}

impl Default for EnumRendererOptimizationMode {
  fn default() -> Self {
    return EnumRendererOptimizationMode::NoOptimizations;
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumRendererHint {
  ForceApiVersion(u32),
  /// Combine primitives with the same material type into a single buffer if possible, both for the vbo and ibo.
  /// ### Argument:
  /// - *true* **Default**: Enables the batching of similar materials from the same shader by appending their vertices in the same vbo,
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
  Optimization(EnumRendererOptimizationMode),
  
  SplitLargeVertexBuffers(Option<usize>),
  SplitLargeIndexBuffers(Option<usize>),
  
  
  /// Track internal api calls for potential errors and warnings when making api calls in the renderer.
  /// ### Argument:
  /// Four possible values can be provided:
  /// - [EnumRendererCallCheckingMode::None]: Disable api call checking altogether.
  ///
  ///   - Note that this setting is most likely what you
  /// would want in a performance-hungry build as it doesn't waste resources for logging states in between api calls. However, you
  /// run the risk of ignoring potential errors and warnings and/or causing a crash down the line if fatal errors do occur.
  ///
  ///
  /// - [EnumRendererCallCheckingMode::Sync]: Make synchronous checks in between every call to monitor internal state for warnings and errors.
  ///
  ///   - This setting is what you would want ideally in a debug build as the error and warning reports are instantaneous, ideal for logging.
  /// However, performance-wise, it is the slowest. Also, the error codes are not as descriptive, since it only reports the
  /// error code and the call itself. For more descriptive and clear error handling, see [EnumRendererCallCheckingMode::Async] or
  /// [EnumRendererCallCheckingMode::SyncAndAsync].
  ///
  ///
  /// - [EnumRendererCallCheckingMode::Async] **Default**: Let the api asynchronously deal with its own warning and error reporting.
  ///
  ///   - This option is a nice
  /// balance between performance and verbosity and thus is the *default* option if this option isn't toggled manually. However,
  /// the only downside of this approach is that the origin of the error reported might be difficult to track down, due to its async
  /// nature. If this is an issue, try [EnumRendererCallCheckingMode::Sync] or [EnumRendererCallCheckingMode::SyncAndAsync] for better error messages
  /// while tracking down the errors.
  ///
  ///
  /// - [EnumRendererCallCheckingMode::SyncAndAsync]: This setting combines both the synchronous and the asynchronous natures for error
  /// checking.
  ///
  ///   - This setting should be selected if you are on a dev build and require the utmost verbosity and timing for your api
  /// error handling in order to prevent error propagation, while having as many details as possible regarding the error.
  /// Sometimes necessary when dealing with obscure bugs and crashes.
  ///
  ApiCallChecking(EnumRendererCallCheckingMode),
  
  /// Enable depth testing to avoid artefacts or overlapping geometry incorrectly displayed onto the screen.
  DepthTest(bool),
  /// Enable culling for a specific face to avoid rendering it when unneeded save on fragment shader calls when rendering.
  /// ### Argument:
  /// Four possible values can be provided:
  /// - [None]: Applies no culling when rendering primitives.
  ///
  /// - Some([EnumRendererCull::Back]) **Default**: Cull only back faces of primitives.
  ///
  /// - Some([EnumRendererCull::Front]): Cull only front faces of primitives. Note that if the winding order is clock-wise,
  /// this essentially is equivalent to [EnumRendererCull::Back] with counter clock-wise winding order.
  ///
  /// - Some([EnumRendererCull::FrontAndBack]): Cull both front and back faces of primitives.
  CullFacing(Option<EnumRendererCull>),
  MSAA(Option<u8>),
  SRGB(bool),
  Blending(Option<(EnumRendererBlendingFactor, EnumRendererBlendingFactor)>),
}

impl EnumRendererHint {
  pub fn is_equivalent(&self, other: &EnumRendererHint) -> bool {
    return std::mem::discriminant(self) == std::mem::discriminant(other);
  }
  
  pub fn get_value(&self) -> &dyn Any {
    return match self {
      EnumRendererHint::Optimization(bool) => bool,
      EnumRendererHint::ApiCallChecking(mode) => mode,
      EnumRendererHint::DepthTest(bool) => bool,
      EnumRendererHint::CullFacing(mode) => mode,
      EnumRendererHint::MSAA(sample_count) => sample_count,
      EnumRendererHint::SRGB(bool) => bool,
      EnumRendererHint::Blending(blend_func) => blend_func,
      EnumRendererHint::SplitLargeVertexBuffers(vertex_limit) => vertex_limit,
      EnumRendererHint::SplitLargeIndexBuffers(index_limit) => index_limit,
      EnumRendererHint::ForceApiVersion(version) => version
    }
  }
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
  TextureError(texture::EnumTextureError),
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

impl From<texture::EnumTextureError> for EnumRendererError {
  fn from(value: texture::EnumTextureError) -> Self {
    return EnumRendererError::TextureError(value);
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
  fn new() -> Self where Self: Sized;
  fn get_api_handle(&mut self) -> &mut dyn Any;
  fn get_api_version(&self) -> f32;
  fn get_max_shader_version_available(&self) -> u16;
  fn check_extension(&self, desired_extension: &str) -> bool;
  fn on_event(&mut self, event: &events::EnumEvent) -> Result<bool, EnumRendererError>;
  fn on_render(&mut self) -> Result<(), EnumRendererError>;
  fn apply(&mut self, window: &mut Window, renderer_options: &Vec<EnumRendererHint>) -> Result<(), EnumRendererError>;
  fn toggle_visibility_of(&mut self, entity_uuid: u64, sub_primitive_offset: Option<usize>, instance_count: usize, visible: bool) -> Result<(), EnumRendererError>;
  fn toggle_primitive_mode(&mut self, mode: EnumRendererRenderPrimitiveAs, entity_uuid: u64, sub_primitive_index: Option<usize>, instance_count: usize) -> Result<(), EnumRendererError>;
  fn get_max_msaa_count(&self) -> Result<u8, EnumRendererError>;
  fn to_string(&self) -> String;
  fn toggle_options(&mut self, renderer_options: &Vec<EnumRendererHint>) -> Result<(), EnumRendererError>;
  fn flush(&mut self) -> Result<(), EnumRendererError>;
  fn enqueue(&mut self, entity: &REntity, shader_associated: &mut Shader) -> Result<(), EnumRendererError>;
  fn dequeue(&mut self, id: u64) -> Result<(), EnumRendererError>;
  fn update_ubo_camera(&mut self, view: Mat4, projection: Mat4) -> Result<(), EnumRendererError>;
  fn update_ubo_model(&mut self, model_transform: Mat4, entity_uuid: u64, instance_offset: Option<usize>, instance_count: usize) -> Result<(), EnumRendererError>;
  fn free(&mut self) -> Result<(), EnumRendererError>;
}

pub struct Renderer {
  pub(crate) m_state: EnumRendererState,
  pub(crate) m_type: EnumRendererApi,
  pub(crate) m_hints: Vec<EnumRendererHint>,
  pub(crate) m_ids: Vec<u64>,
  m_api: Box<dyn TraitContext>,
}

impl Default for Renderer {
  fn default() -> Self {
    let hints = vec![EnumRendererHint::ApiCallChecking(Default::default()),
      EnumRendererHint::SRGB(true), EnumRendererHint::DepthTest(true),
      EnumRendererHint::Blending(Some((EnumRendererBlendingFactor::SrcAlpha, EnumRendererBlendingFactor::default()))),
      EnumRendererHint::Optimization(Default::default()),
      EnumRendererHint::CullFacing(Some(Default::default())),
      EnumRendererHint::MSAA(None)];
    
    return Self {
      m_state: EnumRendererState::Created,
      m_type: EnumRendererApi::default(),
      m_hints: hints.clone(),
      m_ids: Vec::with_capacity(10),
      m_api: Box::new(GlContext::new()),
    };
  }
}

impl TraitHint<EnumRendererHint> for Renderer {
  fn set_hint(&mut self, hint: EnumRendererHint) {
    if let Some(position) = self.m_hints.iter().position(|h| h.is_equivalent(&hint)) {
      self.m_hints.remove(position);
    }
    
    self.m_hints.push(hint);
  }
  
  fn reset_hints(&mut self) {
    self.m_hints = vec![EnumRendererHint::ApiCallChecking(Default::default()),
      EnumRendererHint::SRGB(true), EnumRendererHint::DepthTest(true),
      EnumRendererHint::Blending(Some((EnumRendererBlendingFactor::SrcAlpha, EnumRendererBlendingFactor::default()))),
      EnumRendererHint::Optimization(Default::default()),
      EnumRendererHint::CullFacing(Some(Default::default()))];
  }
}

impl TraitApply<EnumRendererError> for Renderer {
  fn apply(&mut self) -> Result<(), EnumRendererError> {
    let window = Engine::get_active_window();
    
    // Set default hints if none specified.
    if self.m_hints.is_empty() {
      self.reset_hints();
    }
    if self.m_type == EnumRendererApi::Vulkan {
      #[cfg(not(feature = "vulkan"))]
      {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot apply Vulkan renderer, vulkan feature not enabled!");
        return Err(EnumRendererError::InvalidApi);
      }
      
      return self.m_api.apply(window, &self.m_hints);
    }
    
    return self.m_api.apply(window, &self.m_hints);
  }
}

impl TraitFree<EnumRendererError> for Renderer {
  fn free(&mut self) -> Result<(), EnumRendererError> {
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
}

impl<'a> Renderer {
  pub fn new(api_chosen: EnumRendererApi) -> Self {
    return match api_chosen {
      EnumRendererApi::OpenGL => {
        Renderer {
          m_state: EnumRendererState::Created,
          m_type: EnumRendererApi::OpenGL,
          m_hints: vec![],
          m_ids: Vec::with_capacity(10),
          m_api: Box::new(GlContext::new()),
        }
      }
      EnumRendererApi::Vulkan => {
        Renderer {
          m_state: EnumRendererState::Created,
          m_type: EnumRendererApi::Vulkan,
          m_hints: vec![],
          m_ids: Vec::with_capacity(10),
          m_api: Box::new(VkContext::new()),
        }
      }
    }
  }
  
  pub fn hide(&mut self, entity_uuid: u64, sub_primitive_offset: Option<usize>, instance_count: usize) -> Result<(), EnumRendererError> {
    log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Asset {0} now hidden", entity_uuid);
    return self.m_api.toggle_visibility_of(entity_uuid, sub_primitive_offset, instance_count, false);
  }
  
  pub fn show(&mut self, entity_uuid: u64, sub_primitive_offset: Option<usize>, instance_count: usize) -> Result<(), EnumRendererError> {
    log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Asset {0} now shown", entity_uuid);
    return self.m_api.toggle_visibility_of(entity_uuid, sub_primitive_offset, instance_count, true);
  }
  
  pub fn toggle_primitive_mode(&mut self, name: &'static str, mode: EnumRendererRenderPrimitiveAs, entity_uuid: u64, instance_offset: Option<usize>,
                               instance_count: usize) -> Result<(), EnumRendererError> {
    self.m_api.toggle_primitive_mode(mode, entity_uuid, instance_offset, instance_count)?;
    
    log!("INFO", "[Renderer] -->\t {0} now shown as \x1b[0;35m{1}\x1b[0m",instance_offset.is_some()
      .then(|| format!("Sub primitive \x1b[0;35m{0}\x1b[0m of asset \x1b[0;35m{1}\x1b[0m", instance_offset.unwrap(), name))
      .unwrap_or(format!("Asset \x1b[0;35m{0}\x1b[0m", name)), mode);
    return Ok(());
  }
  
  pub fn toggle_msaa(&mut self, _sample_count: Option<u32>) -> Result<(), EnumRendererError> {
    todo!()
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
  
  pub fn get_type(&self) -> EnumRendererApi {
    return self.m_type;
  }
  
  pub fn get_api_handle(&mut self) -> &mut dyn Any {
    return self.m_api.get_api_handle();
  }
  
  pub fn enqueue(&mut self, r_entity: &mut REntity, shader_associated: &mut Shader) -> Result<(), EnumRendererError> {
    let mut new_id = 0;
    while self.m_ids.contains(&new_id) {
       new_id += 1;
    }
    r_entity.m_renderer_id = new_id;
    self.m_ids.push(new_id);
    return self.m_api.enqueue(r_entity, shader_associated);
  }
  
  pub fn dequeue(&mut self, id: u64, _primitive_index_selected: Option<usize>) -> Result<(), EnumRendererError> {
    return self.m_api.dequeue(id);
  }
  
  pub fn update_ubo_camera(&mut self, view: Mat4, projection: Mat4) -> Result<(), EnumRendererError> {
    return self.m_api.update_ubo_camera(view, projection);
  }
  
  pub fn update_ubo_model(&mut self, model_transform: Mat4, entity_uuid: u64, instance_offset: Option<usize>, instance_count: usize) -> Result<(), EnumRendererError> {
    return self.m_api.update_ubo_model(model_transform, entity_uuid, instance_offset, instance_count);
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