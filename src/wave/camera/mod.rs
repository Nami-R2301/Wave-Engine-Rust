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

use crate::wave::math::Vec3;
use crate::wave::graphics::renderer::TraitSendableEntity;
use crate::wave::math::Mat4;

pub enum EnumErrors {
  InvalidDimensions,
  InvalidMatrix,
}

/*
///////////////////////////////////   Orthographic Camera  ///////////////////////////////////
///////////////////////////////////                        ///////////////////////////////////
///////////////////////////////////                        ///////////////////////////////////
 */

pub struct OrthographicCamera {
  m_width: u32,
  m_height: u32,
  m_z_rear: u32,
  m_z_far: u32,
  m_focal_point: Vec3<f32>,
  m_view_projection_matrix: Mat4,
}

/*
///////////////////////////////////   Perspective Camera  ///////////////////////////////////
///////////////////////////////////                       ///////////////////////////////////
///////////////////////////////////                       ///////////////////////////////////
 */

pub struct PerspectiveCamera {
  m_fov: f32,
  m_width: u32,
  m_height: u32,
  m_z_rear: u32,
  m_z_far: u32,
  m_focal_point: Vec3<f32>,
  m_view_projection_matrix: Mat4,
}

impl PerspectiveCamera {
  pub fn new() -> Self {
    return PerspectiveCamera {
      m_fov: 0.0,
      m_width: 0,
      m_height: 0,
      m_z_rear: 0,
      m_z_far: 0,
      m_focal_point: Vec3::new(),
      m_view_projection_matrix: Mat4::new(true),
    }
  }
  
  pub fn get_matrix(&self) -> &Mat4 {
    return &self.m_view_projection_matrix;
  }
  
  fn set_view_projection(&mut self) {
  
  }
}

impl TraitSendableEntity for PerspectiveCamera {
  fn send(&mut self) -> Result<(), crate::wave::graphics::renderer::EnumErrors> {
    todo!()
  }
  
  fn resend(&mut self) -> Result<(), crate::wave::graphics::renderer::EnumErrors> {
    todo!()
  }
  
  fn free(&mut self) -> Result<(), crate::wave::graphics::renderer::EnumErrors> {
    todo!()
  }
  
  fn is_sent(&self) -> bool {
    todo!()
  }
}