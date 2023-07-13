/*
  MIT License

  Copyright (c) Nami Reghbati, created on 07/13/23

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

// Operator overloading.
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/*
/////////////////////////////////// VECTORS ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}

pub struct Vec3<T> {
  x: T,
  y: T,
  z: T,
}

impl Add for Vec2<u32> {
  type Output = Vec2<u32>;
  
  fn add(self, other_vec2: Self) -> Vec2<u32> {
    return Vec2 {
      x: self.x + other_vec2.x,
      y: self.y + other_vec2.y,
    };
  }
}

impl std::fmt::Display for Vec2<u32> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec2 (u32)] --> x: {}, y: {}", self.x, self.y)
  }
}

impl std::fmt::Display for Vec2<u64> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec2 (u64)] --> x: {}, y: {}", self.x, self.y)
  }
}

impl std::fmt::Display for Vec2<i32> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec2 (i32)] --> x: {}, y: {}", self.x, self.y)
  }
}

impl std::fmt::Display for Vec2<i64> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec2 (i64)] --> x: {}, y: {}", self.x, self.y)
  }
}

impl std::fmt::Display for Vec2<f32> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec2 (f32)] --> x: {}, y: {}", self.x, self.y)
  }
}

impl std::fmt::Display for Vec2<f64> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec2 (f64)] --> x: {}, y: {}", self.x, self.y)
  }
}

impl AddAssign for Vec2<u32> {
  fn add_assign(&mut self, other_vec2: Vec2<u32>) {
    self.x = self.x + other_vec2.x;
    self.y = self.y + other_vec2.y;
  }
}

impl Add for Vec2<u64> {
  type Output = Vec2<u64>;
  
  fn add(self, other_vec2: Self) -> Vec2<u64> {
    return Vec2 {
      x: self.x + other_vec2.x,
      y: self.y + other_vec2.y,
    };
  }
}

impl AddAssign for Vec2<u64> {
  fn add_assign(&mut self, other_vec2: Vec2<u64>) {
    self.x = self.x + other_vec2.x;
    self.y = self.y + other_vec2.y;
  }
}

impl Add for Vec2<i32> {
  type Output = Vec2<i32>;
  
  fn add(self, other_vec2: Self) -> Vec2<i32> {
    return Vec2 {
      x: self.x + other_vec2.x,
      y: self.y + other_vec2.y,
    };
  }
}

impl AddAssign for Vec2<i32> {
  fn add_assign(&mut self, other_vec2: Vec2<i32>) {
    self.x = self.x + other_vec2.x;
    self.y = self.y + other_vec2.y;
  }
}

impl Add for Vec2<i64> {
  type Output = Vec2<i64>;
  
  fn add(self, other_vec2: Self) -> Vec2<i64> {
    return Vec2 {
      x: self.x + other_vec2.x,
      y: self.y + other_vec2.y,
    };
  }
}

impl AddAssign for Vec2<i64> {
  fn add_assign(&mut self, other_vec2: Vec2<i64>) {
    self.x = self.x + other_vec2.x;
    self.y = self.y + other_vec2.y;
  }
}

impl Add for Vec2<f32> {
  type Output = Vec2<f32>;
  
  fn add(self, other_vec2: Self) -> Vec2<f32> {
    return Vec2 {
      x: self.x + other_vec2.x,
      y: self.y + other_vec2.y,
    };
  }
}

impl AddAssign for Vec2<f32> {
  fn add_assign(&mut self, other_vec2: Vec2<f32>) {
    self.x = self.x + other_vec2.x;
    self.y = self.y + other_vec2.y;
  }
}

impl Add for Vec2<f64> {
  type Output = Vec2<f64>;
  
  fn add(self, other_vec2: Self) -> Vec2<f64> {
    return Vec2 {
      x: self.x + other_vec2.x,
      y: self.y + other_vec2.y,
    };
  }
}

impl AddAssign for Vec2<f64> {
  fn add_assign(&mut self, other_vec2: Vec2<f64>) {
    self.x = self.x + other_vec2.x;
    self.y = self.y + other_vec2.y;
  }
}

impl Sub for Vec2<u32> {
  type Output = Vec2<u32>;
  
  fn sub(self, other_vec2: Self) -> Vec2<u32> {
    return Vec2 {
      x: self.x - other_vec2.x,
      y: self.y - other_vec2.y,
    };
  }
}

impl SubAssign for Vec2<u32> {
  fn sub_assign(&mut self, other_vec2: Vec2<u32>) {
    self.x = self.x - other_vec2.x;
    self.y = self.y - other_vec2.y;
  }
}

impl Sub for Vec2<u64> {
  type Output = Vec2<u64>;
  
  fn sub(self, other_vec2: Self) -> Vec2<u64> {
    return Vec2 {
      x: self.x - other_vec2.x,
      y: self.y - other_vec2.y,
    };
  }
}

impl SubAssign for Vec2<u64> {
  fn sub_assign(&mut self, other_vec2: Vec2<u64>) {
    self.x = self.x - other_vec2.x;
    self.y = self.y - other_vec2.y;
  }
}

impl Sub for Vec2<i32> {
  type Output = Vec2<i32>;
  
  fn sub(self, other_vec2: Self) -> Vec2<i32> {
    return Vec2 {
      x: self.x - other_vec2.x,
      y: self.y - other_vec2.y,
    };
  }
}

impl SubAssign for Vec2<i32> {
  fn sub_assign(&mut self, other_vec2: Vec2<i32>) {
    self.x = self.x - other_vec2.x;
    self.y = self.y - other_vec2.y;
  }
}

impl Sub for Vec2<i64> {
  type Output = Vec2<i64>;
  
  fn sub(self, other_vec2: Self) -> Vec2<i64> {
    return Vec2 {
      x: self.x - other_vec2.x,
      y: self.y - other_vec2.y,
    };
  }
}

impl SubAssign for Vec2<i64> {
  fn sub_assign(&mut self, other_vec2: Vec2<i64>) {
    self.x = self.x - other_vec2.x;
    self.y = self.y - other_vec2.y;
  }
}

impl Sub for Vec2<f32> {
  type Output = Vec2<f32>;
  
  fn sub(self, other_vec2: Self) -> Vec2<f32> {
    return Vec2 {
      x: self.x - other_vec2.x,
      y: self.y - other_vec2.y,
    };
  }
}

impl SubAssign for Vec2<f32> {
  fn sub_assign(&mut self, other_vec2: Vec2<f32>) {
    self.x = self.x - other_vec2.x;
    self.y = self.y - other_vec2.y;
  }
}

impl Sub for Vec2<f64> {
  type Output = Vec2<f64>;
  
  fn sub(self, other_vec2: Self) -> Vec2<f64> {
    return Vec2 {
      x: self.x - other_vec2.x,
      y: self.y - other_vec2.y,
    };
  }
}

impl SubAssign for Vec2<f64> {
  fn sub_assign(&mut self, other_vec2: Vec2<f64>) {
    self.x = self.x - other_vec2.x;
    self.y = self.y - other_vec2.y;
  }
}

impl Mul for Vec2<u32> {
  type Output = Vec2<u32>;
  
  fn mul(self, other_vec2: Self) -> Vec2<u32> {
    return Vec2 {
      x: self.x * other_vec2.x,
      y: self.y * other_vec2.y,
    };
  }
}

impl MulAssign for Vec2<u32> {
  fn mul_assign(&mut self, other_vec2: Vec2<u32>) {
    self.x = self.x * other_vec2.x;
    self.y = self.y * other_vec2.y;
  }
}

impl Mul for Vec2<u64> {
  type Output = Vec2<u64>;
  
  fn mul(self, other_vec2: Self) -> Vec2<u64> {
    return Vec2 {
      x: self.x * other_vec2.x,
      y: self.y * other_vec2.y,
    };
  }
}

impl MulAssign for Vec2<u64> {
  fn mul_assign(&mut self, other_vec2: Vec2<u64>) {
    self.x = self.x * other_vec2.x;
    self.y = self.y * other_vec2.y;
  }
}

impl Mul for Vec2<i32> {
  type Output = Vec2<i32>;
  
  fn mul(self, other_vec2: Self) -> Vec2<i32> {
    return Vec2 {
      x: self.x * other_vec2.x,
      y: self.y * other_vec2.y,
    };
  }
}

impl MulAssign for Vec2<i32> {
  fn mul_assign(&mut self, other_vec2: Vec2<i32>) {
    self.x = self.x * other_vec2.x;
    self.y = self.y * other_vec2.y;
  }
}

impl Mul for Vec2<i64> {
  type Output = Vec2<i64>;
  
  fn mul(self, other_vec2: Self) -> Vec2<i64> {
    return Vec2 {
      x: self.x * other_vec2.x,
      y: self.y * other_vec2.y,
    };
  }
}

impl MulAssign for Vec2<i64> {
  fn mul_assign(&mut self, other_vec2: Vec2<i64>) {
    self.x = self.x * other_vec2.x;
    self.y = self.y * other_vec2.y;
  }
}

impl Mul for Vec2<f32> {
  type Output = Vec2<f32>;
  
  fn mul(self, other_vec2: Self) -> Vec2<f32> {
    return Vec2 {
      x: self.x * other_vec2.x,
      y: self.y * other_vec2.y,
    };
  }
}

impl MulAssign for Vec2<f32> {
  fn mul_assign(&mut self, other_vec2: Vec2<f32>) {
    self.x = self.x * other_vec2.x;
    self.y = self.y * other_vec2.y;
  }
}

impl Mul for Vec2<f64> {
  type Output = Vec2<f64>;
  
  fn mul(self, other_vec2: Self) -> Vec2<f64> {
    return Vec2 {
      x: self.x * other_vec2.x,
      y: self.y * other_vec2.y,
    };
  }
}

impl MulAssign for Vec2<f64> {
  fn mul_assign(&mut self, other_vec2: Vec2<f64>) {
    self.x = self.x * other_vec2.x;
    self.y = self.y * other_vec2.y;
  }
}

impl Div for Vec2<u32> {
  type Output = Vec2<u32>;
  
  fn div(self, other_vec2: Self) -> Vec2<u32> {
    return Vec2 {
      x: self.x / other_vec2.x,
      y: self.y / other_vec2.y,
    };
  }
}

impl DivAssign for Vec2<u32> {
  fn div_assign(&mut self, other_vec2: Vec2<u32>) {
    self.x = self.x / other_vec2.x;
    self.y = self.y / other_vec2.y;
  }
}

impl Div for Vec2<u64> {
  type Output = Vec2<u64>;
  
  fn div(self, other_vec2: Self) -> Vec2<u64> {
    return Vec2 {
      x: self.x / other_vec2.x,
      y: self.y / other_vec2.y,
    };
  }
}

impl DivAssign for Vec2<u64> {
  fn div_assign(&mut self, other_vec2: Vec2<u64>) {
    self.x = self.x / other_vec2.x;
    self.y = self.y / other_vec2.y;
  }
}

impl Div for Vec2<i32> {
  type Output = Vec2<i32>;
  
  fn div(self, other_vec2: Self) -> Vec2<i32> {
    return Vec2 {
      x: self.x / other_vec2.x,
      y: self.y / other_vec2.y,
    };
  }
}

impl DivAssign for Vec2<i32> {
  fn div_assign(&mut self, other_vec2: Vec2<i32>) {
    self.x = self.x / other_vec2.x;
    self.y = self.y / other_vec2.y;
  }
}

impl Div for Vec2<i64> {
  type Output = Vec2<i64>;
  
  fn div(self, other_vec2: Self) -> Vec2<i64> {
    return Vec2 {
      x: self.x / other_vec2.x,
      y: self.y / other_vec2.y,
    };
  }
}

impl DivAssign for Vec2<i64> {
  fn div_assign(&mut self, other_vec2: Vec2<i64>) {
    self.x = self.x / other_vec2.x;
    self.y = self.y / other_vec2.y;
  }
}

impl Div for Vec2<f32> {
  type Output = Vec2<f32>;
  
  fn div(self, other_vec2: Self) -> Vec2<f32> {
    return Vec2 {
      x: self.x / other_vec2.x,
      y: self.y / other_vec2.y,
    };
  }
}

impl DivAssign for Vec2<f32> {
  fn div_assign(&mut self, other_vec2: Vec2<f32>) {
    self.x = self.x / other_vec2.x;
    self.y = self.y / other_vec2.y;
  }
}

impl Div for Vec2<f64> {
  type Output = Vec2<f64>;
  
  fn div(self, other_vec2: Self) -> Vec2<f64> {
    return Vec2 {
      x: self.x / other_vec2.x,
      y: self.y / other_vec2.y,
    };
  }
}

impl DivAssign for Vec2<f64> {
  fn div_assign(&mut self, other_vec2: Vec2<f64>) {
    self.x = self.x / other_vec2.x;
    self.y = self.y / other_vec2.y;
  }
}

impl std::fmt::Display for Vec3<u32> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec3 (u32)] --> x :{}, y: {}, z: {}", self.x, self.y, self.z)
  }
}

impl std::fmt::Display for Vec3<u64> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec3 (u64)] --> x :{}, y: {}, z: {}", self.x, self.y, self.z)
  }
}

impl std::fmt::Display for Vec3<i32> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec3 (i32)] --> x :{}, y: {}, z: {}", self.x, self.y, self.z)
  }
}

impl std::fmt::Display for Vec3<i64> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec3 (i64)] --> x :{}, y: {}, z: {}", self.x, self.y, self.z)
  }
}

impl std::fmt::Display for Vec3<f32> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec3 (f32)] --> x :{}, y: {}, z: {}", self.x, self.y, self.z)
  }
}

impl std::fmt::Display for Vec3<f64> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec3 (f64)] --> x :{}, y: {}, z: {}", self.x, self.y, self.z)
  }
}

impl Add for Vec3<u32> {
  type Output = Vec3<u32>;
  
  fn add(self, other_vector3: Self) -> Vec3<u32> {
    return Vec3 {
      x: self.x + other_vector3.x,
      y: self.y + other_vector3.y,
      z: self.z + other_vector3.z,
    };
  }
}

impl AddAssign for Vec3<u32> {
  fn add_assign(&mut self, other_vec3: Vec3<u32>) {
    self.x = self.x + other_vec3.x;
    self.y = self.y + other_vec3.y;
    self.z = self.z + other_vec3.z;
  }
}

impl Add for Vec3<u64> {
  type Output = Vec3<u64>;
  
  fn add(self, other_vector3: Self) -> Vec3<u64> {
    return Vec3 {
      x: self.x + other_vector3.x,
      y: self.y + other_vector3.y,
      z: self.z + other_vector3.z,
    };
  }
}

impl AddAssign for Vec3<u64> {
  fn add_assign(&mut self, other_vec3: Vec3<u64>) {
    self.x = self.x + other_vec3.x;
    self.y = self.y + other_vec3.y;
    self.z = self.z + other_vec3.z;
  }
}

impl Add for Vec3<i32> {
  type Output = Vec3<i32>;
  
  fn add(self, other_vector3: Self) -> Vec3<i32> {
    return Vec3 {
      x: self.x + other_vector3.x,
      y: self.y + other_vector3.y,
      z: self.z + other_vector3.z,
    };
  }
}

impl AddAssign for Vec3<i32> {
  fn add_assign(&mut self, other_vec3: Vec3<i32>) {
    self.x = self.x + other_vec3.x;
    self.y = self.y + other_vec3.y;
    self.z = self.z + other_vec3.z;
  }
}

impl Add for Vec3<i64> {
  type Output = Vec3<i64>;
  
  fn add(self, other_vector3: Self) -> Vec3<i64> {
    return Vec3 {
      x: self.x + other_vector3.x,
      y: self.y + other_vector3.y,
      z: self.z + other_vector3.z,
    };
  }
}

impl AddAssign for Vec3<i64> {
  fn add_assign(&mut self, other_vec3: Vec3<i64>) {
    self.x = self.x + other_vec3.x;
    self.y = self.y + other_vec3.y;
    self.z = self.z + other_vec3.z;
  }
}

impl Add for Vec3<f32> {
  type Output = Vec3<f32>;
  
  fn add(self, other_vector3: Self) -> Vec3<f32> {
    return Vec3 {
      x: self.x + other_vector3.x,
      y: self.y + other_vector3.y,
      z: self.z + other_vector3.z,
    };
  }
}

impl AddAssign for Vec3<f32> {
  fn add_assign(&mut self, other_vec3: Vec3<f32>) {
    self.x = self.x + other_vec3.x;
    self.y = self.y + other_vec3.y;
    self.z = self.z + other_vec3.z;
  }
}

impl Add for Vec3<f64> {
  type Output = Vec3<f64>;
  
  fn add(self, other_vector3: Self) -> Vec3<f64> {
    return Vec3 {
      x: self.x + other_vector3.x,
      y: self.y + other_vector3.y,
      z: self.z + other_vector3.z,
    };
  }
}

impl AddAssign for Vec3<f64> {
  fn add_assign(&mut self, other_vec3: Vec3<f64>) {
    self.x = self.x + other_vec3.x;
    self.y = self.y + other_vec3.y;
    self.z = self.z + other_vec3.z;
  }
}

impl Sub for Vec3<u32> {
  type Output = Vec3<u32>;
  
  fn sub(self, other_vec2: Self) -> Vec3<u32> {
    return Vec3 {
      x: self.x - other_vec2.x,
      y: self.y - other_vec2.y,
      z: self.x - other_vec2.z,
    };
  }
}

impl SubAssign for Vec3<u32> {
  fn sub_assign(&mut self, other_vec3: Vec3<u32>) {
    self.x = self.x - other_vec3.x;
    self.y = self.y - other_vec3.y;
    self.z = self.z - other_vec3.z;
  }
}

impl Sub for Vec3<u64> {
  type Output = Vec3<u64>;
  
  fn sub(self, other_vec2: Self) -> Vec3<u64> {
    return Vec3 {
      x: self.x - other_vec2.x,
      y: self.y - other_vec2.y,
      z: self.x - other_vec2.z,
    };
  }
}

impl SubAssign for Vec3<u64> {
  fn sub_assign(&mut self, other_vec3: Vec3<u64>) {
    self.x = self.x - other_vec3.x;
    self.y = self.y - other_vec3.y;
    self.z = self.z - other_vec3.z;
  }
}

impl Sub for Vec3<i32> {
  type Output = Vec3<i32>;
  
  fn sub(self, other_vec2: Self) -> Vec3<i32> {
    return Vec3 {
      x: self.x - other_vec2.x,
      y: self.y - other_vec2.y,
      z: self.x - other_vec2.z,
    };
  }
}

impl SubAssign for Vec3<i32> {
  fn sub_assign(&mut self, other_vec3: Vec3<i32>) {
    self.x = self.x - other_vec3.x;
    self.y = self.y - other_vec3.y;
    self.z = self.z - other_vec3.z;
  }
}

impl Sub for Vec3<i64> {
  type Output = Vec3<i64>;
  
  fn sub(self, other_vec2: Self) -> Vec3<i64> {
    return Vec3 {
      x: self.x - other_vec2.x,
      y: self.y - other_vec2.y,
      z: self.x - other_vec2.z,
    };
  }
}

impl SubAssign for Vec3<i64> {
  fn sub_assign(&mut self, other_vec3: Vec3<i64>) {
    self.x = self.x - other_vec3.x;
    self.y = self.y - other_vec3.y;
    self.z = self.z - other_vec3.z;
  }
}

impl Sub for Vec3<f32> {
  type Output = Vec3<f32>;
  
  fn sub(self, other_vec2: Self) -> Vec3<f32> {
    return Vec3 {
      x: self.x - other_vec2.x,
      y: self.y - other_vec2.y,
      z: self.x - other_vec2.z,
    };
  }
}

impl SubAssign for Vec3<f32> {
  fn sub_assign(&mut self, other_vec3: Vec3<f32>) {
    self.x = self.x - other_vec3.x;
    self.y = self.y - other_vec3.y;
    self.z = self.z - other_vec3.z;
  }
}

impl Sub for Vec3<f64> {
  type Output = Vec3<f64>;
  
  fn sub(self, other_vec2: Self) -> Vec3<f64> {
    return Vec3 {
      x: self.x - other_vec2.x,
      y: self.y - other_vec2.y,
      z: self.x - other_vec2.z,
    };
  }
}

impl SubAssign for Vec3<f64> {
  fn sub_assign(&mut self, other_vec3: Vec3<f64>) {
    self.x = self.x - other_vec3.x;
    self.y = self.y - other_vec3.y;
    self.z = self.z - other_vec3.z;
  }
}

impl Mul for Vec3<u32> {
  type Output = Vec3<u32>;
  
  fn mul(self, other_vec3: Self) -> Vec3<u32> {
    return Vec3 {
      x: self.x * other_vec3.x,
      y: self.y * other_vec3.y,
      z: self.z * other_vec3.z,
    };
  }
}

impl MulAssign for Vec3<u32> {
  fn mul_assign(&mut self, other_vec3: Vec3<u32>) {
    self.x = self.x * other_vec3.x;
    self.y = self.y * other_vec3.y;
    self.z = self.z * other_vec3.z;
  }
}

impl Mul for Vec3<u64> {
  type Output = Vec3<u64>;
  
  fn mul(self, other_vec3: Self) -> Vec3<u64> {
    return Vec3 {
      x: self.x * other_vec3.x,
      y: self.y * other_vec3.y,
      z: self.z * other_vec3.z,
    };
  }
}

impl MulAssign for Vec3<u64> {
  fn mul_assign(&mut self, other_vec3: Vec3<u64>) {
    self.x = self.x * other_vec3.x;
    self.y = self.y * other_vec3.y;
    self.z = self.z * other_vec3.z;
  }
}

impl Mul for Vec3<i32> {
  type Output = Vec3<i32>;
  
  fn mul(self, other_vec3: Self) -> Vec3<i32> {
    return Vec3 {
      x: self.x * other_vec3.x,
      y: self.y * other_vec3.y,
      z: self.z * other_vec3.z,
    };
  }
}

impl MulAssign for Vec3<i32> {
  fn mul_assign(&mut self, other_vec3: Vec3<i32>) {
    self.x = self.x * other_vec3.x;
    self.y = self.y * other_vec3.y;
    self.z = self.z * other_vec3.z;
  }
}

impl Mul for Vec3<i64> {
  type Output = Vec3<i64>;
  
  fn mul(self, other_vec3: Self) -> Vec3<i64> {
    return Vec3 {
      x: self.x * other_vec3.x,
      y: self.y * other_vec3.y,
      z: self.z * other_vec3.z,
    };
  }
}

impl MulAssign for Vec3<i64> {
  fn mul_assign(&mut self, other_vec3: Vec3<i64>) {
    self.x = self.x * other_vec3.x;
    self.y = self.y * other_vec3.y;
    self.z = self.z * other_vec3.z;
  }
}

impl Mul for Vec3<f32> {
  type Output = Vec3<f32>;
  
  fn mul(self, other_vec3: Self) -> Vec3<f32> {
    return Vec3 {
      x: self.x * other_vec3.x,
      y: self.y * other_vec3.y,
      z: self.z * other_vec3.z,
    };
  }
}

impl MulAssign for Vec3<f32> {
  fn mul_assign(&mut self, other_vec3: Vec3<f32>) {
    self.x = self.x * other_vec3.x;
    self.y = self.y * other_vec3.y;
    self.z = self.z * other_vec3.z;
  }
}

impl Mul for Vec3<f64> {
  type Output = Vec3<f64>;
  
  fn mul(self, other_vec3: Self) -> Vec3<f64> {
    return Vec3 {
      x: self.x * other_vec3.x,
      y: self.y * other_vec3.y,
      z: self.z * other_vec3.z,
    };
  }
}

impl MulAssign for Vec3<f64> {
  fn mul_assign(&mut self, other_vec3: Vec3<f64>) {
    self.x = self.x * other_vec3.x;
    self.y = self.y * other_vec3.y;
    self.z = self.z * other_vec3.z;
  }
}

impl Div for Vec3<u32> {
  type Output = Vec3<u32>;
  
  fn div(self, other_vec3: Self) -> Vec3<u32> {
    return Vec3 {
      x: self.x / other_vec3.x,
      y: self.y / other_vec3.y,
      z: self.z / other_vec3.z,
    };
  }
}

impl DivAssign for Vec3<u32> {
  fn div_assign(&mut self, other_vec3: Vec3<u32>) {
    self.x = self.x / other_vec3.x;
    self.y = self.y / other_vec3.y;
    self.z = self.z / other_vec3.z;
  }
}

impl Div for Vec3<u64> {
  type Output = Vec3<u64>;
  
  fn div(self, other_vec3: Self) -> Vec3<u64> {
    return Vec3 {
      x: self.x / other_vec3.x,
      y: self.y / other_vec3.y,
      z: self.z / other_vec3.z,
    };
  }
}

impl DivAssign for Vec3<u64> {
  fn div_assign(&mut self, other_vec3: Vec3<u64>) {
    self.x = self.x / other_vec3.x;
    self.y = self.y / other_vec3.y;
    self.z = self.z / other_vec3.z;
  }
}

impl Div for Vec3<i32> {
  type Output = Vec3<i32>;
  
  fn div(self, other_vec3: Self) -> Vec3<i32> {
    return Vec3 {
      x: self.x / other_vec3.x,
      y: self.y / other_vec3.y,
      z: self.z / other_vec3.z,
    };
  }
}

impl DivAssign for Vec3<i32> {
  fn div_assign(&mut self, other_vec3: Vec3<i32>) {
    self.x = self.x / other_vec3.x;
    self.y = self.y / other_vec3.y;
    self.z = self.z / other_vec3.z;
  }
}

impl Div for Vec3<i64> {
  type Output = Vec3<i64>;
  
  fn div(self, other_vec3: Self) -> Vec3<i64> {
    return Vec3 {
      x: self.x / other_vec3.x,
      y: self.y / other_vec3.y,
      z: self.z / other_vec3.z,
    };
  }
}

impl DivAssign for Vec3<i64> {
  fn div_assign(&mut self, other_vec3: Vec3<i64>) {
    self.x = self.x / other_vec3.x;
    self.y = self.y / other_vec3.y;
    self.z = self.z / other_vec3.z;
  }
}

impl Div for Vec3<f32> {
  type Output = Vec3<f32>;
  
  fn div(self, other_vec3: Self) -> Vec3<f32> {
    return Vec3 {
      x: self.x / other_vec3.x,
      y: self.y / other_vec3.y,
      z: self.z / other_vec3.z,
    };
  }
}

impl DivAssign for Vec3<f32> {
  fn div_assign(&mut self, other_vec3: Vec3<f32>) {
    self.x = self.x / other_vec3.x;
    self.y = self.y / other_vec3.y;
    self.z = self.z / other_vec3.z;
  }
}

impl Div for Vec3<f64> {
  type Output = Vec3<f64>;
  
  fn div(self, other_vec3: Self) -> Vec3<f64> {
    return Vec3 {
      x: self.x / other_vec3.x,
      y: self.y / other_vec3.y,
      z: self.z / other_vec3.z,
    };
  }
}

impl DivAssign for Vec3<f64> {
  fn div_assign(&mut self, other_vec3: Vec3<f64>) {
    self.x = self.x / other_vec3.x;
    self.y = self.y / other_vec3.y;
    self.z = self.z / other_vec3.z;
  }
}

/*
/////////////////////////////////// MATRICES ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */