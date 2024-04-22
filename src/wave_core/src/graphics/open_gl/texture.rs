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


use gl::types::{GLint, GLsizei};
use num::Integer;
use stb_image::image::Image;
use crate::check_gl_call;
use crate::graphics::open_gl::renderer::EnumOpenGLError;
use crate::graphics::texture::{EnumTextureDataAlignment, EnumTextureFormat, EnumTextureTarget, EnumTextureInfo, TraitTexture};
use crate::utils::macros::logger::*;
#[cfg(feature = "debug")]
use crate::Engine;
use crate::{S_ENGINE};
use crate::graphics::renderer::EnumRendererError;
use crate::utils::texture_loader::TextureInfo;


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EnumGlTextureError {
  InvalidInternalFormat,
  InternalError(EnumOpenGLError),
}

#[allow(unused)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum EnumGlTextureInternalFormat {
  Rgba32F = gl::RGBA32F,
  Rgba32I = gl::RGBA32I,
  Rgba32Ui = gl::RGBA32UI,
  Rgba16 = gl::RGBA16,
  Rgba16F = gl::RGBA16F,
  Rgba16I = gl::RGBA16I,
  Rgba16Ui = gl::RGBA16UI,
  Rgba8 = gl::RGBA8,
  Rgba8Ui = gl::RGBA8UI,
  SRgba8 = gl::SRGB8_ALPHA8,
  Rgb10A2 = gl::RGB10_A2,
  Rgb10A2Ui = gl::RGB10_A2UI,
  R11FG11FB10F = gl::R11F_G11F_B10F,
  Rgb8 = gl::RGB8,
  Rgb8I = gl::RGB8I,
  Rgb8Ui = gl::RGB8UI,
  R32F = gl::R32F,
  R32I = gl::R32I,
  R32Ui = gl::R32UI,
  R16F = gl::R16F,
  R16I = gl::R16I,
  R16Ui = gl::R16UI,
  R8 = gl::R8,
  R8I = gl::R8I,
  R8ui = gl::R8UI,
  Rgba16Snorm = gl::RGBA16_SNORM,
  Rgba8Snorm = gl::RGBA8_SNORM,
  Rgb32F = gl::RGB32F,
  Rgb32I = gl::RGB32I,
  Rgb32Ui = gl::RGB32UI,
  Rgb16Snorm = gl::RGB16_SNORM,
  Rgb8Snorm = gl::RGB8_SNORM,
  Rgb16 = gl::RGB16,
  Rgb16F = gl::RGB16F,
  Rgb16I = gl::RGB16I,
  Rgb16Ui = gl::RGB16UI,
  Srgb8 = gl::SRGB8,
  Rgb9E5 = gl::RGB9_E5,
  Rg16Snorm = gl::RG16_SNORM,
  Rg8Snorm = gl::RG8_SNORM,
  CompressedSignedRedRGTC1 = gl::COMPRESSED_SIGNED_RED_RGTC1,
  CompressedRGBADXT3 = 0x83f2,
  DepthComponent32F = gl::DEPTH_COMPONENT32F,
  DepthComponent24 = gl::DEPTH_COMPONENT24,
  DepthComponent16 = gl::DEPTH_COMPONENT16,
  DepthComponent32FStencil8 = gl::DEPTH32F_STENCIL8,
  DepthComponent24Stencil8 = gl::DEPTH24_STENCIL8,
}

pub(crate) struct GlTexture<T> {
  m_id: u32,
  m_slot: u16,
  m_texture: TextureInfo<T>,
  m_level: u32,
  m_ms: Option<u32>,
  m_format: u32,
  m_internal_target: u32,
  m_internal_type: u32,
  m_internal_format: u32,
}

impl<T> Default for GlTexture<T> {
  fn default() -> Self {
    return Self {
      m_id: 0,
      m_slot: 7,
      m_texture: TextureInfo {
        m_type: Default::default(),
        m_data: Image {
          width: 0,
          height: 0,
          depth: 0,
          data: vec![],
        },
      },
      m_level: 0,
      m_ms: None,
      m_format: gl::RGBA,
      m_internal_target: gl::TEXTURE_2D_ARRAY,
      m_internal_type: gl::UNSIGNED_BYTE,
      m_internal_format: gl::RGBA8,
    };
  }
}

