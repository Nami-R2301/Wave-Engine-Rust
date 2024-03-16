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

use crate::log;
use crate::wave_core::{EnumError, events};
use crate::wave_core::layers::{EnumLayerType, TraitLayer};
use crate::wave_core::window::{Window};

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
  
  fn on_new(&mut self) -> Result<(), EnumError> {
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Creating window...");
    unsafe {
      (*self.m_context).submit()?
    };
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Created window successfully");
    return Ok(());
  }
  
  fn on_sync_event(&mut self) -> Result<(), EnumError> {
    return Ok(());
  }
  
  fn on_async_event(&mut self, event: &events::EnumEvent) -> Result<bool, EnumError> {
    return unsafe {
      Ok((*self.m_context).on_event(event))
    };
  }
  
  fn on_update(&mut self, _time_step: f64) -> Result<(), EnumError> {
    return unsafe {
      (*self.m_context).on_update().map_err(|err| EnumError::from(err))
    };
  }
  
  fn on_render(&mut self) -> Result<(), EnumError> {
    unsafe { (*self.m_context).refresh() };
    return Ok(());
  }
  
  fn on_delete(&mut self) -> Result<(), EnumError> {
    return unsafe {
      (*self.m_context).on_delete().map_err(|err| EnumError::from(err))
    };
  }
}