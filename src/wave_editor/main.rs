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
use wave_engine::wave_core::{Engine, events, input, camera, math};
use wave_engine::wave_core::assets::asset_loader;
use wave_engine::wave_core::assets::renderable_assets;
use wave_engine::wave_core::events::{EnumEventMask};
use wave_engine::wave_core::graphics::renderer;
use wave_engine::wave_core::graphics::renderer::Renderer;
use wave_engine::wave_core::graphics::shader;
use wave_engine::wave_core::layers::{EnumLayerType, Layer, TraitLayer};
use wave_engine::wave_core::ui::ui_imgui::Imgui;
use wave_engine::wave_core::layers::imgui_layer::ImguiLayer;
use wave_engine::wave_core::window::{self, Window};

pub struct Editor {
  m_shaders: Vec<shader::Shader>,
  m_renderable_assets: Vec<renderable_assets::REntity>,
  m_cameras: Vec<camera::Camera>,
  m_wireframe_on: bool
}

impl Editor {
  pub fn default() -> Self {
    return Editor {
      m_shaders: Vec::new(),
      m_renderable_assets: Vec::new(),
      m_cameras: Vec::new(),
      m_wireframe_on: true
    };
  }
}

impl TraitLayer for Editor {
  fn get_type(&self) -> EnumLayerType {
    return EnumLayerType::App;
  }
  
  fn on_submit(&mut self) -> Result<(), wave_core::EnumError> {
    let window = Engine::get_active_window();
    let renderer = Engine::get_active_renderer();
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Loading shaders...");
    
    let vertex_shader = shader::ShaderStage::default(shader::EnumShaderStage::Vertex);
    let fragment_shader = shader::ShaderStage::default(shader::EnumShaderStage::Fragment);
    
    let shader = shader::Shader::new(vec![vertex_shader, fragment_shader])?;
    self.m_shaders.push(shader);
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Loaded shaders successfully");
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending shaders to GPU...");
    
    // Sourcing and compilation.
    self.m_shaders[0].submit()?;
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Shaders sent to GPU successfully");
    let aspect_ratio: f32 = window.get_aspect_ratio();
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending asset 'awp.obj' to GPU...");
    
    // self.m_renderable_assets.push(REntity::default());
    self.m_renderable_assets.push(renderable_assets::REntity::new(asset_loader::ResLoader::new("awp.obj")?,
      renderable_assets::EnumEntityType::Mesh(false)));
    self.m_renderable_assets[0].translate(math::Vec3::new(&[10.0, -10.0, 50.0]));
    self.m_renderable_assets[0].rotate(math::Vec3::new(&[90.0, -90.0, 0.0]));
    self.m_renderable_assets[0].submit(&mut self.m_shaders[0])?;
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Asset sent to GPU successfully");
    self.m_cameras.push(camera::Camera::new(camera::EnumCameraType::Perspective(75, aspect_ratio, 0.01, 1000.0), None));
    renderer.submit_camera(&self.m_cameras[0])?;
    
    #[cfg(feature = "imgui")]
    {
      let mut imgui_layer: Layer = Layer::new("Imgui", ImguiLayer::new(Imgui::new(renderer.m_type, window)));
      imgui_layer.enable_async_polling_for(EnumEventMask::c_input | EnumEventMask::c_window);
      Engine::push_layer(imgui_layer)?;
    }
    
    // Show our window when we are ready to present.
    window.show();
    return Ok(());
  }
  
