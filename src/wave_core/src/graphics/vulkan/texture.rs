/*
 MIT License

 Copyright (c) 2024 Nami Reghbati

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

use stb_image::image::Image;
use crate::graphics::renderer::EnumRendererError;
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan::renderer::EnumVkContextError;
use crate::graphics::texture::{EnumTextureType, TraitTexture};

#[allow(unused)]
#[cfg(feature = "vulkan")]
#[derive(Debug, PartialEq)]
pub enum EnumVkTextureError {
  InvalidInternalFormat,
  InternalError(EnumVkContextError)
}

#[allow(unused)]
#[cfg(feature = "vulkan")]
pub(crate) struct VkTexture<T> {
  m_data: Image<T>
}

#[cfg(feature = "vulkan")]
impl<T> VkTexture<T> {
  pub(crate) fn default() -> Self {
    return Self {
      m_data: Image {
        width: 0,
        height: 0,
        depth: 0,
        data: vec![],
      },
    }
  }
  
  pub(crate) fn new(_texture_type: EnumTextureType, _data: T) -> Self {
    todo!()
  }
}

#[cfg(feature = "vulkan")]
impl<T> TraitTexture for VkTexture<T> {
  fn apply(&mut self) -> Result<(), EnumRendererError> {
    todo!()
  }
  
  fn clear(&mut self) -> Result<(), EnumRendererError> {
    todo!()
  }
  
  fn free(&mut self) -> Result<(), EnumRendererError> {
    todo!()
  }
}