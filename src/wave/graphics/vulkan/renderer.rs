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

pub extern crate ash;

use std::ops::{BitAnd, BitOr};
use ash::extensions::{ext, khr};
use ash::vk::{self, PhysicalDeviceType, TaggedStructure};

use std::fmt::{Display, Formatter};
use std::collections::HashSet;

use crate::log;
use crate::wave::assets::renderable_assets::REntity;
use crate::wave::camera::PerspectiveCamera;
use crate::wave::graphics::renderer::{EnumFeature, EnumError, TraitContext};
use crate::wave::graphics::shader::Shader;
use crate::wave::window::Window;


/*
///////////////////////////////////   Vulkan renderer    ///////////////////////////////////
///////////////////////////////////                      ///////////////////////////////////
///////////////////////////////////                      ///////////////////////////////////
 */

#[derive(Debug, Copy, Clone, PartialEq)]
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
  SwapError,
  SwapImagesError,
}

#[derive(Debug, PartialEq)]
pub struct VkQueueFamilyIndices {
  m_graphics_family_index: Option<u32>,
  m_present_family_index: Option<u32>,
  // Add more family indices for desired queue pipeline features.
}

impl VkQueueFamilyIndices {
  pub fn default() -> Self {
    return VkQueueFamilyIndices {
      m_graphics_family_index: None,
      m_present_family_index: None,
    };
  }
}

#[derive(Debug)]
pub struct VkSwapChainProperties {
  m_capabilities: vk::SurfaceCapabilitiesKHR,
  m_formats: Vec<vk::SurfaceFormatKHR>,
  m_present_modes: Vec<vk::PresentModeKHR>,
  m_ideal_format: vk::Format,
  m_ideal_present_mode: vk::PresentModeKHR,
  m_swap_extent: vk::Extent2D,
}

impl VkSwapChainProperties {
  pub fn default() -> Self {
    return Self {
      m_capabilities: Default::default(),
      m_formats: vec![],
      m_present_modes: vec![],
      m_ideal_format: Default::default(),
      m_ideal_present_mode: Default::default(),
      m_swap_extent: Default::default(),
    };
  }
  
  pub fn new(capabilities: vk::SurfaceCapabilitiesKHR, formats: Vec<vk::SurfaceFormatKHR>, present_modes: Vec<vk::PresentModeKHR>) -> Self {
    return Self {
      m_capabilities: capabilities,
      m_formats: formats,
      m_present_modes: present_modes,
      m_ideal_format: vk::Format::default(),
      m_ideal_present_mode: vk::PresentModeKHR::default(),
      m_swap_extent: vk::Extent2D::default(),
    };
  }
}

impl Display for VkSwapChainProperties {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut present_mode_str: String = String::from("Details : ");
    match self.m_ideal_present_mode {
      vk::PresentModeKHR::IMMEDIATE => {
        present_mode_str += "Submit as fast as possible";
      }
      vk::PresentModeKHR::FIFO => {
        present_mode_str += "Images sync with display (vsync)";
      }
      vk::PresentModeKHR::FIFO_RELAXED => {
        present_mode_str += "Images sync with display unless late (vsync V2)";
      }
      vk::PresentModeKHR::SHARED_CONTINUOUS_REFRESH => {
        present_mode_str += "Images sync with display unless early (vsync V3)";
      }
      _ => {
        present_mode_str += "Unrecognized presentation mode";
      }
    }
    
    write!(f, "Swap chain info :\n{0:115}\
    Supported usage => {1:?}\n{0:115}\
    Min image count => {2:?}\n{0:115}\
    Max image count => {3:?}\n{0:115}\
    Surface format picked (color depth) => {4:?}\n{0:115}\
    Presentation mode picked (buffer swapping) => {5:?}\n{0:117}{6}\n{0:115}\
    Extent (resolution) => Width: {7}, Height: {8}", "",
      self.m_capabilities.supported_usage_flags,
      self.m_capabilities.min_image_count,
      self.m_capabilities.max_image_count,
      self.m_ideal_format,
      self.m_ideal_present_mode,
      present_mode_str,
      self.m_swap_extent.width,
      self.m_swap_extent.height)
  }
}

pub struct VkContext {
  m_entry: ash::Entry,
  m_instance: ash::Instance,
  m_physical_device: vk::PhysicalDevice,
  m_queue_family_indices: VkQueueFamilyIndices,
  m_logical_device: ash::Device,
  m_surface: khr::Surface,
  m_surface_khr: vk::SurfaceKHR,
  m_swap_chain_properties: VkSwapChainProperties,
  m_swap_chain: Option<khr::Swapchain>,
  m_swap_chain_khr: vk::SwapchainKHR,
  m_swap_chain_images: Vec<vk::Image>,
  m_swap_chain_image_views: Vec<vk::ImageView>,
  m_debug_report_callback: Option<(ext::DebugUtils, vk::DebugUtilsMessengerEXT)>,
}

impl VkContext {
  pub fn create_swap_chain(&mut self, vsync_preferred: bool) -> Result<(), EnumError> {
    // Setup swap chain.
    let mut swap_chain_properties = VkContext::query_swap_properties(&self.m_surface, self.m_physical_device, self.m_surface_khr)?;
    let extent = VkContext::pick_swap_extent(&swap_chain_properties.m_capabilities, None)?;
    let format = VkContext::pick_swap_format(&swap_chain_properties.m_formats)?;
    
    let present_mode: vk::PresentModeKHR = VkContext::pick_swap_presentation_mode(vsync_preferred,
      &swap_chain_properties.m_present_modes);
    
    swap_chain_properties.m_ideal_format = format.format;
    swap_chain_properties.m_swap_extent = extent;
    swap_chain_properties.m_ideal_present_mode = present_mode;
    
    // How many images we would like to have in the swap chain.
    let mut image_count: u32 = swap_chain_properties.m_capabilities.min_image_count + 1;
    
    // Make sure we don't go over the max image count supported. 0 here means there is no MAX.
    if swap_chain_properties.m_capabilities.max_image_count > 0 && image_count >
      swap_chain_properties.m_capabilities.max_image_count {
      image_count = swap_chain_properties.m_capabilities.max_image_count;
    }
    
    let mut swap_chain_create_info = vk::SwapchainCreateInfoKHR::default();
    swap_chain_create_info.min_image_count = image_count;
    swap_chain_create_info.surface = self.m_surface_khr;
    swap_chain_create_info.image_format = format.format;
    swap_chain_create_info.image_color_space = format.color_space;
    swap_chain_create_info.image_extent = extent;
    swap_chain_create_info.present_mode = present_mode;
    swap_chain_create_info.image_array_layers = 1;
    swap_chain_create_info.image_usage = vk::ImageUsageFlags::COLOR_ATTACHMENT;
    
    // Specify how to handle swap chain images that will be used across multiple queue families.
    if self.m_queue_family_indices.m_graphics_family_index != self.m_queue_family_indices.m_present_family_index {
      swap_chain_create_info.image_sharing_mode = vk::SharingMode::CONCURRENT;
      swap_chain_create_info.queue_family_index_count = 2;
      swap_chain_create_info.p_queue_family_indices =
        [self.m_queue_family_indices.m_graphics_family_index.unwrap(),
          self.m_queue_family_indices.m_present_family_index.unwrap()].as_ptr();
    } else {
      swap_chain_create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
    }
    
    // Specify that a certain transform should be applied to images in the swap chain if it is supported.
    swap_chain_create_info.pre_transform = swap_chain_properties.m_capabilities.current_transform;
    // Specify if the alpha channel should be used for blending with other windows in the window system.
    swap_chain_create_info.composite_alpha = vk::CompositeAlphaFlagsKHR::OPAQUE;
    
    /*
    If the clipped member is set to VK_TRUE then that means that we don't care about the color of
    pixels that are obscured, for example because another window is in front of them. Unless you
    really need to be able to read these pixels back and get predictable results, you'll get the
    best performance by enabling clipping.
    */
    swap_chain_create_info.clipped = vk::TRUE;
    
    // Create swap chain.
    let swap_chain = khr::Swapchain::new(&self.m_instance, &self.m_logical_device);
    let swap_chain_khr = unsafe {
      swap_chain.create_swapchain(&swap_chain_create_info, None)
    };
    
    match swap_chain_khr {
      Ok(_) => {}
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Could not create swap chain, Vulkan \
      returned with : {:?}", err);
        return Err(EnumError::VulkanError(EnumVulkanErrors::SwapError));
      }
    }
    
    self.m_swap_chain_properties = swap_chain_properties;
    self.m_swap_chain = Some(swap_chain);
    self.m_swap_chain_khr = swap_chain_khr.unwrap();
    
    return Ok(());
  }
  
  pub fn create_swap_image_views(&mut self) -> Result<(), EnumError> {
    self.m_swap_chain_image_views.reserve_exact(self.m_swap_chain_images.len());
    
    for &swap_image in self.m_swap_chain_images.iter() {
      let mut image_view_create_info: vk::ImageViewCreateInfo = vk::ImageViewCreateInfo::default();
      image_view_create_info.s_type = vk::ImageViewCreateInfo::STRUCTURE_TYPE;
      image_view_create_info.view_type = vk::ImageViewType::TYPE_2D;
      image_view_create_info.image = swap_image;
      image_view_create_info.format = self.m_swap_chain_properties.m_ideal_format;
      image_view_create_info.components.r = vk::ComponentSwizzle::R;
      image_view_create_info.components.g = vk::ComponentSwizzle::G;
      image_view_create_info.components.b = vk::ComponentSwizzle::B;
      image_view_create_info.components.a = vk::ComponentSwizzle::A;
      image_view_create_info.subresource_range.aspect_mask = vk::ImageAspectFlags::COLOR;
      image_view_create_info.subresource_range.base_mip_level = 0;
      image_view_create_info.subresource_range.level_count = 1;
      image_view_create_info.subresource_range.base_array_layer = 0;
      image_view_create_info.subresource_range.layer_count = 1;
      
      match unsafe { self.m_logical_device.create_image_view(&image_view_create_info, None) } {
        Ok(image_view) => {
          self.m_swap_chain_image_views.push(image_view);
        }
        Err(err) => {
          log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot create image view : \
          Vulkan returned with Err => {:?}", err);
          return Err(EnumError::VulkanError(EnumVulkanErrors::SwapImagesError));
        }
      }
    }
    
    return Ok(());
  }
  
  pub fn create_pipeline(&mut self) -> Result<(), EnumError> {
    
    return Ok(());
  }
  
  /// Load window extensions for the Vulkan surface.
  ///
  /// ### Arguments:
  ///   * `window_context`: A reference to an existing Glfw context.
  ///   * `additional_extensions`: Optional vector of strings specifying
  /// the additional instance extension names requested to load in when creating the Vulkan instance.
  ///
  /// ### Returns:
  ///   * `Result<Vec<std::ffi::CString>, EnumError>`:
  /// A list of nul-terminated strings of extension names to enable during instance creation if
  /// successful, otherwise an [EnumError] on any error encountered.
  ///
  pub fn load_extensions(window_context: &glfw::Glfw, additional_extensions: Option<Vec<&str>>) -> Result<Vec<std::ffi::CString>, EnumError> {
    
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
    return Err(EnumError::ContextError);
  }
  
  /// Load validation layers for the Vulkan instance.
  ///
  /// ### Arguments:
  ///   * `requested_additional_extensions`: Optional vector of strings specifying
  /// the additional instance extensions requested to load in when creating the Vulkan instance.
  ///
  /// ### Returns:
  ///   * `Result<Vec<std::ffi::CString>, EnumError>`:
  /// A list of nul-terminated strings of layer names to enable during instance creation if
  /// successful, otherwise an [EnumError] on any error encountered.
  ///
  pub fn load_layers(additional_layers: Option<Vec<&str>>) -> Result<Vec<std::ffi::CString>, EnumError> {
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
  
  /// Check if the requested Vulkan instance extensions are supported.
  ///
  /// ### Arguments:
  ///   * `entry`: A reference to an existing Vulkan entry.
  ///   * `extension_names`: Vector of C strings specifying the Vulkan instance extension names requested to
  /// load in when creating the Vulkan instance.
  ///
  /// ### Returns:
  ///   * `Result<(), EnumError>`: Nothing if successful,
  /// otherwise an [EnumError::VulkanError(EnumVulkanErrors::NotSupported)] if any of the supplied extension names is not supported.
  ///
  pub fn check_extension_support(entry: &ash::Entry, extension_names: &Vec<std::ffi::CString>) -> Result<(), EnumError> {
    if entry.enumerate_instance_extension_properties(None).is_err() {
      return Err(EnumError::VulkanError(EnumVulkanErrors::ExtensionError));
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
        return Err(EnumError::VulkanError(EnumVulkanErrors::ExtensionError));
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
  ///   * `Result<(), EnumError>`: Nothing if successful,
  /// otherwise an [EnumError::VulkanError(EnumVulkanErrors::NotSupported)] if any of the supplied layer names is not supported.
  ///
  pub fn check_layer_support(entry: &ash::Entry, layer_names: &Vec<std::ffi::CString>) -> Result<(), EnumError> {
    if entry.enumerate_instance_extension_properties(None).is_err() {
      return Err(EnumError::VulkanError(EnumVulkanErrors::ExtensionError));
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
        return Err(EnumError::VulkanError(EnumVulkanErrors::ExtensionError));
      }
    }
    return Ok(());
  }
  
  pub fn check_device_extension_support(ash_instance: &ash::Instance, vk_physical_device: &vk::PhysicalDevice, extension_names: &Vec<std::ffi::CString>) -> Result<(), EnumError> {
    let extension_properties = unsafe {
      ash_instance.enumerate_device_extension_properties(*vk_physical_device)
    };
    if extension_properties.is_err() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot retrieve device extensions: None available!");
      return Err(EnumError::VulkanError(EnumVulkanErrors::ExtensionError));
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
        return Err(EnumError::VulkanError(EnumVulkanErrors::ExtensionError));
      }
    }
    
    return Ok(());
  }
  
  fn get_driver_version(version_raw: u32, vendor_id: u32) -> String {
    return match vendor_id {
      0x10DE => {
        format!("{0}.{1}.{2}.{3}", (version_raw >> 22).bitand(0x3ff),
          (version_raw >> 14).bitand(0x0ff), (version_raw >> 6).bitand(0x0ff),
          version_raw.bitand(0x003f))
      },
      0x8086 => {
        #[cfg(target_os = "windows")]
        return format!("{0}.{1}", version_raw >> 14, version_raw.bitand(0x3fff));
        
        #[cfg(not(target_os = "windows"))]
        return format!("{0}.{1}.{2}", version_raw >> 22, (version_raw >> 12).bitand(0x3ff),
        version_raw.bitand(0xfff));
      },
      _ => format!("{0}.{1}.{2}", version_raw >> 22, (version_raw >> 12).bitand(0x3ff),
        version_raw.bitand(0xfff))
    }
  }
  
  fn create_instance(window: &mut Window, additional_extensions: Option<Vec<&str>>, additional_layers: Option<Vec<&str>>) -> Result<(ash::Entry, ash::Instance), EnumError> {
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
    
    let extensions = VkContext::load_extensions(window.get_api_ref(),
      additional_extensions)?;
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
          
          Err(EnumError::VulkanError(EnumVulkanErrors::InstanceError))
        }
      };
    }
  }
  
  fn create_logical_device(ash_instance: &ash::Instance, vk_physical_device: vk::PhysicalDevice, graphics_queue_family_index: u32, additional_extensions: Option<Vec<&str>>) -> Result<ash::Device, EnumError> {
    let physical_device_features = unsafe {
      ash_instance.get_physical_device_features(vk_physical_device)
    };
    
    // Get swap chain extension.
    let mut required_device_extensions: Vec<std::ffi::CString> =
      vec![std::ffi::CString::new(khr::Swapchain::name().to_bytes())
        .expect("Failed to convert swap chain name to CString in VKContext::new()!")];
    
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
      .queue_family_index(graphics_queue_family_index)
      .queue_priorities(&[1.0])
      .build();
    
    let device_create_info = vk::DeviceCreateInfo::builder()
      .queue_create_infos(&[device_queue_create_info])
      .enabled_extension_names(&required_device_extensions_ptr)
      .enabled_features(&physical_device_features)
      .build();
    
    let vk_device = unsafe {
      ash_instance.create_device(vk_physical_device, &device_create_info, None)
    };
    if vk_device.is_err() {
      return Err(EnumError::VulkanError(EnumVulkanErrors::LogicalDeviceError));
    }
    
    return Ok(vk_device.unwrap());
  }
  
  /// Setup debug logging for API calls that redirect to custom debug callback.
  ///
  /// ### Arguments:
  ///   * `entry`: A reference to an existing Vulkan entry.
  ///   * `instance`: A reference to an existing Vulkan instance that is **yet to be created**.
  ///
  /// ### Returns:
  ///   * `Result<(ext::DebugUtils, vk::DebugUtilsMessengerEXT), EnumError>`:
  /// A tuple containing the created debug messenger and debug extension if
  /// successful, otherwise an [EnumError] on any error encountered.
  ///
  fn set_debug(entry: &ash::Entry, instance: &ash::Instance) -> Result<(ext::DebugUtils, vk::DebugUtilsMessengerEXT), EnumError> {
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
          Err(EnumError::VulkanError(EnumVulkanErrors::DebugError))
        }
      };
    }
    #[cfg(not(feature = "debug"))]
    {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] --> Cannot setup debug callback: Debug feature not enabled!");
      return Err(EnumError::VulkanError(EnumVulkanErrors::DebugError));
    }
  }
  
  /// Pick the first suitable Vulkan physical device.
  ///
  /// ### Arguments:
  ///   * `instance`: A reference to an existing Vulkan instance that is **yet to be created**.
  ///   * `window`: A reference to a [Glfw] window context.
  ///
  /// ### Returns:
  ///   * `Result<vk::PhysicalDevice, EnumError>`: A suitable Vulkan physical device if successful
  /// , otherwise an [EnumError].
  ///
  fn pick_physical_device(ash_instance: &ash::Instance, surface: &khr::Surface, surface_khr: vk::SurfaceKHR) -> Result<vk::PhysicalDevice, EnumError> {
    let devices = unsafe { ash_instance.enumerate_physical_devices() };
    if devices.is_err() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Error getting physical devices! \
        Error => {:#?}", devices.unwrap());
      return Err(EnumError::VulkanError(EnumVulkanErrors::PhysicalDeviceError));
    }
    
    // Check if we have found at least one.
    if devices.as_ref().unwrap().len() == 0 {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Not Vulkan physical devices found!");
      return Err(EnumError::VulkanError(EnumVulkanErrors::PhysicalDeviceError));
    }
    
    // Check if device is suitable, aka if it has presentation support.
    let device = devices.unwrap()
      .into_iter()
      .find(|&device| VkContext::is_device_suitable(ash_instance, surface, surface_khr, device));
    
    
    if device.is_none() {
      log!(EnumLogColor::Red, "ERROR",
        "[Renderer] -->\t Vulkan Physical device does not support graphics for queue family 0!");
      return Err(EnumError::VulkanError(EnumVulkanErrors::PhysicalDeviceError));
    }
    
    return Ok(device.unwrap());
  }
  
  fn pick_swap_format(surface_formats: &Vec<vk::SurfaceFormatKHR>) -> Result<vk::SurfaceFormatKHR, EnumError> {
    let ideal_format = surface_formats.into_iter()
      .find(|&&khr_format| {
        return khr_format.format == vk::Format::B8G8R8A8_SRGB && khr_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR;
      });
    
    if ideal_format.is_some() {
      // Pick first one.
      return Ok(*ideal_format.unwrap());
    }
    
    log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot pick a suitable surface color format : Not supported!");
    return Err(EnumError::VulkanError(EnumVulkanErrors::SurfaceError));
  }
  
  fn pick_swap_presentation_mode(vsync_requested: bool, surface_present_modes: &Vec<vk::PresentModeKHR>) -> vk::PresentModeKHR {
    if vsync_requested && surface_present_modes.contains(&vk::PresentModeKHR::MAILBOX) {
      return vk::PresentModeKHR::MAILBOX;
    }
    if vsync_requested && surface_present_modes.contains(&vk::PresentModeKHR::FIFO) {
      log!(EnumLogColor::Yellow, "WARN", "[Renderer] -->\t Cannot pick presentation mode (MAILBOX)! \
       Defaulting to FIFO...");
      return vk::PresentModeKHR::FIFO;
    }
    if vsync_requested {
      log!(EnumLogColor::Yellow, "WARN", "[Renderer] -->\t Cannot pick presentation mode (MAILBOX) \
    nor FIFO! Defaulting to IMMEDIATE...");
    }
    return vk::PresentModeKHR::IMMEDIATE;
  }
  
  fn pick_swap_extent(surface_capabilities: &vk::SurfaceCapabilitiesKHR, desired_dimensions: Option<[u32; 2]>) -> Result<vk::Extent2D, EnumError> {
    use num::clamp;
    
    if surface_capabilities.current_extent.width != u32::MAX {
      return Ok(surface_capabilities.current_extent);
    }
    if desired_dimensions.is_some() {
      let min = surface_capabilities.min_image_extent;
      let max = surface_capabilities.max_image_extent;
      let width = desired_dimensions.unwrap()[0].min(max.width).max(min.width);
      let height = desired_dimensions.unwrap()[1].min(max.height).max(min.height);
      return Ok(vk::Extent2D {
        width,
        height,
      });
    }
    if Window::get().is_none() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot get framebuffer size for swap \
      extent : No active window context");
      return Err(EnumError::VulkanError(EnumVulkanErrors::SwapError));
    }
    
    let (width, height) = unsafe { (*Window::get().unwrap()).m_api_window.get_framebuffer_size() };
    let actual_width: u32 = clamp(width as u32, surface_capabilities.min_image_extent.width,
      surface_capabilities.max_image_extent.width);
    let actual_height: u32 = clamp(height as u32, surface_capabilities.min_image_extent.height,
      surface_capabilities.max_image_extent.height);
    
    return Ok(vk::Extent2D {
      width: actual_width,
      height: actual_height,
    });
  }
  
  fn query_queue_families(ash_instance: &ash::Instance, surface: &khr::Surface, surface_khr: vk::SurfaceKHR, vk_physical_device: vk::PhysicalDevice) -> Result<VkQueueFamilyIndices, EnumError> {
    let queue_families = unsafe {
      ash_instance.get_physical_device_queue_family_properties(vk_physical_device)
    };
    
    if queue_families.is_empty() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot retrieve queue families for physical \
      device {0:#?}!", vk_physical_device);
      return Err(EnumError::VulkanError(EnumVulkanErrors::PhysicalDeviceError));
    }
    
    let mut queue_family_indices: VkQueueFamilyIndices = VkQueueFamilyIndices::default();
    
    // Find graphics queue family.
    let mut graphic_family_index: u32 = 0;
    let mut present_family_index: u32 = 0;
    
    for queue_family in queue_families {
      // Check for graphics support.
      if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
        queue_family_indices.m_graphics_family_index = Some(graphic_family_index)
      }
      
      // Check for presentation support.
      if unsafe {
        surface.get_physical_device_surface_support(vk_physical_device,
          present_family_index, surface_khr)
      }.is_ok() {
        queue_family_indices.m_present_family_index = Some(present_family_index);
      }
      graphic_family_index += 1;
      present_family_index += 1;
    }
    
    return Ok(queue_family_indices);
  }
  
  fn query_swap_properties(surface: &khr::Surface, vk_device: vk::PhysicalDevice, khr_surface: vk::SurfaceKHR) -> Result<VkSwapChainProperties, EnumError> {
    let formats = unsafe {
      // Query the supported SwapChain format-color space pairs for a surface.
      surface.get_physical_device_surface_formats(vk_device, khr_surface)
    };
    if formats.is_err() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot choose an ideal surface format, \
        Vulkan returned result : {:?}", formats.unwrap());
      return Err(EnumError::VulkanError(EnumVulkanErrors::SwapError));
    }
    
    // Query the basic capabilities of a surface, needed in order to create a SwapChain.
    let capabilities = unsafe {
      surface.get_physical_device_surface_capabilities(vk_device, khr_surface)
    };
    if capabilities.is_err() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot choose an ideal surface format, \
        Vulkan returned result : {:?}", formats.unwrap());
      return Err(EnumError::VulkanError(EnumVulkanErrors::SwapError));
    }
    
    // Query the supported presentation modes for a surface.
    let present_modes = unsafe {
      surface.get_physical_device_surface_present_modes(vk_device, khr_surface)
    };
    if present_modes.is_err() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot choose an ideal surface format, \
        Vulkan returned result : {:?}", formats.unwrap());
      return Err(EnumError::VulkanError(EnumVulkanErrors::SwapError));
    }
    
    let swap_chain_info = VkSwapChainProperties::new(capabilities.unwrap(), formats.unwrap(),
      present_modes.unwrap());
    return Ok(swap_chain_info);
  }
  
  fn is_device_suitable(ash_instance: &ash::Instance, surface: &khr::Surface, surface_khr: vk::SurfaceKHR, vk_physical_device: vk::PhysicalDevice) -> bool {
    // Check if graphics family queue exists.
    let queue_families = VkContext::query_queue_families(ash_instance,
      surface, surface_khr, vk_physical_device);
    
    if queue_families.is_err() {
      return false;
    }
    
    if queue_families.as_ref().unwrap().m_graphics_family_index.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] --.\t Physical device {0:#?} \
      does not have a queue dedicated for graphics exchange!", vk_physical_device);
      return false;
    }
    if queue_families.as_ref().unwrap().m_present_family_index.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] --.\t Physical device {0:#?} \
      does not have a queue that supports presentation!", vk_physical_device);
      return false;
    }
    
    return true;
  }
}

impl TraitContext for VkContext {
  fn on_new(window: &mut Window) -> Result<Self, EnumError> {
    let (ash_entry, ash_instance) =
      VkContext::create_instance(window, None, None)?;
    #[allow(unused)]
      let debug_callback: Option<(ext::DebugUtils, vk::DebugUtilsMessengerEXT)> = None;
    
    // Create surface (graphic context).
    let vk_surface = khr::Surface::new(&ash_entry, &ash_instance);
    let mut khr_surface = vk::SurfaceKHR::default();
    
    window.init_vulkan_surface(&ash_instance, &mut khr_surface);
    
    // Pick ideal physical device for surface and load corresponding logical device.
    let vk_physical_device = VkContext::pick_physical_device(&ash_instance,
      &vk_surface, khr_surface)?;
    let queue_family_indices = VkContext::query_queue_families(&ash_instance,
      &vk_surface, khr_surface, vk_physical_device)?;
    let ash_logical_device = VkContext::create_logical_device(&ash_instance, vk_physical_device,
      queue_family_indices.m_graphics_family_index.unwrap_or(0), None)?;
    
    Ok(VkContext {
      m_entry: ash_entry,
      m_instance: ash_instance,
      m_surface: vk_surface,
      m_surface_khr: khr_surface,
      m_debug_report_callback: debug_callback,
      m_physical_device: vk_physical_device,
      m_queue_family_indices: queue_family_indices,
      m_logical_device: ash_logical_device,
      m_swap_chain_properties: VkSwapChainProperties::default(),
      m_swap_chain: None,
      m_swap_chain_khr: Default::default(),
      m_swap_chain_images: Default::default(),
      m_swap_chain_image_views: Default::default()
    })
  }
  
  fn get_api_version(&self) -> f32 {
    let device_properties = unsafe {
      self.m_instance.get_physical_device_properties(self.m_physical_device)
    };
    
    let major = vk::api_version_major(device_properties.api_version);
    let minor = vk::api_version_minor(device_properties.api_version);
    
    let to_str = format!("{0}.{1}", major, minor);
    let to_float: f32 = to_str.parse::<f32>().unwrap_or(-1.0);
    
    return to_float;
  }
  
  fn on_events(&mut self, window_event: glfw::WindowEvent) -> Result<bool, EnumError> {
    return match window_event {
      _ => Ok(false)
    }
  }
  
  fn on_delete(&mut self) -> Result<(), EnumError> {
    unsafe {
      self.m_swap_chain_image_views.iter().for_each(|image_view| {
        self.m_logical_device.destroy_image_view(*image_view, None);
      });
      
      if self.m_swap_chain.is_some() {
        self.m_swap_chain.as_ref().unwrap().destroy_swapchain(self.m_swap_chain_khr, None);
      }
      if self.m_logical_device.device_wait_idle().is_err() {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot wait for device \
         (Vulkan logical device) to finish!");
        return Err(EnumError::VulkanError(EnumVulkanErrors::LogicalDeviceError));
      }
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
    return Ok(());
  }
  
  fn submit(&mut self, features: &HashSet<EnumFeature>) -> Result<(), EnumError> {
    // Toggle features.
    for feature in features {
      self.toggle(*feature)?;
    }
    
    let window = Window::get();
    if window.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot create swap chain : No active \
      window context to present images onto!");
      return Err(EnumError::VulkanError(EnumVulkanErrors::SwapError));
    }
    // Create swap chain.
    self.create_swap_chain(unsafe { (*window.unwrap()).m_vsync })?;
    
    let swap_chain_images = unsafe {
      if self.m_swap_chain.is_none() {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot retrieve swap chain images : \
        No active swap chain!");
        return Err(EnumError::VulkanError(EnumVulkanErrors::SwapError));
      }
      self.m_swap_chain.as_ref().unwrap().get_swapchain_images(self.m_swap_chain_khr)
    };
    if swap_chain_images.is_err() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Could not retrieve swap chain images : \
        Vulkan returned with : {:?}", swap_chain_images.unwrap());
      return Err(EnumError::VulkanError(EnumVulkanErrors::SwapImagesError));
    }
    
