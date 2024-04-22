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

use std::fmt::{Display, Formatter};

use crate::{TraitApply, TraitFree, TraitHint};
use crate::graphics::open_gl::texture::{EnumGlTextureError, GlTexture};
use crate::graphics::renderer::{EnumRendererApi, EnumRendererError};
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan::texture::EnumVkTextureError;
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan::texture::VkTexture;
use crate::Engine;
#[cfg(feature = "vulkan")]
use crate::utils::macros::logger::*;
use crate::utils::texture_loader::{TextureInfo, TextureLoader};
use crate::window::EnumWindowState;

static mut S_TEXTURE_ID_COUNTER: u64 = 0;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EnumTextureHint {
  BatchTextures(bool)
}

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

#[allow(unused)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum EnumTextureInfo {
  Texture1D(EnumTextureTarget, u32, EnumTextureFormat, u32, EnumTextureDataAlignment, u16),
  Texture2D(EnumTextureTarget, u32, EnumTextureFormat, u32, u32, EnumTextureDataAlignment, u16),
  Texture3D(EnumTextureTarget, u32, EnumTextureFormat, u32, u32, u32, EnumTextureDataAlignment, u16),
  TextureArray(Vec<(EnumTextureInfo, Vec<u8>)>),
  CubeMap([(EnumCubeMapFace, u32, EnumTextureFormat, u32, u32, EnumTextureDataAlignment, u16); 6]),
}

impl Default for EnumTextureInfo {
  fn default() -> Self {
    return EnumTextureInfo::Texture2D(EnumTextureTarget::default(), 0, EnumTextureFormat::default(), 0, 0, EnumTextureDataAlignment::default(), 6);
  }
}

impl EnumTextureInfo {
  pub(crate) fn get_target(&self) -> EnumTextureTarget {
    match self {
      EnumTextureInfo::Texture1D(target, _, _, _, _, _) => *target,
      EnumTextureInfo::Texture2D(target, _, _, _, _, _, _) => *target,
      EnumTextureInfo::Texture3D(target, _, _, _, _, _, _, _) => *target,
      EnumTextureInfo::TextureArray(_) => EnumTextureTarget::Texture2DArray,
      EnumTextureInfo::CubeMap(_) => EnumTextureTarget::TextureCubeMap,
    }
  }
  
  pub(crate) fn get_format(&self) -> EnumTextureFormat {
    match self {
      EnumTextureInfo::Texture1D(_, _, format, _, _, _) => *format,
      EnumTextureInfo::Texture2D(_, _, format, _, _, _, _) => *format,
      EnumTextureInfo::Texture3D(_, _, format, _, _, _, _, _) => *format,
      EnumTextureInfo::TextureArray(vec) => {
        if !vec.is_empty() {
          return vec[0].0.get_format();
        }
        return EnumTextureFormat::default();
      }
      EnumTextureInfo::CubeMap(faces) => {
        if !faces.is_empty() {
          return faces[0].2;
        }
        return EnumTextureFormat::default();
      }
    }
  }
  
  pub(crate) fn get_mipmap_level(&self) -> u32 {
    match self {
      EnumTextureInfo::Texture1D(_, mipmap, _, _, _, _) => *mipmap,
      EnumTextureInfo::Texture2D(_, mipmap, _, _, _, _, _) => *mipmap,
      EnumTextureInfo::Texture3D(_, mipmap, _, _, _, _, _, _) => *mipmap,
      EnumTextureInfo::TextureArray(vec) => {
        if !vec.is_empty() {
          return vec[0].0.get_mipmap_level();
        }
        return 0;
      }
      EnumTextureInfo::CubeMap(faces) => {
        if !faces.is_empty() {
          return faces[0].1;
        }
        return 0;
      }
    }
  }
  
  pub(crate) fn get_width(&self) -> usize {
    return match self {
      EnumTextureInfo::Texture1D(_, _, _, width, _, _) => *width as usize,
      EnumTextureInfo::Texture2D(_, _, _, width, _, _, _) => *width as usize,
      EnumTextureInfo::Texture3D(_, _, _, width, _, _, _, _) => *width as usize,
      EnumTextureInfo::TextureArray(vec) => {
        if let Some((max_depth, _)) = vec.last() {
          return max_depth.get_width();
        }
        return 0;
      }
      EnumTextureInfo::CubeMap(faces) => {
        if let Some(face) = faces.last() {
          return face.3 as usize;
        }
        return 0;
      }
    };
  }
  
  pub(crate) fn get_height(&self) -> usize {
    return match self {
      EnumTextureInfo::Texture1D(_, _, _, _, _, _) => 0,
      EnumTextureInfo::Texture2D(_, _, _, _, height, _, _) => *height as usize,
      EnumTextureInfo::Texture3D(_, _, _, _, height, _, _, _) => *height as usize,
      EnumTextureInfo::TextureArray(vec) => {
        if let Some((max_depth, _)) = vec.last() {
          return max_depth.get_height();
        }
        return 0;
      }
      EnumTextureInfo::CubeMap(faces) => {
        if let Some(face) = faces.last() {
          return face.4 as usize;
        }
        return 0;
      }
    };
  }
  
