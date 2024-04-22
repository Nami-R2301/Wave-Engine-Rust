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
use std::ops::BitAnd;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Color {
  pub m_rgba: u32
}

impl Debug for Color {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Color] --> ({0}): r: {1}, g: {2}, b: {3}, a: {4}", self.m_rgba, self.m_rgba.bitand(0x000000FF),
      self.m_rgba.bitand(0x0000FF00) >> 8, self.m_rgba.bitand(0x00FF0000) >> 16, self.m_rgba.bitand(0xFF000000) >> 24)
  }
}

impl Color {
  pub fn default() -> Self {
    return Color::from([1.0, 1.0, 1.0, 1.0]);
  }
  
  pub fn reset(&mut self) {
    self.m_rgba = Self::default().m_rgba;
  }
  
  pub fn as_u8(&self) -> [u8; 4] {
    return [self.m_rgba.bitand(0x000000FF) as u8,
      (self.m_rgba.bitand(0x0000FF00) >> 8) as u8,
      (self.m_rgba.bitand(0x00FF0000) >> 16) as u8,
      (self.m_rgba.bitand(0xFF000000) >> 24) as u8];
  }
  
  pub fn as_f32(&self) -> [f32; 4] {
    return [self.m_rgba.bitand(0x000000FF) as f32 / 255.0,
      (self.m_rgba.bitand(0x0000FF00) >> 8) as f32 / 255.0,
      (self.m_rgba.bitand(0x00FF0000) >> 16) as f32 / 255.0,
      (self.m_rgba.bitand(0xFF000000) >> 24) as f32 / 255.0];
  }
}

impl Into<[u8; 4]> for Color {
  fn into(self) -> [u8; 4] {
    return self.as_u8();
  }
}

impl Into<[f32; 4]> for Color {
  fn into(self) -> [f32; 4] {
    return self.as_f32();
  }
}

impl From<[u8; 4]> for Color {
  fn from(rgba: [u8; 4]) -> Self {
    // (RGBA) -> (ABGR)
    let red: u32 = rgba[0] as u32;
    let green: u32 = (rgba[1] as u32) << 8;
    let blue: u32 = (rgba[2] as u32) << 16;
    let alpha: u32 = (rgba[3] as u32) << 24;
    return Color {
      m_rgba: red + green + blue + alpha
    }
  }
}

impl From<[f32; 4]> for Color {
  fn from(rgba: [f32; 4]) -> Self {
    // (RGBA) -> (ABGR)
    let red: u32 = (rgba[0] * 255.0) as u32;
    let green: u32 = ((rgba[1] * 255.0) as u32) << 8;
    let blue: u32 = ((rgba[2] * 255.0) as u32) << 16;
    let alpha: u32 = ((rgba[3] * 255.0) as u32) << 24;
    return Color {
      m_rgba: red + green + blue + alpha
    }
  }
}

impl From<u32> for Color {
  fn from(rgba: u32) -> Self {
    return Color {
      m_rgba: rgba
    }
  }
}