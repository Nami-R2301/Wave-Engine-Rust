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
use wave_engine::wave_core::graphics::shader;
use wave_engine::wave_core::layers::{EnumLayerType, Layer, TraitLayer};
use wave_engine::wave_core::ui::ui_imgui::Imgui;
use wave_engine::wave_core::layers::imgui_layer::ImguiLayer;

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
  
  fn on_new(&mut self) -> Result<(), wave_core::EnumError> {
    let window = Engine::get_active_window();
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Loading shaders...");
    
    let vertex_shader = shader::ShaderStage::default(shader::EnumShaderStage::Vertex);
    let fragment_shader = shader::ShaderStage::default(shader::EnumShaderStage::Fragment);
    
    let shader = shader::Shader::new(vec![vertex_shader, fragment_shader])?;
    log!("INFO", "{0}", shader);
    self.m_shaders.push(shader);
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Loaded shaders successfully");
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending shaders to GPU...");
    
    // Sourcing and compilation.
    self.m_shaders[0].submit()?;
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Shaders sent to GPU successfully");
    let aspect_ratio: f32 = window.m_window_resolution.0 as f32 / window.m_window_resolution.1 as f32;
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending asset 'awp.obj' to GPU...");
    
    // self.m_renderable_assets.push(REntity::default());
    self.m_renderable_assets.push(renderable_assets::REntity::new(asset_loader::ResLoader::new("awp.obj")?,
      renderable_assets::EnumEntityType::Object, false));
    self.m_renderable_assets[0].translate(math::Vec3::new(&[10.0, -10.0, 50.0]));
    self.m_renderable_assets[0].rotate(math::Vec3::new(&[90.0, -90.0, 0.0]));
    self.m_renderable_assets[0].send(&mut self.m_shaders[0])?;
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Asset sent to GPU successfully");
    self.m_cameras.push(camera::Camera::new(camera::EnumCameraType::Perspective(75, aspect_ratio, 0.01, 1000.0), None));
    let renderer = Engine::get_active_renderer();
    renderer.send_camera(&self.m_cameras[0])?;
    
    #[cfg(feature = "imgui")]
    {
      let imgui_layer: Layer = Layer::new("Imgui", ImguiLayer::new(Imgui::new(renderer.m_type, window)));
      let layer_type = imgui_layer.get_type();
      Engine::push_layer(imgui_layer);
      Engine::enable_async_polling_for(layer_type, EnumEventMask::c_keyboard | EnumEventMask::c_mouse_btn | EnumEventMask::c_window);
    }
    
    // Making editor poll keyboard and mouse button events during async call.
    Engine::enable_async_polling_for(self.get_type(), EnumEventMask::c_keyboard | EnumEventMask::c_mouse_btn);
    Engine::enable_sync_polling_for(self);
    
    // Create and show our window when we are done.
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
            renderer.toggle(renderer::EnumFeature::Wireframe(!self.m_wireframe_on))?;
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
  
  fn on_delete(&mut self) -> Result<(), wave_core::EnumError> {
    return Ok(());
  }
}

fn main() -> Result<(), wave_core::EnumError> {
  
  // Instantiate an app layer containing our app and essentially making the layer own it.
  let my_app: Layer = Layer::new("Wave Engine Editor", Editor::default());
  
  // Supply it to our engine. Engine will NOT construct app and will only init the engine
  // with the supplied GPU API of choice as its renderer.
  let mut engine: Engine = Engine::new(Some(renderer::EnumApi::OpenGL), my_app)?;
  
  // Init and execute the app in game loop and return if there's a close event or if an error occurred.
  return engine.run();
}
