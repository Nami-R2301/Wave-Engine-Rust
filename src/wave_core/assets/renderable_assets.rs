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

use crate::wave_core::math::{Vec2, Vec3};
use crate::log;
use crate::wave_core::assets::asset_loader::ResLoader;
use crate::wave_core::{Engine};
use crate::wave_core::graphics::renderer;
use crate::wave_core::graphics::color::Color;
use crate::wave_core::graphics::shader::Shader;
use crate::wave_core::math::{Mat4};

/*
///////////////////////////////////   OpenGL Renderable entity  ///////////////////////////////////
///////////////////////////////////                             ///////////////////////////////////
///////////////////////////////////                             ///////////////////////////////////
 */

static mut S_ENTITIES_ID_CACHE: Lazy<HashSet<u32>> = Lazy::new(|| HashSet::new());

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash)]
pub enum EnumVertexMemberOffset {
  Id = 0,
  AtPos =  size_of::<u32>(),
  AtNormal = size_of::<u32>() + (size_of::<f32>() * 3),
  AtColor = size_of::<u32>() + (size_of::<f32>() * 3) + (size_of::<f32>() * 3),
  AtTexCoords = size_of::<u32>() + (size_of::<f32>() * 3) + (size_of::<f32>() * 3) + size_of::<Color>()
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash)]
pub enum EnumEntityType {
  Object,
  Text
}

pub(crate) trait TraitVertex {
  fn get_id(&self) -> u32;
  fn register(&mut self, id: u32);
  fn clear(&mut self);
}

#[repr(C)]
pub struct Vertex {
  pub m_id: u32,
  pub m_position: Vec3<f32>,
  pub m_normal: Vec3<f32>,
  pub m_color: Color,
  pub m_texture_coords: Vec2<f32>
}

impl Vertex {
  pub fn default() -> Self {
    return Self {
      m_id: u32::MAX,
      m_position: Vec3::default(),
      m_normal: Vec3::new(&[0.0, 0.0, 1.0]),
      m_color: Color::default(),
      m_texture_coords: Vec2::default()
    }
  }
}

impl TraitVertex for Vertex {
  fn get_id(&self) -> u32 {
    return self.m_id;
  }
  
  fn register(&mut self, id: u32) {
    self.m_id = id;
  }
  
  fn clear(&mut self) {
    self.m_position = Vec3::default();
    self.m_normal = Vec3::default();
    self.m_texture_coords = Vec2::default();
    self.m_color = Color::default();
  }
}

pub struct REntity {
  // Mouse picking ID per vertex.
  pub(crate) m_data: Vec<Vertex>,
  pub(crate) m_type: EnumEntityType,
  // UUID given by the renderer to differentiate entities in batch rendering.
  m_renderer_id: u64,
  // Transformations applied to the entity, to be eventually applied to the model matrix.
  m_transform: [Vec3<f32>; 3],
  m_sent: bool,
  m_changed: bool,
  m_flat_shaded: bool
}

impl REntity {
  pub fn default() -> Result<Self, renderer::EnumError> {
    let mut new_entity: REntity = REntity {
      m_data: ResLoader::new("cube.obj")?,
      m_type: EnumEntityType::Object,
      m_renderer_id: u64::MAX,
      m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
      m_sent: false,
      m_changed: false,
      m_flat_shaded: false
    };
    
    new_entity.register();
    new_entity.translate(Vec3::new(&[0.0, 0.0, 10.0]));
    return Ok(new_entity);
  }
  pub fn new(data: Vec<Vertex>, data_type: EnumEntityType, is_flat_shaded: bool) -> Self {
    let mut new_entity: REntity = REntity {
      m_data: data,
      m_type: data_type,
      m_renderer_id: u64::MAX,
      m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
      m_sent: false,
      m_changed: false,
      m_flat_shaded: is_flat_shaded
    };
    
    new_entity.register();
    return new_entity;
  }
  
  pub fn size_of() -> usize {
    return size_of::<u32>() + (size_of::<f32>() * 3) + (size_of::<f32>() * 3) + size_of::<Color>() +
      (size_of::<f32>() * 2);
  }
  
  pub fn vertex_count(&self) -> usize {
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
      self.m_data[index].register(new_id);
    }
  }
  
  pub fn set_type(&mut self, new_type: EnumEntityType) {
    self.m_type = new_type;
  }
  
  pub fn translate(&mut self, amount: Vec3<f32>) {
    self.m_transform[0] += Vec3::new(&[amount.x, amount.y, -amount.z]);
    self.m_changed = true;
  }
  
  pub fn rotate(&mut self, amount: Vec3<f32>) {
    let mut copy = self.m_transform[1];
    
    // Inverse x and y to correspond to the right orientation.
    copy.x = amount.y;
    copy.y = amount.x;
    copy.z = -amount.z;
    
    self.m_transform[1] += copy;
    self.m_changed = true;
  }
  
  pub fn scale(&mut self, amount: Vec3<f32>) {
    self.m_transform[2] += amount;
    self.m_changed = true;
  }
  
  pub fn resend_transform(&mut self, shader_associated: &mut Shader) -> Result<(), renderer::EnumError> {
    if !self.m_sent {
      log!(EnumLogColor::Red, "ERROR", "[RAssets] -->\t Cannot update shader ({0}) of entity, entity not sent previously!",
        shader_associated.get_id());
      return Err(renderer::EnumError::EntityNotFound);
    }
    
    // Only update if the entity changed.
    if self.m_changed {
      let renderer = Engine::get_active_renderer();
      renderer.update(shader_associated, self.get_matrix())?;
      self.m_changed = false;
    }
    return Ok(());
  }
  
  pub fn send(&mut self, shader_associated: &mut Shader) -> Result<(), renderer::EnumError> {
    let renderer = Engine::get_active_renderer();
    
    return match renderer.enqueue(self, shader_associated) {
      Ok(_) => {
        self.m_sent = true;
        self.m_changed = false;
        Ok(())
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Entity sent unsuccessfully to GPU! \
              Error => {0:?}", err);
        Err(err)
      }
    };
  }
  
  pub fn resend(&mut self, _shader_associated: &mut Shader) -> Result<(), renderer::EnumError> {
    todo!()
  }
  
  pub fn free(&mut self, _shader_associated: &mut Shader) -> Result<(), renderer::EnumError> {
    let renderer = Engine::get_active_renderer();
    
    return match renderer.dequeue(self.m_renderer_id) {
      Ok(_) => {
        self.m_sent = false;
        self.m_changed = false;
        Ok(())
      }
      Err(err) => Err(err)
    };
  }
  
  pub fn is_sent(&self) -> bool {
    return self.m_sent;
  }
  
  pub fn has_changed(&self) -> bool {
    return self.m_changed;
  }
  
  pub fn get_uuid(&self) -> u64 {
    return self.m_renderer_id;
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
    return self.m_type == other.m_type && self.m_data[0].get_id() == other.m_data[0].get_id();
  }
  
  fn ne(&self, other: &Self) -> bool {
    return !self.eq(other);
  }
}