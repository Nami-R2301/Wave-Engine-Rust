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

extern crate ash;


use std::fmt::Debug;
use std::ptr::{null_mut};

use ash::vk::{self, TaggedStructure};
use ash::extensions::{ext, khr};

use crate::log;
use crate::wave::assets::renderable_assets::REntity;
use crate::wave::Engine;
use crate::wave::graphics::shader::GlslShader;
use crate::wave::window::GlfwWindow;

use super::buffer::*;

static mut S_VULKAN: Option<VkApp> = None;
static mut S_STATE: EnumState = EnumState::Ok;

#[cfg(feature = "OpenGL")]
use crate::wave::graphics::buffer::{EnumAttributeType, GlVertexAttribute};

#[cfg(feature = "OpenGL")]
static S_ERROR_CALLBACK: gl::types::GLDEBUGPROC = Some(gl_error_callback);
static mut S_STATS: Stats = Stats {
  m_entities_sent_count: 0,
  m_shader_bound_count: 0,
  m_vao_bound_count: 0,
  m_ibo_bound_count: 0,
  m_texture_bound_count: 0,
};

static mut S_BATCH: BatchPrimitives = BatchPrimitives {
  m_shaders: vec![],
  m_vao_buffers: vec![],
  m_vbo_buffers: vec![],
};

const REQUIRED_LAYERS: [&str; 2] = ["VK_LAYER_KHRONOS_profiles", "VK_LAYER_KHRONOS_validation"];

#[macro_export]
macro_rules! check_gl_call {
    () => {};
    ($name:literal, $gl_function:expr) => {
      unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
      unsafe {
        $gl_function;
        let error = gl::GetError();
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t Error when executing gl call! \
          Code => 0x{1:x}", $name, error);
          return Err(EnumErrors::GlError(error));
        }
      }
    };
    ($name:literal, let mut $var:ident: $var_type:ty = $gl_function:expr) => {
      unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
      let mut $var:$var_type = unsafe { $gl_function };
      unsafe {
        let error = gl::GetError();
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t Error when executing gl call! \
               Code => 0x{1:x}", $name, error);
          return Err(EnumErrors::GlError(error));
        }
      }
    };
    ($name:literal, let $var:ident: $var_type:ty = $gl_function:expr) => {
      unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
      let $var:$var_type = unsafe { $gl_function };
      unsafe {
        let error = gl::GetError();
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t Error when executing gl call! \
             Code => 0x{1:x}", $name, error);
          return Err(EnumErrors::GlError(error));
        }
      }
    };
    ($name:literal, $var:ident = $gl_function:expr) => {
      unsafe { while gl::GetError() != gl::NO_ERROR {} };
      unsafe { $var = $gl_function; }
      unsafe {
        let error = gl::GetError();
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t Error when executing gl call! \
           Code => 0x{1:x}", $name, error);
          return Err(EnumErrors::GlError(error));
        }
      }
    };
}

#[derive(Debug)]
pub enum EnumApi {
  OpenGL,
  Vulkan,
}

