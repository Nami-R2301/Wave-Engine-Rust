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

// Operator overloading.
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::ptr::eq;

/*
///////////////////////////////////   VEC2  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

#[derive(Copy, Clone)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}

#[derive(Copy, Clone)]
pub struct Vec3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

///////////////////// DEBUG ////////////////////////

impl<T: std::fmt::Debug> std::fmt::Debug for Vec2<T> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    format.debug_struct("Vec2")
      .field("x", &self.x)
      .field("y", &self.y)
      .finish()
  }
}

///////////////////// DISPLAY ////////////////////////

impl<T: std::fmt::Display> std::fmt::Display for Vec2<T> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec2] --> x :{}, y: {}", &self.x, &self.y)
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
    return !eq(&self, &other_vec2);
  }
}

///////////////////// ARITHMETIC ////////////////////////

// For references.
impl<T> Add for &Vec2<T> where for<'a> &'a T: Add<&'a T, Output = T> {
  type Output = Vec2<T>;
  
  fn add(self, other_vec2: &Vec2<T>) -> Vec2<T> {
    return Vec2 {
      x: &self.x + &other_vec2.x,
      y: &self.y + &other_vec2.y,
    };
  }
}

// For values.
impl<T> Add for Vec2<T> where for<'a> &'a T: Add<&'a T, Output = T> {
  type Output = Vec2<T>;
  
  fn add(self, other_vec2: Vec2<T>) -> Vec2<T> {
    return Vec2 {
      x: &self.x + &other_vec2.x,
      y: &self.y + &other_vec2.y,
    };
  }
}


impl<T> AddAssign for Vec2<T> where for<'a> &'a T: Add<&'a T, Output = T> {
  fn add_assign(&mut self, other_vec2: Vec2<T>) {
    self.x = &self.x + &other_vec2.x;
    self.y = &self.y + &other_vec2.y;
  }
}

// For references.
impl<T> Sub for &Vec2<T> where for<'a> &'a T: Sub<&'a T, Output = T> {
  type Output = Vec2<T>;
  
  fn sub(self, other_vec2: &Vec2<T>) -> Vec2<T> {
    return Vec2 {
      x: &self.x - &other_vec2.x,
      y: &self.y - &other_vec2.y,
    };
  }
}

// For values.
impl<T> Sub for Vec2<T> where for<'a> &'a T: Sub<&'a T, Output = T> {
  type Output = Vec2<T>;
  
  fn sub(self, other_vec2: Vec2<T>) -> Vec2<T> {
    return Vec2 {
      x: &self.x - &other_vec2.x,
      y: &self.y - &other_vec2.y,
    };
  }
}

impl<T> SubAssign for Vec2<T> where for<'a> &'a T: Sub<&'a T, Output = T> {
  fn sub_assign(&mut self, other_vec2: Vec2<T>) {
    self.x = &self.x - &other_vec2.x;
    self.y = &self.y - &other_vec2.y;
  }
}

// For references.
impl<T> Mul for &Vec2<T> where for<'a> &'a T: Mul<&'a T, Output = T> {
  type Output = Vec2<T>;
  
  fn mul(self, other_vec2: &Vec2<T>) -> Vec2<T> {
    return Vec2 {
      x: &self.x * &other_vec2.x,
      y: &self.y * &other_vec2.y,
    };
  }
}

// For values.
impl<T> Mul for Vec2<T> where for<'a> &'a T: Mul<&'a T, Output = T> {
  type Output = Vec2<T>;
  
  fn mul(self, other_vec2: Vec2<T>) -> Vec2<T> {
    return Vec2 {
      x: &self.x * &other_vec2.x,
      y: &self.y * &other_vec2.y,
    };
  }
}

impl<T> MulAssign for Vec2<T> where for<'a> &'a T: Mul<&'a T, Output = T> {
  fn mul_assign(&mut self, other_vec2: Vec2<T>) {
    self.x = &self.x * &other_vec2.x;
    self.y = &self.y * &other_vec2.y;
  }
}


// For references.
impl<T> Div for &Vec2<T> where for<'a> &'a T: Div<&'a T, Output = T> {
  type Output = Vec2<T>;
  
  fn div(self, other_vec2: &Vec2<T>) -> Vec2<T> {
    return Vec2 {
      x: &self.x / &other_vec2.x,
      y: &self.y / &other_vec2.y,
    };
  }
}

// For values.
impl<T> Div for Vec2<T> where for<'a> &'a T: Div<&'a T, Output = T> {
  type Output = Vec2<T>;
  
  fn div(self, other_vec2: Vec2<T>) -> Vec2<T> {
    return Vec2 {
      x: &self.x / &other_vec2.x,
      y: &self.y / &other_vec2.y,
    };
  }
}

impl<T> DivAssign for Vec2<T> where for<'a> &'a T: Div<&'a T, Output = T> {
  fn div_assign(&mut self, other_vec2: Vec2<T>) {
    self.x = &self.x / &other_vec2.x;
    self.y = &self.y / &other_vec2.y;
  }
}

/*
///////////////////////////////////   VEC3  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

///////////////////// DEBUG ////////////////////////

impl<T : std::fmt::Debug> std::fmt::Debug for Vec3<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Vec3")
      .field("x", &self.x)
      .field("y", &self.y)
      .field("z", &self.z)
      .finish()
  }
}

///////////////////// DISPLAY ////////////////////////

impl<T: std::fmt::Display> std::fmt::Display for Vec3<T> {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(format, "[Vec3] --> x :{}, y: {}, z: {}", &self.x, &self.y, &self.z)
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
    return !eq(&self, &other_vec3);
  }
}

impl<T> Add for Vec3<T> where for<'a> &'a T: Add<&'a T, Output = T> {
  type Output = Vec3<T>;
  
  fn add(self, other_vec2: Self) -> Vec3<T> {
    return Vec3 {
      x: &self.x + &other_vec2.x,
      y: &self.y + &other_vec2.y,
      z: &self.z + &other_vec2.z,
    };
  }
}

impl<T> AddAssign for Vec3<T> where for<'a> &'a T: Add<&'a T, Output = T> {
  fn add_assign(&mut self, other_vec3: Vec3<T>) {
    self.x = &self.x + &other_vec3.x;
    self.y = &self.y + &other_vec3.y;
    self.z = &self.z + &other_vec3.z;
  }
}

impl<T> Sub for Vec3<T> where for<'a> &'a T: Sub<&'a T, Output = T> {
  type Output = Vec3<T>;
  
  fn sub(self, other_vec3: Self) -> Vec3<T> {
    return Vec3 {
      x: &self.x - &other_vec3.x,
      y: &self.y - &other_vec3.y,
      z: &self.z - &other_vec3.z,
    };
  }
}

impl<T> SubAssign for Vec3<T> where for<'a> &'a T: Sub<&'a T, Output = T> {
  fn sub_assign(&mut self, other_vec3: Vec3<T>) {
    self.x = &self.x - &other_vec3.x;
    self.y = &self.y - &other_vec3.y;
    self.z = &self.z - &other_vec3.z;
  }
}

impl<T> Mul for Vec3<T> where for<'a> &'a T: Mul<&'a T, Output = T> {
  type Output = Vec3<T>;
  
  fn mul(self, other_vec3: Self) -> Vec3<T> {
    return Vec3 {
      x: &self.x * &other_vec3.x,
      y: &self.y * &other_vec3.y,
      z: &self.z * &other_vec3.z,
    };
  }
}

impl<T> MulAssign for Vec3<T> where for<'a> &'a T: Mul<&'a T, Output = T> {
  fn mul_assign(&mut self, other_vec3: Vec3<T>) {
    self.x = &self.x * &other_vec3.x;
    self.y = &self.y * &other_vec3.y;
    self.z = &self.z * &other_vec3.z;
  }
}

impl<T> Div for Vec3<T> where for<'a> &'a T: Div<&'a T, Output = T> {
  type Output = Vec3<T>;
  
  fn div(self, other_vec3: Self) -> Vec3<T> {
    return Vec3 {
      x: &self.x / &other_vec3.x,
      y: &self.y / &other_vec3.y,
      z: &self.z / &other_vec3.z,
    };
  }
}

impl<T> DivAssign for Vec3<T> where for<'a> &'a T: Div<&'a T, Output = T> {
  fn div_assign(&mut self, other_vec3: Vec3<T>) {
    self.x = &self.x / &other_vec3.x;
    self.y = &self.y / &other_vec3.y;
    self.z = &self.z / &other_vec3.z;
  }
}

/*
/////////////////////////////////// MATRICES ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */