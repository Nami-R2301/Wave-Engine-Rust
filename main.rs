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

// Wave core.
use wave_editor::wave_core::*;
use wave_editor::wave_core::layers::*;
use wave_editor::wave_core::graphics::renderer::{self};
use wave_editor::wave_core::window::{self};

// App.
use wave_editor::Editor;

fn main() -> Result<(), EnumEngineError> {
  
  // Apply default options.
  let mut window = window::Window::default();
  let mut renderer = renderer::Renderer::default();
  
  // Override some options.
  window.hint(window::EnumWindowHint::TargetApi(renderer::EnumRendererApi::OpenGL));  // Force OpenGL.
  window.hint(window::EnumWindowHint::WindowMode(window::EnumWindowMode::Windowed));  // Force Windowed.
  renderer.hint(renderer::EnumRendererHint::TargetApi(renderer::EnumRendererApi::OpenGL));  // Force OpenGL.
  
  let main_app_layer: Layer = Layer::new("My App", EmptyApp::default());
  
  // Supply all app layers to our editor. This will NOT 'apply()' editor nor engine, only filling in the structs.
  // Note that calling 'default()' will default to Vulkan for the windowing and rendering context if supported,
  // otherwise falling back to OpenGL.
  let mut editor: Editor = Editor::new(window, renderer, vec![main_app_layer]);
  
  // Applying and executing the editor in game loop. Returning upon a close event or if an error occurred.
  return editor.run();
  // Dropping all layers (including editor), followed by all engine components.
}
