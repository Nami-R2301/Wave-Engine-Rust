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

use crate::wave_core::EnumError;
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
    todo!()
  }
  
  fn on_event(&mut self) -> Result<bool, EnumError> {
    todo!()
  }
  
  fn on_update(&mut self) -> Result<(), EnumError> {
    todo!()
  }
  
  fn on_render(&mut self) -> Result<(), EnumError> {
    todo!()
  }
  
  fn on_delete(&mut self) -> Result<(), EnumError> {
    todo!()
  }
}