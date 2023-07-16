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

extern crate num;
use num::Zero;

#[derive(Debug, Copy, Clone)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec4<T> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}

/*
///////////////////////////////////   VEC2  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

impl<T: Zero> Vec2<T> {
  pub fn new() -> Vec2<T> {
    return Vec2 { x: T::zero(), y: T::zero() };
  }
  pub fn new_shared() -> Box<Vec2<T>> {
    return Box::new(Vec2 { x: T::zero(), y: T::zero() });
  }
  pub fn delete(&mut self) {
    self.x = T::zero();
    self.y = T::zero();
  }
}

///////////////////// DISPLAY ////////////////////////

impl<T: std::fmt::Display> std::fmt::Display for Vec2<T> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec2] --> x: {0:.3}, y: {1:.3}", &self.x, &self.y)
  }
}

///////////////////// EQUALITY ////////////////////////

impl<T> PartialEq for Vec2<T> where for<'a> &'a T: PartialEq {
  fn eq(&self, other_vec2: &Vec2<T>) -> bool {
    if (self as *const Vec2<T>) == (other_vec2 as *const Vec2<T>) {
      return true;
    }
    return &self.x == &other_vec2.x && &self.y == &other_vec2.y;
  }
  
  fn ne(&self, other_vec2: &Vec2<T>) -> bool {
    return !std::ptr::eq(&self, &other_vec2);
  }
}

///////////////////// ARITHMETIC ////////////////////////

impl<T> std::ops::Add for &Vec2<T> where for<'a> &'a T: std::ops::Add<&'a T, Output=T> {
  type Output = Vec2<T>;
  
  fn add(self, other_vec2: &Vec2<T>) -> Vec2<T> {
    return Vec2 {
      x: &self.x + &other_vec2.x,
      y: &self.y + &other_vec2.y,
    };
  }
}

impl<T> std::ops::AddAssign for Vec2<T> where for<'a> &'a T: std::ops::Add<&'a T, Output=T> {
  fn add_assign(&mut self, other_vec2: Vec2<T>) {
    self.x = &self.x + &other_vec2.x;
    self.y = &self.y + &other_vec2.y;
  }
}

impl<T> std::ops::Sub for &Vec2<T> where for<'a> &'a T: std::ops::Sub<&'a T, Output=T> {
  type Output = Vec2<T>;
  
  fn sub(self, other_vec2: &Vec2<T>) -> Vec2<T> {
    return Vec2 {
      x: &self.x - &other_vec2.x,
      y: &self.y - &other_vec2.y,
    };
  }
}

impl<T> std::ops::SubAssign for Vec2<T> where for<'a> &'a T: std::ops::Sub<&'a T, Output=T> {
  fn sub_assign(&mut self, other_vec2: Vec2<T>) {
    self.x = &self.x - &other_vec2.x;
    self.y = &self.y - &other_vec2.y;
  }
}

impl<T> std::ops::Mul for &Vec2<T> where for<'a> &'a T: std::ops::Mul<&'a T, Output=T> {
  type Output = Vec2<T>;
  
  fn mul(self, other_vec2: &Vec2<T>) -> Vec2<T> {
    return Vec2 {
      x: &self.x * &other_vec2.x,
      y: &self.y * &other_vec2.y,
    };
  }
}

impl<T> std::ops::MulAssign for Vec2<T> where for<'a> &'a T: std::ops::Mul<&'a T, Output=T> {
  fn mul_assign(&mut self, other_vec2: Vec2<T>) {
    self.x = &self.x * &other_vec2.x;
    self.y = &self.y * &other_vec2.y;
  }
}

impl<T> std::ops::Div for &Vec2<T> where for<'a> &'a T: std::ops::Div<&'a T, Output=T> {
  type Output = Vec2<T>;
  
  fn div(self, other_vec2: &Vec2<T>) -> Vec2<T> {
    return Vec2 {
      x: &self.x / &other_vec2.x,
      y: &self.y / &other_vec2.y,
    };
  }
}

impl<T> std::ops::DivAssign for Vec2<T> where for<'a> &'a T: std::ops::Div<&'a T, Output=T> {
  fn div_assign(&mut self, other_vec2: Vec2<T>) {
    self.x = &self.x / &other_vec2.x;
    self.y = &self.y / &other_vec2.y;
  }
}

///////////////////// INDEXING ////////////////////////

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

impl<T: Zero> Vec3<T> {
  pub fn new() -> Vec3<T> {
    return Vec3 { x: T::zero(), y: T::zero(), z: T::zero() };
  }
  pub fn new_shared() -> Box<Vec3<T>> {
    return Box::new(Vec3 { x: T::zero(), y: T::zero(), z: T::zero() });
  }
  pub fn delete(&mut self) {
    self.x = T::zero();
    self.y = T::zero();
    self.z = T::zero();
  }
}

///////////////////// DISPLAY ////////////////////////

impl<T: std::fmt::Display> std::fmt::Display for Vec3<T> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec3] --> x: {0:.3}, y: {1:.3}, z: {2:.3}", &self.x, &self.y, &self.z)
  }
}

///////////////////// EQUALITY ////////////////////////

impl<T> PartialEq for Vec3<T> where for<'a> &'a T: PartialEq {
  fn eq(&self, other_vec3: &Vec3<T>) -> bool {
    if (self as *const Vec3<T>) == (other_vec3 as *const Vec3<T>) {
      return true;
    }
    return &self.x == &other_vec3.x && &self.y == &other_vec3.y && &self.z == &other_vec3.z;
  }
  
  fn ne(&self, other_vec3: &Vec3<T>) -> bool {
    return !std::ptr::eq(&self, &other_vec3);
  }
}

///////////////////// ARITHMETIC ////////////////////////

impl<T> std::ops::Add for &Vec3<T> where for<'a> &'a T: std::ops::Add<&'a T, Output=T> {
  type Output = Vec3<T>;
  
  fn add(self, other_vec3: &Vec3<T>) -> Vec3<T> {
    return Vec3 {
      x: &self.x + &other_vec3.x,
      y: &self.y + &other_vec3.y,
      z: &self.z + &other_vec3.z,
    };
  }
}

impl<T> std::ops::AddAssign for Vec3<T> where for<'a> &'a T: std::ops::Add<&'a T, Output=T> {
  fn add_assign(&mut self, other_vec3: Vec3<T>) {
    self.x = &self.x + &other_vec3.x;
    self.y = &self.y + &other_vec3.y;
    self.z = &self.z + &other_vec3.z;
  }
}

impl<T> std::ops::Sub for &Vec3<T> where for<'a> &'a T: std::ops::Sub<&'a T, Output=T> {
  type Output = Vec3<T>;
  
  fn sub(self, other_vec3: &Vec3<T>) -> Vec3<T> {
    return Vec3 {
      x: &self.x - &other_vec3.x,
      y: &self.y - &other_vec3.y,
      z: &self.z - &other_vec3.z,
    };
  }
}

impl<T> std::ops::SubAssign for Vec3<T> where for<'a> &'a T: std::ops::Sub<&'a T, Output=T> {
  fn sub_assign(&mut self, other_vec3: Vec3<T>) {
    self.x = &self.x - &other_vec3.x;
    self.y = &self.y - &other_vec3.y;
    self.z = &self.z - &other_vec3.z;
  }
}

impl<T> std::ops::Mul for &Vec3<T> where for<'a> &'a T: std::ops::Mul<&'a T, Output=T> {
  type Output = Vec3<T>;
  
  fn mul(self, other_vec3: &Vec3<T>) -> Vec3<T> {
    return Vec3 {
      x: &self.x * &other_vec3.x,
      y: &self.y * &other_vec3.y,
      z: &self.z * &other_vec3.z,
    };
  }
}

impl<T> std::ops::MulAssign for Vec3<T> where for<'a> &'a T: std::ops::Mul<&'a T, Output=T> {
  fn mul_assign(&mut self, other_vec3: Vec3<T>) {
    self.x = &self.x * &other_vec3.x;
    self.y = &self.y * &other_vec3.y;
    self.z = &self.z * &other_vec3.z;
  }
}

impl<T> std::ops::Div for &Vec3<T> where for<'a> &'a T: std::ops::Div<&'a T, Output=T> {
  type Output = Vec3<T>;
  
  fn div(self, other_vec3: &Vec3<T>) -> Vec3<T> {
    return Vec3 {
      x: &self.x / &other_vec3.x,
      y: &self.y / &other_vec3.y,
      z: &self.z / &other_vec3.z,
    };
  }
}

impl<T> std::ops::DivAssign for Vec3<T> where for<'a> &'a T: std::ops::Div<&'a T, Output=T> {
  fn div_assign(&mut self, other_vec3: Vec3<T>) {
    self.x = &self.x / &other_vec3.x;
    self.y = &self.y / &other_vec3.y;
    self.z = &self.z / &other_vec3.z;
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

impl<T: Zero> Vec4<T> {
  pub fn new() -> Vec4<T> {
    return Vec4 { x: T::zero(), y: T::zero(), z: T::zero(), w: T::zero() };
  }
  pub fn new_shared() -> Box<Vec4<T>> {
    return Box::new(Vec4 { x: T::zero(), y: T::zero(), z: T::zero(), w: T::zero() });
  }
  pub fn delete(&mut self) {
    self.x = T::zero();
    self.y = T::zero();
    self.z = T::zero();
    self.w = T::zero();
  }
}

///////////////////// DISPLAY ////////////////////////

impl<T: std::fmt::Display> std::fmt::Display for Vec4<T> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec4] --> x: {0:.3}, y: {1:.3}, z: {2:.3}, w: {3:.3}",
      &self.x, &self.y, &self.z, &self.w)
  }
}

///////////////////// EQUALITY ////////////////////////

impl<T> PartialEq for Vec4<T> where for<'a> &'a T: PartialEq {
  fn eq(&self, other_vec4: &Vec4<T>) -> bool {
    if (self as *const Vec4<T>) == (other_vec4 as *const Vec4<T>) {
      return true;
    }
    return &self.x == &other_vec4.x && &self.y == &other_vec4.y && &self.z == &other_vec4.z &&
      &self.w == &other_vec4.w;
  }
  
  fn ne(&self, other_vec4: &Vec4<T>) -> bool {
    return !std::ptr::eq(&self, &other_vec4);
  }
}

///////////////////// ARITHMETIC ////////////////////////

impl<T> std::ops::Add for &Vec4<T> where for<'a> &'a T: std::ops::Add<&'a T, Output=T> {
  type Output = Vec4<T>;
  
  fn add(self, other_vec4: &Vec4<T>) -> Vec4<T> {
    return Vec4 {
      x: &self.x + &other_vec4.x,
      y: &self.y + &other_vec4.y,
      z: &self.z + &other_vec4.z,
      w: &self.w + &other_vec4.w,
    };
  }
}

impl<T> std::ops::AddAssign for Vec4<T> where for<'a> &'a T: std::ops::Add<&'a T, Output=T> {
  fn add_assign(&mut self, other_vec4: Vec4<T>) {
    self.x = &self.x + &other_vec4.x;
    self.y = &self.y + &other_vec4.y;
    self.z = &self.z + &other_vec4.z;
    self.w = &self.w + &other_vec4.w;
  }
}

impl<T> std::ops::Sub for &Vec4<T> where for<'a> &'a T: std::ops::Sub<&'a T, Output=T> {
  type Output = Vec4<T>;
  
  fn sub(self, other_vec4: &Vec4<T>) -> Vec4<T> {
    return Vec4 {
      x: &self.x - &other_vec4.x,
      y: &self.y - &other_vec4.y,
      z: &self.z - &other_vec4.z,
      w: &self.w - &other_vec4.w,
    };
  }
}

impl<T> std::ops::SubAssign for Vec4<T> where for<'a> &'a T: std::ops::Sub<&'a T, Output=T> {
  fn sub_assign(&mut self, other_vec4: Vec4<T>) {
    self.x = &self.x - &other_vec4.x;
    self.y = &self.y - &other_vec4.y;
    self.z = &self.z - &other_vec4.z;
    self.w = &self.w - &other_vec4.w;
  }
}

impl<T> std::ops::Mul for &Vec4<T> where for<'a> &'a T: std::ops::Mul<&'a T, Output=T> {
  type Output = Vec4<T>;
  
  fn mul(self, other_vec4: &Vec4<T>) -> Vec4<T> {
    return Vec4 {
      x: &self.x * &other_vec4.x,
      y: &self.y * &other_vec4.y,
      z: &self.z * &other_vec4.z,
      w: &self.w * &other_vec4.w,
    };
  }
}

impl<T> std::ops::MulAssign for Vec4<T> where for<'a> &'a T: std::ops::Mul<&'a T, Output=T> {
  fn mul_assign(&mut self, other_vec4: Vec4<T>) {
    self.x = &self.x * &other_vec4.x;
    self.y = &self.y * &other_vec4.y;
    self.z = &self.z * &other_vec4.z;
    self.w = &self.w * &other_vec4.w;
  }
}

impl<T> std::ops::Div for &Vec4<T> where for<'a> &'a T: std::ops::Div<&'a T, Output=T> {
  type Output = Vec4<T>;
  
  fn div(self, other_vec4: &Vec4<T>) -> Vec4<T> {
    return Vec4 {
      x: &self.x / &other_vec4.x,
      y: &self.y / &other_vec4.y,
      z: &self.z / &other_vec4.z,
      w: &self.w / &other_vec4.w,
    };
  }
}

impl<T> std::ops::DivAssign for Vec4<T> where for<'a> &'a T: std::ops::Div<&'a T, Output=T> {
  fn div_assign(&mut self, other_vec4: Vec4<T>) {
    self.x = &self.x / &other_vec4.x;
    self.y = &self.y / &other_vec4.y;
    self.z = &self.z / &other_vec4.z;
    self.w = &self.w / &other_vec4.w;
  }
}

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
/////////////////////////////////// 4X4 MATRICES        ///////////////////////////////////
///////////////////////////////////  (ROW MAJOR ORDER)  ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
 */

