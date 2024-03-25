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

use std::fmt::{Display, Formatter};
use std::mem::size_of;
use std::ops::Index;

use crate::Engine;
use crate::graphics::color::Color;
use crate::graphics::renderer;
use crate::graphics::shader::Shader;
use crate::math::{Mat4, Vec2, Vec3};

/*
///////////////////////////////////   OpenGL Renderable entity  ///////////////////////////////////
///////////////////////////////////                             ///////////////////////////////////
///////////////////////////////////                             ///////////////////////////////////
 */

static mut S_ENTITY_ID_COUNTER: u32 = 0;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash)]
pub enum EnumVertexMemberOffset {
  AtEntityID = 0,
  AtPos = (EnumVertexMemberOffset::AtEntityID as usize) + size_of::<u32>(),
  AtNormal = (EnumVertexMemberOffset::AtPos as usize) + (size_of::<f32>() * 3),
  AtColor = (EnumVertexMemberOffset::AtNormal as usize) + (size_of::<f32>() * 3),
  AtTexCoords = (EnumVertexMemberOffset::AtColor as usize) + size_of::<Color>(),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash)]
pub enum EnumPrimitiveType {
  Sprite,
  Mesh(bool),
  Quad,
}

pub trait TraitPrimitive {
  fn total_vertex_count(&self) -> usize;
  fn total_index_count(&self) -> usize;
  fn get_name(&self) -> &str;
  fn get_vertices(&self) -> &Vec<Vertex>;
  fn get_indices(&self) -> &Vec<u32>;
  // For glDrawElements 3rd parameter to be gl::UNSIGNED_INT (> 65635 vertices indexed).
  fn has_sub_primitives(&self) -> bool;
  fn get_sub_meshes_ref(&self) -> Option<&Vec<Mesh>>;
  fn get_sub_meshes_mut(&mut self) -> Option<&mut Vec<Mesh>>;
  fn get_size(&self) -> usize  where Self: Sized {
    return size_of::<Self>();
  }
  fn get_entity_id(&self) -> u32;
  fn is_empty(&self) -> bool;
}

#[repr(C)]
#[derive(Clone)]
pub struct Vertex {
  pub m_entity_id: u32,
  // Id to differentiate instances in shaders to apply different textures for example or different transformations.
  pub m_position: Vec3<f32>,
  pub m_normal: Vec3<f32>,
  pub m_color: Color,
  pub m_texture_coords: Vec2<f32>,
}

impl Vertex {
  pub fn default() -> Self {
    return Self {
      m_entity_id: 0,
      m_position: Vec3::default(),
      m_normal: Vec3::new(&[0.0, 0.0, 1.0]),
      m_color: Color::default(),
      m_texture_coords: Vec2::default(),
    };
  }
  
  pub fn get_id(&self) -> u32 {
    return self.m_entity_id;
  }
  
  pub fn register(&mut self, id: u32) {
    self.m_entity_id = id;
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
  m_indices: Vec<u32>,
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
      vertices[position].m_position = Vec3::new(&[vertex.x, vertex.y, 0.0]);
      vertices[position].m_entity_id = unsafe { S_ENTITY_ID_COUNTER }
    }
    
    for (position, texture_coord) in data.texture_coords_iter(0).enumerate() {
      vertices[position].m_texture_coords = Vec2::new(&[texture_coord.x, texture_coord.y]);
    }
    
    unsafe { S_ENTITY_ID_COUNTER += 1 }
    
    return Self {
      m_vertices: vertices,
      m_indices: indices,
      m_name: String::from(data.name.as_ref()),
    };
  }
}

impl TraitPrimitive for Sprite {
  fn total_vertex_count(&self) -> usize {
    return self.m_vertices.len();
  }
  
  fn total_index_count(&self) -> usize {
    return self.m_indices.len();
  }
  
  fn get_name(&self) -> &str {
    return &self.m_name;
  }
  
  fn get_vertices(&self) -> &Vec<Vertex> {
    return &self.m_vertices;
  }
  
  fn get_indices(&self) -> &Vec<u32> {
    return &self.m_indices;
  }
  
  fn has_sub_primitives(&self) -> bool {
    return false;
  }
  
  fn get_sub_meshes_ref(&self) -> Option<&Vec<Mesh>> {
    return None;
  }
  
  fn get_sub_meshes_mut(&mut self) -> Option<&mut Vec<Mesh>> {
    return None;
  }
  
  fn get_entity_id(&self) -> u32 {
    return (!self.m_vertices.is_empty()).then(|| self.m_vertices[0].m_entity_id)
      .unwrap_or(0);
  }
  
  fn is_empty(&self) -> bool {
    return self.m_vertices.is_empty();
  }
}

