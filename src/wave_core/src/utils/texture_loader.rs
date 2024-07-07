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

use std::any::Any;

#[cfg(feature = "debug")]
use crate::Engine;
use crate::graphics::texture::{EnumTextureDataAlignment, EnumTextureFormat, EnumTextureInfo, EnumTextureLoaderError, EnumTextureTarget};
use crate::TraitHint;
use crate::utils::macros::logger::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumTextureLoaderHint {
  TextureType(EnumTextureTarget),
  MaxDimensions((u32, u32, u32)),
  MaxMipMapLevel(u32),
  TargetFormat(EnumTextureFormat),
  IsHdr(bool),
  DataEncodedWith(EnumTextureDataAlignment),
  FlipUvs(bool),
  BindLess(bool),
}

impl EnumTextureLoaderHint {
  pub fn get_value(&self) -> &dyn Any {
    let result: &dyn Any;
    match self {
      EnumTextureLoaderHint::TextureType(value) => result = value,
      EnumTextureLoaderHint::MaxDimensions(value) => result = value,
      EnumTextureLoaderHint::MaxMipMapLevel(value) => result = value,
      EnumTextureLoaderHint::TargetFormat(value) => result = value,
      EnumTextureLoaderHint::IsHdr(value) => result = value,
      EnumTextureLoaderHint::DataEncodedWith(value) => result = value,
      EnumTextureLoaderHint::FlipUvs(bool) => result = bool,
      EnumTextureLoaderHint::BindLess(bool) => result = bool
    };
    return result;
  }
  
  pub fn is_equivalent(&self, other: &Self) -> bool {
    return std::mem::discriminant(self) == std::mem::discriminant(other);
  }
}

pub struct TextureInfo<T> {
  pub(crate) m_type: EnumTextureInfo,
  pub(crate) m_data: stb_image::image::Image<T>,
}

impl<T: Clone> Clone for TextureInfo<T> {
  fn clone(&self) -> Self {
    return Self {
      m_type: self.m_type.clone(),
      m_data: stb_image::image::Image {
        width: self.m_data.width,
        height: self.m_data.height,
        depth: self.m_data.depth,
        data: self.m_data.data.clone(),
      },
    }
  }
}

impl<T: Clone> TextureInfo<T> {
  pub(crate) fn get_type(&self) -> EnumTextureInfo {
    return self.m_type.clone();
  }
  
  pub(crate) fn get_data(&self) -> Vec<T> {
    return self.m_data.data.clone();
  }
}

#[allow(unused)]
pub struct TextureLoader {
  m_hints: Vec<EnumTextureLoaderHint>,
}

impl TraitHint<EnumTextureLoaderHint> for TextureLoader {
  fn set_hint(&mut self, hint: EnumTextureLoaderHint) {
    if let Some(hint_found) = self.m_hints.iter().position(|h| h.is_equivalent(&hint)) {
      self.m_hints.remove(hint_found);
    }
    
    self.m_hints.push(hint);
  }
  
  fn reset_hints(&mut self) {
    self.m_hints.clear();
  }
}

impl TextureLoader {
  pub fn new() -> Self {
    return Self {
      m_hints: Vec::with_capacity(9)
    };
  }
  
  pub fn load_from_folder(&self, folder_path_str: &str) -> Result<Vec<TextureInfo<u8>>, std::io::Error> {
    let texture_path = std::path::Path::new(folder_path_str);
    let mut textures = Vec::with_capacity(5);
    
    if !texture_path.exists() || !texture_path.is_dir() {
      log!(EnumLogColor::Red, "ERROR", "[Asset] -->\t Cannot load textures from folder {0}, folder either doesn't exist \
      or is not a folder!", texture_path.to_str().unwrap());
      return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    }
    
    let mut sorted_entries: Vec<_> = std::fs::read_dir(texture_path)?
      .filter_map(|r| r.ok())
      .collect();
    sorted_entries.sort_by_key(|dir| dir.path());
    
    for entry in sorted_entries {
      log!(EnumLogColor::Purple, "INFO", "[TexLoader] -->\t Loading texture {0:?} from folder {1:?}...",
          entry.file_name(), texture_path);
      let entry_name = entry.path();
      if let Ok(texture_info) = self.load(entry_name.to_str().unwrap()) {
        textures.push(texture_info);
      }
    }
    return Ok(textures);
  }
  
