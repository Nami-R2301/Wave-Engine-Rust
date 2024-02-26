/*
 MIT License

 Copyright (c) 2023 Nami Reghbati

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

use wave_engine::*;
use wave_engine::wave_core::Engine;
use wave_engine::wave_core::graphics::shader::{Shader, ShaderStage, EnumShaderStage, EnumShaderSource};
use wave_engine::wave_core::graphics::renderer::{self, Renderer};
use wave_engine::wave_core::ui::ui_imgui::Imgui;
use wave_engine::wave_core::math::Vec3;
use wave_engine::wave_core::camera::{Camera, EnumCameraType};
use wave_engine::wave_core::assets::asset_loader::ResLoader;
use wave_engine::wave_core::assets::renderable_assets::{TraitRenderableEntity, REntity};

pub struct Editor {
  m_ui: Option<Imgui>,
  m_shaders: Vec<Shader>,
  m_renderable_assets: Vec<REntity>,
  m_cameras: Vec<Camera>,
}

impl Editor {
  pub fn default() -> Self {
    return Editor {
      m_ui: None,
      m_shaders: Vec::new(),
      m_renderable_assets: Vec::new(),
      m_cameras: Vec::new(),
    };
  }
}

impl wave_core::TraitApp for Editor {
  fn on_new(&mut self) -> Result<(), wave_core::EnumError> {
    let engine = Engine::get();
    let window = unsafe { (*engine).get_window() };
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Loading shaders...");
    
    let vertex_shader = ShaderStage::new(EnumShaderStage::Vertex,
      EnumShaderSource::FromFile(String::from("res/shaders/gl_x_spv_test.vert")));
    let fragment_shader = ShaderStage::new(EnumShaderStage::Fragment,
      EnumShaderSource::FromFile(String::from("res/shaders/gl_x_spv_test.frag")));
    
    let shader = Shader::new(vec![vertex_shader, fragment_shader])?;
    
    log!("INFO", "{0}", shader);
    
    self.m_shaders.push(shader);
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Loaded shaders successfully");
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending shaders to GPU...");
    // Sourcing and compilation.
    self.m_shaders[0].submit()?;
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Shaders sent to GPU successfully");
    
    let aspect_ratio: f32 = window.m_window_resolution.0 as f32 / window.m_window_resolution.1 as f32;
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending asset 'awp.obj' to GPU...");
    self.m_renderable_assets.push(REntity::from(ResLoader::new("awp.obj")?));
    self.m_renderable_assets[0].translate(Vec3::new(&[10.0, -10.0, 50.0]));
    self.m_renderable_assets[0].rotate(Vec3::new(&[-90.0, 90.0, 0.0]));
    
    self.m_renderable_assets[0].send(&mut self.m_shaders[0])?;
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Asset sent to GPU successfully");
    
    self.m_cameras.push(Camera::new(EnumCameraType::Perspective(75, aspect_ratio, 0.01, 1000.0), None));
    let renderer = unsafe { (*engine).get_renderer() };
    renderer.setup_camera(&self.m_cameras[0])?;
    
    // Setup imgui layer.
    unsafe {
      if (*engine).get_renderer().m_type == renderer::EnumApi::OpenGL {
        self.m_ui = Some(Imgui::new(renderer::EnumApi::OpenGL, window)?);
      }
    }
    
    // Show our window when we are done.
    window.show();
    return Ok(());
  }
  
  fn on_event(&mut self, window_event: &glfw::WindowEvent) -> bool {
    if self.m_ui.is_some() {
      self.m_ui.as_mut().unwrap().on_event(window_event);
    }
    return false;
  }
  
  fn on_update(&mut self, _time_step: f64) -> Result<bool, wave_core::EnumError> {
    if self.m_ui.is_some() {
      self.m_ui.as_mut().unwrap().on_update();
    }
    return Ok(false);
  }
  
  fn on_render(&mut self) -> Result<(), wave_core::EnumError> {
    let renderer = Renderer::get_active();
    unsafe { (*renderer).on_render()? };
    
    if self.m_ui.is_some() {
      self.m_ui.as_mut().unwrap().on_render();
    }
    
    return Ok(());
  }
  
  fn on_delete(&mut self) -> Result<(), wave_core::EnumError> {
    return Ok(());
  }
}

///
/// Example entrypoint to the application **executable** for the client. Substitute this out with
/// your own app.
///
/// ### Returns : Nothing
///
/// ## Example :
/// ```text
/// pub struct ExampleApp {}
///
/// impl TraitApp for ExampleApp {
///   // Create app-specific assets before entering the game loop.
///   fn on_new(&mut self) {
///     todo!()
///   }
///
///   // Delete app-specific assets before going out of scope and dropping.
///   fn on_delete(&mut self) {
///     todo!()
///   }
///
///   // Process app-specific events.
///   fn on_event(&mut self) {
///     todo!()
///   }
///
///   // Update app-specific data.
///   fn on_update(&mut self, time_step: f64) {
///     todo!()
///   }
///
///   /* App-specific directives before the window refresh (window swapping) in the main loop.
///    * Note, that any additional rendering in this function will only take effect after window swapping,
///    * and that the render color and depth buffers of the window are automatically cleared
///    * prior to this function call.
///   */
///   fn on_render(&self) {
///     todo!()
///   }
/// }
/// ```
///

fn main() -> Result<(), wave_core::EnumError> {
  
  // Instantiate an empty app on the heap to make sure all of its resources are ref-counted
  // like `std::shared_ptr` in C++.
  let my_app: Box<Editor> = Box::new(Editor::default());
  
  // Supply it to our engine. Engine will NOT construct app and will only init the engine
  // with the supplied GPU API of choice as its renderer.
  let mut engine: Engine = Engine::new(Some(renderer::EnumApi::OpenGL), Some(my_app))?;
  
  // Execute the app in game loop and return if there's a close event or if an error occurred.
  return engine.run();
}