#[derive(Debug, Copy, Clone)]
pub enum EnumFeature {
  Debug(bool),
  DepthTest(bool),
  CullFacing(bool, GLenum),
  Wireframe(bool),
  MSAA(bool),
  SRGB(bool),
  Blending(bool, GLenum, GLenum),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumErrors {
  Init,
  NotImplemented,
  ContextError,
  InvalidEntity,
  EntityNotFound,
  GlError(GLenum),
  VulkanError(EnumVulkanErrors),
  CError,
  ShaderError,
  WrongOffset,
  WrongSize,
  NoAttributes,
  NoActiveWindow,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumVulkanErrors {
  NotSupported,
  EntryError,
  InstanceError,
  DebugError,
  DeviceError,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumState {
  Ok,
  Error,
  CriticalError,
}

pub trait TraitRenderableEntity {
  fn send(&mut self, shader_associated: &mut GlslShader) -> Result<(), EnumErrors>;
  fn resend(&mut self, shader_associated: &mut GlslShader) -> Result<(), EnumErrors>;
  fn free(&mut self, shader_associated: &mut GlslShader) -> Result<(), EnumErrors>;
  fn is_sent(&self) -> bool;
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

struct BatchPrimitives {
  m_shaders: Vec<u32>,
  m_vao_buffers: Vec<GlVao>,
  m_vbo_buffers: Vec<GlVbo>,
}

trait TraitRenderer {}

pub fn init() -> Result<(), EnumErrors> {
  #[cfg(feature = "OpenGL")]
  {
    gl::load_with(|f_name| GlfwWindow::get_active_context().expect("Not window context loaded!")
      .get_proc_address_raw(f_name));
    
    let current_window = Engine::get_active_window();
    if current_window.is_none() {
      check_gl_call!("Renderer", gl::Viewport(0, 0, 640, 480));
    } else {
      check_gl_call!("Renderer", gl::Viewport(0, 0, (*current_window.unwrap()).get_size().x,
                              (*current_window.unwrap()).get_size().y));
    }
    check_gl_call!("Renderer", gl::ClearColor(0.15, 0.15, 0.15, 1.0));
    
    check_gl_call!("Renderer", gl::FrontFace(gl::CW));
    return Ok(());
  }
  #[cfg(feature = "Vulkan")]
  {
    unsafe { S_VULKAN = Some(VkApp::new()?) };
    return Ok(());
  }
}


pub fn shutdown() -> Result<(), EnumErrors> {
  if unsafe { S_STATE == EnumState::Error } {
    return Err(EnumErrors::NotImplemented);
  }
  
  return Ok(());
}

pub fn begin() {}

pub fn end() {}

pub fn batch() {}

pub fn flush() {}

pub fn send(sendable_entity: &REntity, _shader_associated: &mut GlslShader) -> Result<(), EnumErrors> {
  if sendable_entity.is_empty() {
    log!(EnumLogColor::Yellow, "WARN", "[Renderer] --> Entity {0} sent has no \
      vertices! Not sending it...", sendable_entity)
  }
  
  #[cfg(feature = "OpenGL")]
  {
    let mut offset: usize = 0;
    
    // Allocate main dynamic vbo to hold all the data provided.
    let mut vbo: GlVbo = GlVbo::new(sendable_entity.size(), sendable_entity.count())?;
    let mut vao: GlVao = GlVao::new()?;
    
    // IDs (Vec3).
    vbo.set_data(sendable_entity.m_entity_id.as_ptr() as *const GLvoid,
      std::mem::size_of::<u32>() * sendable_entity.m_entity_id.len(), offset)?;
    offset += std::mem::size_of::<u32>() * sendable_entity.m_entity_id.len();
    
    // Positions (Vec3s).
    vbo.set_data(sendable_entity.m_vertices.as_ptr() as *const GLvoid,
      std::mem::size_of::<f32>() * sendable_entity.m_vertices.len(), offset)?;
    offset += std::mem::size_of::<f32>() * sendable_entity.m_vertices.len();
    
    // Normals (Vec3s).
    vbo.set_data(sendable_entity.m_normals.as_ptr() as *const GLvoid,
      std::mem::size_of::<f32>() * sendable_entity.m_normals.len(), offset)?;
    offset += std::mem::size_of::<f32>() * sendable_entity.m_normals.len();
    
    // Colors (Colors).
    vbo.set_data(sendable_entity.m_colors.as_ptr() as *const GLvoid,
      std::mem::size_of::<f32>() * sendable_entity.m_colors.len(), offset)?;
    offset += std::mem::size_of::<f32>() * sendable_entity.m_colors.len();
    
    // Texture coordinates (Vec2s).
    vbo.set_data(sendable_entity.m_texture_coords.as_ptr() as *const GLvoid,
      std::mem::size_of::<f32>() * sendable_entity.m_texture_coords.len(), offset)?;
    
    offset = 0;
    
    // Establish vao attributes.
    let mut attributes: Vec<GlVertexAttribute> = Vec::with_capacity(5);
    
    attributes.push(GlVertexAttribute::new(EnumAttributeType::UnsignedInt(1),
      false, offset));
    offset += std::mem::size_of::<u32>() * sendable_entity.m_entity_id.len();
    
    attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec3,
      false, offset));
    offset += std::mem::size_of::<f32>() * sendable_entity.m_vertices.len();
    
    attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec3,
      false, offset));
    offset += std::mem::size_of::<f32>() * sendable_entity.m_normals.len();
    
    attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec4,
      false, offset));
    offset += std::mem::size_of::<f32>() * sendable_entity.m_colors.len();
    
    attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec2,
      false, offset));
    
    // Enable vertex attributes.
    vao.enable_attributes(attributes)?;
    
    unsafe {
      S_BATCH.m_shaders.push(_shader_associated.m_program_id);
      S_BATCH.m_vao_buffers.push(vao);
      S_BATCH.m_vbo_buffers.push(vbo);
    }
  }
  
  return Ok(());
}

