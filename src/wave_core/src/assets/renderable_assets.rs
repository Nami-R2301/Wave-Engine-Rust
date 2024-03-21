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

use crate::utils::macros::logger::*;
use crate::math::{Vec2, Vec3};
use crate::{Engine};
use crate::graphics::renderer;
use crate::graphics::color::Color;
use crate::graphics::shader::Shader;
use crate::math::{Mat4};

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
  AtPos = size_of::<u32>(),
  AtNormal = size_of::<u32>() + (size_of::<f32>() * 3),
  AtColor = size_of::<u32>() + (size_of::<f32>() * 3) + (size_of::<f32>() * 3),
  AtTexCoords = size_of::<u32>() + (size_of::<f32>() * 3) + (size_of::<f32>() * 3) + size_of::<Color>(),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash)]
pub enum EnumPrimitiveType {
  Sprite,
  Mesh(bool),
  Quad,
}

pub trait TraitPrimitive {
  fn len(&self) -> usize;
  fn get_name(&self) -> &str;
  fn get_vertices(&self) -> &Vec<Vertex>;
  fn has_submeshes(&self) -> bool;
  fn is_empty(&self) -> bool;
}

#[repr(C)]
#[derive(Clone)]
pub struct Vertex {
  pub m_id: u32,
  pub m_position: Vec3<f32>,
  pub m_normal: Vec3<f32>,
  pub m_color: Color,
  pub m_texture_coords: Vec2<f32>,
}

impl Vertex {
  pub fn default() -> Self {
    return Self {
      m_id: u32::MAX,
      m_position: Vec3::default(),
      m_normal: Vec3::new(&[0.0, 0.0, 1.0]),
      m_color: Color::default(),
      m_texture_coords: Vec2::default(),
    };
  }
  
  pub fn get_id(&self) -> u32 {
    return self.m_id;
  }
  
  pub fn register(&mut self, id: u32) {
    self.m_id = id;
  }
  
  pub fn clear(&mut self) {
    self.m_position = Vec3::default();
    self.m_normal = Vec3::default();
    self.m_texture_coords = Vec2::default();
    self.m_color = Color::default();
  }
}

#[repr(C)]
pub struct Sprite {
  m_name: String,
  m_vertices: Vec<Vertex>,
}

impl Sprite {
  pub fn new(data: assimp::Mesh) -> Self {
    let mut vertices: Vec<Vertex> = Vec::new();
    vertices.resize(data.num_vertices as usize, Vertex::default());
    let mut indices: Vec<u32> = Vec::with_capacity(data.num_faces as usize);
    
    for face in data.face_iter() {
      for index in 0..face.num_indices {
        indices.push(face[index as isize]);
      }
    }
    
    for (position, vertex) in data.vertex_iter().enumerate() {
      let mut new_id = rand::random::<u32>();
      unsafe {
        while S_ENTITIES_ID_CACHE.contains(&new_id) {
          new_id = rand::random();
        }
      }
      vertices[position].m_id = new_id;
      vertices[position].m_position = Vec3::new(&[vertex.x, vertex.y, 0.0]);
    }
    
    for (position, texture_coord) in data.texture_coords_iter(0).enumerate() {
      vertices[position].m_texture_coords = Vec2::new(&[texture_coord.x, texture_coord.y]);
    }
    
    return Self {
      m_vertices: vertices,
      m_name: String::from(data.name.as_ref())
    };
  }
}

impl TraitPrimitive for Sprite {
  fn len(&self) -> usize {
    return self.m_vertices.len();
  }
  
  fn get_name(&self) -> &str {
    return &self.m_name;
  }
  fn get_vertices(&self) -> &Vec<Vertex> {
    return &self.m_vertices;
  }
  
  fn has_submeshes(&self) -> bool {
    return false;
  }
  
  fn is_empty(&self) -> bool {
    return self.m_vertices.is_empty();
  }
}

#[repr(C)]
pub struct Mesh {
  m_name: String,
  m_vertices: Vec<Vertex>,
  m_submeshes: Vec<Self>
}

impl Mesh {
  pub fn new(data: assimp::Scene) -> Self {
    
    let mut result = Self {
      m_name: "Empty".to_string(),
      m_vertices: vec![],
      m_submeshes: vec![],
    };
    
    for (position, mesh) in data.mesh_iter().enumerate() {
      let mut vertices: Vec<Vertex> = Vec::new();
      vertices.resize(mesh.num_vertices as usize, Vertex::default());
      
      for (position, vertex) in mesh.vertex_iter().enumerate() {
        let mut new_id = rand::random::<u32>();
        unsafe {
          while S_ENTITIES_ID_CACHE.contains(&new_id) {
            new_id = rand::random();
          }
        }
        vertices[position].m_id = new_id;
        vertices[position].m_position = Vec3::new(&[vertex.x, vertex.y, vertex.z]);
      }
      
      for (position, normal) in mesh.normal_iter().enumerate() {
        vertices[position].m_normal = Vec3::new(&[normal.x, normal.y, normal.z]);
      }
      
      for (position, texture_coord) in mesh.texture_coords_iter(0).enumerate() {
        vertices[position].m_texture_coords = Vec2::new(&[texture_coord.x, texture_coord.y]);
      }
      
      if position == 0 {
        result.m_name = String::from(mesh.name.as_ref());
        result.m_vertices = vertices;
        continue;
      }
      result.m_submeshes.push(Self {
        m_name: String::from(mesh.name.as_ref()),
        m_vertices: vertices,
        m_submeshes: vec![],
      });
    }
    
    return result;
  }
}