  pub(crate) fn get_depth(&self) -> u16 {
    return match self {
      EnumTextureInfo::Texture1D(_, _, _, _, _, _) => 0,
      EnumTextureInfo::Texture2D(_, _, _, _, _, _, _) => 0,
      EnumTextureInfo::Texture3D(_, _, _, _, _, depth, _, _) => *depth as u16,
      EnumTextureInfo::TextureArray(vec) => {
        if let Some((max_depth, _)) = vec.last() {
          return max_depth.get_depth();
        }
        return 0;
      }
      EnumTextureInfo::CubeMap(_) => 0,
    };
  }
  
  pub(crate) fn get_data_type(&self) -> EnumTextureDataAlignment {
    match self {
      EnumTextureInfo::Texture1D(_, _, _, _, data_type, _) => *data_type,
      EnumTextureInfo::Texture2D(_, _, _, _, _, data_type, _) => *data_type,
      EnumTextureInfo::Texture3D(_, _, _, _, _, _, data_type, _) => *data_type,
      EnumTextureInfo::TextureArray(vec) => {
        if !vec.is_empty() {
          return vec[0].0.get_data_type();
        }
        return EnumTextureDataAlignment::default();
      }
      EnumTextureInfo::CubeMap(faces) => {
        if !faces.is_empty() {
          return faces[0].5;
        }
        return EnumTextureDataAlignment::default();
      }
    }
  }
  
