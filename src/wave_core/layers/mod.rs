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

use std::cmp::Ordering;
use std::ops::Deref;
use crate::wave_core;
use crate::wave_core::events;

pub mod app_layer;
pub mod window_layer;
pub mod renderer_layer;
pub mod imgui_layer;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumLayerType {
  Window = 0,
  Imgui = 1,
  Renderer = 2,
  App = 3
}

pub struct Layer {
  pub m_uuid: u64,
  pub m_name: &'static str,
  m_priority: u32,
  m_type: EnumLayerType,
  m_data: Box<dyn TraitLayer>,
}

impl Eq for Layer {}

impl PartialEq<Self> for Layer {
  fn eq(&self, other: &Self) -> bool {
    return self.m_type == other.m_type;
  }
}

impl PartialOrd<Self> for Layer {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return Option::from(self.m_priority.cmp(&other.m_priority));
  }
}

impl Ord for Layer {
  fn cmp(&self, other: &Self) -> Ordering {
    return self.partial_cmp(other).unwrap();
  }
}

pub trait TraitLayer {
  fn on_new(&mut self) -> Result<(), wave_core::EnumError>;
  fn on_event(&mut self, event: &events::EnumEvent) -> Result<bool, wave_core::EnumError>;
  fn on_update(&mut self, time_step: f64) -> Result<(), wave_core::EnumError>;
  fn on_render(&mut self) -> Result<(), wave_core::EnumError>;
  fn on_delete(&mut self) -> Result<(), wave_core::EnumError>;
}

impl Layer {
  pub fn new<T: TraitLayer + 'static>(name: &'static str, ty: EnumLayerType, data: T) -> Self {
    return Self {
      m_uuid: 0,
      m_name: name,
      m_priority: ty as u32,
      m_type: ty,
      m_data: Box::new(data),
    }
  }
  
  pub fn is(&self, layer_type: EnumLayerType) -> bool {
    return self.m_type == layer_type;
  }
  
  pub fn try_cast<T: TraitLayer + 'static>(&self) -> Option<&T> {
    return unsafe { Some(&*(self.m_data.deref() as *const dyn TraitLayer as *const T)) };
  }
  
  pub fn on_new(&mut self) -> Result<(), wave_core::EnumError> {
    return self.m_data.on_new();
  }
  
  pub fn on_event(&mut self, event: &events::EnumEvent) -> Result<bool, wave_core::EnumError> {
    return self.m_data.on_event(event);
  }
  
  pub fn on_update(&mut self, time_step: f64) -> Result<(), wave_core::EnumError> {
    return self.m_data.on_update(time_step);
  }
  
  pub fn on_render(&mut self) -> Result<(), wave_core::EnumError> {
    return self.m_data.on_render();
  }
  
  pub fn on_delete(&mut self) -> Result<(), wave_core::EnumError> {
    return  self.m_data.on_delete();
  }
  
}