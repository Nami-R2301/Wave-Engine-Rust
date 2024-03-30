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
use stb_image::image::{Image, LoadResult};
use crate::graphics::open_gl::texture::{EnumGlTextureError, GlTexture};
use crate::graphics::renderer::{EnumRendererApi, EnumRendererError};
#[cfg(feature = "vulkan")]
use crate::graphics::vulkan::texture::VkTexture;
use crate::utils::macros::logger::*;
use crate::Engine;
use crate::graphics::vulkan::texture::EnumVkTextureError;

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
  ProxyTexture2D,
  TextureRect,
  ProxyTextureRect,
  TextureCubeMap,
  TextureCubeMapArray,
  ProxyTextureCubeMap,
  DepthStencil,
  ColorAttachment(u32),
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

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EnumCubeMapFace {
  Left,
  Right,
  Top,
  Bottom,
  Front,
  Back,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EnumTextureDataType {
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EnumTextureType {
  Texture2D(u32, EnumTextureFormat, u32, u32, EnumTextureDataType),
  Texture2DMs(u32, EnumTextureFormat, u32, u32, EnumTextureDataType, u32),
  Texture3D(u32, EnumTextureFormat, u32, u32, u32, EnumTextureDataType),
  Texture2DArray(Vec<(u32, EnumTextureFormat, u32, u32, u32, EnumTextureDataType)>),
  Texture2DArrayMs(Vec<(u32, EnumTextureFormat, u32, u32, u32, EnumTextureDataType, u32)>),
  CubeMap([(EnumCubeMapFace, u32, EnumTextureFormat, u32, u32, EnumTextureDataType); 6]),
}

impl EnumTextureType {
  pub(crate) fn get_target(&self) -> EnumTextureTarget {
    match self {
      EnumTextureType::Texture2D(_, _, _, _, _) => EnumTextureTarget::Texture2D,
      EnumTextureType::Texture3D(_, _, _, _, _, _) => EnumTextureTarget::Texture3D,
      EnumTextureType::Texture2DArray(_) => EnumTextureTarget::Texture2DArray,
      EnumTextureType::CubeMap(_) => EnumTextureTarget::TextureCubeMap,
      EnumTextureType::Texture2DMs(_, _, _, _, _, sample_count) => EnumTextureTarget::Texture2DMs(*sample_count),
      EnumTextureType::Texture2DArrayMs(vec) => {
        if vec.is_empty() {
          panic!("Cannot retrieve texture array format, textureArray2D is empty!");
        }
        EnumTextureTarget::Texture2DArrayMs(vec[0].6)
      }
    }
  }
  
  pub(crate) fn get_format(&self) -> EnumTextureFormat {
    match self {
      EnumTextureType::Texture2D(_, format, _, _, _) => *format,
      EnumTextureType::Texture3D(_, format, _, _, _, _) => *format,
      EnumTextureType::Texture2DArray(vec) => {
        if vec.is_empty() {
          panic!("Cannot retrieve texture array format, textureArray2D is empty!");
        }
        vec[0].1
      },
      EnumTextureType::CubeMap(faces) => {
        if faces.is_empty() {
          panic!("Cannot retrieve cube map format, CubeMap is empty!");
        }
        faces[0].2
      }
      EnumTextureType::Texture2DMs(_, format, _, _, _, _) => *format,
      EnumTextureType::Texture2DArrayMs(vec) => {
        if vec.is_empty() {
          panic!("Cannot retrieve texture array format, textureArray2D is empty!");
        }
        vec[0].1
      }
    }
  }
  
  pub(crate) fn get_mipmap_level(&self) -> u32 {
    match self {
      EnumTextureType::Texture2D(mipmap, _, _, _, _) => *mipmap,
      EnumTextureType::Texture3D(mipmap, _, _, _, _, _) => *mipmap,
      EnumTextureType::Texture2DArray(vec) => {
        if vec.is_empty() {
          panic!("Cannot retrieve texture array data type, textureArray2D is empty!");
        }
        vec[0].0
      },
      EnumTextureType::CubeMap(faces) => {
        if faces.is_empty() {
          panic!("Cannot retrieve cube map data type, CubeMap is empty!");
        }
        faces[0].1
      }
      EnumTextureType::Texture2DMs(mipmap, _, _, _, _, _) => *mipmap,
      EnumTextureType::Texture2DArrayMs(vec) => {
        if vec.is_empty() {
          panic!("Cannot retrieve texture array data type, textureArray2D is empty!");
        }
        vec[0].0
      }
    }
  }
  
  pub(crate) fn get_data_type(&self) -> EnumTextureDataType {
    match self {
      EnumTextureType::Texture2D(_, _, _, _, data_type) => *data_type,
      EnumTextureType::Texture3D(_, _, _, _, _, data_type) => *data_type,
      EnumTextureType::Texture2DArray(vec) => {
        if vec.is_empty() {
          panic!("Cannot retrieve texture array data type, textureArray2D is empty!");
        }
        vec[0].5
      },
      EnumTextureType::CubeMap(faces) => {
        if faces.is_empty() {
          panic!("Cannot retrieve cube map data type, CubeMap is empty!");
        }
        faces[0].5
      }
      EnumTextureType::Texture2DMs(_, _, _, _, data_type, _) => *data_type,
      EnumTextureType::Texture2DArrayMs(vec) => {
        if vec.is_empty() {
          panic!("Cannot retrieve texture array data type, textureArray2D is empty!");
        }
        vec[0].5
      }
    }
  }
}



#[derive(Debug, Copy, Clone, Hash)]
pub enum EnumTextureHint {
  TextureType(std::mem::Discriminant<EnumTextureType>),
  TargetApi(EnumRendererApi),
  TargetDimensions(u32, u32),
  TargetMipMapLevel(u32),
  TargetFormat(EnumTextureFormat),
  MaxTextureSize(usize),
  IsHdr(bool),
  Multisample(Option<u32>),
  ColorChannels(EnumTextureFormat),
  DataEncodedWith(EnumTextureDataType)
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
  Deleted
}

//////////////////////////// EQ /////////////////////////////////////

impl PartialEq for EnumTextureHint {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (EnumTextureHint::TargetApi(_), EnumTextureHint::TargetApi(_)) => true,
      (EnumTextureHint::TargetDimensions(_, _), EnumTextureHint::TargetDimensions(_, _)) => true,
      (EnumTextureHint::TargetMipMapLevel(_), EnumTextureHint::TargetMipMapLevel(_)) => true,
      (EnumTextureHint::TargetFormat(_), EnumTextureHint::TargetFormat(_)) => true,
      (EnumTextureHint::MaxTextureSize(_), EnumTextureHint::MaxTextureSize(_)) => true,
      (EnumTextureHint::IsHdr(_), EnumTextureHint::IsHdr(_)) => true,
      _ => false
    }
  }
}

