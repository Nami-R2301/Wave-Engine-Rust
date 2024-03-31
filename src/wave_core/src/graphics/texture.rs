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
use std::fmt::{Display, Formatter};

use stb_image::image::{Image, LoadResult};

use crate::Engine;
use crate::graphics::open_gl::texture::{EnumGlTextureError, GlTexture};
use crate::graphics::renderer::{EnumRendererApi, EnumRendererError};
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan::texture::EnumVkTextureError;
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan::texture::VkTexture;
use crate::utils::macros::logger::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EnumTextureTarget {
  Texture1D,
  Texture1DArray,
  ProxyTexture1DArray,
  Texture2D,
  Texture2DMs(u32),
  Texture2DArray,
  Texture2DArrayMs(u32),
  Texture3D,
  Texture3DMs(u32),
  ProxyTexture2D,
  TextureRect,
  ProxyTextureRect,
  TextureCubeMap,
  TextureCubeMapArray,
  ProxyTextureCubeMap,
  DepthStencil,
  ColorAttachment(u32),
}

impl Default for EnumTextureTarget {
  fn default() -> Self {
    return EnumTextureTarget::Texture2D;
  }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EnumTextureFormat {
  Red,
  Rg,
  Rgb,
  Bgr,
  Rgba,
  Bgra,
}

impl Default for EnumTextureFormat {
  fn default() -> Self {
    return EnumTextureFormat::Rgb;
  }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EnumCubeMapFace {
  Left,
  Right,
  Top,
  Bottom,
  Front,
  Back,
}

impl Default for EnumCubeMapFace {
  fn default() -> Self {
    return EnumCubeMapFace::Back;
  }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EnumTextureDataAlignment {
  UnsignedByte,
  UnsignedByte233Reverse,
  UnsignedByte332,
  Byte,
  UnsignedShort,
  UnsignedShort565,
  UnsignedShort565Reverse,
  UnsignedShort4444,
  UnsignedShort4444Reverse,
  UnsignedShort5551,
  UnsignedShort1555Reverse,
  Short,
  UnsignedInt,
  UnsignedInt8888,
  UnsignedInt8888Reverse,
  UnsignedInt10_10_10_2,
  UnsignedInt2_10_10_10Reverse,
  Int,
  Float,
}

impl Default for EnumTextureDataAlignment {
  fn default() -> Self {
    return EnumTextureDataAlignment::UnsignedByte;
  }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum EnumTexture {
  Texture1D(EnumTextureTarget, u32, EnumTextureFormat, u32, EnumTextureDataAlignment),
  Texture2D(EnumTextureTarget, u32, EnumTextureFormat, u32, u32, EnumTextureDataAlignment),
  Texture3D(EnumTextureTarget, u32, EnumTextureFormat, u32, u32, u32, EnumTextureDataAlignment),
  Texture2DArray(Vec<(EnumTextureTarget, u32, EnumTextureFormat, u32, u32, u32, EnumTextureDataAlignment)>),
  CubeMap([(EnumCubeMapFace, u32, EnumTextureFormat, u32, u32, EnumTextureDataAlignment); 6]),
}

impl Default for EnumTexture {
  fn default() -> Self {
    return EnumTexture::Texture2D(EnumTextureTarget::default(), 0, EnumTextureFormat::default(), 0, 0, EnumTextureDataAlignment::default());
  }
}

impl EnumTexture {
  pub(crate) fn get_target(&self) -> EnumTextureTarget {
    match self {
      EnumTexture::Texture1D(target, _, _, _, _) => *target,
      EnumTexture::Texture2D(target, _, _, _, _, _) => *target,
      EnumTexture::Texture3D(target, _, _, _, _, _, _) => *target,
      EnumTexture::Texture2DArray(_) => EnumTextureTarget::Texture2DArray,
      EnumTexture::CubeMap(_) => EnumTextureTarget::TextureCubeMap,
    }
  }
  
