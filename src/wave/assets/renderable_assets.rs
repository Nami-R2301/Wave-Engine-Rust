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
use std::fmt::{Debug, Display};
use std::mem::size_of;

use once_cell::sync::Lazy;

use crate::log;
use crate::wave::graphics::renderer::{EnumErrors, Renderer};
use crate::wave::graphics::shader::TraitShader;
use crate::wave::math::{Mat4, Vec3};

/*
///////////////////////////////////   OpenGL Renderable entity  ///////////////////////////////////
///////////////////////////////////                             ///////////////////////////////////
///////////////////////////////////                             ///////////////////////////////////
 */

static mut S_ENTITIES_ID_CACHE: Lazy<HashSet<u32>> = Lazy::new(|| HashSet::new());

pub trait TraitRenderableEntity {
  fn send(&mut self, shader_associated: &mut dyn TraitShader) -> Result<(), EnumErrors>;
  fn resend(&mut self, shader_associated: &mut dyn TraitShader) -> Result<(), EnumErrors>;
  fn free(&mut self, shader_associated: &mut dyn TraitShader) -> Result<(), EnumErrors>;
  fn is_sent(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct REntity {
  // Mouse picking ID per vertex.
  pub m_entity_id: Vec<u32>,
  pub m_vertices: Vec<f32>,
  pub m_normals: Vec<f32>,
  pub m_colors: Vec<f32>,
  pub m_texture_coords: Vec<f32>,
  // UUID given by the renderer to differentiate entities in batch rendering.
  m_renderer_id: u64,
  m_model_matrix: Mat4,
  // Transformations applied to the entity, to be eventually applied to the model matrix.
  m_transform: [Vec3<f32>; 3],
  m_sent: bool,
}

impl REntity {
  pub fn new() -> Self {
    return REntity {
      m_renderer_id: u64::MAX,
      m_entity_id: Vec::new(),
      m_vertices: Vec::new(),
      m_normals: Vec::new(),
      m_colors: Vec::new(),
      m_texture_coords: Vec::new(),
      m_model_matrix: Mat4::new(1.0),
      m_transform: [Vec3::new(), Vec3::new(), Vec3::from(&[1.0, 1.0, 1.0])],
      m_sent: false,
    };
  }
  
  pub fn size(&self) -> usize {
    return (size_of::<u32>() * self.m_entity_id.len()) +
      (size_of::<f32>() * self.m_vertices.len()) +
      (size_of::<f32>() * self.m_normals.len()) +
      (size_of::<f32>() * self.m_colors.len()) +
      (size_of::<f32>() * self.m_texture_coords.len());
  }
  
  pub fn count(&self) -> usize {
    return self.m_entity_id.len();
  }
  
  pub fn is_empty(&self) -> bool {
    return self.m_vertices.is_empty();
  }
  
  pub fn register(&mut self) {
    let mut new_id = rand::random::<u32>();
    unsafe {
      while S_ENTITIES_ID_CACHE.contains(&new_id) {
        new_id = rand::random::<u32>();
      }
    }
    
    if self.m_vertices.len() % 3 == 0 {
      self.m_entity_id = Vec::with_capacity(self.m_vertices.len() / 3);
    } else {
      self.m_entity_id = Vec::with_capacity(self.m_vertices.len() / 2);
    }
    
    for _index in 0..self.m_entity_id.capacity() {
      self.m_entity_id.push(new_id);
    }
  }
  
  pub fn translate(&mut self, amount: Vec3<f32>) {
    self.m_transform[0] = amount;
  }
  
  pub fn rotate(&mut self, amount: Vec3<f32>) {
    // Inverse x and y to correspond to the right orientation.
    self.m_transform[1].x = amount.x;
    self.m_transform[1].y = amount.y;
    self.m_transform[1].z = amount.z;
  }
  
  pub fn scale(&mut self, amount: Vec3<f32>) {
    self.m_transform[2] = amount;
  }
  
  pub fn get_matrix(&mut self) -> &Mat4 {
    self.m_model_matrix = Mat4::apply_model(&self.m_transform[0],
      &self.m_transform[1], &self.m_transform[2]);
    return &self.m_model_matrix;
  }
}

///////////////////////////////////   DISPLAY  ///////////////////////////////////

impl Display for REntity {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[REntity] --> \nSent => {0}\nIDs => {1:#?}\nPositions => {2:#?}\n\
     Normals => {3:#?}\nColors => {4:#?}\nTexture coords => {5:#?}", self.m_sent, self.m_entity_id,
      self.m_vertices, self.m_normals, self.m_colors, self.m_texture_coords)
  }
}

///////////////////////////////////   EQUALITY  ///////////////////////////////////

impl PartialEq for REntity {
  fn eq(&self, other: &Self) -> bool {
    return self.m_entity_id == other.m_entity_id;
  }
  
  fn ne(&self, other: &Self) -> bool {
    return !self.eq(other);
  }
}

impl TraitRenderableEntity for REntity {
  fn send(&mut self, shader_associated: &mut dyn TraitShader) -> Result<(), EnumErrors> {
    let renderer = Renderer::get().as_mut()
      .expect("[REntity] -->\t Cannot send REntity, renderer is null! Exiting...");
    
    return match renderer.m_api.send(self, shader_associated) {
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
  
  fn resend(&mut self, _shader_associated: &mut dyn TraitShader) -> Result<(), EnumErrors> {
    todo!()
  }
  
  fn free(&mut self, _shader_associated: &mut dyn TraitShader) -> Result<(), EnumErrors> {
    let renderer = Renderer::get().as_mut()
      .expect("[REntity] -->\t Cannot free REntity, renderer is null! Exiting...");
    
    return match renderer.m_api.free(&self.m_renderer_id) {
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