#[derive(Debug, Copy, Clone)]
pub struct GlMatrix {
  pub value_ptr: Vec4<Vec4<f32>>,
}

impl GlMatrix {
  pub fn new(initialize_identity: bool) -> Box<GlMatrix> {
    if initialize_identity {
      return Box::new(GlMatrix {
        value_ptr: Vec4 {
          x: Vec4 { x: 1.0, y: 0.0, z: 0.0, w: 0.0 },
          y: Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 },
          z: Vec4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 },
          w: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
        }
      });
    }
    return Box::new(GlMatrix {
      value_ptr: Vec4 {
        x: Vec4::new(),
        y: Vec4::new(),
        z: Vec4::new(),
        w: Vec4::new(),
      }
    });
  }
  pub fn delete(&mut self) {
    self.value_ptr.x.delete();
    self.value_ptr.y.delete();
    self.value_ptr.z.delete();
    self.value_ptr.w.delete();
  }
}

///////////////////// DISPLAY ////////////////////////

impl std::fmt::Display for GlMatrix {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[GlMatrix] -->  {0:.3}, {1:.3}, {2:.3}, {3:.3}\n\
                                    {4:.3}, {5:.3}, {6:.3}, {7:.3}\n\
                                    {8:.3}, {9:.3}, {10:.3}, {11:.3}\n\
                                    {12:.3}, {13:.3}, {14:.3}, {15:.3}\n",
      &self.value_ptr[0][0], &self.value_ptr[0][1], &self.value_ptr[0][2], &self.value_ptr[0][3],
      &self.value_ptr[1][0], &self.value_ptr[1][1], &self.value_ptr[1][2], &self.value_ptr[1][3],
      &self.value_ptr[2][0], &self.value_ptr[2][1], &self.value_ptr[2][2], &self.value_ptr[2][3],
      &self.value_ptr[3][0], &self.value_ptr[3][1], &self.value_ptr[3][2], &self.value_ptr[3][3])
  }
}

///////////////////// INDEXING ////////////////////////

impl std::ops::Index<usize> for GlMatrix {
  type Output = Vec4<f32>;
  
  fn index(&self, index: usize) -> &Self::Output {
    return &self.value_ptr[index];
  }
}

impl std::ops::IndexMut<usize> for GlMatrix {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    return &mut self.value_ptr[index];
  }
}