#[repr(C)]
pub struct Mesh {
  m_name: String,
  m_vertices: Vec<Vertex>,
  m_indices: Vec<u32>,
  m_sub_meshes: Vec<Self>,
}

impl Mesh {
  pub fn new(data: assimp::Scene) -> Self {
    let mut result = Self {
      m_name: "Empty".to_string(),
      m_vertices: vec![],
      m_indices: vec![],
      m_sub_meshes: Vec::new(),
    };
    
    for (position, mesh) in data.mesh_iter().enumerate() {
      let mut vertices: Vec<Vertex> = Vec::with_capacity(mesh.num_vertices as usize);
      vertices.resize(mesh.num_vertices as usize, Vertex::default());
      let mut indices: Vec<u32> = Vec::with_capacity((mesh.num_faces * 3) as usize);
      
      for face in mesh.face_iter() {
        indices.push(*face.index(0));
        indices.push(*face.index(1));
        indices.push(*face.index(2));
      }
      
      for (position, vertex) in mesh.vertex_iter().enumerate() {
        vertices[position].m_position = Vec3::new(&[vertex.x, vertex.y, vertex.z]);
        vertices[position].m_entity_id = unsafe { S_ENTITY_ID_COUNTER };
      }
      
      for (position, normal) in mesh.normal_iter().enumerate() {
        vertices[position].m_normal = Vec3::new(&[normal.x, normal.y, normal.z]);
      }
      
      for (position, texture_coord) in mesh.texture_coords_iter(0).enumerate() {
        vertices[position].m_texture_coords = Vec2::new(&[texture_coord.x, texture_coord.y]);
      }
      
      unsafe { S_ENTITY_ID_COUNTER += 1 };
      
      if position == 0 {
        result.m_name = String::from(mesh.name.as_ref());
        result.m_vertices = vertices;
        result.m_indices = indices;
        continue;
      }
      result.m_sub_meshes.push(Self {
        m_name: String::from(mesh.name.as_ref()),
        m_vertices: vertices,
        m_indices: indices,
        m_sub_meshes: Vec::new(),
      });
    }
    
    return result;
  }
}

impl TraitPrimitive for Mesh {
  fn total_vertex_count(&self) -> usize {
    return self.m_vertices.len() + (!self.m_sub_meshes.is_empty()).then(|| {
      let mut count = 0;
      for sub in self.m_sub_meshes.iter() {
        count += sub.total_vertex_count();
      }
      return count;
    }).unwrap_or(0);
  }
  
  fn total_index_count(&self) -> usize {
    return self.m_indices.len() + (!self.m_sub_meshes.is_empty()).then(|| {
      let mut count = 0;
      for sub in self.m_sub_meshes.iter() {
        count += sub.total_index_count();
      }
      return count;
    }).unwrap_or(0);
  }
  
  fn get_name(&self) -> &str {
    return &self.m_name;
  }
  
  fn get_vertices(&self) -> &Vec<Vertex> {
    return &self.m_vertices;
  }
  
  fn get_indices(&self) -> &Vec<u32> {
    return &self.m_indices;
  }
  
  fn has_sub_primitives(&self) -> bool {
    return !self.m_sub_meshes.is_empty();
  }
  
  fn get_sub_meshes_ref(&self) -> Option<&Vec<Mesh>> {
    return Some(&self.m_sub_meshes);
  }
  
  fn get_sub_meshes_mut(&mut self) -> Option<&mut Vec<Mesh>> {
    return Some(&mut self.m_sub_meshes);
  }
  
  fn get_entity_id(&self) -> u32 {
    return (!self.m_vertices.is_empty()).then(|| self.m_vertices[0].m_entity_id)
      .unwrap_or(0);
  }
  
  fn is_empty(&self) -> bool {
    return self.m_vertices.is_empty();
  }
}

