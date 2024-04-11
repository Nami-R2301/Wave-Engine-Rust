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

use crate::math::Mat4;
use crate::math::Vec3;

pub enum EnumError {
  InvalidDimensions,
  InvalidMatrix,
}

pub enum EnumCameraType {
  Perspective(u32, f32, f32, f32),
  Orthographic(u32, u32, f32, f32),
}

pub trait TraitCamera {
  fn get_projection_matrix(&self) -> Mat4;
  fn get_view_matrix(&self) -> Mat4;
  fn has_changed(&self) -> bool;
  fn set_up_vector(&mut self, to_this: Vec3<f32>);
  fn translate(&mut self, by: Vec3<f32>);
  fn rotate(&mut self, by: Vec3<f32>);
  fn scale(&mut self, by: Vec3<f32>);
  fn on_update(&mut self, time_step: f64);
  fn to_string(&self) -> String;
}

pub struct Camera {
  m_api: Box<dyn TraitCamera>,
}

impl Camera {
  pub fn default() -> Self {
    return Self {
      m_api: Box::new(PerspectiveCamera::default()),
    };
  }
  
  pub fn new(camera_type: EnumCameraType, apply_transform: Option<[Vec3<f32>; 3]>) -> Self {
    return match camera_type {
      EnumCameraType::Perspective(fov, aspect_ratio, z_near, z_far) => {
        let mut perspective = PerspectiveCamera::new(fov, aspect_ratio, z_near, z_far);
        if apply_transform.is_some() {
          perspective.m_transforms = apply_transform.unwrap();
        }
        Self {
          m_api: Box::new(perspective),
        }
      }
      EnumCameraType::Orthographic(width, height, z_near, z_far) => {
        let mut orthographic = OrthographicCamera::new(width, height, z_near, z_far);
        if apply_transform.is_some() {
          orthographic.m_transforms = apply_transform.unwrap();
        }
        Self {
          m_api: Box::new(orthographic),
        }
      }
    };
  }
  
  pub fn get_projection_matrix(&self) -> Mat4 {
    return self.m_api.get_projection_matrix();
  }
  pub fn get_view_matrix(&self) -> Mat4 {
    return self.m_api.get_view_matrix();
  }
  pub fn on_update(&mut self, time_step: f64) {
    return self.m_api.on_update(time_step);
  }
  pub fn has_changed(&self) -> bool {
    return self.m_api.has_changed();
  }
  pub fn set_up_vector(&mut self, to_this: Vec3<f32>) {
    return self.m_api.set_up_vector(to_this);
  }
  pub fn translate(&mut self, amount_x: f32, amount_y: f32, amount_z: f32) {
    return self.m_api.translate(Vec3::new(&[amount_x, amount_y, -amount_z]));
  }
  pub fn rotate(&mut self, amount_x: f32, amount_y: f32, amount_z: f32) {
    return self.m_api.rotate(Vec3::new(&[amount_y, amount_x, -amount_z]));
  }
  pub fn scale(&mut self, amount_x: f32, amount_y: f32, amount_z: f32) {
    return self.m_api.scale(Vec3::new(&[amount_x, amount_y, amount_z]));
  }
}

/*
///////////////////////////////////   Orthographic Camera  ///////////////////////////////////
///////////////////////////////////                        ///////////////////////////////////
///////////////////////////////////                        ///////////////////////////////////
 */

#[allow(unused)]
pub struct OrthographicCamera {
  m_width: u32,
  m_height: u32,
  m_z_rear: f32,
  m_z_far: f32,
  m_transforms: [Vec3<f32>; 3],
}

impl TraitCamera for OrthographicCamera {
  fn get_projection_matrix(&self) -> Mat4 {
    todo!()
  }
  
  fn get_view_matrix(&self) -> Mat4 {
    todo!()
  }
  
  fn has_changed(&self) -> bool {
    todo!()
  }
  
  #[allow(unused)]
  fn set_up_vector(&mut self, to_this: Vec3<f32>) {
    todo!()
  }
  
  #[allow(unused)]
  fn translate(&mut self, by: Vec3<f32>) {
    todo!()
  }
  
  #[allow(unused)]
  fn rotate(&mut self, by: Vec3<f32>) {
    todo!()
  }
  
  #[allow(unused)]
  fn scale(&mut self, by: Vec3<f32>) {
    todo!()
  }
  