pub fn draw() -> Result<(), EnumErrors> {
  #[cfg(feature = "OpenGL")]
  {
    for index in 0usize..unsafe { S_BATCH.m_shaders.len() } {
      check_gl_call!("Renderer", gl::UseProgram(S_BATCH.m_shaders[index]));
      unsafe { S_BATCH.m_vao_buffers[index].bind()?; }
      check_gl_call!("Renderer", gl::DrawArrays(gl::TRIANGLES, 0,
          S_BATCH.m_vbo_buffers[index].m_count as GLsizei));
    };
  }
  return Ok(());
}

pub fn free(_entity_sent_id: &u64) -> Result<(), EnumErrors> {
  todo!()
}

pub fn get_renderer_info() -> String {
  unsafe {
    #[cfg(feature = "OpenGL")]
    {
      let renderer_info = std::ffi::CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8)
        .to_str().unwrap_or("Cannot retrieve renderer information!");
      
      let str: String = format!("Renderer Hardware => {0}", renderer_info);
      return str;
    }
    #[cfg(feature = "Vulkan")]
    {
      return "Renderer Hardware => NVIDIA Corporation".to_string();
    }
  }
}

pub fn get_api_info() -> String {
  unsafe {
    #[cfg(feature = "OpenGL")]
    {
      let api_vendor = std::ffi::CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8)
        .to_str().unwrap_or("Cannot retrieve api vendor information!");
      let api_version = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
        .to_str().unwrap_or("Cannot retrieve api version information!");
      
      return format!("Renderer SDK => {0}, OpenGL {1}", api_vendor, api_version);
    }
    #[cfg(feature = "Vulkan")]
    {
      return "Renderer SDK => Vulkan SDK, Api version 1.3".to_string();
    }
  }
}

pub fn get_shading_info() -> String {
  unsafe {
    #[cfg(feature = "OpenGL")]
    {
      let shading_info = std::ffi::CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8)
        .to_str().unwrap_or("Cannot retrieve shading language information!");
      
      return format!("Shading Language (GLSL) => {0}", shading_info);
    }
    #[cfg(feature = "Vulkan")]
    {
      return "Shading Language (GLSL) => 4.60".to_string();
    }
  }
}

pub fn get_state() -> EnumState {
  return unsafe { S_STATE };
}

pub fn get_callback() {}

pub fn get_stats() -> &'static Stats {
  unsafe { return &S_STATS; }
}