impl<T> GlTexture<T> {
  pub(crate) fn new(texture_info: TextureInfo<T>) -> Self {
    let (target, sample_count) = Self::convert_target_to_internal_target(texture_info.m_type.get_target());
    let (format, internal_format) = Self::convert_format_to_internal_format(texture_info.m_type.get_format());
    
    let texture_slot: u16 = texture_info.m_type.get_slot();
    
    return Self {
      m_id: 0,
      m_slot: texture_slot,
      m_level: texture_info.m_type.get_mipmap_level(),
      m_internal_target: target,
      m_internal_format: internal_format,
      m_internal_type: Self::convert_type_to_internal_type(texture_info.m_type.get_data_type()),
      m_texture: texture_info,
      m_ms: sample_count,
      m_format: format,
    };
  }
  
  fn convert_target_to_internal_target(target: EnumTextureTarget) -> (u32, Option<u32>) {
    return match target {
      EnumTextureTarget::Texture1D => (gl::TEXTURE_1D, None),
      EnumTextureTarget::Texture1DArray => (gl::TEXTURE_1D_ARRAY, None),
      EnumTextureTarget::ProxyTexture1DArray => (gl::PROXY_TEXTURE_1D_ARRAY, None),
      EnumTextureTarget::Texture2D => (gl::TEXTURE_2D, None),
      EnumTextureTarget::ProxyTexture2D => (gl::PROXY_TEXTURE_2D, None),
      EnumTextureTarget::TextureRect => (gl::TEXTURE_RECTANGLE, None),
      EnumTextureTarget::ProxyTextureRect => (gl::PROXY_TEXTURE_RECTANGLE, None),
      EnumTextureTarget::TextureCubeMap => (gl::TEXTURE_CUBE_MAP, None),
      EnumTextureTarget::ProxyTextureCubeMap => (gl::PROXY_TEXTURE_CUBE_MAP, None),
      EnumTextureTarget::DepthStencil => (gl::TEXTURE_2D_MULTISAMPLE, None),
      EnumTextureTarget::ColorAttachment(_) => (gl::TEXTURE_2D_MULTISAMPLE, None),
      EnumTextureTarget::Texture2DMs(sample_count) => (gl::TEXTURE_2D_MULTISAMPLE, Some(sample_count)),
      EnumTextureTarget::Texture2DArray => (gl::TEXTURE_2D_ARRAY, None),
      EnumTextureTarget::Texture2DArrayMs(sample_count) => (gl::TEXTURE_2D_MULTISAMPLE_ARRAY, Some(sample_count)),
      EnumTextureTarget::Texture3D => (gl::TEXTURE_3D, None),
      EnumTextureTarget::Texture3DMs(sample_count) => (gl::TEXTURE_3D, Some(sample_count)),
      EnumTextureTarget::TextureCubeMapArray => (gl::TEXTURE_CUBE_MAP_ARRAY, None)
    };
  }
  
  fn convert_format_to_internal_format(format: EnumTextureFormat) -> (u32, u32) {
    return match format {
      EnumTextureFormat::Red => (gl::RED, gl::R8),
      EnumTextureFormat::Rg => (gl::RG, gl::RG8),
      EnumTextureFormat::Rgb => (gl::RGB, gl::RGB8),
      EnumTextureFormat::Bgr => (gl::BGR, gl::BGR_INTEGER),
      EnumTextureFormat::Rgba => (gl::RGBA, gl::RGBA8),
      EnumTextureFormat::Bgra => (gl::BGRA_INTEGER, gl::BGRA),
    };
  }
  
  fn convert_type_to_internal_type(texture_type: EnumTextureDataAlignment) -> u32 {
    return match texture_type {
      EnumTextureDataAlignment::UnsignedByte => gl::UNSIGNED_BYTE,
      EnumTextureDataAlignment::UnsignedByte233Reverse => gl::UNSIGNED_BYTE_2_3_3_REV,
      EnumTextureDataAlignment::UnsignedByte332 => gl::UNSIGNED_BYTE_3_3_2,
      EnumTextureDataAlignment::Byte => gl::BYTE,
      EnumTextureDataAlignment::UnsignedShort => gl::UNSIGNED_SHORT,
      EnumTextureDataAlignment::UnsignedShort565 => gl::UNSIGNED_SHORT_5_6_5,
      EnumTextureDataAlignment::UnsignedShort565Reverse => gl::UNSIGNED_SHORT_5_6_5_REV,
      EnumTextureDataAlignment::UnsignedShort4444 => gl::UNSIGNED_SHORT_4_4_4_4,
      EnumTextureDataAlignment::UnsignedShort4444Reverse => gl::UNSIGNED_SHORT_4_4_4_4_REV,
      EnumTextureDataAlignment::UnsignedShort5551 => gl::UNSIGNED_SHORT_5_5_5_1,
      EnumTextureDataAlignment::UnsignedShort1555Reverse => gl::UNSIGNED_SHORT_1_5_5_5_REV,
      EnumTextureDataAlignment::Short => gl::SHORT,
      EnumTextureDataAlignment::UnsignedInt => gl::UNSIGNED_INT,
      EnumTextureDataAlignment::UnsignedInt8888 => gl::UNSIGNED_INT_8_8_8_8,
      EnumTextureDataAlignment::UnsignedInt8888Reverse => gl::UNSIGNED_INT_8_8_8_8_REV,
      EnumTextureDataAlignment::UnsignedInt10_10_10_2 => gl::UNSIGNED_INT_10_10_10_2,
      EnumTextureDataAlignment::UnsignedInt2_10_10_10Reverse => gl::UNSIGNED_INT_2_10_10_10_REV,
      EnumTextureDataAlignment::Int => gl::INT,
      EnumTextureDataAlignment::Float => gl::FLOAT
    };
  }
}

