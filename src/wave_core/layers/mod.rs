/*
 MIT License

 Copyright (c) 2024 Nami Reghbati

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

use crate::wave_core;

pub mod app_layer;
pub mod window_layer;
pub mod renderer_layer;
pub mod shader_layer;
pub mod imgui_layer;

pub struct Layer {
  pub m_uuid: u64,
  pub m_name: &'static str,
  m_data: Box<dyn std::any::Any>
}

pub trait TraitLayer {
  fn on_new(&mut self) -> Result<(), wave_core::EnumError>;
  fn on_event(&mut self) -> Result<bool, wave_core::EnumError>;
  fn on_update(&mut self) -> Result<(), wave_core::EnumError>;
  fn on_render(&mut self) -> Result<(), wave_core::EnumError>;
  fn on_delete(&mut self) -> Result<(), wave_core::EnumError>;
}

impl Layer {
  pub fn new<T: std::any::Any + TraitLayer>(name: &'static str, data: T) -> Self {
    return Self {
      m_uuid: 0,
      m_name: name,
      m_data: Box::new(data),
    }
  }
  
  pub fn is<T: std::any::Any>(&self) -> bool {
    return self.m_data.is::<T>();
  }
  
  pub fn get<T: TraitLayer + 'static>(&mut self) -> Option<&mut T> {
    return self.m_data.downcast_mut::<T>();
  }
}