  pub(crate) fn get_format(&self) -> EnumTextureFormat {
    match self {
      EnumTexture::Texture1D(_, _, format, _, _) => *format,
      EnumTexture::Texture2D(_, _, format, _, _, _) => *format,
      EnumTexture::Texture3D(_, _, format, _, _, _, _) => *format,
      EnumTexture::Texture2DArray(vec) => {
        if !vec.is_empty() {
          return vec[0].2;
        }
        return EnumTextureFormat::default();
      }
      EnumTexture::CubeMap(faces) => {
        if faces.is_empty() {
          return faces[0].2;
        }
        return EnumTextureFormat::default();
      }
    }
  }
  
  pub(crate) fn get_mipmap_level(&self) -> u32 {
    match self {
      EnumTexture::Texture1D(_, mipmap, _, _, _) => *mipmap,
      EnumTexture::Texture2D(_, mipmap, _, _, _, _) => *mipmap,
      EnumTexture::Texture3D(_, mipmap, _, _, _, _, _) => *mipmap,
      EnumTexture::Texture2DArray(vec) => {
        if vec.is_empty() {
          return vec[0].1;
        }
        return 0;
      }
      EnumTexture::CubeMap(faces) => {
        if faces.is_empty() {
          return faces[0].1;
        }
        return 0;
      }
    }
  }
  
  pub(crate) fn get_data_type(&self) -> EnumTextureDataAlignment {
    match self {
      EnumTexture::Texture1D(_, _, _, _, data_type) => *data_type,
      EnumTexture::Texture2D(_, _, _, _, _, data_type) => *data_type,
      EnumTexture::Texture3D(_, _, _, _, _, _, data_type) => *data_type,
      EnumTexture::Texture2DArray(vec) => {
        if vec.is_empty() {
          return vec[0].6;
        }
        return EnumTextureDataAlignment::default();
      }
      EnumTexture::CubeMap(faces) => {
        if faces.is_empty() {
          return faces[0].5;
        }
        return EnumTextureDataAlignment::default();
      }
    }
  }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumTextureHint {
  TextureType(EnumTextureTarget),
  TargetApi(EnumRendererApi),
  TargetDimensions((u32, u32, u32)),
  TargetMipMapLevel(u32),
  TargetFormat(EnumTextureFormat),
  IsHdr(bool),
  DataEncodedWith(EnumTextureDataAlignment),
}

impl EnumTextureHint {
  pub fn get_value(&self) -> &dyn Any {
    let result: &dyn Any;
    match self {
      EnumTextureHint::TextureType(value) => result = value,
      EnumTextureHint::TargetApi(value) => result = value,
      EnumTextureHint::TargetDimensions(value) => result = value,
      EnumTextureHint::TargetMipMapLevel(value) => result = value,
      EnumTextureHint::TargetFormat(value) => result = value,
      EnumTextureHint::IsHdr(value) => result = value,
      EnumTextureHint::DataEncodedWith(value) => result = value,
    };
    return result;
  }
  
  pub fn is(&self, other: &EnumTextureHint) -> bool {
    return std::mem::discriminant(self) == std::mem::discriminant(other);
  }
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum EnumTextureError {
  FileError(String),
  InvalidSize,
  InvalidMipMap,
  InvalidFormat,
  OpenGLError(EnumGlTextureError),
  #[cfg(feature = "vulkan")]
  VulkanError(EnumVkTextureError),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum EnumTextureState {
  Created,
  Sent,
  Deleted,
}

//////////////////////////// DISPLAY /////////////////////////////////////

impl Display for EnumCubeMapFace {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EnumCubeMapFace::Left => write!(f, "Left"),
      EnumCubeMapFace::Right => write!(f, "Right"),
      EnumCubeMapFace::Top => write!(f, "Top"),
      EnumCubeMapFace::Bottom => write!(f, "Bottom"),
      EnumCubeMapFace::Front => write!(f, "Front"),
      EnumCubeMapFace::Back => write!(f, "Back")
    }
  }
}

impl Display for EnumTextureTarget {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EnumTextureTarget::Texture1D => write!(f, "Texture 1D"),
      EnumTextureTarget::Texture1DArray => write!(f, "Texture 1D array"),
      EnumTextureTarget::ProxyTexture1DArray => write!(f, "Proxy texture 1D array"),
      EnumTextureTarget::Texture2D => write!(f, "Texture 2D"),
      EnumTextureTarget::Texture2DMs(sample) => write!(f, "Texture 2D Multisample (x{0})", sample),
      EnumTextureTarget::Texture2DArray => write!(f, "Texture 2D array"),
      EnumTextureTarget::Texture2DArrayMs(sample) => write!(f, "Texture 2D array Multisample (x{0})", sample),
      EnumTextureTarget::Texture3D => write!(f, "Texture 3D"),
      EnumTextureTarget::Texture3DMs(sample_count) => write!(f, "Texture 3D Multisample (x{0})", sample_count),
      EnumTextureTarget::ProxyTexture2D => write!(f, "Proxy texture 2D"),
      EnumTextureTarget::TextureRect => write!(f, "Texture rectangle"),
      EnumTextureTarget::ProxyTextureRect => write!(f, "Proxy texture rectangle"),
      EnumTextureTarget::TextureCubeMap => write!(f, "Texture cube map"),
      EnumTextureTarget::TextureCubeMapArray => write!(f, "Texture cube map array"),
      EnumTextureTarget::ProxyTextureCubeMap => write!(f, "Proxy texture cube map"),
      EnumTextureTarget::DepthStencil => write!(f, "Depth stencil"),
      EnumTextureTarget::ColorAttachment(number) => write!(f, "Color attachment {0}", number),
    }
  }
}

impl Display for EnumTextureDataAlignment {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EnumTextureDataAlignment::UnsignedByte => write!(f, "Unsigned byte (8 bits)"),
      EnumTextureDataAlignment::UnsignedByte233Reverse => write!(f, "Unsigned byte 2_3_3 reverse (8 bits)"),
      EnumTextureDataAlignment::UnsignedByte332 => write!(f, "Unsigned byte 3_3_2 (8 bits)"),
      EnumTextureDataAlignment::Byte => write!(f, "Byte"),
      EnumTextureDataAlignment::UnsignedShort => write!(f, "Unsigned short (32 bits)"),
      EnumTextureDataAlignment::UnsignedShort565 => write!(f, "Unsigned short 5_6_5 (32 bits)"),
      EnumTextureDataAlignment::UnsignedShort565Reverse => write!(f, "Unsigned short 5_6_5 reverse (32 bits)"),
      EnumTextureDataAlignment::UnsignedShort4444 => write!(f, "Unsigned short 4_4_4_4 (32 bits)"),
      EnumTextureDataAlignment::UnsignedShort4444Reverse => write!(f, "Unsigned short 4_4_4_4 reverse (32 bits)"),
      EnumTextureDataAlignment::UnsignedShort5551 => write!(f, "Unsigned short 5_5_5_1 (32 bits)"),
      EnumTextureDataAlignment::UnsignedShort1555Reverse => write!(f, "Unsigned short 1_5_5_5 reverse (32 bits)"),
      EnumTextureDataAlignment::Short => write!(f, "Short (32 bits)"),
      EnumTextureDataAlignment::UnsignedInt => write!(f, "Unsigned int (32 bits)"),
      EnumTextureDataAlignment::UnsignedInt8888 => write!(f, "Unsigned int 8_8_8_8 (32 bits)"),
      EnumTextureDataAlignment::UnsignedInt8888Reverse => write!(f, "Unsigned int 8_8_8_8 reverse (32 bits)"),
      EnumTextureDataAlignment::UnsignedInt10_10_10_2 => write!(f, "Unsigned int 10_10_10_2 (32 bits)"),
      EnumTextureDataAlignment::UnsignedInt2_10_10_10Reverse => write!(f, "Unsigned int 2_10_10_10 reverse (32 bits)"),
      EnumTextureDataAlignment::Int => write!(f, "Int"),
      EnumTextureDataAlignment::Float => write!(f, "Float"),
    }
  }
}

impl Display for EnumTextureFormat {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EnumTextureFormat::Red => write!(f, "Red"),
      EnumTextureFormat::Rg => write!(f, "Rg"),
      EnumTextureFormat::Rgb => write!(f, "Rgb"),
      EnumTextureFormat::Bgr => write!(f, "Bgr"),
      EnumTextureFormat::Rgba => write!(f, "Rgba"),
      EnumTextureFormat::Bgra => write!(f, "Bgra"),
    }
  }
}

