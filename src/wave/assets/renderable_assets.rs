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

use std::collections::HashSet;
use std::fmt::{Display};
use std::mem::size_of;

use once_cell::sync::Lazy;

use crate::wave::math::{Vec2, Vec3};
use crate::log;
use crate::wave::graphics::color::Color;
use crate::wave::graphics::renderer::{EnumError, Renderer};
use crate::wave::graphics::shader::Shader;
use crate::wave::math::{Mat4};

/*
///////////////////////////////////   OpenGL Renderable entity  ///////////////////////////////////
///////////////////////////////////                             ///////////////////////////////////
///////////////////////////////////                             ///////////////////////////////////
 */

static mut S_ENTITIES_ID_CACHE: Lazy<HashSet<u32>> = Lazy::new(|| HashSet::new());

pub trait TraitRenderableEntity {
  fn send(&mut self, shader_associated: &mut Shader) -> Result<(), EnumError>;
  fn resend(&mut self, shader_associated: &mut Shader) -> Result<(), EnumError>;
  fn free(&mut self, shader_associated: &mut Shader) -> Result<(), EnumError>;
  fn is_sent(&self) -> bool;
}

#[repr(C)]
pub struct Vertex {
  pub m_id: u32,
  pub m_position: Vec3<f32>,
  pub m_normal: Vec3<f32>,
  pub color: Color,
  pub m_texture_coords: Vec2<f32>
}

impl Vertex {
  pub fn default() -> Self {
    return Self {
      m_id: u32::MAX,
      m_position: Vec3::default(),
      m_normal: Vec3::new(&[0.0, 0.0, 1.0]),
      color: Color::default(),
      m_texture_coords: Vec2::default()
    }
  }
}

pub struct REntity {
  // Mouse picking ID per vertex.
  pub m_data: Vec<Vertex>,
  // UUID given by the renderer to differentiate entities in batch rendering.
  m_renderer_id: u64,
  // Transformations applied to the entity, to be eventually applied to the model matrix.
  m_transform: [Vec3<f32>; 3],
  m_sent: bool,
  m_flat_shaded: bool
}

impl REntity {
  pub fn default() -> Self {
    return REntity {
      m_data: Vec::new(),
      m_renderer_id: u64::MAX,
      m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
      m_sent: false,
      m_flat_shaded: false
    };
  }
  
  pub fn new(data: Vec<Vertex>, is_flat_shaded: bool) -> Self {
    return REntity {
      m_data: data,
      m_renderer_id: u64::MAX,
      m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
      m_sent: false,
      m_flat_shaded: is_flat_shaded
    };
  }
  
  pub fn size(&self) -> usize {
    return size_of::<u32>() + (size_of::<f32>() * 3) + (size_of::<f32>() * 3) + size_of::<Color>() +
      (size_of::<f32>() * 2);
  }
  
  pub fn count(&self) -> usize {
    return self.m_data.len();
  }
  
  pub fn is_empty(&self) -> bool {
    return self.m_data.is_empty();
  }
  
  pub fn is_flat_shaded(&self) -> bool {
    return self.m_flat_shaded;
  }
  
  pub fn register(&mut self) {
    let mut new_id = rand::random::<u32>();
    unsafe {
      while S_ENTITIES_ID_CACHE.contains(&new_id) {
        new_id = rand::random::<u32>();
      }
    }
    for index in 0..self.m_data.len() {
      self.m_data[index].m_id = new_id;
    }
  }
  
  pub fn translate(&mut self, amount: Vec3<f32>) {
    self.m_transform[0] = Vec3::new(&[amount.x, amount.y, -amount.z]);
  }
  
  pub fn rotate(&mut self, amount: Vec3<f32>) {
    // Inverse x and y to correspond to the right orientation.
    self.m_transform[1].x = amount.x;
    self.m_transform[1].y = amount.y;
    self.m_transform[1].z = -amount.z;
  }
  
  pub fn scale(&mut self, amount: Vec3<f32>) {
    self.m_transform[2] = amount;
  }
  
  pub fn get_matrix(&self) -> Mat4 {
    return Mat4::apply_model(&self.m_transform[0],
      &self.m_transform[1], &self.m_transform[2]);
  }
}

///////////////////////////////////   DISPLAY  ///////////////////////////////////

impl Display for REntity {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[REntity] --> \nSent => {0}", self.m_sent)
  }
}

///////////////////////////////////   EQUALITY  ///////////////////////////////////

impl PartialEq for REntity {
  fn eq(&self, other: &Self) -> bool {
    return self.m_data[0].m_id == other.m_data[0].m_id;
  }
  
  fn ne(&self, other: &Self) -> bool {
    return !self.eq(other);
  }
}

impl TraitRenderableEntity for REntity {
  fn send(&mut self, shader_associated: &mut Shader) -> Result<(), EnumError> {
    let renderer = Renderer::get()
      .expect("[REntity] -->\t Cannot send REntity, renderer is null! Exiting...");
    
    return match unsafe { (*renderer).enqueue(self, shader_associated) } {
      Ok(_) => {
        self.m_sent = true;
        Ok(())
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Entity sent unsuccessfully to GPU! \
              Error => {0:?}", err);
        Err(err)
      }
    };
  }
  
  fn resend(&mut self, _shader_associated: &mut Shader) -> Result<(), EnumError> {
    todo!()
  }
  
  fn free(&mut self, _shader_associated: &mut Shader) -> Result<(), EnumError> {
    let renderer = Renderer::get()
      .expect("[REntity] -->\t Cannot free REntity, renderer is null! Exiting...");
    
    return match unsafe { (*renderer).dequeue(&self.m_renderer_id) } {
      Ok(_) => {
        self.m_sent = false;
        Ok(())
      }
      Err(err) => { Err(err) }
    };
  }
  
  fn is_sent(&self) -> bool {
    return self.m_sent;
  }
}