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

use crate::create_vec;
use crate::wave::math::{Vec2, Vec3};

// Color component (rgba) for vertices.
create_vec!(Color<T> {r, g, b, a, });
impl<T> std::ops::Index<usize> for Color<T> {
  type Output = T;
  
  fn index(&self, index: usize) -> &T {
    return match index {
      0 => &self.r,
      1 => &self.g,
      2 => &self.b,
      3 => &self.a,
      _ => &self.r,
    };
  }
}

impl<T> std::ops::IndexMut<usize> for Color<T> {
  fn index_mut(&mut self, index: usize) -> &mut T {
    return match index {
      0 => &mut self.r,
      1 => &mut self.g,
      2 => &mut self.b,
      3 => &mut self.a,
      _ => &mut self.r,
    };
  }
}

/*
///////////////////////////////////   OpenGL VERTEX 2D  ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
 */

#[derive(Debug, Clone)]
pub struct GlVertex2D {
  pub m_id: i32,
  pub m_position: Vec2<f32>,
  pub m_color: Color<f32>,
  pub m_texture_coords: Vec2<f32>
}

impl GlVertex2D {
  pub fn new() -> Self {
    return GlVertex2D {
      m_id: -1,
      m_position: Vec2::new(),
      m_color: Color::new(),
      m_texture_coords: Vec2::new(),
    }
  }
  
  pub fn register(&mut self, new_id: i32) {
    self.m_id = new_id;
  }
}

///////////////////////////////////   DISPLAY  ///////////////////////////////////

impl std::fmt::Display for GlVertex2D {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[GlVertex3D] --> ID => {0}; Position => {1}; Color => {2}; \
    Texture coords => {3}", self.m_id, self.m_position, self.m_color, self.m_texture_coords)
  }
}

///////////////////////////////////   EQUALITY  ///////////////////////////////////

impl PartialEq for GlVertex2D {
  fn eq(&self, other: &Self) -> bool {
    return self.m_id == other.m_id;
  }
}

/*
///////////////////////////////////   OpenGL VERTEX 3D  ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
 */

#[derive(Debug, Clone)]
pub struct GlVertex3D {
  pub m_id: i32,
  pub m_position: Vec3<f32>,
  pub m_normal: Vec3<f32>,
  pub m_color: Color<f32>,
  pub m_texture_coords: Vec2<f32>
}

impl GlVertex3D {
  pub fn new() -> Self {
    return GlVertex3D {
      m_id: -1,
      m_position: Vec3::new(),
      m_normal: Vec3::new(),
      m_color: Color::new(),
      m_texture_coords: Vec2::new(),
    }
  }
  
  pub fn register(&mut self, new_id: i32) {
    self.m_id = new_id;
  }
}


///////////////////////////////////   DISPLAY  ///////////////////////////////////

impl std::fmt::Display for GlVertex3D {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[GlVertex3D] --> ID => {0}; Position => {1}; Normal => {2}; Color => {3}; \
    Texture coords => {4}", self.m_id, self.m_position, self.m_normal, self.m_color,
      self.m_texture_coords)
  }
}

///////////////////////////////////   EQUALITY  ///////////////////////////////////

impl PartialEq for GlVertex3D {
  fn eq(&self, other: &Self) -> bool {
    return self.m_id == other.m_id;
  }
}