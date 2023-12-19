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

use crate::wave::math::Mat4;
use crate::wave::math::Vec3;

pub enum EnumErrors {
  InvalidDimensions,
  InvalidMatrix,
}

/*
///////////////////////////////////   Orthographic Camera  ///////////////////////////////////
///////////////////////////////////                        ///////////////////////////////////
///////////////////////////////////                        ///////////////////////////////////
 */

// pub struct OrthographicCamera {
//   m_width: u32,
//   m_height: u32,
//   m_z_rear: u32,
//   m_z_far: u32,
//   m_matrix: Mat4,
// }

/*
///////////////////////////////////   Perspective Camera  ///////////////////////////////////
///////////////////////////////////                       ///////////////////////////////////
///////////////////////////////////                       ///////////////////////////////////
 */

pub struct PerspectiveCamera {
  m_fov: f32,
  m_aspect_ratio: f32,
  m_z_near: f32,
  m_z_far: f32,
  m_matrix: Mat4,
}

impl PerspectiveCamera {
  pub fn new() -> Self {
    return PerspectiveCamera {
      m_fov: 0.0,
      m_aspect_ratio: 4.0/3.0,
      m_z_near: 0.0,
      m_z_far: 0.0,
      // View + projection matrix.
      m_matrix: Mat4::new(1.0),
    }
  }
  
  pub fn from(fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
    return PerspectiveCamera {
      m_fov: fov,
      m_aspect_ratio: aspect_ratio,
      m_z_near: z_near,
      m_z_far: z_far,
      m_matrix: Mat4::new(1.0)
    }
  }
  
  pub fn get_matrix(&self) -> &Mat4 {
    return &self.m_matrix;
  }
  
  pub fn set_view_projection(&mut self) {
    let up: Vec3<f32> = Vec3::from(&[0.0, 1.0, 0.0]);
    let direction: Vec3<f32> = Vec3::from(&[0.0, 0.0, 1.0]);
    let right: Vec3<f32> = up.cross(direction.clone());
    
    let projection_matrix: Mat4 = Mat4::apply_perspective(self.m_fov, self.m_aspect_ratio, self.m_z_near, self.m_z_far);
    let view_matrix: Mat4 = Mat4::from(
      [
        [right.x,     right.y,     right.z,            self.m_matrix[0][3]],
        [up.x,        up.y,        up.z,               self.m_matrix[1][3]],
        [direction.x, direction.y, direction.z,        self.m_matrix[2][3]],
        
        [self.m_matrix[3][0], self.m_matrix[3][1], self.m_matrix[3][2], self.m_matrix[3][3]],
      ]);
    
    self.m_matrix = (projection_matrix * view_matrix).transpose();
  }
  
  pub fn update_projection(&mut self, fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) {
    self.m_fov = fov;
    self.m_aspect_ratio = aspect_ratio;
    self.m_z_near = z_near;
    self.m_z_far = z_far;
    
    self.set_view_projection();
  }
}