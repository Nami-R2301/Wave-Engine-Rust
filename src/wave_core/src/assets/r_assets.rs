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
pub enum EnumPrimitive {
  Sprite,
  Mesh(EnumMaterial),
  Quad,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash)]
pub enum EnumMaterial {
  Flat,
  Smooth,
  Noisy,
  Shiny,
  Metallic,
  Matte
}

impl Default for EnumMaterial {
  fn default() -> Self {
    return EnumMaterial::Smooth;
  }
}

impl Default for EnumPrimitive {
  fn default() -> Self {
    return EnumPrimitive::Sprite;
  }
}

pub trait TraitPrimitive {
  fn get_name(&self) -> &str;
  fn get_vertices(&self) -> &Vec<Vertex>;
  fn get_indices(&self) -> &Vec<u32>;
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

impl TraitPrimitive for Sprite {
  fn get_name(&self) -> &str {
    return &self.m_name;
  }
  
  fn get_vertices(&self) -> &Vec<Vertex> {
    return &self.m_vertices;
  }
  
  fn get_indices(&self) -> &Vec<u32> {
    return &self.m_indices;
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
}

impl TraitPrimitive for Mesh {
  fn get_name(&self) -> &str {
    return &self.m_name;
  }
  
  fn get_vertices(&self) -> &Vec<Vertex> {
    return &self.m_vertices;
  }
  
  fn get_indices(&self) -> &Vec<u32> {
    return &self.m_indices;
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
  pub(crate) m_sub_meshes: Vec<Box<dyn TraitPrimitive>>,
  pub(crate) m_type: EnumPrimitive,
  // Transformations applied to the entity, to be eventually applied to the model matrix.
  m_transform: [Vec3<f32>; 3],
  m_sent: bool,
  m_changed: bool,
}

impl REntity {
  pub fn default() -> Result<Self, renderer::EnumRendererError> {
    let mut new_entity: REntity = REntity {
      m_sub_meshes: vec![Box::new(Sprite {
        m_name: "".to_string(),
        m_vertices: vec![],
        m_indices: vec![],
      })],
      m_renderer_id: u64::MAX,
      m_type: EnumPrimitive::default(),
      m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
      m_sent: false,
      m_changed: false,
    };
    
    new_entity.translate(Vec3::new(&[0.0, 0.0, 10.0]));
    return Ok(new_entity);
  }
  
  pub fn new(scene: assimp::Scene, data_type: EnumPrimitive) -> Self {
    let mut data: Vec<Box<dyn TraitPrimitive>> = Vec::with_capacity(scene.num_meshes as usize);
    
    // Offset of indices to shift to the next sub-mesh indices, in order to synchronize indices between sub-meshes
    // and join all sub-mesh indices together all referencing that same primitive to avoid drawing every sub-mesh separately.
    let mut base_index: usize = 0;
    
    for mesh in scene.mesh_iter() {
      let mut vertices: Vec<Vertex> = Vec::with_capacity(mesh.num_vertices as usize);
      vertices.resize(mesh.num_vertices as usize, Vertex::default());
      let mut indices: Vec<u32> = Vec::with_capacity((mesh.num_faces * 3) as usize);
      
      for face in mesh.face_iter() {
        indices.push(*face.index(0) + base_index as u32);
        indices.push(*face.index(1) + base_index as u32);
        indices.push(*face.index(2) + base_index as u32);
      }
      base_index += vertices.len();
      
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
      
      match data_type {
        EnumPrimitive::Sprite => {
          data.push(Box::new(Sprite {
            m_name: String::from(mesh.name.as_ref()),
            m_vertices: vertices,
            m_indices: indices,
          }));
        }
        EnumPrimitive::Mesh(_) => {
          data.push(Box::new(Sprite {
            m_name: String::from(mesh.name.as_ref()),
            m_vertices: vertices,
            m_indices: indices,
          }));
        }
        _ => todo!()
      }
    }
    
    let new_entity: REntity;
    
    match data_type {
      EnumPrimitive::Sprite => {
        new_entity = REntity {
          m_renderer_id: u64::MAX,
          m_sub_meshes: data,
          m_type: EnumPrimitive::Sprite,
          m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
          m_sent: false,
          m_changed: false,
        };
      }
      EnumPrimitive::Mesh(material) => {
        new_entity = REntity {
          m_renderer_id: u64::MAX,
          m_sub_meshes: data,
          m_type: EnumPrimitive::Mesh(material),
          m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
          m_sent: false,
          m_changed: false
        };
      }
      EnumPrimitive::Quad => todo!()
    }
    
    return new_entity;
  }
  
  pub fn get_size(&self) -> usize {
    return match self.m_type {
      EnumPrimitive::Sprite => {
        size_of::<u32>() + (size_of::<f32>() * 3) + (size_of::<f32>() * 3) + size_of::<Color>() +
          (size_of::<f32>() * 2)
      }
      EnumPrimitive::Mesh(_) => {
        size_of::<u32>()                // Entity ID (uint)
          + (size_of::<f32>() * 3)      // Position (Vec3<f32>)
          + (size_of::<f32>() * 3)      // Normal (Vec3<f32>)
          + size_of::<u32>()            // Color (uint)
          + (size_of::<f32>() * 2)      // Vec2<f32
      }
      EnumPrimitive::Quad => {
        size_of::<u32>() + (size_of::<f32>() * 3) + size_of::<Color>() + (size_of::<f32>() * 2)
      }
    };
  }
  
  pub fn get_total_vertex_count(&self) -> usize {
    let mut count = 0;
    for sub_mesh in self.m_sub_meshes.iter() {
      count += sub_mesh.get_vertices().len()
    }
    return count;
  }
  
  pub fn get_total_index_count(&self) -> usize {
    let mut count = 0;
    for sub_mesh in self.m_sub_meshes.iter() {
      count += sub_mesh.get_indices().len()
    }
    return count;
  }
  
  pub fn is_empty(&self) -> bool {
    return self.m_sub_meshes.is_empty();
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

impl Display for dyn TraitPrimitive {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Vertices:\t{1}\n{0:115}Indices:\t{2}", "", self.get_vertices().len(), self.get_indices().len())
  }
}

impl Display for REntity {
  fn fmt(&self, format: &mut Formatter<'_>) -> std::fmt::Result {
    write!(format, "Type: {0:?}\n{2:115}Sent?: {1}\n{2:115}Data:", self.m_type, self.m_sent, "")?;
    
    for (sub_mesh_index, sub_mesh) in self.m_sub_meshes.iter().enumerate() {
      write!(format, "\n{0:117}[{1}]:\n{0:119}{2}", "", sub_mesh_index + 1, sub_mesh)?;
    }
    return Ok(());
  }
}

///////////////////////////////////   EQUALITY  ///////////////////////////////////

impl PartialEq for REntity {
  fn eq(&self, other: &Self) -> bool {
    if self.is_empty() {
      return self.m_type == other.m_type && self.get_total_vertex_count() == other.get_total_vertex_count()
        && self.get_total_index_count() == other.get_total_index_count();
    }
    return self.m_type == other.m_type && self.m_sub_meshes[0].get_vertices()[0].get_id() == other.m_sub_meshes[0].get_vertices()[0].get_id();
  }
  
  fn ne(&self, other: &Self) -> bool {
    return !self.eq(other);
  }
}