pub struct REntity {
  pub(crate) m_renderer_id: u64,
  pub(crate) m_data: Box<dyn TraitPrimitive>,
  pub(crate) m_type: EnumPrimitiveType,
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
        m_indices: vec![],
      }),
      m_renderer_id: u64::MAX,
      m_type: EnumPrimitiveType::Sprite,
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
          m_renderer_id: u64::MAX,
          m_data: data,
          m_type: EnumPrimitiveType::Sprite,
          m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
          m_sent: false,
          m_changed: false,
          m_flat_shaded: false,
        };
      }
      EnumPrimitiveType::Mesh(is_flat_shaded) => {
        new_entity = REntity {
          m_renderer_id: u64::MAX,
          m_data: data,
          m_type: EnumPrimitiveType::Mesh(is_flat_shaded),
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
        size_of::<u32>()                // Entity ID (uint)
          + (size_of::<f32>() * 3)      // Position (Vec3<f32>)
          + (size_of::<f32>() * 3)      // Normal (Vec3<f32>)
          + size_of::<u32>()            // Color (uint)
          + (size_of::<f32>() * 2)      // Vec2<f32
      }
      EnumPrimitiveType::Quad => {
        size_of::<u32>() + (size_of::<f32>() * 3) + size_of::<Color>() + (size_of::<f32>() * 2)
      }
    };
  }
  
  pub fn total_vertex_count(&self) -> usize {
    return self.m_data.total_vertex_count();
  }
  
  pub fn total_index_count(&self) -> usize {
    return self.m_data.total_index_count();
  }
  
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
  
  pub fn apply(&mut self, shader_associated: &mut Shader) -> Result<(), renderer::EnumRendererError> {
    let renderer = Engine::get_active_renderer();
    
    renderer.enqueue(self, shader_associated)?;
    self.m_sent = true;
    self.m_changed = false;
    return Ok(());
  }
  
  pub fn resend(&mut self, _shader_associated: &mut Shader) -> Result<(), renderer::EnumRendererError> {
    todo!()
  }
  
  pub fn hide(&mut self, primitive_index_selected: Option<usize>) {
    if self.m_sent {
      let renderer = Engine::get_active_renderer();
      renderer.hide(self.get_uuid(), primitive_index_selected);
    }
  }
  
  pub fn show(&mut self, primitive_index_selected: Option<usize>) {
    if self.m_sent {
      let renderer = Engine::get_active_renderer();
      renderer.show(self.get_uuid(), primitive_index_selected);
    }
  }
  
  pub fn free(&mut self, primitive_index_selected: Option<usize>) -> Result<(), renderer::EnumRendererError> {
    if self.m_sent {
      let renderer = Engine::get_active_renderer();
      
      renderer.dequeue(self.get_uuid(), primitive_index_selected)?;
      self.m_sent = false;
      self.m_changed = false;
      return Ok(());
    }
    return Ok(());
  }
  
  pub fn is_sent(&self) -> bool {
    return self.m_sent;
  }
  
  pub fn has_changed(&self) -> bool {
    return self.m_changed;
  }
  
  pub fn get_sub_meshes_ref(&self) -> Option<&Vec<Mesh>> {
    return self.m_data.get_sub_meshes_ref();
  }
  
  pub fn get_sub_meshes_mut(&mut self) -> Option<&mut Vec<Mesh>> {
    return self.m_data.get_sub_meshes_mut();
  }
  
  pub fn get_uuid(&self) -> u64 {
    return self.m_renderer_id;
  }
  
  pub fn get_matrix(&self) -> Mat4 {
    return Mat4::apply_transformations(&self.m_transform[0],
      &self.m_transform[1], &self.m_transform[2]);
  }
  
  pub(crate) fn set_uuid(&mut self, renderer_id: u64) {
    self.m_renderer_id = renderer_id;
  }
}

///////////////////////////////////   DISPLAY  ///////////////////////////////////

impl Display for Mesh {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Vertices:\t{1}\n{0:115}Indices:\t{2}", "", self.total_vertex_count(), self.total_index_count())?;
    return Ok(());
  }
}

impl Display for dyn TraitPrimitive {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Vertices:\t{1}\n{0:115}Indices:\t{2}", "", self.get_vertices().len(), self.get_indices().len())?;
    
    if let Some(sub_primitives) = self.get_sub_meshes_ref() {
      for (position, sub_primitive) in sub_primitives.iter().enumerate() {
        write!(f, "\n{1:115}Sub mesh [{0}]:\n{1:117}Vertices:\t{2}\n{1:117}Indices:\t{3}",
          position + 1, "", sub_primitive.m_vertices.len(), sub_primitive.m_indices.len())?;
      }
    }
    return Ok(());
  }
}

impl Display for REntity {
  fn fmt(&self, format: &mut Formatter<'_>) -> std::fmt::Result {
    write!(format, "Type: {0:?}\n{2:115}Sent?: {1}\n{2:115}Data:\n{2:117}{3}", self.m_type, self.m_sent, "", self.m_data)
  }
}

///////////////////////////////////   EQUALITY  ///////////////////////////////////

impl PartialEq for REntity {
  fn eq(&self, other: &Self) -> bool {
    if self.is_empty() {
      return self.m_type == other.m_type && self.m_data.total_vertex_count() == other.m_data.total_vertex_count()
        && self.m_data.total_index_count() == other.total_index_count();
    }
    return self.m_type == other.m_type && self.m_data.get_vertices()[0].get_id() == other.m_data.get_vertices()[0].get_id();
  }
  
  fn ne(&self, other: &Self) -> bool {
    return !self.eq(other);
  }
}