pub fn toggle_feature(feature: EnumFeature) -> Result<(), EnumErrors> {
  match feature {
    EnumFeature::Debug(flag) => {
      if flag {
        #[cfg(feature = "debug")]
        {
          #[cfg(feature = "OpenGL")]
          {
            check_gl_call!("Renderer", gl::Enable(gl::DEBUG_OUTPUT));
            check_gl_call!("Renderer", gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS));
            check_gl_call!("Renderer", gl::DebugMessageCallback(S_ERROR_CALLBACK, std::ptr::null()));
            check_gl_call!("Renderer", gl::DebugMessageControl(gl::DONT_CARE, gl::DEBUG_TYPE_OTHER,
            gl::DONT_CARE, 0, std::ptr::null(), gl::FALSE));
            log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Debug mode enabled")
          }
        }
      } else {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::Disable(gl::DEBUG_OUTPUT));
          check_gl_call!("Renderer", gl::Disable(gl::DEBUG_OUTPUT_SYNCHRONOUS));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Debug mode disabled")
        }
      }
    }
    EnumFeature::DepthTest(flag) => {
      if flag {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::Enable(gl::DEPTH_TEST));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Depth test enabled")
        }
      } else {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::Disable(gl::DEPTH_TEST));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Depth test disabled")
        }
      }
    }
    EnumFeature::MSAA(flag) => {
      if flag {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::Enable(gl::MULTISAMPLE));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t MSAA enabled")
        }
      } else {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::Disable(gl::MULTISAMPLE));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t MSAA disabled")
        }
      }
    }
    EnumFeature::Blending(flag, s_factor, d_factor) => {
      if flag {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::Enable(gl::BLEND));
          check_gl_call!("Renderer", gl::BlendFunc(s_factor, d_factor));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Blending enabled")
        }
      } else {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::Disable(gl::BLEND));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Blending disabled")
        }
      }
    }
    EnumFeature::Wireframe(flag) => {
      if flag {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Wireframe mode enabled")
        }
      } else {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Wireframe mode disabled")
        }
      }
    }
    EnumFeature::SRGB(flag) => {
      if flag {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::Enable(gl::FRAMEBUFFER_SRGB));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t SRGB framebuffer enabled")
        }
      } else {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::Disable(gl::FRAMEBUFFER_SRGB));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t SRGB framebuffer disabled")
        }
      }
    }
    EnumFeature::CullFacing(flag, face) => {
      if flag {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::Enable(gl::CULL_FACE));
          check_gl_call!("Renderer", gl::CullFace(face));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Cull facing enabled")
        }
      } else {
        #[cfg(feature = "OpenGL")]
        {
          check_gl_call!("Renderer", gl::Disable(gl::CULL_FACE));
          log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Cull facing disabled")
        }
      }
    }
  }
  return Ok(());
}

/*
///////////////////////////////////   Vulkan    ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
 */

#[derive(Clone)]
pub struct VkApp {
  m_entry: ash::Entry,
  m_instance: ash::Instance,
  m_surface: khr::Surface,
  m_surface_khr: vk::SurfaceKHR,
  m_debug_report_callback: Option<(ext::DebugUtils, vk::DebugUtilsMessengerEXT)>,
}

impl VkApp {
  pub fn new() -> Result<Self, EnumErrors> {
    let window_context = GlfwWindow::get_active_context();
    if window_context.is_none() {
      return Err(EnumErrors::NoActiveWindow);
    }
    
    return match VkApp::create_instance(window_context.as_ref().unwrap()) {
      Ok((entry, vk_instance)) => {
        let vk_surface = khr::Surface::new(&entry, &vk_instance);
        let khr_surface = vk::SurfaceKHR::default();
        
        #[cfg(feature = "debug")]
        {
          // For debug callback function
          let mut debug_create_info = vk::DebugUtilsMessengerCreateInfoEXT::default();
          debug_create_info.s_type = vk::DebugUtilsMessengerCreateInfoEXT::STRUCTURE_TYPE;
          debug_create_info.message_severity |= vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR |
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE;
          debug_create_info.message_type |= vk::DebugUtilsMessageTypeFlagsEXT::GENERAL |
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION |
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE;
          debug_create_info.pfn_user_callback = Some(vulkan_debug_callback);
          debug_create_info.p_user_data = null_mut(); // Optional
          
          let debug_utils = ext::DebugUtils::new(&entry, &vk_instance);
          let debug_messenger = unsafe {
            debug_utils
              .create_debug_utils_messenger(&debug_create_info, None)
              .unwrap()
          };
          
          let vulkan_instance = VkApp {
            m_entry: entry,
            m_instance: vk_instance,
            m_surface: vk_surface,
            m_surface_khr: khr_surface,
            m_debug_report_callback: Some((debug_utils, debug_messenger)),
          };
          unsafe { S_VULKAN = Some(vulkan_instance.clone()) };
          Ok(vulkan_instance)
        }
        
        #[cfg(not(feature = "debug"))]
        {
          let mut vulkan_instance = VkApp {
            m_entry: entry,
            m_instance: vk_instance,
            m_surface: vk_surface,
            m_surface_khr: khr_surface,
            m_debug_report_callback: None,
          };
          unsafe { S_VULKAN = Some(vulkan_instance.clone()) };
          Ok(vulkan_instance)
        }
      }
      Err(err) => {
        Err(err)
      }
    };
  }
  
