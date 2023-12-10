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

use crate::log;
use crate::wave::assets::renderable_assets::REntity;
use crate::wave::graphics::shader::{TraitShader};
use crate::wave::window::GlfwWindow;

pub static mut S_RENDERER: Option<&mut Renderer> = None;

#[derive(Debug, Copy, Clone, PartialEq)]
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
  NoApi,
  NotImplemented,
  ContextError,
  InvalidEntity,
  EntityNotFound,
  #[cfg(feature = "OpenGL")]
  GlError(GLenum),
  #[cfg(feature = "Vulkan")]
  VulkanError(EnumVulkanErrors),
  CError,
  ShaderError,
  WrongOffset,
  WrongSize,
  NoAttributes,
}

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
  fn new(window: &mut GlfwWindow) -> Result<Self, EnumErrors> where Self: Sized;
  fn get_type(&self) -> EnumApi;
  fn to_string(&self) -> String;
  fn toggle_feature(&mut self, feature: EnumFeature) -> Result<(), EnumErrors>;
  fn begin(&mut self);
  fn end(&mut self);
  fn batch(&mut self);
  fn flush(&mut self);
  fn send(&mut self, entity: &REntity, shader_associated: &mut dyn TraitShader) -> Result<(), EnumErrors>;
  fn draw(&mut self) -> Result<(), EnumErrors>;
  fn free(&mut self, id: &u64) -> Result<(), EnumErrors>;
  fn shutdown(&mut self) -> Result<(), EnumErrors>;
}

pub struct Renderer {
  m_state: EnumState,
  pub m_api: Box<dyn TraitContext>,
}

impl Renderer {
  pub fn new(window: &mut GlfwWindow) -> Result<Renderer, EnumErrors> {
    #[cfg(feature = "Vulkan")]
    return Ok(Renderer {
      m_state: EnumState::Ok,
      m_api: Box::new(VkContext::new(window)?),
    });
    
    #[cfg(feature = "OpenGL")]
    return Ok(Renderer {
      m_state: EnumState::Ok,
      m_api: Box::new(GlContext::new(window)?),
    });
  }
  
  pub fn shutdown(&mut self) -> Result<(), EnumErrors> {
    if self.m_state == EnumState::Error {
      return Err(EnumErrors::NotImplemented);
    }
    self.m_state = EnumState::Shutdown;
    
    return self.m_api.shutdown();
  }
  
  pub fn get_state(&self) -> EnumState {
    return self.m_state;
  }
  
  pub fn get() -> &'static mut Option<&'static mut Renderer> {
    return unsafe { &mut S_RENDERER };
  }
}

impl Display for Renderer {
  fn fmt(&self, format: &mut Formatter<'_>) -> std::fmt::Result {
    write!(format, "State => {0:#?}\n{1:113}Api => {2:#?}\n{1:113}{3}", self.m_state, "", self.m_api.get_type(),
      self.m_api.to_string())
  }
}

/*
///////////////////////////////////   Vulkan    ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
 */

#[cfg(feature = "Vulkan")]
extern crate ash;

use std::fmt::{Display, Formatter};

#[cfg(feature = "Vulkan")]
use std::ops::BitOr;

#[cfg(feature = "Vulkan")]
use ash::extensions::{ext, khr};
#[cfg(feature = "Vulkan")]
use ash::vk::{self, TaggedStructure, PhysicalDeviceType};

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg(feature = "Vulkan")]
pub enum EnumVulkanErrors {
  NotSupported,
  EntryError,
  LayerError,
  ExtensionError,
  InstanceError,
  DebugError,
  PhysicalDeviceError,
  LogicalDeviceError,
  SurfaceError,
}

#[cfg(feature = "Vulkan")]
struct VkQueueFamilyIndices {
  m_graphics_family_index: Option<u32>,
  // Add more family indices for desired queue pipeline features.
}

#[cfg(feature = "Vulkan")]
impl VkQueueFamilyIndices {
  pub fn default() -> Self {
    return VkQueueFamilyIndices {
      m_graphics_family_index: None,
    };
  }
}

#[cfg(feature = "Vulkan")]
pub struct VkContext {
  m_type: EnumApi,
  m_entry: ash::Entry,
  m_instance: ash::Instance,
  m_physical_device: vk::PhysicalDevice,
  m_logical_device: ash::Device,
  m_surface: khr::Surface,
  m_surface_khr: vk::SurfaceKHR,
  m_debug_report_callback: Option<(ext::DebugUtils, vk::DebugUtilsMessengerEXT)>,
}