  pub(crate) fn get_slot(&self) -> u16 {
    return match self {
      EnumTextureInfo::Texture1D(_, _, _, _, _, slot) => *slot,
      EnumTextureInfo::Texture2D(_, _, _, _, _, _, slot) => *slot,
      EnumTextureInfo::Texture3D(_, _, _, _, _, _, _, slot) => *slot,
      EnumTextureInfo::TextureArray(vec) => {
        if !vec.is_empty() {
          return vec[0].0.get_slot();
        }
        return 5;
      }
      EnumTextureInfo::CubeMap(faces) => {
        if !faces.is_empty() {
          return faces[0].6;
        }
        return 5;
      }
    };
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

impl Display for EnumTextureInfo {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return match self {
      EnumTextureInfo::Texture1D(target, level, format, width,
        tex_type, tex_slot) => {
        write!(f, "{1}:\n{0:115}Mipmap level: {2}\n{0:115}Format: {3}\n{0:115}Dimensions: \
        (x: {4})\n{0:115}Data alignment: {5}\n{0:115}Slot: {6}", "", target, level, format, width, tex_type, tex_slot)
      }
      EnumTextureInfo::Texture2D(target, level, format, width, height,
        tex_type, tex_slot) => {
        write!(f, "{1}:\n{0:115}Mipmap level: {2}\n{0:115}Format: {3}\n{0:115}Dimensions: \
        (x: {4}, y: {5})\n{0:115}Data alignment: {6}\n{0:115}Slot: {7}", "", target, level, format, width, height, tex_type, tex_slot)
      }
      EnumTextureInfo::TextureArray(vec) => {
        write!(f, "Texture 2D Array:")?;
        
        for (texture, _data) in vec {
          write!(f, "\n{0:115}Texture 2D:\n{0:117}Mipmap level: {1}\n{0:117}Format: {2}\n{0:117}Dimensions: \
        (x: {3}, y: {4}, depth: {5})\n{0:117}Data alignment: {6}\n{0:117}Slot: {7}", "", texture.get_mipmap_level(),
            texture.get_format(), texture.get_width(), texture.get_height(), texture.get_depth(), texture.get_data_type(), texture.get_slot())?;
        }
        Ok(())
      }
      EnumTextureInfo::Texture3D(target, level, format, width, height,
        depth, tex_type, tex_slot) => {
        write!(f, "{1}:\n{0:115}Mipmap level: {2}\n{0:115}Format: {3}\n{0:115}Dimensions: \
        (x: {4}, y: {5}, z: {6})\n{0:115}Data alignment: {7}\n{0:115}Slot: {8}", "", target, level, format, width, height, depth,
          tex_type, tex_slot)
      }
      EnumTextureInfo::CubeMap(faces) => {
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
  fn get_depth(&self) -> u16;
  fn get_size(&self) -> (usize, usize);
  fn set_depth(&mut self, depth: u16);
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

impl TraitHint<EnumTextureHint> for Texture {
  fn set_hint(&mut self, hint: EnumTextureHint) {
    if let Some(position) = self.m_hints.iter().position(|h| h == &hint) {
      self.m_hints.remove(position);
    }
    self.m_hints.push(hint);
  }
  
  fn reset_hints(&mut self) {
    self.m_hints.clear();
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
  pub fn new<T: 'static>(api_chosen: EnumRendererApi, texture_info: TextureInfo<T>) -> Self {
    let new_uuid = unsafe { S_TEXTURE_ID_COUNTER };
    unsafe { S_TEXTURE_ID_COUNTER += 1 };
    
    return match api_chosen {
      EnumRendererApi::OpenGL => {
        Self {
          m_uuid: new_uuid,
          m_state: EnumTextureState::Created,
          m_api: Box::new(GlTexture::<T>::new(texture_info)),
          m_hints: vec![],
        }
      }
      EnumRendererApi::Vulkan => {
        Self {
          m_uuid: new_uuid,
          m_state: EnumTextureState::Created,
          m_api: Box::new(VkTexture::<T>::new(texture_info)),
          m_hints: vec![],
        }
      }
    };
  }
  
  #[allow(unused)]
  pub(crate) fn get_depth(&self) -> u16 {
    return self.m_api.get_depth();
  }
  
  #[allow(unused)]
  pub(crate) fn get_size(&self) -> (usize, usize) {
    return self.m_api.get_size();
  }
  
  #[allow(unused)]
  pub(crate) fn set_depth(&mut self, depth: u16) {
    self.m_api.set_depth(depth);
  }
}

impl Default for Texture {
  fn default() -> Self {
    let new_uuid = unsafe { S_TEXTURE_ID_COUNTER };
    unsafe { S_TEXTURE_ID_COUNTER += 1 };
    
    let texture_loader_preset = TextureLoader::new();
    let texture_info = texture_loader_preset.load("res/textures/default.png")
      .expect("Cannot retrieve default texture at 'res/textures/default.png'!");
    
    return Self {
      m_uuid: new_uuid,
      m_state: EnumTextureState::Created,
      m_api: Box::new(GlTexture::<u8>::new(texture_info)),
      m_hints: vec![],
    };
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    log!(EnumLogColor::Purple, "INFO", "[Texture] -->\t Dropping texture {0} successfully", self.m_uuid);
    if Engine::get_active_window().m_state != EnumWindowState::Closed {
      match self.free() {
        Err(err) => {
          log!(EnumLogColor::Red, "ERROR", "[Texture] -->\t Error while freeing texture {0}, Error => {1}", self.m_uuid, err);
        }
        _ => {}
      }
    }
    log!(EnumLogColor::Green, "INFO", "[Texture] -->\t Dropped texture {0} successfully", self.m_uuid);
  }
}

#[allow(unused)]
pub struct TextureArray {
  pub(crate) m_textures: Vec<TextureInfo<u8>>,
  pub(crate) m_max_depth: u16,
  m_api: EnumRendererApi
}

impl TextureArray {
  pub fn new(api_chosen: EnumRendererApi, textures_info: Vec<TextureInfo<u8>>) -> Self {
    let mut to_texture_array: Vec<TextureInfo<u8>> = Vec::with_capacity(textures_info.len());
    let mut depth_counter: u16 = 0;
    
    for texture_info in textures_info.into_iter() {
      let new_texture_info = TextureInfo {
        m_type: EnumTextureInfo::Texture3D(texture_info.m_type.get_target(), texture_info.m_type.get_mipmap_level(),
          texture_info.m_type.get_format(), texture_info.m_type.get_width() as u32, texture_info.m_type.get_height() as u32,
          depth_counter as u32, texture_info.m_type.get_data_type(), texture_info.m_type.get_slot()),
        m_data: texture_info.m_data,
      };
      to_texture_array.push(new_texture_info);
      
      depth_counter += 1;
    }
    return Self {
      m_textures: to_texture_array,
      m_max_depth: depth_counter,
      m_api: api_chosen
    };
  }
  
  pub fn get_current_depth(&self) -> u16 {
    return self.m_max_depth;
  }
  
  pub fn append(&mut self, textures_info: Vec<TextureInfo<u8>>) {
    let mut to_texture_array: Vec<TextureInfo<u8>> = Vec::with_capacity(textures_info.len());
    let mut depth_counter: u16 = self.m_max_depth;
    
    for texture_info in textures_info.into_iter() {
      let new_texture_info = TextureInfo {
        m_type: EnumTextureInfo::Texture3D(texture_info.m_type.get_target(), texture_info.m_type.get_mipmap_level(),
          texture_info.m_type.get_format(), texture_info.m_type.get_width() as u32, texture_info.m_type.get_height() as u32,
          depth_counter as u32, texture_info.m_type.get_data_type(), texture_info.m_type.get_slot()),
        m_data: texture_info.m_data,
      };
      to_texture_array.push(new_texture_info);
      
      depth_counter += 1;
    }
    self.m_textures.append(&mut to_texture_array);
    self.m_max_depth = depth_counter;
  }
  
  pub fn as_texture(&self) -> Texture {
    let mut converted: Vec<(EnumTextureInfo, Vec<u8>)> = Vec::with_capacity(self.m_max_depth as usize);
    let texture_width = self.m_textures[0].m_type.get_width();
    let texture_height = self.m_textures[0].m_type.get_height();
    
    for texture_info in self.m_textures.iter() {
      converted.push((texture_info.get_type(), texture_info.get_data()));
    }
    
    let texture_info: TextureInfo<u8> = TextureInfo {
      m_type: EnumTextureInfo::TextureArray(converted),
      m_data: stb_image::image::Image {
        width: texture_width,
        height: texture_height,
        depth: self.m_max_depth as usize,
        data: vec![],
      }
    };
    
    return Texture::new(self.m_api, texture_info);
  }
}