  pub fn create_instance(window_context: &glfw::Glfw) -> Result<(ash::Entry, ash::Instance), EnumErrors> {
    let entry = ash::Entry::linked();
    
    let app_name = std::ffi::CString::new("Wave Engine Rust").unwrap();
    let engine_name = std::ffi::CString::new("Wave Engine").unwrap();
    let mut app_info = vk::ApplicationInfo::default();
    app_info.p_application_name = app_name.as_ptr();
    app_info.p_engine_name = engine_name.as_ptr();
    app_info.engine_version = vk::make_api_version(0, 0, 1, 0);
    app_info.api_version = vk::API_VERSION_1_3;
    
    let extensions = VkApp::load_extensions(window_context)?;
    let c_extensions_ptr = extensions
      .iter()
      .map(|c_extension| c_extension.as_ptr())
      .collect::<Vec<*const std::ffi::c_char>>();
    
    let layers = VkApp::load_layers(&entry);
    let mut instance_create_info = vk::InstanceCreateInfo::default();
    instance_create_info.enabled_layer_count = layers.len() as u32;
    instance_create_info.pp_enabled_layer_names = layers.as_ptr();
    instance_create_info.enabled_extension_count = c_extensions_ptr.len() as u32;
    instance_create_info.pp_enabled_extension_names = c_extensions_ptr.as_ptr();
    instance_create_info.p_application_info = &app_info;
    
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
      instance_create_info.flags |= vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;
    }
    
    unsafe {
      return match entry.create_instance(&instance_create_info, None) {
        Ok(vk_instance) => {
          Ok((entry, vk_instance))
        }
        Err(err) => {
          log!(EnumLogColor::Red, "ERROR", "[Renderer] --> \t Vulkan Error : {:#?}", err);
          
          Err(EnumErrors::VulkanError(EnumVulkanErrors::InstanceError))
        }
      };
    }
  }
  
  /// Load window extensions for the Vulkan surface.
  pub fn load_extensions(window_context: &glfw::Glfw) -> Result<Vec<std::ffi::CString>, EnumErrors> {
    // Get required extensions.
    let window_extensions = window_context.get_required_instance_extensions();
    if window_extensions.is_some() {
      let mut c_extensions = window_extensions.unwrap()
        .iter()
        .map(|extension| std::ffi::CString::new(extension.as_bytes())
          .expect("Failed to create C string from extension in load_extensions()"))
        .collect::<Vec<std::ffi::CString>>();
      
      #[cfg(feature = "debug")]
      {
        c_extensions.push(std::ffi::CString::new("VK_EXT_debug_utils")
          .expect("Failed to convert to C string!"));
      }
      
      #[cfg(any(target_os = "macos", target_os = "ios"))]
      {
        c_extensions.push(std::ffi::CString::from(vk::KhrPortabilityEnumerationFn::name()));
        // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
        c_extensions.push(std::ffi::CString::from(vk::KhrGetPhysicalDeviceProperties2Fn::name()));
      }
      
      return Ok(c_extensions);
    }
    return Err(EnumErrors::ContextError);
  }
  
  /// Load validation layers for the Vulkan instance.
  pub fn load_layers(entry: &ash::Entry) -> Vec<*const std::ffi::c_char> {
    VkApp::check_validation_layer_support(entry);
    
    let layers = unsafe {
      vec![std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0").as_ptr()]
    };
    
    return layers;
  }
  
  /// Check if the required validation set in `REQUIRED_LAYERS`
  /// are supported by the Vulkan instance.
  ///
  /// # Panics
  ///
  /// Panic if at least one on the layer is not supported.
  pub fn check_validation_layer_support(entry: &ash::Entry) {
    for required in REQUIRED_LAYERS.iter() {
      let found = entry
        .enumerate_instance_layer_properties()
        .unwrap()
        .iter()
        .any(|layer| {
          let name = unsafe { std::ffi::CStr::from_ptr(layer.layer_name.as_ptr()) };
          let name = name.to_str().expect("Failed to get layer name pointer");
          required == &name
        });
      
      if !found {
        panic!("Validation layer not supported: {}", required);
      }
    }
  }
}

