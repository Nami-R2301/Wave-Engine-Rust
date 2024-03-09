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

use crate::wave_core::{EnumError, events};
use crate::wave_core::layers::{EnumLayerType, TraitLayer};
use crate::wave_core::ui::ui_imgui::Imgui;

pub struct ImguiLayer {
  m_ui: Imgui
}

impl ImguiLayer {
  pub fn new(imgui: Imgui) -> Self {
    return Self {
      m_ui: imgui
    }
  }
}

impl TraitLayer for ImguiLayer {
  fn get_type(&self) -> EnumLayerType {
    return EnumLayerType::Imgui;
  }
  
  fn on_new(&mut self) -> Result<(), EnumError> {
    return Ok(());
  }
  
  fn on_sync_event(&mut self) -> Result<(), EnumError> {
    todo!()
  }
  
  fn on_async_event(&mut self, event: &events::EnumEvent) -> Result<bool, EnumError> {
    return Ok(self.m_ui.on_event(event));
  }
  
  fn on_update(&mut self, _time_step: f64) -> Result<(), EnumError> {
    return Ok(self.m_ui.on_update());
  }
  
  fn on_render(&mut self) -> Result<(), EnumError> {
    return Ok(self.m_ui.on_render());
  }
  
  fn on_delete(&mut self) -> Result<(), EnumError> {
    return Ok(());
  }
}