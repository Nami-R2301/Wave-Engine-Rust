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
use rand::Rng;

use crate::{Engine, log, TraitFree};
use crate::assets::asset_loader::AssetInfo;
use crate::graphics::renderer::{EnumRendererError, EnumRendererHint, EnumRendererOptimizationMode, EnumRendererRenderPrimitiveAs};
use crate::graphics::shader::Shader;
use crate::graphics::color::Color;
use crate::graphics::texture::{TextureArray};
use crate::math::{Mat4, Vec2, Vec3};
use crate::utils::macros::logger::*;

static mut S_ENTITY_ID_COUNTER: u32 = 0;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash)]
pub enum EnumVertexMemberOffset {
  EntityIDOffset = 0,
  TextureInfoOffset = (EnumVertexMemberOffset::EntityIDOffset as usize) + size_of::<u32>(),
  PositionOffset = (EnumVertexMemberOffset::TextureInfoOffset as usize) + size_of::<i32>(),
  NormalOffset = (EnumVertexMemberOffset::PositionOffset as usize) + (size_of::<f32>() * 3),
  ColorOffset = (EnumVertexMemberOffset::NormalOffset as usize) + size_of::<u32>(),
  TexCoordsOffset = (EnumVertexMemberOffset::ColorOffset as usize) + size_of::<Color>(),
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
  Matte,
}

#[derive(Debug, PartialEq, Hash)]
pub enum EnumPrimitiveMapMethod {
  OnePerSubPrimitive,
  OnePerSubPrimitiveWithOffset(u16),
  OnePerSubPrimitiveReversed,
  Randomized,
  Custom(Vec<(usize, u16)>),
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum EnumSubPrimitivePortion {
  Nothing,
  Some(usize),
  Everything,
}

impl Default for EnumPrimitive {
  fn default() -> Self {
    return EnumPrimitive::Mesh(EnumMaterial::default());
  }
}

impl Default for EnumPrimitiveMapMethod {
  fn default() -> Self {
    return EnumPrimitiveMapMethod::OnePerSubPrimitive;
  }
}

impl Default for EnumMaterial {
  fn default() -> Self {
    return EnumMaterial::Smooth;
  }
}

pub trait TraitPrimitive {
  fn get_name(&self) -> &str;
  fn get_vertices_ref(&self) -> &Vec<Vertex>;
  fn get_vertices_mut(&mut self) -> &mut Vec<Vertex>;
  fn get_indices(&self) -> &Vec<u32>;
  fn get_entity_id(&self) -> u32;
  fn is_empty(&self) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
  // ID to differentiate instances in shaders to apply different textures for example or different transformations.
  pub m_entity_id: u32,
  pub m_texture_info: i32,
  pub m_position: Vec3<f32>,
  pub m_normal: u32,
  pub m_color: Color,
  pub m_texture_coords: Vec2<f32>,
}

impl Vertex {
  pub fn default() -> Self {
    return Self {
      m_entity_id: 0,
      m_texture_info: -1,
      m_position: Vec3::default(),
      m_normal: 0,
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
    self.m_texture_info = -1;
    self.m_normal = 0;
    self.m_texture_coords = Vec2::default();
    self.m_color = Color::default();
  }
}

pub struct Sprite {
  m_name: String,
  m_vertices: Vec<Vertex>,
  m_indices: Vec<u32>,
}

impl TraitPrimitive for Sprite {
  fn get_name(&self) -> &str {
    return &self.m_name;
  }
  
  fn get_vertices_ref(&self) -> &Vec<Vertex> {
    return &self.m_vertices;
  }
  
