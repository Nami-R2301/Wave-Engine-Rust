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
use wave_editor::wave_core::events::*;
use wave_editor::wave_core::graphics::renderer::{self};
use wave_editor::wave_core::window::{self};

// App.
use wave_editor::Editor;

fn main() -> Result<(), EnumEngineError> {
  
  // Instantiate an app layer containing our app and essentially making the layer own it.
  let mut my_app: Layer = Layer::new("Wave Engine Editor", Editor::default());
  // Making editor poll input events during async call.
  my_app.enable_async_polling_for(EnumEventMask::Input | EnumEventMask::WindowClose);
  // Make editor synchronously poll on each frame interval (after async), for movement and spontaneous event handling.
  my_app.enable_sync_polling();
  my_app.set_sync_interval(EnumSyncInterval::Every(10))?;
  
  let mut window = window::Window::new()?;
  window.window_hint(window::EnumWindowOption::RendererApi(renderer::EnumApi::OpenGL));
  window.window_hint(window::EnumWindowOption::WindowMode(window::EnumWindowMode::Windowed));
  window.window_hint(window::EnumWindowOption::Resizable(true));
  window.window_hint(window::EnumWindowOption::DebugApi(true));
  window.window_hint(window::EnumWindowOption::Maximized(true));
  
  let mut renderer = renderer::Renderer::new(renderer::EnumApi::OpenGL)?;
  renderer.renderer_hint(renderer::EnumRendererOption::ApiCallChecking(renderer::EnumCallCheckingType::Async));
  renderer.renderer_hint(renderer::EnumRendererOption::SRGB(true));
  renderer.renderer_hint(renderer::EnumRendererOption::Wireframe(true));
  
  // Supply it to our engine. Engine will NOT construct app and will only init the engine
  // with the supplied GPU API of choice as its renderer. Note that passing None will default to Vulkan if
  // supported, otherwise falling back to OpenGL.
  let mut engine: Engine = Engine::new(window, renderer, my_app)?;
  
  // Init and execute the app in game loop and return if there's a close event or if an error occurred.
  return engine.run();
}
