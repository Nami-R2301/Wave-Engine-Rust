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

/*
///////////////////////////////////   VEC2  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

use std::mem::size_of;
use super::super::impl_struct;

impl_struct!(Vec2<T> { x, y, });

impl Vec2<f32> {
  pub fn vec_len(&self) -> f32 {
    return (self.x.powi(2) + self.y.powi(2))
      .sqrt();  // Return NaN or the distance.
  }
}

impl<T> std::ops::Index<usize> for Vec2<T> {
  type Output = T;
  
  fn index(&self, index: usize) -> &T {
    return match index {
      0 => &self.x,
      1 => &self.y,
      _ => &self.x,
    };
  }
}

impl<T> std::ops::IndexMut<usize> for Vec2<T> {
  fn index_mut(&mut self, index: usize) -> &mut T {
    return match index {
      0 => &mut self.x,
      1 => &mut self.y,
      _ => &mut self.x,
    };
  }
}

/*
///////////////////////////////////   VEC3  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

impl_struct!(Vec3<T> {x, y, z, });

impl Vec3<f32> {
  pub fn cross(&self, other: Self) -> Self {
    return Vec3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    };
  }
  
  pub fn dot(&self, other: Self) -> f32 {
    return (self.x * other.x) + (self.y * other.y) + (self.x * other.z);
  }
  
  pub fn vec_len(&self) -> f32 {
    return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
      .sqrt();  // Return NaN or the distance.
  }
}

///////////////////// INDEXING ////////////////////////

impl<T> std::ops::Index<usize> for Vec3<T> {
  type Output = T;
  
  fn index(&self, index: usize) -> &T {
    return match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => &self.x,
    };
  }
}

impl<T> std::ops::IndexMut<usize> for Vec3<T> {
  fn index_mut(&mut self, index: usize) -> &mut T {
    return match index {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      _ => &mut self.x,
    };
  }
}

/*
///////////////////////////////////   VEC4  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

impl_struct!(Vec4<T> { x, y, z, w, });

///////////////////// INDEXING ////////////////////////

impl<T> std::ops::Index<usize> for Vec4<T> {
  type Output = T;
  
  fn index(&self, index: usize) -> &T {
    return match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      3 => &self.w,
      _ => &self.x,
    };
  }
}

impl<T> std::ops::IndexMut<usize> for Vec4<T> {
  fn index_mut(&mut self, index: usize) -> &mut T {
    return match index {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      3 => &mut self.w,
      _ => &mut self.x,
    };
  }
}

/*
///////////////////////////////////   4X4 MATRICES      ///////////////////////////////////
///////////////////////////////////  (ROW MAJOR ORDER)  ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
 */

#[derive(Debug, Copy, Clone)]
pub struct Mat4 {
  pub m_value_ptr: Vec4<Vec4<f32>>,
}