  fn on_sync_event(&mut self) -> Result<(), wave_core::EnumError> {
    // Process synchronous events.
    let time_step = Engine::get_time_step();
    
    if Engine::is_key(input::EnumKey::W, input::EnumAction::Held) {
      self.m_renderable_assets[0].translate(math::Vec3::new(&[0.0, 10.0 * time_step as f32, 0.0]));
    }
    if Engine::is_key(input::EnumKey::A, input::EnumAction::Held) {
      self.m_renderable_assets[0].translate(math::Vec3::new(&[-10.0 * time_step as f32, 0.0, 0.0]));
    }
    if Engine::is_key(input::EnumKey::S, input::EnumAction::Held) {
      self.m_renderable_assets[0].translate(math::Vec3::new(&[0.0, -10.0 * time_step as f32, 0.0]));
    }
    if Engine::is_key(input::EnumKey::D, input::EnumAction::Held) {
      self.m_renderable_assets[0].translate(math::Vec3::new(&[10.0 * time_step as f32, 0.0, 0.0]));
    }
    if Engine::is_key(input::EnumKey::Up, input::EnumAction::Held) {
      self.m_renderable_assets[0].rotate(math::Vec3::new(&[0.0, 25.0 * time_step as f32, 0.0]));
    }
    if Engine::is_key(input::EnumKey::Left, input::EnumAction::Held) {
      self.m_renderable_assets[0].rotate(math::Vec3::new(&[-25.0 * time_step as f32, 0.0, 0.0]));
    }
    if Engine::is_key(input::EnumKey::Down, input::EnumAction::Held) {
      self.m_renderable_assets[0].rotate(math::Vec3::new(&[0.0, -25.0 * time_step as f32, 0.0]));
    }
    if Engine::is_key(input::EnumKey::Right, input::EnumAction::Held) {
      self.m_renderable_assets[0].rotate(math::Vec3::new(&[25.0 * time_step as f32, 0.0, 0.0]));
    }
    return Ok(());
  }
  
  
  fn on_async_event(&mut self, event: &events::EnumEvent) -> Result<bool, wave_core::EnumError> {
    // Process asynchronous events.
    return match event {
      events::EnumEvent::KeyEvent(key, action, repeat_count, modifiers) => {
        let renderer = Engine::get_active_renderer();
        match (key, action, repeat_count, modifiers) {
          (input::EnumKey::Num1, input::EnumAction::Pressed, _, &input::EnumModifiers::Alt) => {
            renderer.toggle(renderer::EnumRendererOption::Wireframe(!self.m_wireframe_on))?;
            self.m_wireframe_on = !self.m_wireframe_on;
            Ok(true)
          }
          (input::EnumKey::Delete, input::EnumAction::Pressed, _, m) => {
            if m.contains(input::EnumModifiers::Shift.intersection(input::EnumModifiers::Alt)) {
              for r_asset in self.m_renderable_assets.iter() {
                renderer.dequeue(r_asset.get_uuid())?;
              }
            }
            return Ok(true);
          }
          _ => Ok(false)
        }
      }
      _ => Ok(false)
    };
  }
  
  fn on_update(&mut self, _time_step: f64) -> Result<(), wave_core::EnumError> {
    if self.m_renderable_assets[0].has_changed() {
      // Update transform Ubo in associated shader.
      self.m_renderable_assets[0].resend_transform(&mut self.m_shaders[0])?;
    }
    return Ok(());
  }
  
  fn on_render(&mut self) -> Result<(), wave_core::EnumError> {
    return Ok(());
  }
  
  fn on_free(&mut self) -> Result<(), wave_core::EnumError> {
    return Ok(());
  }
  
  fn to_string(&self) -> String {
    let mut final_str: String;
    final_str = format!("\n{0:115}Shaders: [{1}]\n{0:115}", "", self.m_shaders.len());
    
    for (position, shader) in self.m_shaders.iter().enumerate() {
      final_str += &format!("[{1}]:\n{0:117}{2}", "", position + 1, shader);
    }
    
    final_str += &format!("\n{0:115}Renderable assets: [{1}]\n{0:115}", "", self.m_renderable_assets.len());
    
    for (position, r_asset) in self.m_renderable_assets.iter().enumerate() {
      final_str += &format!("[{1}]:\n{0:117}{2}", "", position + 1, r_asset);
    }
    
    return final_str;
  }
}

fn main() -> Result<(), wave_core::EnumError> {
  
  // Instantiate an app layer containing our app and essentially making the layer own it.
  let mut my_app: Layer = Layer::new("Wave Engine Editor", Editor::default());
  // Making editor poll input events during async call.
  my_app.enable_async_polling_for(EnumEventMask::c_input | EnumEventMask::c_window_close);
  // Make editor synchronously poll on each frame (after async), for movement and spontaneous event handling.
  my_app.enable_sync_polling();
  
  let mut window = Window::new()?;
  window.window_hint(window::EnumWindowOption::RendererApi(renderer::EnumApi::OpenGL));
  window.window_hint(window::EnumWindowOption::WindowMode(window::EnumWindowMode::Windowed));
  window.window_hint(window::EnumWindowOption::Resizable(true));
  window.window_hint(window::EnumWindowOption::DebugApi(true));
  window.window_hint(window::EnumWindowOption::Maximized(true));
  
  let mut renderer = Renderer::new(renderer::EnumApi::OpenGL)?;
  renderer.renderer_hint(renderer::EnumRendererOption::ApiCallChecking(renderer::EnumCallCheckingType::Async));
  renderer.renderer_hint(renderer::EnumRendererOption::Wireframe(true));
  
  // Supply it to our engine. Engine will NOT construct app and will only init the engine
  // with the supplied GPU API of choice as its renderer. Note that passing None will default to Vulkan if
  // supported, otherwise falling back to OpenGL.
  let mut engine: Engine = Engine::new(window, renderer, my_app)?;
  
  // Init and execute the app in game loop and return if there's a close event or if an error occurred.
  return engine.run();
}