impl Display for EnumTexture {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return match self {
      EnumTexture::Texture1D(target, level, format, width,
        tex_type) => {
        write!(f, "{1}:\n{0:115}Mipmap level: {2}\n{0:115}Format: {3}\n{0:115}Dimensions: \
        (x: {4})\n{0:115}Data alignment: {5}", "", target, level, format, width, tex_type)
      }
      EnumTexture::Texture2D(target, level, format, width, height,
        tex_type) => {
        write!(f, "{1}:\n{0:115}Mipmap level: {2}\n{0:115}Format: {3}\n{0:115}Dimensions: \
        (x: {4}, y: {5})\n{0:115}Data alignment: {6}", "", target, level, format, width, height, tex_type)
      }
      EnumTexture::Texture2DArray(textures) => {
        write!(f, "Texture 2D Array:")?;
        
        for texture in textures {
          write!(f, "\n{0:115}{1}:\n{0:117}Mipmap level: {2}\n{0:117}Format: {3}\n{0:117}Dimensions: \
        (x: {4}, y: {5}, z: {6})\n{0:117}Data alignment: {7}", "", texture.0, texture.1, texture.2, texture.3, texture.4, texture.5,
            texture.6)?
        }
        Ok(())
      }
      EnumTexture::Texture3D(target, level, format, width, height,
        depth, tex_type) => {
        write!(f, "{1}:\n{0:115}Mipmap level: {2}\n{0:115}Format: {3}\n{0:115}Dimensions: \
        (x: {4}, y: {5}, z: {6})\n{0:115}Data alignment: {7}", "", target, level, format, width, height, depth, tex_type)
      }
      EnumTexture::CubeMap(faces) => {
        write!(f, "CubeMap:")?;
        
        for face in faces {
          write!(f, "\n{0:115}Face: {1}:\n{0:117}Mipmap level: {2}\n{0:117}Format: {3}\n{0:117}\
          Dimensions: (x: {4}, y: {5})\n{0:117}Data alignment: {6}", "", face.0, face.1, face.2, face.3, face.4, face.5)?
        }
        Ok(())
      }
    };
  }
}

pub(crate) trait TraitTexture {
  fn apply(&mut self) -> Result<(), EnumRendererError>;
  fn clear(&mut self) -> Result<(), EnumRendererError>;
  fn free(&mut self) -> Result<(), EnumRendererError>;
}

#[allow(unused)]
pub struct Texture {
  m_state: EnumTextureState,
  m_api: Box<dyn TraitTexture>,
  m_hints: Vec<EnumTextureHint>,
}

