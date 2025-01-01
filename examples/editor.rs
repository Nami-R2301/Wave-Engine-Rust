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
use std::fmt::Display;
use wave_core::events::EnumEventMask;
use wave_core::graphics::renderer::{EnumRendererApi, Renderer, TraitContext};
use wave_core::layers::{EnumLayerType, Layer};
use wave_core::window::Window;
use wave_core::{Engine, EnumEngineError};
use wave_core::graphics::open_gl::renderer::GlContext;
use wave_editor::Editor;

/// Any element that implements the TraitOption trait will have `.set_option(...)` && `.reset_option()`
/// implemented, and you can use these to customize and configure those elements BEFORE running the
/// engine. By default, all of them will supply pre-configured hints and apply them only ONCE.
///```ignore
/// let mut window = Window::new("Example", EnumRendererApi::OpenGL);
/// // Make window appear as borderless when created (windowed by default).
/// window.set_option(EnumWindowHint::WindowMode(EnumWindowMode::Borderless));
/// // Make window resizable (off by default).
/// window.set_option(EnumWindowHint::Resizable(true));
/// // Set target refresh rate (unlimited by default).
/// window.set_option(EnumWindowHint::RefreshRate(Some(60)));
/// // Disable vsync (on by default).
/// window.set_option(EnumWindowHint::VSync(false));
/// ...
/// window.apply()?;  // Will apply all the hints provided. Can fail.

/// By default, all layers poll on every frame, meaning they perform a task that is run every frame
/// (like rendering or for event handling). For any layer you wish to disable this feature use: \
/// `.disable_polling()`;

// fn create_custom_logger() -> Layer {
//   // Add custom logger (override the default logger supplied by the engine).
//   let mut logger = Logger::new();
//   // Set a mask to specify what type of logging to display in the terminal.
//   logger.set_option(EnumLoggerOption::Verbosity(
//     EnumLoggerVerboseMask::Info |
//       EnumLoggerVerboseMask::Warning |
//       EnumLoggerVerboseMask::Error));
//
//   // Add timestamp to every log (default is true).
//   logger.set_option(ShowTime(false));
//   // Add file path of function to every log (default is true).
//   logger.set_option(ShowFileTrace(false));
//   // Add parent function + line to every log (default is true).
//   logger.set_option(ShowFunctionTrace(false));
//   // Add custom log tag at the beginning of logs of type info.
//   logger.set_option(SetTag(EnumLoggerTag::Info, "TEST"));
//   // Set a layer mask to specify which layers we would like to log for.
//   logger.set_option(Scope(EnumLoggerScopeMask::Engine | EnumLoggerScopeMask::App));
//   logger.set_option(Colors(true));  // Enable colors for different log types.
//   logger.set_option(BreakLinesAt(80, "|\t\t"));  // Col number, prefix for next lines.
//
//   let mut logger_layer = Layer::new("Logger", logger);
//   logger_layer.enable_polling_at(EnumSyncInterval::EveryTime(Time::from(1.0)))  // Every second.
//   return logger_layer;
// }

fn main() -> Result<(), EnumEngineError> {
  // Use a custom logger with our preferences instead of the default one used by the engine.
  // let logger_layer = create_custom_logger();
  
  // Create a window context layer for a GUI app with default hints.
  let window = Window::new("Editor (OpenGL)", EnumRendererApi::OpenGL);
  // Create a renderer layer for graphics rendering onto window framebuffer with default hints.
  let renderer = Renderer::new(GlContext::new());
  // Our own custom app layer overlaying everything.
  let editor = Editor::new();
  
  let window_layer = Layer::new_window_layer("Window", window);
  let renderer_layer = Layer::new_renderer_layer("Renderer", renderer);
  let mut editor_layer = Layer::new("Editor", Box::new(editor), EnumLayerType::App,
    EnumEventMask::WindowClose | EnumEventMask::WindowFocus | EnumEventMask::Input);
  
  // Customize each step function using closures.
  editor_layer.bake_fn(|data, _, engine| {
    let editor_cast = unsafe { &mut *(data as *mut dyn Display as *mut Editor) };
    editor_cast.on_bake(engine)
  });
  
  editor_layer.event_fn(|data, event, engine| {
    let editor_cast = unsafe { &mut *(data as *mut dyn Display as *mut Editor) };
    editor_cast.on_event(event, engine)
  });
  
  editor_layer.frame_fn(|data, engine| {
    let editor_cast = unsafe { &mut *(data as *mut dyn Display as *mut Editor) };
    editor_cast.on_frame(engine)
  });
  
  
  editor_layer.free_fn(|data| {
    let editor_cast = unsafe { &mut *(data as *mut dyn Display as *mut Editor) };
    editor_cast.on_free()
  });
  
  // Supply all layers to our engine.
  // Note: Order does not matter, they are sorted internally by layer type.
  let mut engine = Engine::new(vec![renderer_layer, window_layer, editor_layer]);
  // Executing layers in run loop. Returning on close event or if an error occurred.
  return engine.run();
}