#[cfg(feature = "Vulkan")]
impl VkContext {
  pub fn create_instance(window_context: &glfw::Glfw, additional_extensions: Option<Vec<&str>>,
                         additional_layers: Option<Vec<&str>>) -> Result<(ash::Entry, ash::Instance), EnumErrors> {
    let entry = ash::Entry::linked();
    
    let app_name = std::ffi::CString::new("Wave Engine Rust").unwrap();
    let engine_name = std::ffi::CString::new("Wave Engine").unwrap();
    let mut app_info = vk::ApplicationInfo::default();
    app_info.p_application_name = app_name.as_ptr();
    app_info.p_engine_name = engine_name.as_ptr();
    app_info.engine_version = vk::make_api_version(0, 0, 1, 0);
    app_info.api_version = vk::API_VERSION_1_3;
    
    // Add debug callback for create_instance() and destroy_instance().
    let mut debug_create_info = vk::DebugUtilsMessengerCreateInfoEXT::default();
    
    // Validate API calls and log output.
    let layers = VkContext::load_layers(additional_layers)?;
    VkContext::check_layer_support(&entry, &layers)?;
    
    let c_layers_ptr: Vec<*const std::ffi::c_char> = layers
      .iter()
      .map(|c_layer| c_layer.as_ptr())
      .collect();
    
    let extensions = VkContext::load_extensions(window_context, additional_extensions)?;
    VkContext::check_extension_support(&entry, &extensions)?;
    
    let c_extensions_ptr: Vec<*const std::ffi::c_char> = extensions
      .iter()
      .map(|c_extension| c_extension.as_ptr())
      .collect();
    
    let mut instance_create_info = vk::InstanceCreateInfo::builder()
      .enabled_extension_names(c_extensions_ptr.as_slice())
      .application_info(&app_info);
    
    #[cfg(feature = "debug")]
    {
      debug_create_info.s_type = vk::DebugUtilsMessengerCreateInfoEXT::STRUCTURE_TYPE;
      debug_create_info.message_severity = vk::DebugUtilsMessageSeverityFlagsEXT::INFO
        .bitor(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING)
        .bitor(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR)
        .bitor(vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE);
      
      debug_create_info.message_type = vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
        .bitor(vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION)
        .bitor(vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE);
      
      debug_create_info.pfn_user_callback = Some(vulkan_debug_callback);
      
      instance_create_info = instance_create_info
        .enabled_layer_names(c_layers_ptr.as_slice())
        .push_next(&mut debug_create_info);
    }
    
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
      instance_create_info.flags |= vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;
    }
    
    unsafe {
      return match entry.create_instance(&instance_create_info.build(), None) {
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
  ///
  /// ### Arguments:
  ///   * `window_context`: A reference to an existing Glfw context.
  ///   * `additional_extensions`: Optional vector of strings specifying
  /// the additional instance extension names requested to load in when creating the Vulkan instance.
  ///
  /// ### Returns:
  ///   * `Result<Vec<std::ffi::CString>, EnumErrors>`:
  /// A list of nul-terminated strings of extension names to enable during instance creation if
  /// successful, otherwise an [EnumErrors] on any error encountered.
  ///
  pub fn load_extensions(window_context: &glfw::Glfw, additional_extensions: Option<Vec<&str>>) -> Result<Vec<std::ffi::CString>, EnumErrors> {
    
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
        c_extensions.push(std::ffi::CString::from(ext::DebugUtils::name()));
      }
      
      #[cfg(any(target_os = "macos", target_os = "ios"))]
      {
        c_extensions.push(std::ffi::CString::from(vk::KhrPortabilityEnumerationFn::name()));
        // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
        c_extensions.push(std::ffi::CString::from(vk::KhrGetPhysicalDeviceProperties2Fn::name()));
      }
      
      // Get additional extensions requested.
      if additional_extensions.is_none() {
        return Ok(c_extensions);
      }
      
      for extension_name in additional_extensions.unwrap() {
        c_extensions.push(std::ffi::CString::new(extension_name)
          .expect("Failed to create C string from extension in load_extensions()"));
      }
      
      return Ok(c_extensions);
    }
    return Err(EnumErrors::ContextError);
  }
  
  /// Load validation layers for the Vulkan instance.
  ///
  /// ### Arguments:
  ///   * `requested_additional_extensions`: Optional vector of strings specifying
  /// the additional instance extensions requested to load in when creating the Vulkan instance.
  ///
  /// ### Returns:
  ///   * `Result<Vec<std::ffi::CString>, EnumErrors>`:
  /// A list of nul-terminated strings of layer names to enable during instance creation if
  /// successful, otherwise an [EnumErrors] on any error encountered.
  ///
  pub fn load_layers(additional_layers: Option<Vec<&str>>) -> Result<Vec<std::ffi::CString>, EnumErrors> {
    // Get required layers.
    let mut c_layers = vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation")
      .expect("Failed to create C string in load_layers()")];
    
    // Get additional layers.
    if additional_layers.is_none() {
      return Ok(c_layers);
    }
    for layer in additional_layers.unwrap() {
      c_layers.push(std::ffi::CString::new(layer)
        .expect("Failed to create C string from extension in load_layers()"))
    }
    
    return Ok(c_layers);
  }
  
  pub fn load_logical_device(ash_instance: &ash::Instance, vk_physical_device: &vk::PhysicalDevice,
                             additional_extensions: Option<Vec<&str>>) -> Result<ash::Device, EnumErrors> {
    let physical_device_features = unsafe {
      ash_instance.get_physical_device_features(*vk_physical_device)
    };
    let queue_family_indices = VkContext::find_queue_families(ash_instance, vk_physical_device)?;
    
    // Get SwapChain extension.
    let mut required_device_extensions: Vec<std::ffi::CString> =
      vec![std::ffi::CString::new(khr::Swapchain::name().to_bytes())
        .expect("Failed to convert swap chain name to CString in VKRenderer::new()!")];
    
    // Get additional extensions.
    if additional_extensions.is_some() {
      for extension in additional_extensions.unwrap() {
        required_device_extensions.push(std::ffi::CString::new(extension.as_bytes())
          .expect("Failed to convert device extension name into CString in load_logical_device()!"));
      }
    }
    
    VkContext::check_device_extension_support(&ash_instance, &vk_physical_device,
      &required_device_extensions)?;
    
    let required_device_extensions_ptr: Vec<*const std::ffi::c_char> = required_device_extensions
      .iter()
      .map(|extension_name| {
        return extension_name.as_ptr();
      })
      .collect();
    
    let device_queue_create_info = vk::DeviceQueueCreateInfo::builder()
      .flags(vk::DeviceQueueCreateFlags::default())
      .queue_family_index(queue_family_indices.m_graphics_family_index.unwrap())
      .queue_priorities(&[1.0])
      .build();
    
    let device_create_info = vk::DeviceCreateInfo::builder()
      .queue_create_infos(&[device_queue_create_info])
      .enabled_extension_names(&required_device_extensions_ptr)
      .enabled_features(&physical_device_features)
      .build();
    
    let vk_device = unsafe {
      ash_instance.create_device(*vk_physical_device, &device_create_info, None)
    };
    if vk_device.is_err() {
      return Err(EnumErrors::VulkanError(EnumVulkanErrors::LogicalDeviceError));
    }
    
    return Ok(vk_device.unwrap());
  }
  
  /// Check if the requested Vulkan instance extensions are supported.
  ///
  /// ### Arguments:
  ///   * `entry`: A reference to an existing Vulkan entry.
  ///   * `extension_names`: Vector of C strings specifying the Vulkan instance extension names requested to
  /// load in when creating the Vulkan instance.
  ///
  /// ### Returns:
  ///   * `Result<(), EnumErrors>`: Nothing if successful,
  /// otherwise an [EnumErrors::VulkanError(EnumVulkanErrors::NotSupported)] if any of the supplied extension names is not supported.
  ///
  pub fn check_extension_support(entry: &ash::Entry, extension_names: &Vec<std::ffi::CString>) -> Result<(), EnumErrors> {
    if entry.enumerate_instance_extension_properties(None).is_err() {
      return Err(EnumErrors::VulkanError(EnumVulkanErrors::ExtensionError));
    }
    
    let all_extensions = entry.enumerate_instance_extension_properties(None).unwrap();
    let mut all_extensions_iter = all_extensions.iter();
    
    // Verify extension support.
    for extension_name in extension_names {
      if !all_extensions_iter.any(|extension| {
        unsafe { return *extension.extension_name.as_ptr() == *extension_name.as_ptr(); };
      }) {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Vulkan extension {:#?} not supported!",
          extension_name.to_str().expect("Failed to convert C String to &str in check_extension_support()"));
        return Err(EnumErrors::VulkanError(EnumVulkanErrors::ExtensionError));
      }
    }
    return Ok(());
  }
  
  /// Check if the required Vulkan layers are supported by the Vulkan instance.
  ///
  /// ### Arguments:
  ///   * `entry`: A reference to an existing Vulkan entry.
  ///   * `layer_names`: Vector of strings specifying the Vulkan instance layer names requested to
  /// load in when creating the Vulkan instance.
  ///
  /// ### Returns:
  ///   * `Result<(), EnumErrors>`: Nothing if successful,
  /// otherwise an [EnumErrors::VulkanError(EnumVulkanErrors::NotSupported)] if any of the supplied layer names is not supported.
  ///
  pub fn check_layer_support(entry: &ash::Entry, layer_names: &Vec<std::ffi::CString>) -> Result<(), EnumErrors> {
    if entry.enumerate_instance_extension_properties(None).is_err() {
      return Err(EnumErrors::VulkanError(EnumVulkanErrors::ExtensionError));
    }
    
    let all_layers = entry.enumerate_instance_layer_properties().unwrap();
    let mut all_layers_iter = all_layers.iter();
    
    // Verify extension support.
    for layer_name in layer_names {
      if !all_layers_iter.any(|layer| {
        unsafe { return *layer.layer_name.as_ptr() == *layer_name.as_ptr(); };
      }) {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Vulkan layer {:#?} not supported!",
          layer_name.to_str().expect("Failed to convert C String to &str in check_layer_support()"));
        return Err(EnumErrors::VulkanError(EnumVulkanErrors::ExtensionError));
      }
    }
    return Ok(());
  }
  
  pub fn check_device_extension_support(ash_instance: &ash::Instance,
                                        vk_physical_device: &vk::PhysicalDevice,
                                        extension_names: &Vec<std::ffi::CString>) -> Result<(), EnumErrors> {
    let extension_properties = unsafe {
      ash_instance.enumerate_device_extension_properties(*vk_physical_device)
    };
    if extension_properties.is_err() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot retrieve device extensions: None available!");
      return Err(EnumErrors::VulkanError(EnumVulkanErrors::ExtensionError));
    }
    
    let available_device_extensions = extension_properties.unwrap();
    let mut device_extension_properties_iter = available_device_extensions.iter();
    
    // Verify extension support.
    for extension_name in extension_names {
      if !device_extension_properties_iter.any(|device_extension| {
        unsafe { return *device_extension.extension_name.as_ptr() == *extension_name.as_ptr(); };
      }) {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Vulkan device extension {:#?} not supported!",
          extension_name.to_str().expect("Failed to convert C String to &str in check_device_extension_support()"));
        return Err(EnumErrors::VulkanError(EnumVulkanErrors::ExtensionError));
      }
    }
    
    return Ok(());
  }
  
  /// Setup debug logging for API calls that redirect to custom debug callback.
  ///
  /// ### Arguments:
  ///   * `entry`: A reference to an existing Vulkan entry.
  ///   * `instance`: A reference to an existing Vulkan instance that is **yet to be created**.
  ///
  /// ### Returns:
  ///   * `Result<(ext::DebugUtils, vk::DebugUtilsMessengerEXT), EnumErrors>`:
  /// A tuple containing the created debug messenger and debug extension if
  /// successful, otherwise an [EnumErrors] on any error encountered.
  ///
  fn set_debug(entry: &ash::Entry, instance: &ash::Instance) -> Result<(ext::DebugUtils,
                                                                        vk::DebugUtilsMessengerEXT), EnumErrors> {
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
      debug_create_info.p_user_data = std::ptr::null_mut(); // Optional
      
      let debug_utils = ext::DebugUtils::new(entry, instance);
      return match unsafe { debug_utils.create_debug_utils_messenger(&debug_create_info, None) } {
        Ok(messenger) => {
          Ok((debug_utils, messenger))
        }
        Err(err) => {
          log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Vulkan error : {:#?}", err);
          Err(EnumErrors::VulkanError(EnumVulkanErrors::DebugError))
        }
      };
    }
    #[cfg(not(feature = "debug"))]
    {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] --> Cannot setup debug callback: Debug feature not enabled!");
      return Err(EnumErrors::VulkanError(EnumVulkanErrors::DebugError));
    }
  }
  
  /// Pick the first suitable Vulkan physical device.
  ///
  /// ### Arguments:
  ///   * `instance`: A reference to an existing Vulkan instance that is **yet to be created**.
  ///   * `window`: A reference to a [Glfw] window context.
  ///
  /// ### Returns:
  ///   * `Result<vk::PhysicalDevice, EnumErrors>`: A suitable Vulkan physical device if successful
  /// , otherwise an [EnumErrors].
  ///
  fn pick_physical_device(ash_instance: &ash::Instance, window: &glfw::Glfw) -> Result<vk::PhysicalDevice, EnumErrors> {
    let devices = unsafe { ash_instance.enumerate_physical_devices() };
    if devices.is_err() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Error getting physical devices! \
        Error => {:#?}", devices.unwrap());
      return Err(EnumErrors::VulkanError(EnumVulkanErrors::PhysicalDeviceError));
    }
    
    // Check if we have found at least one.
    if devices.as_ref().unwrap().len() == 0 {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Not Vulkan physical devices found!");
      return Err(EnumErrors::VulkanError(EnumVulkanErrors::PhysicalDeviceError));
    }
    
    // Check if device is suitable.
    let device = devices.unwrap()
      .into_iter()
      .find(|device| window.get_physical_device_presentation_support_raw(ash_instance.handle(),
        *device, 0) && VkContext::is_device_suitable(ash_instance, device));
    
    
    if device.is_none() {
      log!(EnumLogColor::Red, "ERROR",
        "[Renderer] -->\t Vulkan Physical device does not support graphics for queue family 0!");
      return Err(EnumErrors::VulkanError(EnumVulkanErrors::PhysicalDeviceError));
    }
    
    return Ok(device.unwrap());
  }
  
  fn find_queue_families(ash_instance: &ash::Instance, vk_physical_device: &vk::PhysicalDevice) -> Result<VkQueueFamilyIndices, EnumErrors> {
    let queue_families = unsafe {
      ash_instance.get_physical_device_queue_family_properties(*vk_physical_device)
    };
    
    if queue_families.is_empty() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot retrieve queue families for physical \
      device {0:#?}!", vk_physical_device);
      return Err(EnumErrors::VulkanError(EnumVulkanErrors::PhysicalDeviceError));
    }
    
    let mut queue_family_indices: VkQueueFamilyIndices = VkQueueFamilyIndices::default();
    
    // Find graphics queue family.
    let mut graphic_family_index: u32 = 0;
    
    for queue_family in queue_families {
      if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
        queue_family_indices.m_graphics_family_index = Some(graphic_family_index)
      }
      graphic_family_index += 1;
    }
    return Ok(queue_family_indices);
  }
  
  fn is_device_suitable(ash_instance: &ash::Instance, vk_physical_device: &vk::PhysicalDevice) -> bool {
    // Check if graphics family queue exists.
    let queue_families = VkContext::find_queue_families(ash_instance, vk_physical_device);
    
    if queue_families.is_err() {
      return false;
    }
    
    if queue_families.unwrap().m_graphics_family_index.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] --.\t Physical device {0:#?} \
      does not have a queue dedicated for graphics exchange!", vk_physical_device);
      return false;
    }
    return true;
  }
}

#[cfg(feature = "Vulkan")]
impl Drop for VkContext {
  fn drop(&mut self) {
    unsafe {
      self.m_logical_device.destroy_device(None);
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

#[cfg(feature = "Vulkan")]
impl TraitContext for VkContext {
  fn new(window: &mut GlfwWindow) -> Result<Self, EnumErrors> {
    return match VkContext::create_instance(window.get_api_ptr(), None, None) {
      Ok((ash_entry, ash_instance)) => {
        #[allow(unused)]
          let debug_callback: Option<(ext::DebugUtils, vk::DebugUtilsMessengerEXT)> = None;
        
        // Create surface (graphic context).
        let vk_surface = khr::Surface::new(&ash_entry, &ash_instance);
        let mut khr_surface = vk::SurfaceKHR::default();
        
        let vk_physical_device = VkContext::pick_physical_device(&ash_instance, window.get_api_ptr())?;
        let ash_logical_device = VkContext::load_logical_device(&ash_instance, &vk_physical_device, None)?;
        
        window.init_vulkan_surface(&ash_instance, &mut khr_surface);
        
        Ok(VkContext {
          m_type: EnumApi::Vulkan,
          m_entry: ash_entry,
          m_instance: ash_instance,
          m_physical_device: vk_physical_device,
          m_logical_device: ash_logical_device,
          m_surface: vk_surface,
          m_surface_khr: khr_surface,
          m_debug_report_callback: debug_callback,
        })
      }
      Err(err) => {
        Err(err)
      }
    };
  }
  
  fn get_type(&self) -> EnumApi {
    return self.m_type;
  }
  
  fn to_string(&self) -> String {
    let device_properties = unsafe {
      self.m_instance.get_physical_device_properties(self.m_physical_device)
    };
    let device_name_str = unsafe { std::ffi::CStr::from_ptr(device_properties.device_name.as_ptr()) }
      .to_str()
      .unwrap_or("[Renderer] -->\t Could not retrieve device name in get_api_info()!");
    
    let mut device_type_str: String = String::new();
    match device_properties.device_type {
      PhysicalDeviceType::DISCRETE_GPU => { device_type_str += "Discrete GPU" },
      PhysicalDeviceType::INTEGRATED_GPU => { device_type_str += "Integrated GPU" },
      PhysicalDeviceType::CPU => { device_type_str += "CPU" },
      _ => { device_type_str += "Other" }
    }
    
    let str: String = format!("Renderer Hardware => {0},\n{1:<113}Driver version => {2}\n{3:<113}\
    Device type => {4}\n{5:<113}Api version => {6}.{7}.{8}", device_name_str, "",
      device_properties.driver_version, "", device_type_str, "",
      vk::api_version_major(device_properties.api_version),
      vk::api_version_minor(device_properties.api_version),
      vk::api_version_patch(device_properties.api_version));
    return str;
  }
  
  fn toggle_feature(&mut self, feature: EnumFeature) -> Result<(), EnumErrors> {
    match feature {
      EnumFeature::Debug(enabled) => {
        if enabled {
          // Toggle on debugging.
          let debug_callback = Some(VkContext::set_debug(&self.m_entry, &self.m_instance)?);
          self.m_debug_report_callback = debug_callback;
        } else  {
          // Toggle off debugging.
          unsafe {
            if let Some((debug_utils, messenger)) = self.m_debug_report_callback.take() {
              debug_utils.destroy_debug_utils_messenger(messenger, None);
            }
          }
          self.m_debug_report_callback = None;
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Debug mode {0}",
          enabled.then(|| return "enabled").unwrap_or("disabled"));
      }
      EnumFeature::DepthTest(_) => {}
      EnumFeature::CullFacing(_, _) => {}
      EnumFeature::Wireframe(_) => {}
      EnumFeature::MSAA(_) => {}
      EnumFeature::SRGB(_) => {}
      EnumFeature::Blending(_, _, _) => {}
    }
    return Ok(());
  }
  
  fn begin(&mut self) {
    todo!()
  }
  
  fn end(&mut self) {
    todo!()
  }
  
  fn batch(&mut self) {
    todo!()
  }
  
  fn flush(&mut self) {
    todo!()
  }
  
  fn send(&mut self, _sendable_entity: &REntity, _shader_associated: &mut dyn TraitShader) -> Result<(), EnumErrors> {
    return Ok(());
  }
  
  fn draw(&mut self) -> Result<(), EnumErrors> {
    return Ok(());
  }
  
  fn free(&mut self, _id: &u64) -> Result<(), EnumErrors> {
    todo!()
  }
  
  fn shutdown(&mut self) -> Result<(), EnumErrors> {
    return Ok(());
  }
}

#[cfg(all(feature = "Vulkan", feature = "debug"))]
unsafe extern "system" fn vulkan_debug_callback(flag: vk::DebugUtilsMessageSeverityFlagsEXT,
                                                type_: vk::DebugUtilsMessageTypeFlagsEXT,
                                                p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
                                                _: *mut std::ffi::c_void) -> vk::Bool32 {
  use vk::DebugUtilsMessageSeverityFlagsEXT as Flag;
  
  let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
  match flag {
    Flag::VERBOSE => {}
    Flag::INFO => {},
    // Flag::INFO => log!("INFO", "{:?} -->\t {:#?}", type_, message),
    Flag::WARNING => log!(EnumLogColor::Yellow, "WARN", "{:?} -->\t {:#?}", type_, message),
    _ => log!(EnumLogColor::Red, "ERROR", "{:?} -->\t {:#?}", type_, message),
  }
  
  return vk::FALSE;
}


/*
///////////////////////////////////   OpenGL    ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
 */

use gl::types::GLenum;

#[cfg(feature = "OpenGL")]
use super::buffer::*;

#[cfg(feature = "OpenGL")]
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

#[cfg(feature = "OpenGL")]
struct BatchPrimitives {
  m_shaders: Vec<u32>,
  m_vao_buffers: Vec<GlVao>,
  m_vbo_buffers: Vec<GlVbo>,
}

#[cfg(feature = "OpenGL")]
impl BatchPrimitives {
  pub fn new() -> Self {
    return BatchPrimitives {
      m_shaders: Vec::new(),
      m_vao_buffers: Vec::new(),
      m_vbo_buffers: Vec::new(),
    };
  }
}


#[cfg(feature = "OpenGL")]
pub struct GlContext {
  m_type: EnumApi,
  m_batch: BatchPrimitives,
  m_debug_callback: gl::types::GLDEBUGPROC,
}

#[cfg(feature = "OpenGL")]
impl TraitContext for GlContext {
  fn new(window: &mut GlfwWindow) -> Result<Self, EnumErrors> {
    // Init context.
    window.init_opengl_surface();
    
    gl::load_with(|f_name| window.get_api_ptr().get_proc_address_raw(f_name));
    
    check_gl_call!("Renderer", gl::Viewport(0, 0, window.get_size().x, window.get_size().y));
    check_gl_call!("Renderer", gl::ClearColor(0.15, 0.15, 0.15, 1.0));
    
    check_gl_call!("Renderer", gl::FrontFace(gl::CW));
    
    return Ok(GlContext {
      m_type: EnumApi::OpenGL,
      m_batch: BatchPrimitives::new(),
      m_debug_callback: Some(gl_error_callback),
    });
  }
  
  fn get_type(&self) -> EnumApi {
    return self.m_type;
  }
  
  fn to_string(&self) -> String {
    unsafe {
      let api_vendor = std::ffi::CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8)
        .to_str().unwrap_or("Cannot retrieve api vendor information!");
      let api_version = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
        .to_str().unwrap_or("Cannot retrieve api version information!");
      let renderer_info = std::ffi::CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8)
        .to_str().unwrap_or("Cannot retrieve renderer information!");
      let shading_info = std::ffi::CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8)
        .to_str().unwrap_or("Cannot retrieve shading language information!");
      
      let str: String = format!("Renderer Hardware => {0},\n{1:<113}Vendor => {2:<15},\n{3:<113}\
      Version => {4:<15},\n{5:<113}Shading Language => {6:<15}",
        renderer_info, "", api_vendor, "", api_version, "", shading_info);
      return str;
    }
  }
  
