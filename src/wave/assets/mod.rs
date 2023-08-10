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

use crate::wave::graphics::renderer::{EnumErrors, TraitSendable};
use crate::wave::graphics::vertex::{GlVertex2D, GlVertex3D};

/*
///////////////////////////////////   OpenGL OBJECT 2D  ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
 */

#[derive(Debug, Clone)]
pub struct GlObject2D {
  m_vertices: GlVertex2D,
  m_indices: Vec<i16>,  // For drawing indices with a lower value than 65535 with GL_SHORT (glDrawElements).
}

impl GlObject2D {
  pub fn new() -> Self {
    return GlObject2D {
      m_vertices: GlVertex2D::new(),
      m_indices: Vec::new(),
    }
  }
}

impl TraitSendable for GlObject2D {
  fn send() -> Result<(), EnumErrors> {
    todo!()
  }
  
  fn free() {
    todo!()
  }
}

/*
///////////////////////////////////   OpenGL OBJECT 3D  ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
 */

#[derive(Debug, Clone)]
pub struct GlObject3D {
  m_vertices: GlVertex3D,
  m_indices: Vec<i16>,  // For drawing indices with a lower value than 65535 with GL_SHORT (glDrawElements).
}

impl GlObject3D {
  pub fn new() -> Self {
    return GlObject3D {
      m_vertices: GlVertex3D::new(),
      m_indices: Vec::new(),
    }
  }
}

impl TraitSendable for GlObject3D {
  fn send() -> Result<(), EnumErrors> {
    todo!()
  }
  
  fn free() {
    todo!()
  }
}