impl Eq for EnumTextureHint {}

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
      EnumTextureTarget::Texture1D => write!(f, "Texture1D"),
      EnumTextureTarget::Texture1DArray => write!(f, "Texture1DArray"),
      EnumTextureTarget::ProxyTexture1DArray => write!(f, "ProxyTexture1DArray"),
      EnumTextureTarget::Texture2D => write!(f, "Texture2D"),
      EnumTextureTarget::Texture2DMs(sample) => write!(f, "Texture2DMs {0}", sample),
      EnumTextureTarget::Texture2DArray => write!(f, "Texture2DArray"),
      EnumTextureTarget::Texture2DArrayMs(sample) => write!(f, "Texture2DArrayMs {0}", sample),
      EnumTextureTarget::Texture3D => write!(f, "Texture3D"),
      EnumTextureTarget::ProxyTexture2D => write!(f, "ProxyTexture2D"),
      EnumTextureTarget::TextureRect => write!(f, "TextureRect"),
      EnumTextureTarget::ProxyTextureRect => write!(f, "ProxyTextureRect"),
      EnumTextureTarget::TextureCubeMap => write!(f, "TextureCubeMap"),
      EnumTextureTarget::TextureCubeMapArray => write!(f, "TextureCubeMapArray"),
      EnumTextureTarget::ProxyTextureCubeMap => write!(f, "ProxyTextureCubeMap"),
      EnumTextureTarget::DepthStencil => write!(f, "DepthStencil"),
      EnumTextureTarget::ColorAttachment(number) => write!(f, "ColorAttachment {0}", number),
    }
  }
}

impl Display for EnumTextureDataType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EnumTextureDataType::UnsignedByte => write!(f, "UnsignedByte"),
      EnumTextureDataType::UnsignedByte233Reverse => write!(f, "UnsignedByte233Reverse"),
      EnumTextureDataType::UnsignedByte332 => write!(f, "UnsignedByte332"),
      EnumTextureDataType::Byte => write!(f, "Byte"),
      EnumTextureDataType::UnsignedShort => write!(f, "UnsignedShort"),
      EnumTextureDataType::UnsignedShort565 => write!(f, "UnsignedShort565"),
      EnumTextureDataType::UnsignedShort565Reverse => write!(f, "UnsignedShort565Reverse"),
      EnumTextureDataType::UnsignedShort4444 => write!(f, "UnsignedShort4444"),
      EnumTextureDataType::UnsignedShort4444Reverse => write!(f, "UnsignedShort4444Reverse"),
      EnumTextureDataType::UnsignedShort5551 => write!(f, "UnsignedShort5551"),
      EnumTextureDataType::UnsignedShort1555Reverse => write!(f, "UnsignedShort1555Reverse"),
      EnumTextureDataType::Short => write!(f, "Short"),
      EnumTextureDataType::UnsignedInt => write!(f, "UnsignedInt"),
      EnumTextureDataType::UnsignedInt8888 => write!(f, "UnsignedInt8888"),
      EnumTextureDataType::UnsignedInt8888Reverse => write!(f, "UnsignedInt8888Reverse"),
      EnumTextureDataType::UnsignedInt10_10_10_2 => write!(f, "UnsignedInt10_10_10_2"),
      EnumTextureDataType::UnsignedInt2_10_10_10Reverse => write!(f, "UnsignedInt2_10_10_10Reverse"),
      EnumTextureDataType::Int => write!(f, "Int"),
      EnumTextureDataType::Float => write!(f, "Float"),
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

impl Display for EnumTextureType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return match self {
      EnumTextureType::Texture2D(level, format, width, height,
        tex_type) => {
        write!(f, "Texture 2D:\n{0:115}Mipmap level: {1}\n{0:115}Format: {2}\n{0:115}Dimensions: \
        (x: {3}, y: {4})\n{0:115}Type: {5}", "", level, format, width, height, tex_type)
      }
      EnumTextureType::Texture2DArray(textures) => {
        write!(f, "Texture 2D Array:")?;
        
        for texture in textures {
          write!(f, "\n{0:115}Texture 2D:\n{0:117}Mipmap level: {1}\n{0:117}Format: {2}\n{0:117}Dimensions: \
        (x: {3}, y: {4}, z: {5})\n{0:117}Type: {6}", "", texture.0, texture.1, texture.2, texture.3, texture.4, texture.5)?
        }
        Ok(())
      }
      EnumTextureType::Texture3D(level, format, width, height,
        depth, tex_type) => {
        write!(f, "Texture 3D:\n{0:115}Mipmap level: {1}\n{0:115}Format: {2}\n{0:115}Dimensions: \
        (x: {3}, y: {4}, z: {5})\n{0:115}Type: {6}", "", level, format, width, height, depth, tex_type)
      }
      EnumTextureType::CubeMap(faces) => {
        write!(f, "CubeMap:")?;
        
        for face in faces {
          write!(f, "\n{0:115}Face: {1}:\n{0:117}Mipmap level: {2}\n{0:117}Format: {3}\n{0:117}\
          Dimensions: (x: {4}, y: {5})\n{0:117}Type: {6}", "", face.0, face.1, face.2, face.3, face.4, face.5)?
        }
        Ok(())
      }
      EnumTextureType::Texture2DMs(level, format, width, height,
        tex_type, sample_count) => {
        write!(f, "Texture 2D (Ms):\n{0:115}Mipmap level: {1}\n{0:115}Format: {2}\n{0:115}Dimensions: \
        (x: {3}, y: {4})\n{0:115}Type: {5}\n{0:115}Sample count: {6}", "", level, format, width, height, tex_type, sample_count)
      }
      EnumTextureType::Texture2DArrayMs(textures) => {
        write!(f, "Texture 2D Array (Ms):")?;
        
        for texture in textures {
          write!(f, "\n{0:115}Texture 2D (Ms):\n{0:117}Mipmap level: {1}\n{0:117}Format: {2}\n{0:117}Dimensions: \
        (x: {3}, y: {4}, z: {5})\n{0:117}Type: {6}\n{0:115}Sample count: {7}", "", texture.0, texture.1, texture.2, texture.3,
            texture.4, texture.5, texture.6)?
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
  m_type: EnumTextureType,
  m_api: Box<dyn TraitTexture>,
  m_hints: Vec<EnumTextureHint>,
}

impl Texture {
  pub fn default() -> Self {
    let mut hints = Vec::with_capacity(4);
    
    hints.push(EnumTextureHint::TargetFormat(EnumTextureFormat::Rgba));
    hints.push(EnumTextureHint::TargetMipMapLevel(0));
    
    return Self {
      m_state: EnumTextureState::Created,
      m_type: EnumTextureType::Texture2D(0, EnumTextureFormat::Rgba, 0, 0, EnumTextureDataType::UnsignedByte),
      m_api: Box::new(VkTexture::<u8>::default()),
      m_hints: hints,
    };
  }
  pub fn new(texture_type: EnumTextureType) -> Self {
    return Self {
      m_state: EnumTextureState::Created,
      m_type: texture_type,
      m_api: Box::new(GlTexture::<u8>::default()),
      m_hints: vec![],
    };
  }
  
  pub fn hint(&mut self, hint: EnumTextureHint) {
    if let Some(position) = self.m_hints.iter().position(|h| *h == hint) {
      self.m_hints.remove(position);
    }
    self.m_hints.push(hint);
  }
  
  pub fn clear_hints(&mut self) {
    self.m_hints.clear();
  }
  
  pub fn apply(&mut self, file_name: &str) -> Result<(), EnumRendererError> {
    let file_loaded = stb_image::image::load(file_name);
    
    let texture_data: (EnumTextureType, Image<u8>);
    match file_loaded {
      LoadResult::Error(message) => {
        log!(EnumLogColor::Red, "ERROR", "[Texture] -->\t Cannot load texture from file {0}, Error => {1}", file_name, message);
        return Err(EnumRendererError::from(EnumTextureError::FileError(message)));
      }
      LoadResult::ImageU8(data) => {
        let mut new_format = EnumTextureFormat::Rgba;
        match data.depth {
          1 => new_format = EnumTextureFormat::Red,
          2 => new_format = EnumTextureFormat::Rg,
          3 => new_format = EnumTextureFormat::Rgb,
          4 => new_format = EnumTextureFormat::Rgba,
          _ => {}
        }
        
        match &self.m_type {
          EnumTextureType::Texture2D(mipmap, _, _, _, tex_type) => {
            texture_data = (EnumTextureType::Texture2D(*mipmap, new_format, data.width as u32, data.height as u32, *tex_type), data);
          }
          EnumTextureType::Texture3D(mipmap, _, _, _, _, tex_type) => {
            texture_data = (EnumTextureType::Texture3D(*mipmap, new_format, data.width as u32, data.height as u32, data.depth as u32,
              *tex_type), data);
          }
          EnumTextureType::Texture2DMs(mipmap, _, _, _, tex_type, sample_count) => {
            texture_data = (EnumTextureType::Texture2DMs(*mipmap, new_format, data.width as u32, data.height as u32, *tex_type, *sample_count),
              data);
          }
          _ => todo!()
        }
      }
      _ => todo!()
    }
    
    self.m_state = EnumTextureState::Sent;
    if self.m_hints.iter().any(|h| *h == EnumTextureHint::TargetApi(EnumRendererApi::OpenGL)) {
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
  
  pub fn free(&mut self) -> Result<(), EnumRendererError> {
    self.m_api.free()?;
    self.m_state = EnumTextureState::Deleted;
    return Ok(());
  }
}