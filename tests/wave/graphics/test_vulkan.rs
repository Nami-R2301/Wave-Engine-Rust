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

#[cfg(feature = "Vulkan")]
use wave_engine::wave::graphics::vulkan::renderer::{ash::vk, VkContext};

#[cfg(feature = "Vulkan")]
#[test]
fn test_instance_extensions() {
  let entry = ash::Entry::linked();
  let window = glfw::init_no_callbacks().expect("Failed to create GLFW window context!");
  
  let vec = entry.enumerate_instance_extension_properties(None)
    .expect("Cannot convert to c string!");
  
  // Test with dynamic extension loading function and layers.
  unsafe {
    let extensions_func = VkContext::load_extensions(&window, None);
    for extension in extensions_func.unwrap() {
      assert!(vec.iter()
        .any(|property| *property.extension_name.as_ptr() == *extension.as_ptr()));
    }
  }
  
  // Test with static byte arrays.
  {
    let extensions_raw = unsafe {
      vec![std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_surface\0").as_ptr(),
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_xcb_surface\0").as_ptr()]
    };
    
    unsafe {
      for extension in extensions_raw {
        assert!(vec.iter()
          .any(|property| *property.extension_name.as_ptr() == *extension));
      }
    }
  }
  // Test with static CString.
  {
    let extensions_raw =
      vec![std::ffi::CString::new("VK_KHR_surface")
        .expect("Cannot convert to C string!"),
        std::ffi::CString::new("VK_KHR_xcb_surface")
          .expect("Cannot convert to C string!")];
    
    unsafe {
      for extension in extensions_raw {
        assert!(vec.iter()
          .any(|property| *property.extension_name.as_ptr() == *extension.as_ptr()));
      }
    }
  }
}

#[cfg(feature = "Vulkan")]
#[test]
fn test_instance_layers() {
  let entry = ash::Entry::linked();
  
  // Validate API instance layers
  unsafe {
    let layers = VkContext::load_layers(None);
    let vec = entry.enumerate_instance_layer_properties()
      .expect("Cannot convert to c string!");
    
    for layer in layers.unwrap() {
      assert!(vec.iter()
        .any(|property| *property.layer_name.as_ptr() == *layer.as_ptr()));
    }
  }
}

#[cfg(feature = "Vulkan")]
#[test]
fn test_instance_creation() {
  let window = glfw::init_no_callbacks().expect("Failed to create GLFW window context!");
  let entry = ash::Entry::linked();
  
  let app_info = vk::ApplicationInfo::default();
  
  // Test with no extensions and layers.
  let instance_create_info = vk::InstanceCreateInfo::default();
  
  match unsafe { entry.create_instance(&instance_create_info, None) } {
    Ok(instance) => {
      unsafe { instance.destroy_instance(None) };
    }
    Err(err) => {
      println!("Error creating Vulkan instance! Error => {:#?}", err);
      assert!(false);
    }
  }
  
  // Test with dynamic extension loading function and layers.
  let extensions =
    VkContext::load_extensions(&window, None)
      .expect("Failed to load Vulkan instance extensions");
  
  VkContext::check_extension_support(&entry, &extensions)
    .expect("Error validating extensions!");
  
  let c_extensions_ptr = extensions
    .iter()
    .map(|c_extension| c_extension.as_ptr())
    .collect::<Vec<*const std::ffi::c_char>>();
  
  let layers = VkContext::load_layers(None)
    .expect("Failed to load Vulkan instance layers");
  
  VkContext::check_layer_support(&entry, &layers)
    .expect("Error validating layers!");
  
  let c_layers_ptr = layers
    .iter()
    .map(|c_layer| c_layer.as_ptr())
    .collect::<Vec<*const std::ffi::c_char>>();
  
  let mut instance_create_info = vk::InstanceCreateInfo::default();
  instance_create_info.enabled_layer_count = c_layers_ptr.len() as u32;
  instance_create_info.pp_enabled_layer_names = c_layers_ptr.as_ptr();
  instance_create_info.enabled_extension_count = c_extensions_ptr.len() as u32;
  instance_create_info.pp_enabled_extension_names = c_extensions_ptr.as_ptr();
  instance_create_info.p_application_info = &app_info;
  
  match unsafe { entry.create_instance(&instance_create_info, None) } {
    Ok(instance) => {
      unsafe { instance.destroy_instance(None) };
    }
    Err(err) => {
      println!("Error creating Vulkan instance! Error => {:#?}", err);
      assert!(false);
    }
  }
  
  // Test with static byte arrays.
  let extensions_raw = unsafe {
    vec![std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_surface\0").as_ptr(),
      std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_xcb_surface\0").as_ptr()]
  };
  VkContext::check_extension_support(&entry, &extensions)
    .expect("Error validating extensions!");
  
  let layers = VkContext::load_layers(None)
  .expect("Failed to load Vulkan instance layers");
  
  VkContext::check_layer_support(&entry, &layers)
    .expect("Error validating layers!");
  
  let c_layers_ptr = layers
    .iter()
    .map(|c_layer| c_layer.as_ptr())
    .collect::<Vec<*const std::ffi::c_char>>();
  
  let mut instance_create_info = vk::InstanceCreateInfo::default();
  instance_create_info.enabled_layer_count = c_layers_ptr.len() as u32;
  instance_create_info.pp_enabled_layer_names = c_layers_ptr.as_ptr();
  instance_create_info.enabled_extension_count = extensions_raw.len() as u32;
  instance_create_info.pp_enabled_extension_names = extensions_raw.as_ptr();
  instance_create_info.p_application_info = &app_info;
  
  match unsafe { entry.create_instance(&instance_create_info, None) } {
    Ok(instance) => {
      unsafe { instance.destroy_instance(None) };
    }
    Err(err) => {
      println!("Error creating Vulkan instance! Error => {:#?}", err);
      assert!(false);
    }
  }
  
  // Test with static CStrings.
  let c_extensions =
    vec![std::ffi::CString::new("VK_KHR_surface")
      .expect("Failed to convert to C string!"),
      std::ffi::CString::new("VK_KHR_xcb_surface")
        .expect("Failed to convert to C string!")];
  
  VkContext::check_extension_support(&entry, &extensions).expect("Error validating extensions!");
  
  let c_extensions_ptr = c_extensions
    .iter()
    .map(|c_extension| c_extension.as_ptr())
    .collect::<Vec<*const std::ffi::c_char>>();
  
  let layers = VkContext::load_layers(None)
    .expect("Failed to load Vulkan instance layers");
  
  VkContext::check_layer_support(&entry, &layers)
    .expect("Error validating layers!");
  
  let c_layers_ptr = layers
    .iter()
    .map(|c_layer| c_layer.as_ptr())
    .collect::<Vec<*const std::ffi::c_char>>();
  
  let mut instance_create_info = vk::InstanceCreateInfo::default();
  instance_create_info.enabled_layer_count = c_layers_ptr.len() as u32;
  instance_create_info.pp_enabled_layer_names = c_layers_ptr.as_ptr();
  instance_create_info.enabled_extension_count = c_extensions_ptr.len() as u32;
  instance_create_info.pp_enabled_extension_names = c_extensions_ptr.as_ptr();
  instance_create_info.p_application_info = &app_info;
  
  match unsafe { entry.create_instance(&instance_create_info, None) } {
    Ok(instance) => {
      unsafe { instance.destroy_instance(None) };
    }
    Err(err) => {
      println!("Error creating Vulkan instance! Error => {:#?}", err);
      assert!(false);
    }
  }
}