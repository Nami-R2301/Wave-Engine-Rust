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


use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32
}

impl Debug for Color {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Color] --> r: {0}, g: {1}, b: {2}, a: {3}", self.r, self.g, self.b, self.a)
  }
}

impl Color {
  pub fn default() -> Self {
    return Color {
      r: 0.0,
      g: 1.0,
      b: 0.0,
      a: 1.0,
    }
  }
  
  pub fn new(rgba: [f32; 4]) -> Self {
    return Color {
      r: rgba[0],
      g: rgba[1],
      b: rgba[2],
      a: rgba[3],
    }
  }
  
  pub fn reset(&mut self) {
    self.r = 0.0;
    self.g = 1.0;
    self.b = 0.0;
    self.a = 1.0;
  }
}