  fn toggle_feature(&mut self, feature: EnumFeature) -> Result<(), EnumErrors> {
    match feature {
      EnumFeature::Debug(flag) => {
        if flag {
          check_gl_call!("Renderer", gl::Enable(gl::DEBUG_OUTPUT));
          check_gl_call!("Renderer", gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS));
          check_gl_call!("Renderer", gl::DebugMessageCallback(self.m_debug_callback, std::ptr::null()));
          check_gl_call!("Renderer", gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE,
            gl::DONT_CARE, 0, std::ptr::null(), gl::TRUE));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::DEBUG_OUTPUT));
          check_gl_call!("Renderer", gl::Disable(gl::DEBUG_OUTPUT_SYNCHRONOUS));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Debug {0}",
          flag.then(|| return "enabled").unwrap_or("disabled"));
      }
      EnumFeature::DepthTest(flag) => {
        if flag {
          check_gl_call!("Renderer", gl::Enable(gl::DEPTH_TEST));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::DEPTH_TEST));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Depth test {0}",
          flag.then(|| return "enabled").unwrap_or("disabled"));
      }
      EnumFeature::MSAA(flag) => {
        if flag {
          check_gl_call!("Renderer", gl::Enable(gl::MULTISAMPLE));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::MULTISAMPLE));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t MSAA {0}",
          flag.then(|| return "enabled").unwrap_or("disabled"));
      }
      EnumFeature::Blending(flag, s_factor, d_factor) => {
        if flag {
          check_gl_call!("Renderer", gl::Enable(gl::BLEND));
          check_gl_call!("Renderer", gl::BlendFunc(s_factor, d_factor));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::BLEND));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Blending {0}",
          flag.then(|| return "enabled").unwrap_or("disabled"));
      }
      EnumFeature::Wireframe(flag) => {
        if flag {
          check_gl_call!("Renderer", gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE));
        } else {
          check_gl_call!("Renderer", gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Wireframe mode {0}",
          flag.then(|| return "enabled").unwrap_or("disabled"));
      }
      EnumFeature::SRGB(flag) => {
        if flag {
          check_gl_call!("Renderer", gl::Enable(gl::FRAMEBUFFER_SRGB));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::FRAMEBUFFER_SRGB));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t SRGB framebuffer {0}",
          flag.then(|| return "enabled").unwrap_or("disabled"));
      }
      EnumFeature::CullFacing(flag, face) => {
        if flag {
          check_gl_call!("Renderer", gl::Enable(gl::CULL_FACE));
          check_gl_call!("Renderer", gl::CullFace(face));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::CULL_FACE));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Cull facing {0}",
          flag.then(|| return "enabled").unwrap_or("disabled"));
      }
    }
    return Ok(());
  }
  
  fn begin(&mut self) {
    todo!()
  }
  
  fn end(&mut self) {
    todo!()
  }
  
  fn batch(&mut self) {
    todo!()
  }
  
  fn flush(&mut self) {
    todo!()
  }
  
  fn send(&mut self, sendable_entity: &REntity, shader_associated: &mut dyn TraitShader) -> Result<(), EnumErrors> {
    if sendable_entity.is_empty() {
      log!(EnumLogColor::Yellow, "WARN", "[Renderer] --> Entity {0} sent has no \
      vertices! Not sending it...", sendable_entity)
    }
    
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
    
    self.m_batch.m_shaders.push(shader_associated.get_id());
    self.m_batch.m_vao_buffers.push(vao);
    self.m_batch.m_vbo_buffers.push(vbo);
    
    return Ok(());
  }
  
  fn draw(&mut self) -> Result<(), EnumErrors> {
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }
    for index in 0usize..self.m_batch.m_shaders.len() {
      check_gl_call!("Renderer", gl::UseProgram(self.m_batch.m_shaders[index]));
      self.m_batch.m_vao_buffers[index].bind()?;
      check_gl_call!("Renderer", gl::DrawArrays(gl::TRIANGLES, 0, self.m_batch.m_vbo_buffers[index].m_count as GLsizei));
    }
    return Ok(());
  }
  
  fn free(&mut self, _id: &u64) -> Result<(), EnumErrors> {
    return Ok(());
  }
  
  fn shutdown(&mut self) -> Result<(), EnumErrors> {
    return Ok(());
  }
}