impl Mat4 {
  pub fn default() -> Self {
    return Self {
      m_value_ptr: Vec4 {
        x: Vec4::default(),
        y: Vec4::default(),
        z: Vec4::default(),
        w: Vec4::default(),
      }
    }
  }
  pub fn new(initialize_identity_value: f32) -> Self {
    if initialize_identity_value != 0.0 {
      return Self {
        m_value_ptr: Vec4 {
          x: Vec4 {
            x: initialize_identity_value,
            y: 0.0,
            z: 0.0,
            w: 0.0,
          },
          y: Vec4 {
            x: 0.0,
            y: initialize_identity_value,
            z: 0.0,
            w: 0.0,
          },
          z: Vec4 {
            x: 0.0,
            y: 0.0,
            z: initialize_identity_value,
            w: 0.0,
          },
          w: Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: initialize_identity_value,
          },
        },
      };
    }
    return Self {
      m_value_ptr: Vec4 {
        x: Vec4::default(),
        y: Vec4::default(),
        z: Vec4::default(),
        w: Vec4::default(),
      },
    };
  }
  
  pub fn new_shared(initialize_identity_value: f32) -> Box<Mat4> {
    return Box::new(Mat4::new(initialize_identity_value));
  }
  
  pub const fn get_size() -> usize {
    return 4 * 4 * 4;  // sizeof(float) * (16 floats).
  }
  
  pub fn delete(&mut self) {
    self.m_value_ptr.x.delete();
    self.m_value_ptr.y.delete();
    self.m_value_ptr.z.delete();
    self.m_value_ptr.w.delete();
  }
  
  pub fn from(vec4s: [[f32; 4]; 4]) -> Self {
    return Mat4 {
      m_value_ptr: Vec4 {
        x: Vec4::new(&vec4s[0]),
        y: Vec4::new(&vec4s[1]),
        z: Vec4::new(&vec4s[2]),
        w: Vec4::new(&vec4s[3]),
      },
    };
  }
  
  pub fn transpose(&self) -> Mat4 {
    let mut result: Mat4 = Mat4::new(0.0);
    
    for i in 0..4usize {
      for j in 0..4usize {
        let mut row_major_value: f32 = self[i][j];
        std::mem::swap(&mut result[j][i], &mut row_major_value);
      }
    }
    return result;
  }
  
  pub fn as_array(&self) -> [f32; 16] {
    return [
      self[0][0], self[0][1], self[0][2], self[0][3],
      self[1][0], self[1][1], self[1][2], self[1][3],
      self[2][0], self[2][1], self[2][2], self[2][3],
      self[3][0], self[3][1], self[3][2], self[3][3]
    ];
  }
  
  pub fn translate_model(translation_vec: &Vec3<f32>) -> Self {
    let mut result = Mat4::new(1.0);
    result[0][3] = translation_vec.x;
    result[1][3] = translation_vec.y;
    result[2][3] = translation_vec.z;
    return result;
  }
  
  pub fn rotate_model(rotation_vec: &Vec3<f32>) -> Self {
    let mut rotation_x: Mat4 = Mat4::new(1.0);
    let mut rotation_y: Mat4 = Mat4::new(1.0);
    let mut rotation_z: Mat4 = Mat4::new(1.0);
    
    let mut rotation_vec_clone = Vec3::default();
    rotation_vec_clone.x = rotation_vec.x.to_radians();
    rotation_vec_clone.y = rotation_vec.y.to_radians();
    rotation_vec_clone.z = rotation_vec.z.to_radians();
    
    // Set angles for rotation on the x-axis.
    rotation_x[1][1] = rotation_vec_clone.x.cos();
    rotation_x[1][2] = -rotation_vec_clone.x.sin();
    rotation_x[2][1] = rotation_vec_clone.x.sin();
    rotation_x[2][2] = rotation_vec_clone.x.cos();
    
    // Set angles for rotation on the y-axis.
    rotation_y[0][0] = rotation_vec_clone.y.cos();
    rotation_y[0][2] = -rotation_vec_clone.y.sin();
    rotation_y[2][0] = rotation_vec_clone.y.sin();
    rotation_y[2][2] = rotation_vec_clone.y.cos();
    
    // Set angles for rotation on the z-axis.
    rotation_z[0][0] = rotation_vec_clone.z.cos();
    rotation_z[0][1] = -rotation_vec_clone.z.sin();
    rotation_z[1][0] = rotation_vec_clone.z.sin();
    rotation_z[1][1] = rotation_vec_clone.z.cos();
    
    return rotation_z * (rotation_y * rotation_x);
  }
  
  pub fn scale_model(scale_vec: &Vec3<f32>) -> Self {
    let mut result = Mat4::new(0.0);
    result[0][0] = scale_vec.x;
    result[1][1] = scale_vec.y;
    result[2][2] = scale_vec.z;
    result[3][3] = 1.0;
    return result;
  }
  
  pub fn apply_model(translation_vec: &Vec3<f32>, rotation_vec: &Vec3<f32>, scale_vec: &Vec3<f32>) -> Self {
    let translation_mat = Mat4::translate_model(translation_vec);
    let rotation_mat = Mat4::rotate_model(rotation_vec);
    let scale_mat = Mat4::scale_model(scale_vec);
    
    return (translation_mat * (rotation_mat * scale_mat)).transpose();
  }
  
  pub fn apply_perspective(fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
    let tan_half_fov: f32 = 1.0 / ((fov.to_radians() / 2.0).tan());
    let z_range: f32 = z_near - z_far;
    let mut result = Mat4::new(0.0);
    
    result[0][0] = tan_half_fov;
    result[1][1] = tan_half_fov * aspect_ratio;
    result[2][2] = (z_far + z_near) / z_range;
    result[2][3] = (2.0 * z_far * z_near) / z_range;
    result[3][2] = -1.0;
    result[3][3] = 0.0;  // Discard w.
    
    return result;
  }
}

