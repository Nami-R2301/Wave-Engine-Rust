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

use crate::utils::macros::logger::*;
#[cfg(feature = "debug")]
use crate::Engine;
use crate::{EnumEngineError, events};
use crate::layers::{EnumLayerType, TraitLayer};
use crate::window::{Window};

pub struct WindowLayer {
  pub(crate) m_context: *mut Window,
}

impl WindowLayer {
  pub fn new(window_context: &mut Window) -> Self {
    return Self {
      m_context: window_context,
    }
  }
}

impl TraitLayer for WindowLayer {
  fn get_type(&self) -> EnumLayerType {
    return EnumLayerType::Window;
  }
  
  fn on_apply(&mut self) -> Result<(), EnumEngineError> {
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Creating window...");
    unsafe {
      (*self.m_context).apply()?
    };
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Created window successfully");
    return Ok(());
  }
  
  fn on_sync_event(&mut self) -> Result<(), EnumEngineError> {
    return Ok(());
  }
  
  fn on_async_event(&mut self, event: &events::EnumEvent) -> Result<bool, EnumEngineError> {
    return unsafe {
      Ok((*self.m_context).on_event(event))
    };
  }
  
  fn on_update(&mut self, _time_step: f64) -> Result<(), EnumEngineError> {
    return unsafe {
      (*self.m_context).on_update().map_err(|err| EnumEngineError::from(err))
    };
  }
  
  fn on_render(&mut self) -> Result<(), EnumEngineError> {
    unsafe { (*self.m_context).refresh() };
    return Ok(());
  }
  
  fn free(&mut self) -> Result<(), EnumEngineError> {
    return unsafe {
      (*self.m_context).free().map_err(|err| EnumEngineError::from(err))
    };
  }
  
  fn to_string(&self) -> String {
    unsafe {
      return format!("\n{0:115}State: {1:?}\n{0:115}Api: {2:?}\n{0:115}Resolution: ({3},{4})\n{0:115}\
      Vsync?: {5}\n{0:115}MSAA?: {6}",
        "",
        (*self.m_context).m_state,
        (*self.m_context).m_api_window.as_ref().unwrap().glfw,
        (*self.m_context).m_window_resolution.unwrap().0, (*self.m_context).m_window_resolution.unwrap().1,
        (*self.m_context).m_vsync, (*self.m_context).m_samples.is_none().then(|| "Disabled").unwrap_or("Enabled"));
    }
  }
}