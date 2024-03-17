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

#[cfg(feature = "vulkan")]
use ash::vk;

use crate::log;
use crate::wave_core::{Engine};
use crate::wave_core::graphics::vulkan::renderer::VkContext;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
enum EnumState {
  NotCreated,
  Created,
  Deleted,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumError {
  InvalidApi,
  InvalidVertexAttributeLocation,
  InvalidVertexBinding,
  InvalidVertexAttributeOffset,
  InvalidVertexOffset,
  InvalidVertexStride,
  InvalidVertexBufferSize,
  VertexBufferCreationError,
  NoActiveRendererError,
}

#[allow(unused)]
pub(crate) struct VkVertexAttribute {
  pub(crate) m_attr_desc: vk::VertexInputAttributeDescription,
}

impl VkVertexAttribute {
  #[allow(unused)]
  pub(crate) fn default() -> Self {
    return Self {
      m_attr_desc: Default::default(),
    };
  }
  
  #[allow(unused)]
  pub(crate) fn new(binding: u32, location: u32, format: vk::Format, offset: u32) -> Result<Self, EnumError> {
    let vk_renderer = Engine::get_active_renderer()
      .get_api_handle()
      .downcast_mut::<VkContext>()
      .expect("[VkBuffer] --> Cannot create VkVertexAttribute : Renderer is not Vulkan!");
    
    let physical_device_limits: vk::PhysicalDeviceLimits = vk_renderer.get_limits();
    
    if location > physical_device_limits.max_vertex_input_attributes {
      log!(EnumLogColor::Red, "ERROR", "[VkBuffer] -->\t Cannot create VkVertexAttribute : Location \
      specified {0} exceeds the maximum physically supported on this device!", location);
      return Err(EnumError::InvalidVertexAttributeLocation);
    }
    
    if binding > physical_device_limits.max_vertex_input_bindings {
      log!(EnumLogColor::Red, "ERROR", "[VkBuffer] -->\t Cannot create VkVertexAttribute : Binding \
      specified {0} exceeds the maximum physically supported on this device!", binding);
      return Err(EnumError::InvalidVertexBinding);
    }
    
    if offset > physical_device_limits.max_vertex_input_attribute_offset {
      log!(EnumLogColor::Red, "ERROR", "[VkBuffer] -->\t Cannot create VkVertexAttribute : Offset \
      specified {0} exceeds the maximum physically supported on this device!", offset);
      return Err(EnumError::InvalidVertexAttributeOffset);
    }
    
    return Ok(Self {
      m_attr_desc: vk::VertexInputAttributeDescription {
        location,
        binding,
        format,
        offset,
      },
    });
  }
}

#[allow(unused)]
pub(crate) struct VkVbo {
  m_state: EnumState,
  m_device: *mut ash::Device,
  m_handle: vk::Buffer,
  m_input_desc: vk::VertexInputBindingDescription,
  m_buffer_info: vk::BufferCreateInfo,
}

impl VkVbo {
  #[allow(unused)]
  pub(crate) fn default() -> Self {
    return Self {
      m_state: EnumState::NotCreated,
      m_device: std::ptr::null_mut(),
      m_handle: Default::default(),
      m_input_desc: Default::default(),
      m_buffer_info: Default::default(),
    };
  }
  
