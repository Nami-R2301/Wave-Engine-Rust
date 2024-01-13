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
use ash::vk;
use crate::log;
use crate::wave::graphics::renderer::Renderer;
use crate::wave::graphics::vulkan::renderer::VkContext;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumError {
  InvalidVertexAttributeLocation,
  InvalidVertexBinding,
  InvalidVertexAttributeOffset,
  InvalidVertexOffset,
  InvalidVertexStride
}

#[allow(unused)]
pub struct VkVertexAttribute {
  m_attr_desc: vk::VertexInputAttributeDescription
}

impl VkVertexAttribute {
  pub fn default() -> Self {
    return Self {
      m_attr_desc: Default::default(),
    }
  }
  
  pub fn new(binding: u32, location: u32, format: vk::Format, offset: u32) -> Result<Self, EnumError> {
    let vk_renderer = unsafe {
      (*Renderer::get()
        .expect("[Buffer] --> Cannot create VkVertexAttribute : Renderer is None!"))
        .get_api_handle()
        .downcast_mut::<VkContext>()
        .expect("[Buffer] --> Cannot create VkVertexAttribute : Renderer is not Vulkan!")
    };
    
    let physical_device_limits: vk::PhysicalDeviceLimits = vk_renderer.get_limits();
    
    if location > physical_device_limits.max_vertex_input_attributes {
      log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Cannot create VkVertexAttribute : Location \
      specified {0} exceeds the maximum physically supported on this device!", location);
      return Err(EnumError::InvalidVertexAttributeLocation);
    }
    
    if binding > physical_device_limits.max_vertex_input_bindings {
      log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Cannot create VkVertexAttribute : Binding \
      specified {0} exceeds the maximum physically supported on this device!", binding);
      return Err(EnumError::InvalidVertexBinding);
    }
    
    if offset > physical_device_limits.max_vertex_input_attribute_offset {
      log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Cannot create VkVertexAttribute : Offset \
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
pub struct VkVbo {
  m_input_desc: vk::VertexInputBindingDescription,
  m_size: usize,
  m_capacity: usize
}

impl VkVbo {
  pub fn default() -> Self {
    return Self {
      m_input_desc: Default::default(),
      m_size: 0,
      m_capacity: 0,
    }
  }
  pub fn new(binding: u32, stride: u32, input_rate: vk::VertexInputRate) -> Result<Self, EnumError> {
    let vk_renderer = unsafe {
      (*Renderer::get()
        .expect("[Buffer] --> Cannot create VkVbo : Renderer is None!"))
        .get_api_handle()
        .downcast_mut::<VkContext>()
        .expect("[Buffer] --> Cannot create VkVbo : Renderer is not Vulkan!")
    };
    
    let physical_device_limits: vk::PhysicalDeviceLimits = vk_renderer.get_limits();
    
    if binding > physical_device_limits.max_vertex_input_bindings {
      log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Cannot create VkVbo : Binding \
      specified {0} exceeds the maximum physically supported on this device!", binding);
      return Err(EnumError::InvalidVertexBinding);
    }
    
    if stride > physical_device_limits.max_vertex_input_binding_stride {
      log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Cannot create VkVbo : Stride \
      specified {0} exceeds the maximum physically supported on this device!", stride);
      return Err(EnumError::InvalidVertexStride);
    }
    
    //TODO Size and capacity of vertex buffer.
    return Ok(Self {
      m_input_desc: vk::VertexInputBindingDescription {
        binding,
        stride,
        input_rate,
      },
      m_size: stride as usize,
      m_capacity: (stride * 2) as usize,
    });
  }
}

#[allow(unused)]
pub struct VkVao {
  m_attr_desc_array: Vec<VkVertexAttribute>
}