  #[allow(unused)]
  fn on_update(&mut self, time_step: f64) {
    todo!()
  }
  
  fn to_string(&self) -> String {
    todo!()
  }
}

impl OrthographicCamera {
  pub fn default() -> Self {
    return Self {
      m_width: 640,
      m_height: 480,
      m_z_rear: 0.1,
      m_z_far: 10.0,
      m_transforms: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
    };
  }
  
  pub fn new(width: u32, height: u32, z_near: f32, z_far: f32) -> Self {
    return Self {
      m_width: width,
      m_height: height,
      m_z_rear: z_near,
      m_z_far: z_far,
      m_transforms: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
    };
  }
}

/*
///////////////////////////////////   Perspective Camera  ///////////////////////////////////
///////////////////////////////////                       ///////////////////////////////////
///////////////////////////////////                       ///////////////////////////////////
 */

pub struct PerspectiveCamera {
  m_fov: u32,
  m_aspect_ratio: f32,
  m_z_near: f32,
  m_z_far: f32,
  m_transforms: [Vec3<f32>; 3],
  m_up_vector: Vec3<f32>,
  m_has_changed: bool,
}

impl TraitCamera for PerspectiveCamera {
  fn get_projection_matrix(&self) -> Mat4 {
    return Mat4::apply_perspective(self.m_fov as f32, self.m_aspect_ratio, self.m_z_near, self.m_z_far);
  }
  
  fn get_view_matrix(&self) -> Mat4 {
    let up: Vec3<f32> = self.m_up_vector;
    let direction: Vec3<f32> = Vec3::new(&[0.0, 0.0, 1.0]);
    let right: Vec3<f32> = up.cross(direction.clone());
    let matrix = Mat4::apply_transformations(&self.m_transforms[0],
      &self.m_transforms[1], &self.m_transforms[2]);
    
    
    return Mat4::from(
      [
        [right.x, right.y, right.z, matrix[0][3]],
        [up.x, up.y, up.z, matrix[1][3]],
        [direction.x, direction.y, direction.z, matrix[2][3]],
        [matrix[3][0], matrix[3][1], matrix[3][2], matrix[3][3]]]
    );
  }
  
  fn has_changed(&self) -> bool {
    return self.m_has_changed;
  }
  
  fn set_up_vector(&mut self, to_this: Vec3<f32>) {
    self.m_up_vector = to_this;
  }
  
  fn translate(&mut self, by: Vec3<f32>) {
    self.m_transforms[0] += by;
    self.m_has_changed = true;
  }
  
  fn rotate(&mut self, by: Vec3<f32>) {
    let mut copy = Vec3::default();
    
    // Inverse x and y to correspond to the right orientation.
    copy.x = by.y;
    copy.y = by.x;
    copy.z = -by.z;
    
    self.m_transforms[1] += by;
    self.m_has_changed = true;
  }
  
  fn scale(&mut self, by: Vec3<f32>) {
    self.m_transforms[2] += by;
    self.m_has_changed = true;
  }
  
  fn on_update(&mut self, _time_step: f64) {
    if self.m_has_changed {
      self.m_has_changed = false;  // Reset state.
    }
  }
  
  fn to_string(&self) -> String {
    todo!()
  }
}

impl PerspectiveCamera {
  pub fn default() -> Self {
    return PerspectiveCamera {
      m_fov: 0,
      m_aspect_ratio: 4.0 / 3.0,
      m_z_near: 0.0,
      m_z_far: 0.0,
      m_up_vector: Vec3::new(&[0.0, 1.0, 0.0]),  // Default to Y-coordinate.
      m_transforms: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
      m_has_changed: false,
    };
  }
  
  pub fn new(fov: u32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
    return PerspectiveCamera {
      m_fov: fov,
      m_aspect_ratio: aspect_ratio,
      m_z_near: z_near,
      m_z_far: z_far,
      m_up_vector: Vec3::new(&[0.0, 1.0, 0.0]),  // Default to Y-coordinate.
      m_transforms: [Vec3::default(), Vec3::default(), Vec3::new(&[1.0, 1.0, 1.0])],
      m_has_changed: false,
    };
  }
  
  pub fn update_projection(&mut self, fov: u32, aspect_ratio: f32, z_near: f32, z_far: f32) {
    self.m_fov = fov;
    self.m_aspect_ratio = aspect_ratio;
    self.m_z_near = z_near;
    self.m_z_far = z_far;
  }
}