#[cfg(feature = "OpenGL")]
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
      gl::DEBUG_SEVERITY_HIGH => { final_error_msg += "Severity => Fatal (High);\n" }
      gl::DEBUG_SEVERITY_MEDIUM => { final_error_msg += "Severity => Fatal (Medium);\n" }
      gl::DEBUG_SEVERITY_LOW => { final_error_msg += "Severity => Warn (Low);\n" }
      gl::DEBUG_SEVERITY_NOTIFICATION => { final_error_msg += "Severity => Warn (Info);\n" }
      _ => { final_error_msg += "Severity => Fatal (Unknown);\n" }
    }
    
    let test = unsafe { std::ffi::CStr::from_ptr(error_message.cast_mut()) };
    let str = test.to_str()
      .expect("[Renderer] -->\t Failed to convert C string to Rust String in gl_error_callback()");
    
    final_error_msg += str;
    
    match severity {
      gl::DEBUG_SEVERITY_HIGH => { log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t {0}", final_error_msg); }
      gl::DEBUG_SEVERITY_MEDIUM => { log!(EnumLogColor::Yellow, "WARN", "[Renderer] -->\t {0}", final_error_msg); }
      gl::DEBUG_SEVERITY_LOW => { log!(EnumLogColor::Yellow, "WARN", "[Renderer] -->\t {0}", final_error_msg); }
      gl::DEBUG_SEVERITY_NOTIFICATION => { log!("INFO", "[Renderer] -->\t {0}", final_error_msg); }
      _ => {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t {0}", final_error_msg);
      }
    }
  }
}