impl Texture {
  pub fn default() -> Self {
    let mut hints = Vec::with_capacity(4);
    
    hints.push(EnumTextureHint::TargetApi(EnumRendererApi::OpenGL));
    hints.push(EnumTextureHint::IsHdr(false));
    hints.push(EnumTextureHint::TargetFormat(EnumTextureFormat::default()));
    hints.push(EnumTextureHint::TargetMipMapLevel(0));
    hints.push(EnumTextureHint::DataEncodedWith(EnumTextureDataAlignment::default()));
    
    return Self {
      m_state: EnumTextureState::Created,
      m_api: Box::new(VkTexture::<u8>::default()),
      m_hints: hints,
    };
  }
  pub fn new() -> Self {
    return Self {
      m_state: EnumTextureState::Created,
      m_api: Box::new(GlTexture::<u8>::default()),
      m_hints: vec![],
    };
  }
  
  pub fn hint(&mut self, hint: EnumTextureHint) {
    if let Some(position) = self.m_hints.iter().position(|h| h.is(&hint)) {
      self.m_hints.remove(position);
    }
    self.m_hints.push(hint);
  }
  
  pub fn clear_hints(&mut self) {
    self.m_hints.clear();
  }
  
  pub fn apply(&mut self, file_name: &str) -> Result<(), EnumRendererError> {
    let file_loaded = stb_image::image::load(file_name);
    
    let mut texture_data: (EnumTexture, Image<u8>) = (EnumTexture::default(), Image {
      width: 0,
      height: 0,
      depth: 0,
      data: vec![],
    });
    
    // Init with default values if case no hints were specified.
    let mut texture_dimensions = (0, 0, 0);
    let mut texture_api = EnumRendererApi::default();
    let mut texture_target = EnumTextureTarget::default();
    let mut texture_mipmap = 0;
    let mut texture_data_type = EnumTextureDataAlignment::default();
    let mut texture_format = EnumTextureFormat::default();
    let mut texture_hdr = false;
    
    // Toggle all provided hints before sending it off to api.
    for hint in self.m_hints.iter() {
      match *hint {
        EnumTextureHint::TextureType(target) => texture_target = target,
        EnumTextureHint::TargetApi(api) => texture_api = api,
        EnumTextureHint::TargetDimensions(dimensions) => texture_dimensions = dimensions,
        EnumTextureHint::TargetMipMapLevel(mipmap) => texture_mipmap = mipmap,
        EnumTextureHint::TargetFormat(format) => texture_format = format,
        EnumTextureHint::DataEncodedWith(data_type) => texture_data_type = data_type,
        EnumTextureHint::IsHdr(bool) => texture_hdr = bool,
      }
    }
    
    match file_loaded {
      LoadResult::Error(message) => {
        log!(EnumLogColor::Red, "ERROR", "[Texture] -->\t Cannot load texture from file {0}, Error => {1}", file_name, message);
        return Err(EnumRendererError::from(EnumTextureError::FileError(message)));
      }
      LoadResult::ImageU8(data) => {
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
        
        self.m_state = EnumTextureState::Sent;
        if texture_api == EnumRendererApi::OpenGL {
          self.m_api = Box::new(GlTexture::new(texture_data.0, texture_data.1));
          return self.m_api.apply();
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
          log!(EnumLogColor::Red, "ERROR", "[Texture] -->\t Cannot crate Vulkan texture, vulkan feature not enabled!");
          return Err(EnumTextureError::VulkanError);
        }
        
        #[cfg(feature = "vulkan")]
        {
          self.m_api = Box::new(VkTexture::new(texture_data.0, texture_data.1));
          return self.m_api.apply();
        }
      }
      LoadResult::ImageF32(_data) => {
        if !texture_hdr {
          log!(EnumLogColor::Red, "ERROR", "[Texture] -->\t Cannot load texture {0:?} as HDR, texture not HDR!", texture_data.0);
          return Err(EnumRendererError::TextureError(EnumTextureError::InvalidFormat));
        }
        
        todo!()
      }
    }
  }
  
  pub fn free(&mut self) -> Result<(), EnumRendererError> {
    self.m_api.free()?;
    self.m_state = EnumTextureState::Deleted;
    return Ok(());
  }
}