  pub fn load(&self, file_path: &str) -> Result<TextureInfo<u8>, EnumTextureLoaderError> {
    // If we are dealing with left hand side coordinates for UVs, like in OpenGL.
    unsafe {
      stb_image::stb_image::stbi_set_flip_vertically_on_load(self.m_hints.contains(&EnumTextureLoaderHint::FlipUvs(true))
        .then(|| 1)
        .unwrap_or(0));
    }
    
    let file_loaded = stb_image::image::load(file_path);
    let mut texture_info: (EnumTextureInfo, stb_image::image::Image<u8>) = (EnumTextureInfo::default(), stb_image::image::Image {
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
        EnumTextureLoaderHint::TextureType(target) => texture_target = target,
        EnumTextureLoaderHint::MaxDimensions(dimensions) => texture_dimensions = dimensions,
        EnumTextureLoaderHint::MaxMipMapLevel(mipmap) => texture_mipmap = mipmap,
        EnumTextureLoaderHint::TargetFormat(format) => texture_format = format,
        EnumTextureLoaderHint::DataEncodedWith(data_type) => texture_data_type = data_type,
        EnumTextureLoaderHint::IsHdr(bool) => texture_hdr = bool,
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
        
        let texture_slot: u16;
        match texture_dimensions {
          (64, 64, _) => texture_slot = 3,
          (128, 128, _) => texture_slot = 4,
          (256, 256, _) => texture_slot = 5,
          (512, 512, _) => texture_slot = 6,
          (1024, 1024, _) => texture_slot = 7,
          (2048, 2048, _) => texture_slot = 8,
          (size_x, size_y, _) => {
            log!(EnumLogColor::Red, "ERROR", "[TexLoader] -->\t Cannot load texture, texture dimensions ({0},{1}) unsupported!", size_x, size_y);
            return Err(EnumTextureLoaderError::InvalidSize);
          }
        }
        
        match texture_target {
          EnumTextureTarget::Texture1D => {
            texture_info = (EnumTextureInfo::Texture1D(texture_target, texture_mipmap, texture_format, texture_dimensions.0,
              texture_data_type, texture_slot), data);
          }
          EnumTextureTarget::Texture2D => {
            texture_info = (EnumTextureInfo::Texture2D(texture_target, texture_mipmap, texture_format, texture_dimensions.0,
              texture_dimensions.1, texture_data_type, texture_slot), data);
          }
          EnumTextureTarget::Texture2DMs(_sample_count) => {
            texture_info = (EnumTextureInfo::Texture2D(texture_target, texture_mipmap, texture_format, texture_dimensions.0,
              texture_dimensions.1, texture_data_type, texture_slot), data);
          }
          EnumTextureTarget::Texture3D => {
            texture_info = (EnumTextureInfo::Texture3D(texture_target, texture_mipmap, texture_format, texture_dimensions.0,
              texture_dimensions.1, 0, texture_data_type, texture_slot), data);
          }
          EnumTextureTarget::Texture3DMs(_sample_count) => {
            texture_info = (EnumTextureInfo::Texture3D(texture_target, texture_mipmap, texture_format, texture_dimensions.0,
              texture_dimensions.1, 0, texture_data_type, texture_slot), data);
          }
          _ => todo!()
        }
      }
      stb_image::image::LoadResult::ImageF32(_data) => {
        if !texture_hdr {
          log!(EnumLogColor::Red, "ERROR", "[Texture] -->\t Cannot load texture {0:?} as HDR, texture not HDR!", texture_info.0);
          return Err(EnumTextureLoaderError::InvalidFormat);
        }
        todo!()
      }
    }
    
    return Ok(TextureInfo {
      m_type: texture_info.0,
      m_data: texture_info.1,
    });
  }
}