  fn get_vertices_mut(&mut self) -> &mut Vec<Vertex> {
    return &mut self.m_vertices;
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

pub struct Mesh {
  m_name: String,
  m_vertices: Vec<Vertex>,
  m_indices: Vec<u32>,
}

impl TraitPrimitive for Mesh {
  fn get_name(&self) -> &str {
    return &self.m_name;
  }
  fn get_vertices_ref(&self) -> &Vec<Vertex> {
    return &self.m_vertices;
  }
  
  fn get_vertices_mut(&mut self) -> &mut Vec<Vertex> {
    return &mut self.m_vertices;
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
  pub(crate) m_name: &'static str,
  pub(crate) m_sub_meshes: Vec<Box<dyn TraitPrimitive>>,
  pub(crate) m_type: EnumPrimitive,
  pub(crate) m_primitive_mode: EnumRendererRenderPrimitiveAs,
  m_last_primitive_mode: EnumRendererRenderPrimitiveAs,
  // Transformations applied to the entity, to be eventually applied to the model matrix.
  m_transform: [Vec3<f32>; 3],
  m_sent: bool,
  m_changed: bool,
}

impl Default for REntity {
  fn default() -> Self {
    let mut vertices: [Vertex; 36] = [Vertex {
      m_entity_id: 0,
      m_texture_info: 0,
      m_position: Vec3::default(),
      m_normal: 0,
      m_color: Color::default(),
      m_texture_coords: Vec2::default(),
    }; 36];
    
    let positions =
      [Vec3::new(&[0.5, -0.5, -0.5]), Vec3::new(&[0.5, -0.5, 0.5]),
        Vec3::new(&[-0.5, -0.5, 0.5]), Vec3::new(&[-0.5, -0.5, -0.5]),
        Vec3::new(&[0.5, 0.5, -0.5]), Vec3::new(&[0.5, 0.5, 0.5]),
        Vec3::new(&[-0.5, 0.5, 0.5]), Vec3::new(&[-0.5, 0.5, -0.5])];
    
    let normals: [Vec3<f32>; 7] =
      [Vec3::new(&[0.0, -1.0, 0.0]), Vec3::new(&[0.0, 1.0, 0.0]),
        Vec3::new(&[1.0, 0.0, 0.00001]), Vec3::new(&[0.0, 0.0, 1.0]),
        Vec3::new(&[-1.0, 0.0, 0.0]), Vec3::new(&[0.0, 0.0, -1.0]),
        Vec3::new(&[1.0, 0.0, 0.0])];
    
    let tex_coords =
      [Vec2::new(&[0.0, 0.0]), Vec2::new(&[1.0, 0.0]),
        Vec2::new(&[1.0, 1.0]), Vec2::new(&[0.0, 1.0])];
    
    for index in 0..positions.len() {
      vertices[index].m_position = positions[index];
    }
    
    for index in 0..normals.len() {
      let x_sign = normals[index].x.is_sign_negative().then(|| 0x1)
        .unwrap_or(0);
      let y_sign = normals[index].y.is_sign_negative().then(|| 0x2)
        .unwrap_or(0);
      let z_sign = normals[index].z.is_sign_negative().then(|| 0x8)
        .unwrap_or(0);
      
      let x_normal_f = normals[index].x.is_sign_negative().then(|| normals[index].x * -100.0)
        .unwrap_or(normals[index].x * 100.0);
      let y_normal_f = normals[index].y.is_sign_negative().then(|| normals[index].y * -100.0)
        .unwrap_or(normals[index].y * 100.0);
      let z_normal_f = normals[index].z.is_sign_negative().then(|| normals[index].z * -100.0)
        .unwrap_or(normals[index].z * 100.0);
      
      let x_normal = (x_normal_f as u32) << 24;
      let y_normal = (y_normal_f as u32) << 16;
      let z_normal = (z_normal_f as u32) << 8;
      
      vertices[index].m_normal = x_normal + y_normal + x_sign + y_sign + z_sign + z_normal;
    }
    
    for index in 0..tex_coords.len() {
      // let x_sign = (tex_coords[index].x >= 0.0).then(|| 0)
      //   .unwrap_or(1) << 31;
      // let y_sign = (tex_coords[index].y >= 0.0).then(|| 0)
      //   .unwrap_or(1) << 15;
      //
      // let x_tex_coord = ((tex_coords[index].x * 100.0) as u32) << 12;
      // let y_tex_coord = ((tex_coords[index].y * 100.0) as u32) << 4;
      //
      // vertices[index].m_texture_coords = x_tex_coord + y_tex_coord + x_sign + y_sign;
      vertices[index].m_texture_coords = Vec2::new(&[tex_coords[index].x, tex_coords[index].y]);
    }
    
    let faces = [1, 0, 0,
      2, 1, 0, 3, 2, 0, 4, 2, 1,
      7, 3, 1, 6, 0, 1, 4, 2, 2,
      5, 3, 2, 5, 3, 2, 1, 0, 2,
      1, 1, 3, 5, 2, 3, 6, 3, 3,
      6, 2, 4, 7, 3, 4, 3, 0, 4,
      4, 3, 5, 0, 0, 5, 3, 1, 5,
      0, 3, 0, 1, 0, 0, 3, 2, 0,
      5, 1, 1, 4, 2, 1, 6, 0, 1,
      0, 1, 6, 4, 2, 6, 1, 0, 6,
      2, 0, 3, 1, 1, 3, 6, 3, 3,
      2, 1, 4, 6, 2, 4, 3, 0, 4,
      7, 2, 5, 4, 2, 5, 3, 1, 5];
    
    let mut new_entity: REntity = REntity {
      m_sub_meshes: vec![Box::new(Mesh {
        m_name: "Default Cube".to_string(),
        m_vertices: Vec::from(vertices),
        m_indices: Vec::from(faces),
      })],
      m_renderer_id: u64::MAX,
      m_name: "Default Cube",
      m_type: EnumPrimitive::default(),
      m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
      m_primitive_mode: EnumRendererRenderPrimitiveAs::Filled,
      m_last_primitive_mode: EnumRendererRenderPrimitiveAs::Filled,
      m_sent: false,
      m_changed: false,
    };
    
    new_entity.translate(0.0, 0.0, 10.0);
    return new_entity;
  }
}

impl TraitFree<EnumRendererError> for REntity {
  fn free(&mut self) -> Result<(), EnumRendererError> {
    if self.m_sent {
      let renderer = Engine::get_active_renderer();
      
      renderer.dequeue(self.get_uuid(), None)?;
      self.m_sent = false;
      self.m_changed = false;
      return Ok(());
    }
    return Ok(());
  }
}

impl REntity {
  pub fn new(asset_info: AssetInfo, data_type: EnumPrimitive, name: &'static str) -> Self {
    let mut data: Vec<Box<dyn TraitPrimitive>> = Vec::with_capacity(asset_info.m_data.num_meshes as usize);
    
    // Offset of indices to shift to the next sub-mesh indices, in order to synchronize indices between sub-meshes
    // and join all sub-mesh indices together all referencing that same primitive to avoid drawing every sub-mesh separately.
    let mut base_index: usize = 0;
    
    for mesh in asset_info.m_data.mesh_iter() {
      let mut vertices: Vec<Vertex> = Vec::with_capacity(mesh.num_vertices as usize);
      vertices.resize(mesh.num_vertices as usize, Vertex::default());
      let mut indices: Vec<u32> = Vec::with_capacity((mesh.num_faces * 3) as usize);
      
      if asset_info.m_is_indexed {
        for face in mesh.face_iter() {
          indices.push(face[0] + base_index as u32);
          indices.push(face[1] + base_index as u32);
          indices.push(face[2] + base_index as u32);
        }
        base_index += vertices.len();
      }
      
      for (position, vertex) in mesh.vertex_iter().enumerate() {
        vertices[position].m_position = Vec3::new(&[vertex.x, vertex.y, vertex.z]);
        vertices[position].m_entity_id = unsafe { S_ENTITY_ID_COUNTER };
      }
      
      for (position, normal) in mesh.normal_iter().enumerate() {
        let x_sign = normal.x.is_sign_negative().then(|| 0x1)
          .unwrap_or(0);
        let y_sign = normal.y.is_sign_negative().then(|| 0x2)
          .unwrap_or(0);
        let z_sign = normal.z.is_sign_negative().then(|| 0x8)
          .unwrap_or(0);
        
        let x_normal_f = normal.x.is_sign_negative().then(|| normal.x * -100.0)
          .unwrap_or(normal.x * 100.0);
        let y_normal_f = normal.y.is_sign_negative().then(|| normal.y * -100.0)
          .unwrap_or(normal.y * 100.0);
        let z_normal_f = normal.z.is_sign_negative().then(|| normal.z * -100.0)
          .unwrap_or(normal.z * 100.0);
        
        let x_normal = (x_normal_f as u32) << 24;
        let y_normal = (y_normal_f as u32) << 16;
        let z_normal = (z_normal_f as u32) << 8;
        
        vertices[position].m_normal = x_normal + y_normal + x_sign + y_sign + z_sign + z_normal;
      }
      
      for (position, texture_coord) in mesh.texture_coords_iter(0).enumerate() {
        // let x_sign = texture_coord.x.is_sign_negative().then(|| 0x1)
        //   .unwrap_or(0);
        // let y_sign = texture_coord.y.is_sign_negative().then(|| 0x2)
        //   .unwrap_or(0);
        //
        // let x_tex_coord = texture_coord.x.is_sign_negative().then(|| ((texture_coord.x * -100.0) as u32) << 16)
        //   .unwrap_or(((texture_coord.x * 1000.0) as u32) << 16);
        // let y_tex_coord = texture_coord.y.is_sign_negative().then(|| ((texture_coord.y * -1000.0) as u32) << 4)
        //   .unwrap_or(((texture_coord.y * 1000.0) as u32) << 4);
        //
        // vertices[position].m_texture_coords = x_sign + x_tex_coord + y_sign + y_tex_coord;
        
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
          data.push(Box::new(Mesh {
            m_name: String::from(mesh.name.as_ref()),
            m_vertices: vertices,
            m_indices: indices,
          }));
        }
        _ => todo!()
      }
    }
    
    return REntity {
      m_renderer_id: u64::MAX,
      m_name: name,
      m_sub_meshes: data,
      m_type: data_type,
      m_transform: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
      m_primitive_mode: EnumRendererRenderPrimitiveAs::Filled,
      m_last_primitive_mode: EnumRendererRenderPrimitiveAs::Filled,
      m_sent: false,
      m_changed: false,
    };
  }
  
  pub fn get_size(&self) -> usize {
    return match self.m_type {
      EnumPrimitive::Sprite | EnumPrimitive::Quad => {
        size_of::<u32>() + (size_of::<f32>() * 2) + size_of::<u32>() + (size_of::<f32>() * 2)
      }
      EnumPrimitive::Mesh(_) => size_of::<Vertex>()
    };
  }
  
  pub fn get_name(&self) -> &str {
    return self.m_name;
  }
  
  pub fn get_primitive_mode(&self) -> EnumRendererRenderPrimitiveAs {
    return self.m_primitive_mode;
  }
  
  pub fn get_primitive_count(&self) -> usize {
    return self.m_sub_meshes.len();
  }
  
  pub fn get_total_vertex_count(&self) -> usize {
    let mut count = 0;
    for sub_mesh in self.m_sub_meshes.iter() {
      count += sub_mesh.get_vertices_ref().len()
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
  
  pub fn toggle_primitive_mode(&mut self, view_as: EnumRendererRenderPrimitiveAs) {
    if self.m_primitive_mode != view_as {
      self.m_primitive_mode = view_as;
      self.m_changed = true;
    }
  }
  
  pub fn is_empty(&self) -> bool {
    return self.m_sub_meshes.is_empty();
  }
  
  pub fn translate(&mut self, amount_x: f32, amount_y: f32, amount_z: f32) {
    self.m_transform[0] += Vec3::new(&[amount_x, amount_y, -amount_z]);
    self.m_changed = true;
  }
  
  pub fn rotate(&mut self, amount_x: f32, amount_y: f32, amount_z: f32) {
    // Inverse x and y to correspond to the right orientation.
    self.m_transform[1] += Vec3::new(&[amount_y, amount_x, -amount_z]);
    self.m_changed = true;
  }
  
  pub fn scale(&mut self, amount_x: f32, amount_y: f32, amount_z: f32) {
    self.m_transform[2] += Vec3::new(&[amount_y, amount_x, amount_z]);
    self.m_changed = true;
  }
  
  pub fn map_texture_array(&mut self, texture_array: &TextureArray, primitive_mapping_method: EnumPrimitiveMapMethod) {
    return match primitive_mapping_method {
      EnumPrimitiveMapMethod::OnePerSubPrimitive => {
        for texture in texture_array.m_textures.iter() {
          let texture_size = texture.m_data.width;
          let texture_depth = texture.m_type.get_depth();
          
          let shifted_texture_size: i32 = (texture_size as i32) << 16;
          let shifted_end_depth: i32 = ((texture_depth + 1) as i32) << 8;
          let mut shifted_start_depth: i32 = texture_depth as i32;
          
          if let Some(primitive) = self.m_sub_meshes.get_mut(texture_depth as usize) {
            for vertex in primitive.get_vertices_mut() {
              if vertex.m_texture_info != -1 {
                shifted_start_depth = vertex.m_texture_info & 0x000000FF;
              }
              vertex.m_texture_info = shifted_texture_size + shifted_end_depth + shifted_start_depth;
            }
            log!(EnumLogColor::Blue, "DEBUG", "[RAsset] -->\t Texture size: {0}, texture depth: {1}\n{2:115}Texture shift: {3}",
        texture_size, texture_depth, "", shifted_texture_size + shifted_end_depth + shifted_start_depth);
          }
        }
      }
      EnumPrimitiveMapMethod::OnePerSubPrimitiveWithOffset(offset) => {
        for (position, texture) in texture_array.m_textures.iter().enumerate() {
          let texture_size = texture.m_data.width;
          let texture_depth = texture.m_type.get_depth() + offset;
          
          let shifted_texture_size: i32 = (texture_size as i32) << 16;
          let shifted_end_depth: i32 = ((texture_depth + 1) as i32) << 8;
          let mut shifted_start_depth: i32 = texture_depth as i32;
          
          if let Some(primitive) = self.m_sub_meshes.get_mut(position) {
            for vertex in primitive.get_vertices_mut() {
              if vertex.m_texture_info != -1 {
                shifted_start_depth = vertex.m_texture_info & 0x000000FF;
              }
              vertex.m_texture_info = shifted_texture_size + shifted_end_depth + shifted_start_depth;
            }
            log!(EnumLogColor::Blue, "DEBUG", "[RAsset] -->\t Texture size: {0}, texture depth: {1}\n{2:115}Texture shift: {3}",
        texture_size, texture_depth, "", shifted_texture_size + shifted_end_depth + shifted_start_depth);
          }
        }
      }
      EnumPrimitiveMapMethod::OnePerSubPrimitiveReversed => {
        for (position, texture) in texture_array.m_textures.iter().rev().enumerate() {
          let texture_size = texture.m_data.width;
          let texture_depth = texture.m_type.get_depth();
          
          let shifted_texture_size: i32 = (texture_size as i32) << 16;
          let shifted_end_depth: i32 = ((texture_depth + 1) as i32) << 8;
          let mut shifted_start_depth: i32 = texture_depth as i32;
          
          if let Some(primitive) = self.m_sub_meshes.get_mut(position) {
            for vertex in primitive.get_vertices_mut() {
              if vertex.m_texture_info != -1 {
                shifted_start_depth = vertex.m_texture_info & 0x000000FF;
              }
              vertex.m_texture_info = shifted_texture_size + shifted_end_depth + shifted_start_depth;
            }
            log!(EnumLogColor::Blue, "DEBUG", "[RAsset] -->\t Texture size: {0}, texture depth: {1}\n{2:115}Texture shift: {3}",
        texture_size, texture_depth, "", shifted_texture_size + shifted_end_depth + shifted_start_depth);
          }
        }
      }
      EnumPrimitiveMapMethod::Randomized => {
        let mut range = rand::thread_rng();
        
        for texture in texture_array.m_textures.iter() {
          let texture_size = texture.m_data.width;
          let texture_depth = range.gen_range(0..texture_array.m_max_depth);
          
          let shifted_texture_size: i32 = (texture_size as i32) << 16;
          let shifted_end_depth: i32 = ((texture_depth + 1) as i32) << 8;
          let mut shifted_start_depth: i32 = texture_depth as i32;
          
          if let Some(primitive) = self.m_sub_meshes.get_mut(texture.m_type.get_depth() as usize) {
            for vertex in primitive.get_vertices_mut() {
              if vertex.m_texture_info != -1 {
                shifted_start_depth = vertex.m_texture_info & 0x000000FF;
              }
              vertex.m_texture_info = shifted_texture_size + shifted_end_depth + shifted_start_depth;
            }
            log!(EnumLogColor::Blue, "DEBUG", "[RAsset] -->\t Texture size: {0}, texture depth: {1}\n{2:115}Texture shift: {3}",
        texture_size, texture_depth, "", shifted_texture_size + shifted_end_depth + shifted_start_depth);
          }
        }
      }
      EnumPrimitiveMapMethod::Custom(map) => {
        for (primitive_index, texture_index) in map.into_iter() {
          let texture_size = texture_array.m_textures[texture_index as usize].m_data.width;
          let texture_depth = texture_array.m_textures[texture_index as usize].m_type.get_depth();
          
          let shifted_texture_size: i32 = (texture_size as i32) << 20;
          let shifted_end_depth: i32 = (texture_depth as i32) << 8;
          let mut shifted_start_depth: i32 = texture_depth as i32;
          
          if let Some(primitive) = self.m_sub_meshes.get_mut(primitive_index) {
            for vertex in primitive.get_vertices_mut() {
              if vertex.m_texture_info != -1 {
                shifted_start_depth = vertex.m_texture_info & 0x000003FF;
              }
              vertex.m_texture_info = shifted_texture_size + shifted_end_depth + shifted_start_depth;
            }
            return;
          }
          log!(EnumLogColor::Red, "ERROR", "[Asset] -->\t Cannot add texture at index {0}, index out of bounds!", primitive_index);
        }
      }
    };
  }
  
  pub fn unmap_texture_at(&mut self, primitive_mapping: Option<Vec<usize>>) {
    if primitive_mapping.is_none() {
      for primitive in self.m_sub_meshes.iter_mut() {
        for vertex in primitive.get_vertices_mut() {
          vertex.m_texture_info = -1;
        }
      }
      return;
    }
    
    for primitive_index in primitive_mapping.unwrap() {
      if let Some(sub_primitive) = self.m_sub_meshes.get_mut(primitive_index) {
        for vertex in sub_primitive.get_vertices_mut() {
          vertex.m_texture_info = -1;
        }
        return;
      }
      log!(EnumLogColor::Red, "ERROR", "[Asset] -->\t Cannot add texture at index {0}, index out of bounds!", primitive_index);
      return;
    }
  }
  
  pub fn apply(&mut self, shader_associated: &mut Shader) -> Result<(), EnumRendererError> {
    let renderer = Engine::get_active_renderer();
    
    renderer.enqueue(self, shader_associated)?;
    
    self.m_sent = true;
    self.m_changed = false;
    return Ok(());
  }
  
  pub fn reapply(&mut self) -> Result<(), EnumRendererError> {
    if self.m_changed && self.m_sent {
      let renderer = Engine::get_active_renderer();
      let matrix = self.get_matrix();
      
      if renderer.m_hints.contains(&EnumRendererHint::Optimization(EnumRendererOptimizationMode::All)) ||
        renderer.m_hints.contains(&EnumRendererHint::Optimization(EnumRendererOptimizationMode::MinimizeDrawCalls)) {
        renderer.update_ubo_model(matrix, self.m_sub_meshes.first().unwrap().get_entity_id() as u64, None, self.m_sub_meshes.len())?;
      } else {
        renderer.update_ubo_model(matrix, self.m_renderer_id, None, self.m_sub_meshes.len())?;
      }
      
      if self.m_last_primitive_mode != self.m_primitive_mode {
        if renderer.m_hints.contains(&EnumRendererHint::Optimization(EnumRendererOptimizationMode::All)) ||
          renderer.m_hints.contains(&EnumRendererHint::Optimization(EnumRendererOptimizationMode::MinimizeDrawCalls)) {
          renderer.toggle_primitive_mode(self.m_name, self.m_primitive_mode, self.m_sub_meshes.first().unwrap().get_entity_id() as u64,
            None, self.m_sub_meshes.len())?;
        } else {
          renderer.toggle_primitive_mode(self.m_name, self.m_primitive_mode, self.m_renderer_id, None, self.m_sub_meshes.len())?;
        }
        self.m_last_primitive_mode = self.m_primitive_mode;
      }
      
      self.m_changed = false;
    }
    return Ok(());
  }
  
  pub fn hide(&mut self, sub_primitive_selected: EnumSubPrimitivePortion) {
    if self.m_sent {
      let renderer = Engine::get_active_renderer();
      
      return match sub_primitive_selected {
        EnumSubPrimitivePortion::Nothing => {}
        EnumSubPrimitivePortion::Some(sub_primitive_index) => {
          if sub_primitive_index < self.m_sub_meshes.len() {
            let _ = renderer.hide(self.m_renderer_id, Some(sub_primitive_index));
          }
        }
        EnumSubPrimitivePortion::Everything => {
          let _ = renderer.hide(self.m_renderer_id, None);
        }
      };
    }
  }
  
  pub fn show(&mut self, sub_primitive_selected: EnumSubPrimitivePortion) {
    if self.m_sent {
      let renderer = Engine::get_active_renderer();
      
      return match sub_primitive_selected {
        EnumSubPrimitivePortion::Nothing => {}
        EnumSubPrimitivePortion::Some(sub_primitive_index) => {
          if sub_primitive_index < self.m_sub_meshes.len() {
            let _ = renderer.show(self.m_renderer_id, Some(sub_primitive_index));
          }
        }
        EnumSubPrimitivePortion::Everything => {
          let _ = renderer.show(self.m_renderer_id, None);
        }
      };
    }
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
}

///////////////////////////////////   DISPLAY  ///////////////////////////////////

impl Display for dyn TraitPrimitive {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Vertices:\t{1}\n{0:115}Indices:\t{2}", "", self.get_vertices_ref().len(), self.get_indices().len())
  }
}

impl Display for REntity {
  fn fmt(&self, format: &mut Formatter<'_>) -> std::fmt::Result {
    write!(format, "UUID: {3}\n{2:113}Type: {0:?}\n{2:113}Sent?: {1}\n{2:113}Data:", self.m_type, self.m_sent, "", self.get_uuid())?;
    
    for (sub_mesh_index, sub_mesh) in self.m_sub_meshes.iter().enumerate() {
      write!(format, "\n{0:113}[{1}]:\n{0:115}{2}", "", sub_mesh_index + 1, sub_mesh)?;
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
    return self.m_type == other.m_type &&
      self.m_sub_meshes[0].get_vertices_ref()[0].get_id() == other.m_sub_meshes[0].get_vertices_ref()[0].get_id();
  }
  
  fn ne(&self, other: &Self) -> bool {
    return !self.eq(other);
  }
}