impl Drop for VkApp {
  fn drop(&mut self) {
    unsafe {
      self.m_surface.destroy_surface(self.m_surface_khr, None);
      #[cfg(feature = "debug")]
      {
        if let Some((debug, messenger)) = self.m_debug_report_callback.take() {
          debug.destroy_debug_utils_messenger(messenger, None);
        }
      }
      self.m_instance.destroy_instance(None);
    }
  }
}

unsafe extern "system" fn vulkan_debug_callback(flag: vk::DebugUtilsMessageSeverityFlagsEXT,
                                                type_: vk::DebugUtilsMessageTypeFlagsEXT,
                                                p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
                                                _: *mut std::ffi::c_void) -> vk::Bool32 {
  use vk::DebugUtilsMessageSeverityFlagsEXT as Flag;
  
  let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
  match flag {
    Flag::VERBOSE => log!("INFO", "{:?} - {:?}", type_, message),
    Flag::INFO => log!("INFO", "{:?} - {:?}", type_, message),
    Flag::WARNING => log!(EnumLogColor::Yellow, "WARN", "{:?} - {:?}", type_, message),
    _ => log!(EnumLogColor::Red, "ERR", "{:?} - {:?}", type_, message),
  }
  
  return vk::FALSE;
}


/*
///////////////////////////////////   OpenGL    ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
 */
extern "system" fn gl_error_callback(error_code: GLenum, e_type: GLenum, _id: GLuint,
                                     severity: GLenum, _length: GLsizei, error_message: *const GLchar,
                                     _user_param: *mut std::ffi::c_void) {
  let mut final_error_msg: String = "".to_string();
  if error_code != gl::NO_ERROR {
    match error_code {
      _ => { final_error_msg += &format!("Code => 0x{0:x}; ", error_code) }
    }
    
    match e_type {
      gl::DEBUG_TYPE_ERROR => { final_error_msg += "Type => Error; "; }
      gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => { final_error_msg += "Type => Deprecated behavior; "; }
      gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => { final_error_msg += "Type => Undefined behavior; "; }
      gl::DEBUG_TYPE_PORTABILITY => { final_error_msg += "Type => Portability; "; }
      gl::DEBUG_TYPE_PERFORMANCE => { final_error_msg += "Type => Performance; "; }
      gl::DEBUG_TYPE_MARKER => { final_error_msg += "Type => Marker; "; }
      gl::DEBUG_TYPE_PUSH_GROUP => { final_error_msg += "Type => Push group; "; }
      gl::DEBUG_TYPE_POP_GROUP => { final_error_msg += "Type => Pop group; "; }
      gl::DEBUG_TYPE_OTHER => { final_error_msg += "Type => Other; "; }
      _ => { final_error_msg = "Type => Unknown; ".to_string(); }
    }
    
    match severity {
      gl::DEBUG_SEVERITY_HIGH => { final_error_msg += "Severity => Fatal (High); " }
      gl::DEBUG_SEVERITY_MEDIUM => { final_error_msg += "Severity => Fatal (Medium); " }
      gl::DEBUG_SEVERITY_LOW => { final_error_msg += "Severity => Warn (Low); " }
      gl::DEBUG_SEVERITY_NOTIFICATION => { final_error_msg += "Severity => Warn (Info); " }
      _ => { final_error_msg += "Severity => Fatal (Unknown); " }
    }
    
    final_error_msg += unsafe {
      &error_message.as_ref().into_iter()
        .map(|&character| character.to_string())
        .collect::<String>()
    };
    log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t {0}", final_error_msg);
  }
}