  #[allow(unused)]
  pub(crate) fn new(alloc_size: usize, binding: u32, stride: u32, input_rate: vk::VertexInputRate,
                    device: &mut ash::Device, concurrent_queues: Option<&[u32]>) -> Result<Self, EnumError> {
    let vk_renderer =  Engine::get_active_renderer()
      .get_api_handle()
      .downcast_mut::<VkContext>()
      .expect("[VkBuffer] --> Cannot create VkVbo : Renderer is not Vulkan!");
    
    let physical_device_limits: vk::PhysicalDeviceLimits = vk_renderer.get_limits();
    
    if binding > physical_device_limits.max_vertex_input_bindings {
      log!(EnumLogColor::Red, "ERROR", "[VkBuffer] -->\t Cannot create VkVbo : Binding \
      specified {0} exceeds the maximum physically supported on this device!", binding);
      return Err(EnumError::InvalidVertexBinding);
    }
    
    if stride > physical_device_limits.max_vertex_input_binding_stride {
      log!(EnumLogColor::Red, "ERROR", "[VkBuffer] -->\t Cannot create VkVbo : Stride \
      specified {0} exceeds the maximum physically supported on this device!", stride);
      return Err(EnumError::InvalidVertexStride);
    }
    
    if alloc_size == 0 {
      log!(EnumLogColor::Red, "ERROR", "[VkBuffer] -->\t Cannot create VkVbo : Size \
      specified {0} must be greater than 0!", stride);
      return Err(EnumError::InvalidVertexBufferSize);
    }
    
    let mut vbo_info = vk::BufferCreateInfo::default();
    vbo_info.usage = vk::BufferUsageFlags::VERTEX_BUFFER;
    vbo_info.sharing_mode = vk::SharingMode::EXCLUSIVE;
    vbo_info.size = alloc_size as vk::DeviceSize;
    
    if concurrent_queues.is_some() {
      vbo_info.sharing_mode = vk::SharingMode::CONCURRENT;
      vbo_info.queue_family_index_count = concurrent_queues.unwrap().len() as u32;
      vbo_info.p_queue_family_indices = concurrent_queues.unwrap().as_ptr();
    }
    
    unsafe {
      return match device.create_buffer(&vbo_info, None) {
        Ok(vk_buffer) => {
          Ok(Self {
            m_state: EnumState::Created,
            m_device: device,
            m_handle: vk_buffer,
            m_input_desc: vk::VertexInputBindingDescription {
              binding,
              stride,
              input_rate,
            },
            m_buffer_info: vbo_info,
          })
        }
        Err(err) => {
          log!(EnumLogColor::Red, "ERROR", "[VkBuffer] -->\t Cannot create Vulkan vertex buffer : \
          Vulkan returned with error => {err:?}");
          Err(EnumError::VertexBufferCreationError)
        }
      };
    }
  }
  
  #[allow(unused)]
  pub(crate) fn bind(&mut self) -> Result<(), EnumError> {
    todo!()
  }
  
  #[allow(unused)]
  pub(crate) fn unbind(&mut self) -> Result<(), EnumError> {
    todo!()
  }
  
  pub(crate) fn free(&mut self) -> Result<(), EnumError> {
    if self.m_state == EnumState::Deleted || self.m_state == EnumState::NotCreated {
      log!(EnumLogColor::Yellow, "WARN", "[VkBuffer] -->\t Cannot delete VkVbo : Already deleted \
      or not created in the first place!");
      return Ok(());
    }
    log!(EnumLogColor::Purple, "INFO", "[VkBuffer] -->\t Freeing VkVbo...");
    unsafe { (*self.m_device).destroy_buffer(self.m_handle, None) };
    self.m_state = EnumState::Deleted;
    log!(EnumLogColor::Green, "INFO", "[VkBuffer] -->\t Freed VkVbo successfully");
    return Ok(());
  }
}

#[allow(unused)]
pub(crate) struct VkVao {
  m_vbo_info: vk::PipelineVertexInputStateCreateInfo,
  m_attr_desc_array: Vec<VkVertexAttribute>,
}

impl VkVao {
  #[allow(unused)]
  pub(crate) fn default() -> Self {
    return Self {
      m_vbo_info: Default::default(),
      m_attr_desc_array: Vec::with_capacity(5),
    };
  }
  
  #[allow(unused)]
  pub(crate) fn new(attributes: Vec<VkVertexAttribute>, vbo_array: &[VkVbo]) -> Result<Self, EnumError> {
    for vbo in vbo_array {
      if !attributes.iter().all(|&ref element| element.m_attr_desc.binding == vbo.m_input_desc.binding) {
        return Err(EnumError::InvalidVertexBinding);
      }
    }
    
    let mut vertex_input_create_info = vk::PipelineVertexInputStateCreateInfo::default();
    vertex_input_create_info.vertex_attribute_description_count = attributes.len() as u32;
    vertex_input_create_info.vertex_binding_description_count = vbo_array.len() as u32;
    vertex_input_create_info.p_vertex_attribute_descriptions = attributes.iter()
      .map(|element| element.m_attr_desc)
      .collect::<Vec<vk::VertexInputAttributeDescription>>()
      .as_ptr();
    vertex_input_create_info.p_vertex_binding_descriptions = vbo_array.iter()
      .map(|element| element.m_input_desc)
      .collect::<Vec<vk::VertexInputBindingDescription>>()
      .as_ptr();
    
    return Ok(Self {
      m_vbo_info: vertex_input_create_info,
      m_attr_desc_array: attributes,
    });
  }
}