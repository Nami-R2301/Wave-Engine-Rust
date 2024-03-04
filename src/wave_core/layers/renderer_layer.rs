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

use crate::{log, wave_core};
use crate::wave_core::{EnumError, events};
use crate::wave_core::graphics::renderer;
use crate::wave_core::graphics::renderer::{Renderer};
use crate::wave_core::layers::TraitLayer;

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
  fn on_new(&mut self) -> Result<(), EnumError> {
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Setting up renderer...");
    
    // Enable features BEFORE finalizing context.
    unsafe {
      (*self.m_context).renderer_hint(renderer::EnumFeature::CullFacing(Some(gl::BACK as i64)));
      (*self.m_context).renderer_hint(renderer::EnumFeature::DepthTest(true));
      #[cfg(feature = "debug")]
      (*self.m_context).renderer_hint(renderer::EnumFeature::ApiCallChecking(renderer::EnumCallCheckingType::SyncAndAsync));
      (*self.m_context).renderer_hint(renderer::EnumFeature::Wireframe(true));
      (*self.m_context).renderer_hint(renderer::EnumFeature::MSAA(None));
      
      // Finalize graphics context with all hinted features to prepare for frame presentation.
      (*self.m_context).submit()?;
      
      log!(EnumLogColor::White, "INFO", "[Renderer] -->\t {0}", *self.m_context);
      log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Setup renderer successfully");
    }
    return Ok(());
  }
  
  fn on_event(&mut self, event: &events::EnumEvent) -> Result<bool, wave_core::EnumError> {
    return unsafe { (*self.m_context).on_event(event).map_err(|err| wave_core::EnumError::from(err)) };
  }
  
  fn on_update(&mut self, _time_step: f64) -> Result<(), EnumError> {
    return Ok(());
  }
  
  fn on_render(&mut self) -> Result<(), EnumError> {
    return unsafe {
      (*self.m_context).on_render().map_err(|err| EnumError::from(err))
    }
  }
  
  fn on_delete(&mut self) -> Result<(), EnumError> {
    return unsafe {
      (*self.m_context).on_delete().map_err(|err| EnumError::from(err))
    }
  }
}