    self.m_swap_chain_images = swap_chain_images.unwrap();
    
    self.create_swap_image_views()?;
    
    return Ok(());
  }
  
  fn get_max_msaa_count(&self) -> Result<u8, EnumError> {
    let device_properties =
      unsafe {
        self.m_instance.get_physical_device_properties(self.m_physical_device)
      };
    let max_color_sample_count = device_properties.limits.framebuffer_color_sample_counts;
    let max_depth_sample_count = device_properties.limits.framebuffer_depth_sample_counts;
    let max_sample_count = max_color_sample_count.min(max_depth_sample_count);
    
    if max_sample_count.contains(vk::SampleCountFlags::TYPE_64) {
      return Ok(64);
    }
    if max_sample_count.contains(vk::SampleCountFlags::TYPE_32) {
      return Ok(32);
    }
    if max_sample_count.contains(vk::SampleCountFlags::TYPE_16) {
      return Ok(16);
    }
    if max_sample_count.contains(vk::SampleCountFlags::TYPE_8) {
      return Ok(8);
    }
    if max_sample_count.contains(vk::SampleCountFlags::TYPE_4) {
      return Ok(4);
    }
    if max_sample_count.contains(vk::SampleCountFlags::TYPE_2) {
      return Ok(2);
    }
    log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot retrieve max sample count for MSAA : \
    MSAA not supported!");
    return Err(EnumError::VulkanError(EnumVulkanErrors::NotSupported));
  }
  
  fn to_string(&self) -> String {
    let device_properties = unsafe {
      self.m_instance.get_physical_device_properties(self.m_physical_device)
    };
    let device_name_str = unsafe { std::ffi::CStr::from_ptr(device_properties.device_name.as_ptr()) }
      .to_str()
      .unwrap_or("[Renderer] -->\t Could not retrieve device name in get_api_info()!");
    
    let device_type_str: String;
    match device_properties.device_type {
      PhysicalDeviceType::DISCRETE_GPU => { device_type_str = "Discrete GPU".to_string() }
      PhysicalDeviceType::INTEGRATED_GPU => { device_type_str = "Integrated GPU".to_string() }
      PhysicalDeviceType::CPU => { device_type_str = "CPU".to_string() }
      _ => { device_type_str = "Other".to_string() }
    };
    
    let info_physical_device: String = format!("Api => Vulkan,\n{0:<113}\
    Api version => {1}.{2}.{3}\n{0:<113}\
    Device name => {4}\n{0:<113}Driver version => {5}\n{0:<113}Device type => {6}",
      "",
      vk::api_version_major(device_properties.api_version),
      vk::api_version_minor(device_properties.api_version),
      vk::api_version_patch(device_properties.api_version),
      device_name_str,
      VkContext::get_driver_version(device_properties.driver_version,
      device_properties.vendor_id), device_type_str);
    
    // Get logical device capabilities and presentation format and extent chosen for swap chain.
    let info_logical_device: String = format!("\n{0:113}{1}", "", self.m_swap_chain_properties);
    
    return info_physical_device + info_logical_device.as_str();
  }
  
  fn toggle(&mut self, feature: EnumFeature) -> Result<(), EnumError> {
    match feature {
      EnumFeature::Debug(enabled) => {
        if enabled {
          // Toggle on debugging.
          #[cfg(feature = "debug")]
          {
            let debug_callback = Some(VkContext::set_debug(&self.m_entry, &self.m_instance)?);
            self.m_debug_report_callback = debug_callback;
          }
        } else {
          // Toggle off debugging.
          unsafe {
            if let Some((debug_utils, messenger)) = self.m_debug_report_callback.take() {
              debug_utils.destroy_debug_utils_messenger(messenger, None);
            }
          }
          self.m_debug_report_callback = None;
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Debug mode {0}",
          enabled
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
      EnumFeature::DepthTest(_) => {}
      EnumFeature::CullFacing(_) => {}
      EnumFeature::Wireframe(_) => {}
      EnumFeature::MSAA(sample_count) => {
        let mut max_sample_count: u8 = 1;
        if sample_count.is_some() {
          max_sample_count = self.get_max_msaa_count()?;
          if sample_count.unwrap() > max_sample_count && sample_count.unwrap() > 2 {
            log!(EnumLogColor::Yellow, "WARN", "[Renderer] -->\t Cannot enable MSAA with X{0}! \
              Defaulting to {1}...", sample_count.unwrap(), max_sample_count);
          } else if max_sample_count == 1 {
            log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot enable MSAA!");
            return Err(EnumError::MSAAError);
          }
          todo!("Apply the new multisample count to the color and depth attachments.");
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t MSAA {0}",
          sample_count.is_some()
          .then(|| return format!("enabled (X{0})", max_sample_count))
          .unwrap_or("disabled".to_string()));
      }
      EnumFeature::SRGB(_) => {}
      EnumFeature::Blending(_, _, _) => {}
    }
    return Ok(());
  }
  
  fn batch(&mut self, _camera: &PerspectiveCamera) -> Result<(), EnumError> {
    todo!()
  }
  
  fn flush(&mut self) {
    todo!()
  }
  
  fn enqueue(&mut self, _sendable_entity: &REntity, _shader_associated: &mut Shader) -> Result<(), EnumError> {
    return Ok(());
  }
  
  fn on_render(&mut self) -> Result<(), EnumError> {
    return Ok(());
  }
  
  fn dequeue(&mut self, _id: &u64) -> Result<(), EnumError> {
    todo!()
  }
  
  fn get_shader_version(&self) -> f32 {
    todo!()
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
    Flag::INFO => {}
    // Flag::INFO => log!("INFO", "{:?} -->\t {:#?}", type_, message),
    Flag::WARNING => log!(EnumLogColor::Yellow, "WARN", "{:?} -->\t {:#?}", type_, message),
    _ => log!(EnumLogColor::Red, "ERROR", "{:?} -->\t {:#?}", type_, message),
  }
  
  return vk::FALSE;
}