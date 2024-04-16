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

use crate::graphics::texture::{EnumTexture, EnumTextureDataAlignment, EnumTextureFormat, EnumTextureHint, EnumTextureLoaderError, EnumTextureTarget, Texture};
use crate::{TraitHint};
use crate::utils::macros::logger::*;
#[cfg(feature = "debug")]
use crate::Engine;

#[allow(unused)]
pub struct TextureLoader {
  m_hints: Vec<EnumTextureHint>,
}

impl Default for TextureLoader {
  fn default() -> Self {
    return Self {
      m_hints: vec![EnumTextureHint::TargetApi(Default::default()), EnumTextureHint::IsHdr(false),
        EnumTextureHint::TargetFormat(Default::default()), EnumTextureHint::TargetMipMapLevel(0),
        EnumTextureHint::DataEncodedWith(Default::default()), EnumTextureHint::FlipPixels(true)],
    };
  }
}

impl TraitHint<EnumTextureHint> for TextureLoader {
  fn set_hint(&mut self, hint: EnumTextureHint) {
    if let Some(hint_found) = self.m_hints.iter().position(|h| h.is_equivalent(&hint)) {
      self.m_hints.remove(hint_found);
    }
    
    self.m_hints.push(hint);
  }
  
  fn reset_hints(&mut self) {
    self.m_hints = vec![EnumTextureHint::TargetApi(Default::default()), EnumTextureHint::IsHdr(false),
      EnumTextureHint::TargetFormat(Default::default()), EnumTextureHint::TargetMipMapLevel(0),
      EnumTextureHint::DataEncodedWith(Default::default()), EnumTextureHint::FlipPixels(true)];
  }
}

impl TextureLoader {
  pub fn new() -> Self {
    return Self {
      m_hints: vec![],
    };
  }
  
  pub fn load(&self, file_path: &str) -> Result<Texture, EnumTextureLoaderError> {
    // If we are dealing with left hand side coordinates for UVs, like in OpenGL.
    if self.m_hints.contains(&EnumTextureHint::FlipPixels(true)) {
      unsafe {
        stb_image::stb_image::stbi_set_flip_vertically_on_load(1);
      }
    }
    
    let file_loaded = stb_image::image::load(file_path);
    let mut texture_data: (EnumTexture, stb_image::image::Image<u8>) = (EnumTexture::default(), stb_image::image::Image {
      width: 0,
      height: 0,
      depth: 0,
      data: vec![],
    });
    
    // Init with default values if case no hints were specified.
    let mut texture_dimensions = (0, 0, 0);
    let mut texture_target = EnumTextureTarget::default();
    let mut texture_mipmap = 0;
    let mut texture_data_type = EnumTextureDataAlignment::default();
    let mut texture_format = EnumTextureFormat::default();
    let mut texture_hdr = false;
    
    // Toggle all provided hints before sending it off to api.
    for hint in self.m_hints.iter() {
      match *hint {
        EnumTextureHint::TextureType(target) => texture_target = target,
        EnumTextureHint::TargetDimensions(dimensions) => texture_dimensions = dimensions,
        EnumTextureHint::TargetMipMapLevel(mipmap) => texture_mipmap = mipmap,
        EnumTextureHint::TargetFormat(format) => texture_format = format,
        EnumTextureHint::DataEncodedWith(data_type) => texture_data_type = data_type,
        EnumTextureHint::IsHdr(bool) => texture_hdr = bool,
        _ => {}
      }
    }
    
    match file_loaded {
      stb_image::image::LoadResult::Error(message) => {
        log!(EnumLogColor::Red, "ERROR", "[Texture] -->\t Cannot load texture from file {0}, Error => {1}", file_path, message);
        return Err(EnumTextureLoaderError::FileError(message));
      }
      stb_image::image::LoadResult::ImageU8(data) => {
        match data.depth {
          1 => texture_format = EnumTextureFormat::Red,
          2 => texture_format = EnumTextureFormat::Rg,
          3 => texture_format = EnumTextureFormat::Rgb,
          4 => texture_format = EnumTextureFormat::Rgba,
          _ => {}
        }
        
        // Check if our specified dimensions are correct, if so take them, otherwise use figure out using data.
        if texture_dimensions > (data.width as u32, data.height as u32, data.depth as u32) || texture_dimensions == (0, 0, 0) {
          texture_dimensions = (data.width as u32, data.height as u32, data.depth as u32);
        }
        
        match texture_target {
          EnumTextureTarget::Texture1D => {
            texture_data = (EnumTexture::Texture1D(texture_target, texture_mipmap, texture_format, texture_dimensions.0,
              texture_data_type), data);
          }
          EnumTextureTarget::Texture2D => {
            texture_data = (EnumTexture::Texture2D(texture_target, texture_mipmap, texture_format, texture_dimensions.0,
              texture_dimensions.1, texture_data_type), data);
          }
          EnumTextureTarget::Texture2DMs(_sample_count) => {
            texture_data = (EnumTexture::Texture2D(texture_target, texture_mipmap, texture_format, texture_dimensions.0,
              texture_dimensions.1, texture_data_type), data);
          }
          EnumTextureTarget::Texture3D => {
            texture_data = (EnumTexture::Texture3D(texture_target, texture_mipmap, texture_format, texture_dimensions.0,
              texture_dimensions.1, texture_dimensions.2, texture_data_type), data);
          }
          EnumTextureTarget::Texture3DMs(_sample_count) => {
            texture_data = (EnumTexture::Texture3D(texture_target, texture_mipmap, texture_format, texture_dimensions.0,
              texture_dimensions.1, texture_dimensions.2, texture_data_type), data);
          }
          _ => todo!()
        }
      }
      stb_image::image::LoadResult::ImageF32(_data) => {
        if !texture_hdr {
          log!(EnumLogColor::Red, "ERROR", "[Texture] -->\t Cannot load texture {0:?} as HDR, texture not HDR!", texture_data.0);
          return Err(EnumTextureLoaderError::InvalidFormat);
        }
        todo!()
      }
    }
    
    return Ok(Texture::new(self.m_hints.clone(), texture_data.0, texture_data.1));
  }
}
