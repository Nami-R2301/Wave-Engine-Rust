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
use crate::wave::graphics::renderer::{EnumErrors, TraitSendableEntity};
use crate::wave::graphics::renderer::GlRenderer;

/*
///////////////////////////////////   OpenGL Renderable entity  ///////////////////////////////////
///////////////////////////////////                             ///////////////////////////////////
///////////////////////////////////                             ///////////////////////////////////
 */

static mut S_ENTITIES_ID_CACHE: Lazy<HashSet<u32>> = Lazy::new(|| HashSet::new());

#[derive(Debug, Clone)]
pub struct GlREntity {
  pub m_renderer_id: u32,
  pub m_data: Object,
  m_sent: bool,
}

impl GlREntity {
  pub fn new() -> Self {
    return GlREntity {
      m_renderer_id: u32::MAX,
      m_data: Object::new(),
      m_sent: false,
    };
  }
  
  pub fn from(gl_data: Object) -> Self {
    let mut new_id: u32 = rand::random::<u32>();
    
    // Keep assigning different ids until we have one available.
    unsafe {
      while S_ENTITIES_ID_CACHE.contains(&new_id) {
        new_id = rand::random::<u32>();
      }
    }
    
    return GlREntity {
      m_renderer_id: new_id,
      m_data: gl_data,
      m_sent: false,
    };
  }
  
  pub fn size(&self) -> usize {
    return size_of::<u32>() * self.m_data.m_ids.len() +
      size_of::<f32>() * self.m_data.m_vertices.len() +
      size_of::<f32>() * self.m_data.m_normals.len() +
      size_of::<f32>() * self.m_data.m_colors.len() +
      size_of::<f32>() * self.m_data.m_texture_coords.len();
  }
  
  pub fn is_empty(&self) -> bool {
    return self.m_data.is_empty();
  }
}

impl Display for GlREntity {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[REntity] --> ID => {0}; Sent => {1}; Vertices => {2};",
      self.m_renderer_id, self.m_sent, self.m_data)
  }
}

impl PartialEq for GlREntity {
  fn eq(&self, other: &Self) -> bool {
    return self.m_renderer_id == other.m_renderer_id;
  }
  
  fn ne(&self, other: &Self) -> bool {
    return !self.eq(other);
  }
}

impl TraitSendableEntity for GlREntity {
  fn send(&mut self) -> Result<(), EnumErrors> {
    return match GlRenderer::send(self) {
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
  
  fn resend(&mut self) -> Result<(), EnumErrors> {
    todo!()
  }
  
  fn free(&mut self) -> Result<(), EnumErrors> {
    return match GlRenderer::free(&self.m_renderer_id) {
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

/*
///////////////////////////////////   OpenGL Object (2D OR 3D)  ///////////////////////////////////
///////////////////////////////////                             ///////////////////////////////////
///////////////////////////////////                             ///////////////////////////////////
 */

#[derive(Debug, Clone)]
pub struct Object {
  pub m_ids: Vec<u32>,
  pub m_vertices: Vec<f32>,
  pub m_normals: Vec<f32>,
  pub m_colors: Vec<f32>,
  pub m_texture_coords: Vec<f32>,
}

impl Object {
  pub fn new() -> Self {
    return Object {
      m_ids: Vec::new(),
      m_vertices: Vec::new(),
      m_normals: Vec::new(),
      m_colors: Vec::new(),
      m_texture_coords: Vec::new(),
    };
  }
  
  pub fn is_empty(&self) -> bool {
    return self.m_vertices.is_empty();
  }
  
  pub fn register(&mut self, new_id: u32) {
    for index in 0..self.m_ids.len() {
      self.m_ids[index] = new_id;
    }
  }
}


///////////////////////////////////   DISPLAY  ///////////////////////////////////

impl Display for Object {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[GlVertex3D] --> IDs => {0:#?}, Positions => {1:#?}; Normals => {2:#?};\
     Colors => {3:#?}; Texture coords => {4:#?}", self.m_ids, self.m_vertices, self.m_normals,
      self.m_colors, self.m_texture_coords)
  }
}

///////////////////////////////////   EQUALITY  ///////////////////////////////////

impl PartialEq for Object {
  fn eq(&self, other: &Self) -> bool {
    return self.m_ids == other.m_ids;
  }
}