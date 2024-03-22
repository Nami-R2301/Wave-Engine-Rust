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
use crate::{Engine, EnumEngineError, events, input};
use crate::graphics::renderer::{Renderer};
use crate::layers::{EnumLayerType, TraitLayer};

pub struct RendererLayer {
  pub(crate) m_context: *mut Renderer
}

impl RendererLayer {
  pub fn new(renderer_context: &mut Renderer) -> Self {
    return Self {
      m_context: renderer_context
    }
  }
}

impl TraitLayer for RendererLayer {
  fn get_type(&self) -> EnumLayerType {
    return EnumLayerType::Renderer;
  }
  
  fn on_apply(&mut self) -> Result<(), EnumEngineError> {
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Setting up renderer...");
    
    // Enable features BEFORE finalizing context.
    unsafe {
      // Finalize graphics context with all hinted features to prepare for frame presentation.
      (*self.m_context).apply(Engine::get_active_window())?;
      
      log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Setup renderer successfully");
    }
    return Ok(());
  }
  
  fn on_sync_event(&mut self) -> Result<(), EnumEngineError> {
    todo!()
  }
  
  fn on_async_event(&mut self, event: &events::EnumEvent) -> Result<bool, EnumEngineError> {
      match event {
        events::EnumEvent::KeyEvent(key, action, repeat_count, modifiers) => {
          match (key, action, repeat_count, modifiers) {
            (input::EnumKey::R, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
              unsafe { (*self.m_context).flush()? };
              return Ok(true);
            }
            _ => {}
          }
        }
        _ => {}
      }
    return unsafe { (*self.m_context).on_event(event).map_err(|err| EnumEngineError::from(err)) };
  }
  
  fn on_update(&mut self, _time_step: f64) -> Result<(), EnumEngineError> {
    return Ok(());
  }
  
  fn on_render(&mut self) -> Result<(), EnumEngineError> {
    return unsafe {
      (*self.m_context).on_render().map_err(|err| EnumEngineError::from(err))
    }
  }
  
  fn free(&mut self) -> Result<(), EnumEngineError> {
    return unsafe {
      (*self.m_context).free().map_err(|err| EnumEngineError::from(err))
    }
  }
  
  fn to_string(&self) -> String {
    unsafe {
      let mut final_str: String;
      final_str = format!("\n{0:115}State: {1:?},\n{0:115}Api: {2:?},\n{0:115}Options: [{3}]",
        "", (*self.m_context).m_state, (*self.m_context).m_type, (*self.m_context).m_options.len());
      
      for (position, option) in  (*self.m_context).m_options.iter().enumerate() {
        final_str += &format!("\n{0:117}[{1}]: {2:?}", "", position + 1, option);
      }
      return final_str;
    }
  }
}