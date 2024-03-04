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
use wave_engine::wave_core::{Engine, input};
use wave_engine::wave_core::assets::asset_loader::ResLoader;
use wave_engine::wave_core::assets::renderable_assets::{EnumEntityType, REntity, TraitRenderableEntity};
use wave_engine::wave_core::camera::{Camera, EnumCameraType};
use wave_engine::wave_core::events::EnumEvent;
use wave_engine::wave_core::graphics::renderer::{self, EnumFeature};
use wave_engine::wave_core::graphics::shader::{EnumShaderStage, Shader, ShaderStage};
use wave_engine::wave_core::input::EnumKey;
use wave_engine::wave_core::math::Vec3;

pub struct Editor {
  m_shaders: Vec<Shader>,
  m_renderable_assets: Vec<REntity>,
  m_cameras: Vec<Camera>,
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

impl wave_core::TraitApp for Editor {
  fn on_new(&mut self) -> Result<(), wave_core::EnumError> {
    let window = Engine::get_active_window();
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Loading shaders...");
    
    let vertex_shader = ShaderStage::default(EnumShaderStage::Vertex);
    let fragment_shader = ShaderStage::default(EnumShaderStage::Fragment);
    
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
    
    // self.m_renderable_assets.push(REntity::default());
    self.m_renderable_assets.push(REntity::new(ResLoader::new("awp.obj")?, EnumEntityType::Object, false));
    self.m_renderable_assets[0].translate(Vec3::new(&[10.0, -10.0, 50.0]));
    self.m_renderable_assets[0].rotate(Vec3::new(&[90.0, -90.0, 0.0]));
    self.m_renderable_assets[0].send(&mut self.m_shaders[0])?;
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Asset sent to GPU successfully");
    self.m_cameras.push(Camera::new(EnumCameraType::Perspective(75, aspect_ratio, 0.01, 1000.0), None));
    let renderer = Engine::get_active_renderer();
    renderer.send_camera(&self.m_cameras[0])?;
    
    #[cfg(feature = "imgui")]
      Engine::push_imgui_layer();
    
    // Show our window when we are done.
    window.show();
    return Ok(());
  }
  
  fn on_event(&mut self, event: &EnumEvent) -> Result<bool, wave_core::EnumError> {
    // Process asynchronous events.
    return match event {
      EnumEvent::KeyPressedEvent(key , modifiers) => {
        let renderer = Engine::get_active_renderer();
        match (key, modifiers) {
          (EnumKey::R, &glfw::Modifiers::Control) => {
            renderer.flush()?;
            Ok(true)
          }
          (EnumKey::F2, _) => {
            renderer.toggle(EnumFeature::Wireframe(!self.m_wireframe_on))?;
            self.m_wireframe_on = !self.m_wireframe_on;
            Ok(true)
          }
          (EnumKey::Delete, m) => {
            if m.contains(glfw::Modifiers::Shift.union(glfw::Modifiers::Alt)) {
              for r_asset in self.m_renderable_assets.iter() {
                renderer.dequeue(r_asset.get_uuid())?;
              }
            }
            Ok(true)
          }
          _ => Ok(false)
        }
      }
      _ => Ok(false)
    }
  }
  
  fn on_update(&mut self, time_step: f64) -> Result<(), wave_core::EnumError> {
    // Process synchronous events.
    
    // Translation.
    if Engine::is_key(EnumKey::W, input::EnumAction::Held) {
      self.m_renderable_assets[0].translate(Vec3::new(&[0.0, 10.0 * time_step as f32, 0.0]));
    }
    if Engine::is_key(EnumKey::A, input::EnumAction::Held) {
      self.m_renderable_assets[0].translate(Vec3::new(&[-10.0 * time_step as f32, 0.0, 0.0]));
    }
    if Engine::is_key(EnumKey::S, input::EnumAction::Held) {
      self.m_renderable_assets[0].translate(Vec3::new(&[0.0, -10.0 * time_step as f32, 0.0]));
    }
    if Engine::is_key(EnumKey::D, input::EnumAction::Held) {
      self.m_renderable_assets[0].translate(Vec3::new(&[10.0 * time_step as f32, 0.0, 0.0]));
    }
    
    // Rotation.
    if Engine::is_key(EnumKey::Up, input::EnumAction::Held) {
      self.m_renderable_assets[0].rotate(Vec3::new(&[0.0, 25.0 * time_step as f32, 0.0]));
    }
    if Engine::is_key(EnumKey::Left, input::EnumAction::Held) {
      self.m_renderable_assets[0].rotate(Vec3::new(&[-25.0 * time_step as f32, 0.0, 0.0]));
    }
    if Engine::is_key(EnumKey::Down, input::EnumAction::Held) {
      self.m_renderable_assets[0].rotate(Vec3::new(&[0.0, -25.0 * time_step as f32, 0.0]));
    }
    if Engine::is_key(EnumKey::Right, input::EnumAction::Held) {
      self.m_renderable_assets[0].rotate(Vec3::new(&[25.0 * time_step as f32, 0.0, 0.0]));
    }
    
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