///////////////////// DISPLAY ////////////////////////

impl std::fmt::Display for Mat4 {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      format,
      "[Mat4] -->  {0:.3}, {1:.3}, {2:.3}, {3:.3}\n\
                                    {4:.3}, {5:.3}, {6:.3}, {7:.3}\n\
                                    {8:.3}, {9:.3}, {10:.3}, {11:.3}\n\
                                    {12:.3}, {13:.3}, {14:.3}, {15:.3}\n",
      &self.m_value_ptr[0][0],
      &self.m_value_ptr[0][1],
      &self.m_value_ptr[0][2],
      &self.m_value_ptr[0][3],
      &self.m_value_ptr[1][0],
      &self.m_value_ptr[1][1],
      &self.m_value_ptr[1][2],
      &self.m_value_ptr[1][3],
      &self.m_value_ptr[2][0],
      &self.m_value_ptr[2][1],
      &self.m_value_ptr[2][2],
      &self.m_value_ptr[2][3],
      &self.m_value_ptr[3][0],
      &self.m_value_ptr[3][1],
      &self.m_value_ptr[3][2],
      &self.m_value_ptr[3][3]
    )
  }
}

///////////////////// INDEXING ////////////////////////

impl std::ops::Index<usize> for Mat4 {
  type Output = Vec4<f32>;
  
  fn index(&self, index: usize) -> &Self::Output {
    return &self.m_value_ptr[index];
  }
}

impl std::ops::IndexMut<usize> for Mat4 {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    return &mut self.m_value_ptr[index];
  }
}

///////////////////// EQUALITY ////////////////////////

impl PartialEq for Mat4 {
  fn eq(&self, other: &Self) -> bool {
    if &self.m_value_ptr == &other.m_value_ptr {
      return true;
    }
    for row in 0..4usize {
      for col in 0..4usize {
        if self[row][col] != other[row][col] {
          return false;
        }
      }
    }
    return true;
  }
  
  fn ne(&self, other: &Self) -> bool {
    return !self.eq(other);
  }
}

impl Eq for Mat4 {}

///////////////////// ARITHMETIC ////////////////////////

impl std::ops::Mul for Mat4 {
  type Output = Mat4;
  
  fn mul(self, other_matrix: Self) -> Mat4 {
    let mut default_matrix: Mat4 = Mat4::new(0.0);
    
    for col in 0..4usize {
      default_matrix.m_value_ptr.x[col] += (self.m_value_ptr.x.x * other_matrix.m_value_ptr.x[col])
        + (self.m_value_ptr.x.y * other_matrix.m_value_ptr.y[col])
        + (self.m_value_ptr.x.z * other_matrix.m_value_ptr.z[col])
        + (self.m_value_ptr.x.w * other_matrix.m_value_ptr.w[col]);
      
      default_matrix.m_value_ptr.y[col] += (self.m_value_ptr.y.x * other_matrix.m_value_ptr.x[col])
        + (self.m_value_ptr.y.y * other_matrix.m_value_ptr.y[col])
        + (self.m_value_ptr.y.z * other_matrix.m_value_ptr.z[col])
        + (self.m_value_ptr.y.w * other_matrix.m_value_ptr.w[col]);
      
      default_matrix.m_value_ptr.z[col] += (self.m_value_ptr.z.x * other_matrix.m_value_ptr.x[col])
        + (self.m_value_ptr.z.y * other_matrix.m_value_ptr.y[col])
        + (self.m_value_ptr.z.z * other_matrix.m_value_ptr.z[col])
        + (self.m_value_ptr.z.w * other_matrix.m_value_ptr.w[col]);
      
      default_matrix.m_value_ptr.w[col] += (self.m_value_ptr.w.x * other_matrix.m_value_ptr.x[col])
        + (self.m_value_ptr.w.y * other_matrix.m_value_ptr.y[col])
        + (self.m_value_ptr.w.z * other_matrix.m_value_ptr.z[col])
        + (self.m_value_ptr.w.w * other_matrix.m_value_ptr.w[col]);
    }
    return default_matrix;
  }
}
