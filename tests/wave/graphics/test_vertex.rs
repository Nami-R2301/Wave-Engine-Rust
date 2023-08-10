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

use wave_engine::wave::math::{Vec2, Vec3};
use wave_engine::wave::graphics::vertex::{Color, GlVertex3D};

#[test]
fn test_equality() {
  let new_vertex = GlVertex3D::new();
  let mut other_vertex = GlVertex3D {
    m_id: -1,
    m_position: Vec3::new(),
    m_normal: Vec3::new(),
    m_color: Color::new(),
    m_texture_coords: Vec2::new(),
  };
  
  assert_eq!(new_vertex, other_vertex);
  other_vertex.m_id = 0;
  assert_ne!(new_vertex, other_vertex);
}

#[test]
fn test_display() {
  let new_vertex = GlVertex3D::new();
  let other_vertex = GlVertex3D {
    m_id: -1,
    m_position: Vec3::new(),
    m_normal: Vec3::new(),
    m_color: Color::new(),
    m_texture_coords: Vec2::new(),
  };
  
  assert_eq!(new_vertex.to_string(), other_vertex.to_string());
  assert_eq!(new_vertex.to_string(), "[GlVertex3D] --> ID => -1; Position => [Vec3] --> x: 0.000, \
   y: 0.000, z: 0.000, ; Normal => [Vec3] --> x: 0.000, y: 0.000, z: 0.000, ; Color => [Color] --> \
   r: 0.000, g: 0.000, b: 0.000, a: 0.000, ; Texture coords => [Vec2] --> x: 0.000, y: 0.000, "
    .to_string())
}