impl<T> TraitTexture for GlTexture<T> {
  fn get_depth(&self) -> u16 {
    return self.m_texture.m_type.get_depth();
  }
  
  fn get_size(&self) -> (usize, usize) {
    return (self.m_texture.m_data.width, self.m_texture.m_data.height);
  }
  
  fn set_depth(&mut self, depth: u16) {
    self.m_texture.m_data.depth = depth as usize;
  }
  
  fn convert_to(&mut self, _format: EnumTextureFormat) -> Result<(), EnumRendererError> {
    todo!()
  }
  
  fn apply(&mut self) -> Result<(), EnumRendererError> {
    #[cfg(feature = "debug")]
    log!(EnumLogColor::Blue, "DEBUG", "[GlTexture] -->\t Storing {0}", self.m_texture.m_type);
    
    check_gl_call!("GlTexture", gl::GenTextures(1, &mut self.m_id));
    check_gl_call!("GlTexture", gl::ActiveTexture(gl::TEXTURE0 + self.m_slot as u32));
    check_gl_call!("GlTexture", gl::BindTexture(self.m_internal_target, self.m_id));
    
    match self.m_internal_target {
      gl::TEXTURE_2D_MULTISAMPLE | gl::TEXTURE_2D_MULTISAMPLE_ARRAY => {
        check_gl_call!("GlTexture", gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint));
        check_gl_call!("GlTexture", gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint));
        
        check_gl_call!("GlTexture", gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as GLint));
        check_gl_call!("GlTexture", gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as GLint));
        check_gl_call!("GlTexture", gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as GLint));
      }
      _ => {
        check_gl_call!("GlTexture", gl::TexParameteri(self.m_internal_target, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint));
        check_gl_call!("GlTexture", gl::TexParameteri(self.m_internal_target, gl::TEXTURE_MIN_FILTER,
          gl::LINEAR_MIPMAP_NEAREST as GLint));
        
        check_gl_call!("GlTexture", gl::TexParameteri(self.m_internal_target, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint));
        check_gl_call!("GlTexture", gl::TexParameteri(self.m_internal_target, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint));
        check_gl_call!("GlTexture", gl::TexParameteri(self.m_internal_target, gl::TEXTURE_WRAP_R, gl::REPEAT as GLint));
      }
    }
    
    if self.m_texture.m_data.depth.is_odd() {
      // Make alignment work for odd color channels or odd dimensions.
      check_gl_call!("GlTexture", gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1));
    }
    
    match self.m_internal_target {
      gl::TEXTURE_2D | gl::TEXTURE_CUBE_MAP | gl::TEXTURE_RECTANGLE | gl::TEXTURE_1D_ARRAY | gl::TEXTURE_2D_MULTISAMPLE => {
        // Check if texture is multi-sampled.
        if self.m_ms.is_some() && self.m_ms.unwrap() > 1 {
          check_gl_call!("GlTexture", gl::TexImage2DMultisample(self.m_internal_target, self.m_ms.unwrap() as GLsizei,
            self.m_internal_format, self.m_texture.m_data.width as GLsizei, self.m_texture.m_data.height as GLsizei, 0));
        }
        
        check_gl_call!("GlTexture", gl::TexImage2D(self.m_internal_target, self.m_level as GLint, self.m_internal_format as GLint,
        self.m_texture.m_data.width as GLsizei, self.m_texture.m_data.height as GLsizei, 0, self.m_format, self.m_internal_type,
          self.m_texture.m_data.data.as_ptr() as *const _));
      }
      gl::TEXTURE_2D_ARRAY | gl::TEXTURE_3D | gl::TEXTURE_2D_MULTISAMPLE_ARRAY => {
        // Check if texture is multi-sampled.
        if self.m_ms.is_some() && self.m_ms.unwrap() > 1 {
          check_gl_call!("GlTexture", gl::TexImage3DMultisample(self.m_internal_target, self.m_ms.unwrap() as GLsizei,
            self.m_internal_format, self.m_texture.m_data.width as GLsizei, self.m_texture.m_data.height as GLsizei,
            self.m_texture.m_type.get_depth() as GLsizei, 0));
        }
        
        match &self.m_texture.m_type {
          EnumTextureInfo::Texture3D(_, _, _, _, _, depth, _, _) => {
            check_gl_call!("GlTexture", gl::TexImage3D(self.m_internal_target, self.m_level as GLint, self.m_internal_format as GLint,
        self.m_texture.m_data.width as GLsizei, self.m_texture.m_data.height as GLsizei, *depth as GLsizei, 0, self.m_format,
          self.m_internal_type, self.m_texture.m_data.data.as_ptr() as *const _));
          }
          EnumTextureInfo::TextureArray(vec) => {
            check_gl_call!("GlTexture", gl::TexImage3D(self.m_internal_target, self.m_level as GLint, self.m_internal_format as GLint,
              self.m_texture.m_data.width as GLsizei, self.m_texture.m_data.height as GLsizei,
              (vec.last().unwrap().0.get_depth() + 1) as GLsizei, 0, self.m_format, self.m_internal_type, std::ptr::null() as *const _));
            
            for texture in vec {
              check_gl_call!("GlTexture", gl::TexSubImage3D(self.m_internal_target, self.m_level as GLint, 0, 0,
                texture.0.get_depth() as GLint, self.m_texture.m_data.width as GLsizei, self.m_texture.m_data.height as GLsizei,
                1 as GLsizei, self.m_format, self.m_internal_type, texture.1.clone().as_ptr() as *const _));
            }
          }
          _ => {}
        }
        check_gl_call!("GlTexture", gl::GenerateMipmap(self.m_internal_target));
      }
      _ => todo!()
    }
    
    return Ok(());
  }
  
  fn clear(&mut self) -> Result<(), EnumRendererError> {
    check_gl_call!("GlTexture", gl::BindTexture(self.m_internal_target, self.m_id));
    
    match self.m_internal_target {
      gl::TEXTURE_2D | gl::TEXTURE_2D_MULTISAMPLE | gl::TEXTURE_CUBE_MAP | gl::TEXTURE_RECTANGLE | gl::TEXTURE_1D_ARRAY => {
        check_gl_call!("GlTexture", gl::TexSubImage2D(self.m_internal_target, self.m_level as GLint, 0, 0, self.m_texture.m_data.width as GLsizei,
          self.m_texture.m_data.height as GLsizei, self.m_format, self.m_internal_type, std::ptr::null() as *const _));
      }
      gl::TEXTURE_2D_ARRAY | gl::TEXTURE_3D => {
        match &self.m_texture.m_type {
          EnumTextureInfo::Texture3D(_, _, _, _, _, depth, _, _) => {
            check_gl_call!("GlTexture", gl::TexImage3D(self.m_internal_target, self.m_level as GLint, self.m_internal_format as GLint,
        self.m_texture.m_data.width as GLsizei, self.m_texture.m_data.height as GLsizei, *depth as GLsizei, 0, self.m_format,
          self.m_internal_type, std::ptr::null() as *const _));
          }
          EnumTextureInfo::TextureArray(vec) => {
            check_gl_call!("GlTexture", gl::TexImage3D(self.m_internal_target, self.m_level as GLint, self.m_internal_format as GLint,
              self.m_texture.m_data.width as GLsizei, self.m_texture.m_data.height as GLsizei,
              (vec.last().unwrap().0.get_depth() + 1) as GLsizei, 0, self.m_format, self.m_internal_type, std::ptr::null() as *const _));
          }
          _ => {}
        }
      }
      _ => todo!()
    }
    return Ok(());
  }
  
  fn free(&mut self) -> Result<(), EnumRendererError> {
    if gl::BindTexture::is_loaded() {
      check_gl_call!("GlTexture", gl::BindTexture(self.m_internal_target, 0));
      check_gl_call!("GlTexture", gl::DeleteTextures(1, &mut self.m_id));
    }
    return Ok(());
  }
}