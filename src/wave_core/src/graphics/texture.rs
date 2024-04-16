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

use stb_image;

use crate::{TraitApply, TraitFree, TraitHint};
use crate::graphics::open_gl::texture::{EnumGlTextureError, GlTexture};
use crate::graphics::renderer::{EnumRendererApi, EnumRendererError};
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan::texture::EnumVkTextureError;
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan::texture::VkTexture;
#[cfg(feature = "debug")]
use crate::Engine;
use crate::utils::macros::logger::*;

static mut S_TEXTURE_ID_COUNTER: u64 = 0;

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
  FlipPixels(bool),
  BindLess(bool)
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
      EnumTextureHint::FlipPixels(bool) => result = bool,
      EnumTextureHint::BindLess(bool) => result = bool
    };
    return result;
  }
  
  pub fn is_equivalent(&self, other: &Self) -> bool {
    return std::mem::discriminant(self) == std::mem::discriminant(other);
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnumAtlasTextureHint {
  TextureFormat(EnumTextureFormat),
  AtlasDimensions((usize, usize)),
  OptimizeMemory(bool),
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum EnumTextureError {
  InvalidSize,
  FileError(String),
  InvalidMipMap,
  InvalidFormat,
  OpenGLError(EnumGlTextureError),
  #[cfg(feature = "vulkan")]
  VulkanError(EnumVkTextureError),
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum EnumTextureLoaderError {
  FileError(String),
  InvalidPath(String),
  InvalidSize,
  InvalidFormat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum EnumTextureState {
  Created,
  Sent,
  Deleted,
}

//////////////////////////// DISPLAY /////////////////////////////////////

impl Display for EnumTextureLoaderError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return match self {
      EnumTextureLoaderError::InvalidPath(_) => write!(f, "Invalid path"),
      EnumTextureLoaderError::InvalidSize => write!(f, "Invalid size"),
      EnumTextureLoaderError::InvalidFormat => write!(f, "Invalid format"),
      EnumTextureLoaderError::FileError(err) => write!(f, "Error reading file, Error => {0}", err)
    };
  }
}

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
  fn convert_to(&mut self, format: EnumTextureFormat) -> Result<(), EnumRendererError>;
  fn apply(&mut self) -> Result<(), EnumRendererError>;
  fn clear(&mut self) -> Result<(), EnumRendererError>;
  fn free(&mut self) -> Result<(), EnumRendererError>;
}

#[allow(unused)]
pub struct Texture {
  m_uuid: u64,
  m_state: EnumTextureState,
  m_api: Box<dyn TraitTexture>,
  m_hints: Vec<EnumTextureHint>,
}

impl Default for Texture {
  fn default() -> Self {
    return Self {
      m_uuid: u64::MAX,
      m_state: EnumTextureState::Created,
      m_api: Box::new(GlTexture::<u8>::default()),
      m_hints: vec![EnumTextureHint::TargetApi(Default::default()), EnumTextureHint::IsHdr(false),
        EnumTextureHint::TargetFormat(Default::default()), EnumTextureHint::TargetMipMapLevel(0),
        EnumTextureHint::DataEncodedWith(Default::default())],
    };
  }
}

impl TraitHint<EnumTextureHint> for Texture {
  fn set_hint(&mut self, hint: EnumTextureHint) {
    if let Some(position) = self.m_hints.iter().position(|h| h.is_equivalent(&hint)) {
      self.m_hints.remove(position);
    }
    self.m_hints.push(hint);
  }
  
  fn reset_hints(&mut self) {
    self.m_hints = vec![EnumTextureHint::TargetApi(Default::default()), EnumTextureHint::IsHdr(false),
      EnumTextureHint::TargetFormat(Default::default()), EnumTextureHint::TargetMipMapLevel(0),
      EnumTextureHint::DataEncodedWith(Default::default())];
  }
}

impl TraitFree<EnumRendererError> for Texture {
  fn free(&mut self) -> Result<(), EnumRendererError> {
    if self.m_state == EnumTextureState::Sent {
      self.m_api.free()?;
      self.m_state = EnumTextureState::Deleted;
    }
    return Ok(());
  }
}

impl TraitApply<EnumRendererError> for Texture {
  fn apply(&mut self) -> Result<(), EnumRendererError> {
    if self.m_state == EnumTextureState::Created {
      self.m_api.apply()?;
      self.m_state = EnumTextureState::Sent;
    }
    return Ok(());
  }
}

impl Texture {
  pub(crate) fn new<T: 'static>(hints: Vec<EnumTextureHint>, texture_type: EnumTexture, data: stb_image::image::Image<T>) -> Self {
    let new_uuid = unsafe { S_TEXTURE_ID_COUNTER };
    unsafe { S_TEXTURE_ID_COUNTER += 1 };
    
    if hints.contains(&EnumTextureHint::TargetApi(EnumRendererApi::Vulkan)) {
      return Self {
        m_uuid: new_uuid,
        m_state: EnumTextureState::Created,
        m_api: Box::new(VkTexture::<T>::new(texture_type, data)),
        m_hints: hints,
      };
    }
    
    return Self {
      m_uuid: new_uuid,
      m_state: EnumTextureState::Created,
      m_api: Box::new(GlTexture::<T>::new(texture_type, data)),
      m_hints: hints,
    };
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    log!(EnumLogColor::Purple, "INFO", "[Texture] -->\t Dropping texture {0} successfully", self.m_uuid);
    match self.free() {
      Ok(_) => {
        log!(EnumLogColor::Green, "INFO", "[Texture] -->\t Dropped texture {0} successfully", self.m_uuid);
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Texture] -->\t Error while freeing texture {0}, Error => {1}", self.m_uuid, err);
      }
    }
  }
}