impl TraitPrimitive for Mesh {
  fn len(&self) -> usize {
    return self.m_vertices.len() + (!self.m_submeshes.is_empty()).then(|| {
      let mut count = 0;
      for sub in self.m_submeshes.iter() {
        count += sub.len();
      }
      return count
    }).unwrap_or(0);
  }
  
  fn get_name(&self) -> &str {
    return &self.m_name;
  }
  
  fn get_vertices(&self) -> &Vec<Vertex> {
    return &self.m_vertices;
  }
  
  fn has_submeshes(&self) -> bool {
    return !self.m_submeshes.is_empty();
  }
  
  fn is_empty(&self) -> bool {
    return self.m_vertices.is_empty();
  }
}

pub struct REntity {
  // Mouse picking ID per vertex.
  pub(crate) m_data: Box<dyn TraitPrimitive>,
  // Vec of meshes/sprites/quad data.
  pub(crate) m_type: EnumPrimitiveType,
  // UUID given by the renderer to differentiate entities in batch rendering.
  pub(crate) m_renderer_id: u64,
  // Transformations applied to the entity, to be eventually applied to the model matrix.
  m_transform: [Vec3<f32>; 3],
  m_sent: bool,
  m_changed: bool,
  m_flat_shaded: bool,
}

impl REntity {
  pub fn default() -> Result<Self, renderer::EnumRendererError> {
    let mut new_entity: REntity = REntity {
      m_data: Box::new(Sprite {
        m_name: "".to_string(),
        m_vertices: vec![],
      }),
      m_type: EnumPrimitiveType::Sprite,
      m_renderer_id: u64::MAX,
      m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
      m_sent: false,
      m_changed: false,
      m_flat_shaded: false,
    };
    
    new_entity.translate(Vec3::new(&[0.0, 0.0, 10.0]));
    return Ok(new_entity);
  }
  pub fn new(data: Box<dyn TraitPrimitive>, data_type: EnumPrimitiveType) -> Self {
    let new_entity: REntity;
    
    match data_type {
      EnumPrimitiveType::Sprite => {
        new_entity = REntity {
          m_data: data,
          m_type: EnumPrimitiveType::Sprite,
          m_renderer_id: u64::MAX,
          m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
          m_sent: false,
          m_changed: false,
          m_flat_shaded: false,
        };
      }
      EnumPrimitiveType::Mesh(is_flat_shaded) => {
        new_entity = REntity {
          m_data: data,
          m_type: EnumPrimitiveType::Mesh(is_flat_shaded),
          m_renderer_id: u64::MAX,
          m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
          m_sent: false,
          m_changed: false,
          m_flat_shaded: is_flat_shaded,
        };
      }
      EnumPrimitiveType::Quad => todo!()
    }
    
    return new_entity;
  }
  
  pub fn size(&self) -> usize {
    return match self.m_type {
      EnumPrimitiveType::Sprite => {
        size_of::<u32>() + (size_of::<f32>() * 3) + (size_of::<f32>() * 3) + size_of::<Color>() +
          (size_of::<f32>() * 2)
      }
      EnumPrimitiveType::Mesh(_) => {
        size_of::<u32>() + (size_of::<f32>() * 3) + (size_of::<f32>() * 3) + size_of::<Color>() +
          (size_of::<f32>() * 2)
      }
      EnumPrimitiveType::Quad => {
        size_of::<u32>() + (size_of::<f32>() * 3) + size_of::<Color>() + (size_of::<f32>() * 2)
      }
    };
  }
  
  pub fn total_vertex_count(&self) -> usize {
    return self.m_data.len();
  }
  
  // pub fn get_submeshes(&self) -> Option<Iter<Vec<Box<dyn TraitPrimitive>>>> {
  //   if !self.m_data.has_submeshes() {
  //     return None;
  //   }
  //   return self.m_data.iter();
  // }
  
  pub fn is_empty(&self) -> bool {
    return self.m_data.is_empty();
  }
  
  pub fn is_flat_shaded(&self) -> bool {
    return self.m_flat_shaded;
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
  
  pub fn resend_transform(&mut self, shader_associated: &mut Shader) -> Result<(), renderer::EnumRendererError> {
    if !self.m_sent {
      log!(EnumLogColor::Red, "ERROR", "[RAssets] -->\t Cannot update shader ({0}) of entity, entity not sent previously!",
        shader_associated.get_id());
      return Err(renderer::EnumRendererError::EntityNotFound);
    }
    
    // Only update if the entity changed.
    if self.m_changed {
      let renderer = Engine::get_active_renderer();
      renderer.update(shader_associated, self.get_matrix())?;
      self.m_changed = false;
    }
    return Ok(());
  }
  
  pub fn submit(&mut self, shader_associated: &mut Shader) -> Result<(), renderer::EnumRendererError> {
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
  
  pub fn resend(&mut self, _shader_associated: &mut Shader) -> Result<(), renderer::EnumRendererError> {
    todo!()
  }
  
  pub fn free(&mut self, _shader_associated: &mut Shader) -> Result<(), renderer::EnumRendererError> {
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
    write!(format, "UUID: {0}\n{3:117}Type: {1:?}\n{3:117}Sent?: {2}", self.m_renderer_id, self.m_type, self.m_sent, "")
  }
}

///////////////////////////////////   EQUALITY  ///////////////////////////////////

impl PartialEq for REntity {
  fn eq(&self, other: &Self) -> bool {
    if self.is_empty() {
      return self.m_type == other.m_type && self.m_data.len() == other.m_data.len();
    }
    return self.m_type == other.m_type && self.m_data.get_vertices()[0].get_id() == other.m_data.get_vertices()[0].get_id();
  }
  
  fn ne(&self, other: &Self) -> bool {
    return !self.eq(other);
  }
}