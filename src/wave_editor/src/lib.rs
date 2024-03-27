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

pub extern crate wave_core;

use std::collections::HashMap;
use wave_core::{camera, Engine, EnumEngineError, input, math};
use wave_core::assets::asset_loader::AssetLoader;
use wave_core::assets::renderable_assets::{EnumPrimitiveType, REntity};
#[allow(unused)]
use wave_core::dependencies::chrono;
use wave_core::events::{EnumEvent, EnumEventMask};
use wave_core::graphics::renderer;
use wave_core::graphics::renderer::{EnumRendererPrimitiveMode, Renderer};
use wave_core::graphics::shader;
use wave_core::layers::{EnumLayerType, EnumSyncInterval, Layer, TraitLayer};
#[allow(unused)]
use wave_core::layers::imgui_layer::ImguiLayer;
#[allow(unused)]
use wave_core::ui::ui_imgui::Imgui;
use wave_core::utils::macros::logger::*;
use wave_core::window::Window;

static mut S_EDITOR: Option<*mut Editor> = None;

pub struct EditorLayer {
  m_editor: *mut Editor,
}

impl EditorLayer {
  pub fn new(editor: &mut Editor) -> Self {
    return Self {
      m_editor: editor
    };
  }
}

impl TraitLayer for EditorLayer {
  fn get_type(&self) -> EnumLayerType {
    return unsafe { (*self.m_editor).get_type() };
  }
  
  fn on_apply(&mut self) -> Result<(), EnumEngineError> {
    return unsafe { (*self.m_editor).on_apply() };
  }
  
  fn on_sync_event(&mut self) -> Result<(), EnumEngineError> {
    return unsafe { (*self.m_editor).on_sync_event() };
  }
  
  fn on_async_event(&mut self, event: &EnumEvent) -> Result<bool, EnumEngineError> {
    return unsafe { (*self.m_editor).on_async_event(event) };
  }
  
  fn on_update(&mut self, time_step: f64) -> Result<(), EnumEngineError> {
    return unsafe { (*self.m_editor).on_update(time_step) };
  }
  
  fn on_render(&mut self) -> Result<(), EnumEngineError> {
    return unsafe { (*self.m_editor).on_render() };
  }
  
  fn free(&mut self) -> Result<(), EnumEngineError> {
    return unsafe { (*self.m_editor).free() };
  }
  
  fn to_string(&self) -> String {
    return unsafe { (*self.m_editor).to_string() };
  }
}

pub struct Editor {
  m_engine: Engine,
  m_renderable_assets: HashMap<&'static str, (shader::Shader, Vec<REntity>)>,
  m_cameras: Vec<camera::Camera>,
  m_wireframe_on: bool,
}

impl Editor {
  pub fn default() -> Result<Self, EnumEngineError> {
    #[cfg(feature = "vulkan")]
    {
      let window = Window::new()?;
      let renderer = Renderer::new(renderer::EnumRendererApi::Vulkan)?;
      
      return Ok(Editor {
        m_engine: Engine::new(window, renderer, vec![])?,
        m_shaders: Vec::new(),
        m_renderable_assets: Vec::new(),
        m_cameras: Vec::new(),
        m_wireframe_on: true,
      });
    }
    
    #[cfg(not(feature = "vulkan"))]
    {
      let window = Window::new()?;
      let renderer = Renderer::new(renderer::EnumRendererApi::OpenGL)?;
      
      return Ok(Editor {
        m_engine: Engine::new(window, renderer, vec![])?,
        m_renderable_assets: HashMap::with_capacity(10),
        m_cameras: Vec::new(),
        m_wireframe_on: true,
      });
    }
  }
  
  pub fn new(window: Window, renderer: Renderer, app_layers: Vec<Layer>) -> Result<Self, EnumEngineError> {
    return Ok(Editor {
      m_engine: Engine::new(window, renderer, app_layers)?,
      m_renderable_assets: HashMap::with_capacity(10),
      m_cameras: Vec::new(),
      m_wireframe_on: true,
    });
  }
  
  pub fn run(&mut self) -> Result<(), EnumEngineError> {
    let mut editor_layer = Layer::new("Editor Layer", EditorLayer::new(self));
    
    // Making editor poll input events during async call.
    editor_layer.enable_async_polling_for(EnumEventMask::Input | EnumEventMask::WindowClose);
    // Make editor synchronously poll on each frame interval (after async), for movement and spontaneous event handling.
    editor_layer.enable_sync_polling();
    editor_layer.set_sync_interval(EnumSyncInterval::EveryFrame)?;
    
    self.m_engine.push_layer(editor_layer, false)?;
    
    unsafe { S_EDITOR = Some(self) };
    
    return self.m_engine.run();
  }
}

impl TraitLayer for Editor {
  fn get_type(&self) -> EnumLayerType {
    return EnumLayerType::Editor;
  }
  
  fn on_apply(&mut self) -> Result<(), EnumEngineError> {
    let window = self.m_engine.get_window_mut();
    let aspect_ratio: f32 = window.get_aspect_ratio();
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Loading shaders...");
    
    let vertex_shader = shader::ShaderStage::default(shader::EnumShaderStage::Vertex);
    let fragment_shader = shader::ShaderStage::default(shader::EnumShaderStage::Fragment);
    let geometry_shader = shader::ShaderStage::default(shader::EnumShaderStage::Geometry);
    
    let mut shader = shader::Shader::new(vec![vertex_shader, fragment_shader, geometry_shader])?;
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Loaded shaders successfully");
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending shaders to GPU...");
    
    // Sourcing and compilation.
    shader.apply()?;
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Shaders sent to GPU successfully");
    
    self.m_cameras.push(camera::Camera::new(camera::EnumCameraType::Perspective(75, aspect_ratio, 0.01, 1000.0), None));
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending assets to GPU...");
    
    let mut awp = REntity::new(AssetLoader::new("awp.obj")?, EnumPrimitiveType::Mesh(false));
    
    awp.translate(math::Vec3::new(&[10.0, -10.0, 50.0]));
    awp.rotate(math::Vec3::new(&[90.0, -90.0, 0.0]));
    awp.apply(&mut shader)?;
    awp.show(None);
    
    let mut mario = REntity::new(AssetLoader::new("mario-obj/Mario.obj")?, EnumPrimitiveType::Mesh(false));
    
    mario.translate(math::Vec3::new(&[-5.0, -5.0, 15.0]));
    mario.rotate(math::Vec3::new(&[0.0, 0.0, 0.0]));
    mario.apply(&mut shader)?;
    mario.show(None);
    
    let mut sphere = REntity::new(AssetLoader::new("sphere.obj")?, EnumPrimitiveType::Mesh(false));
    
    sphere.translate(math::Vec3::new(&[5.0, 0.0, 20.0]));
    sphere.rotate(math::Vec3::new(&[0.0, 0.0, 0.0]));
    sphere.apply(&mut shader)?;
    sphere.show(None);
    
    self.m_renderable_assets.insert("Smooth objects", (shader, vec![awp, sphere, mario]));
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Asset sent to GPU successfully");
    
    let renderer = self.m_engine.get_renderer_mut();
    renderer.update_ubo_camera(self.m_cameras[0].get_view_matrix(), self.m_cameras[0].get_projection_matrix())?;
    
    // let mut imgui_layer: Layer = Layer::new("Imgui",
    //   ImguiLayer::new(Imgui::new(self.m_engine.get_renderer_mut().m_type, self.m_engine.get_window_mut())));
    // imgui_layer.enable_async_polling_for(EnumEventMask::Input | EnumEventMask::Window);
    // self.m_engine.push_layer(imgui_layer, true)?;
    
    // Show our window when we are ready to present.
    let window = self.m_engine.get_window_mut();
    window.show();
    return Ok(());
  }
  
  fn on_sync_event(&mut self) -> Result<(), EnumEngineError> {
    // Process synchronous events.
    let time_step = self.m_engine.get_time_step();
    
    if Engine::is_key(input::EnumKey::W, input::EnumAction::Held) {
      self.m_cameras[0].translate(math::Vec3::new(&[0.0, 10.0 * time_step as f32, 0.0]));
    }
    if Engine::is_key(input::EnumKey::A, input::EnumAction::Held) {
      self.m_cameras[0].translate(math::Vec3::new(&[-10.0 * time_step as f32, 0.0, 0.0]));
    }
    if Engine::is_key(input::EnumKey::S, input::EnumAction::Held) {
      self.m_cameras[0].translate(math::Vec3::new(&[0.0, -10.0 * time_step as f32, 0.0]));
    }
    if Engine::is_key(input::EnumKey::D, input::EnumAction::Held) {
      self.m_cameras[0].translate(math::Vec3::new(&[10.0 * time_step as f32, 0.0, 0.0]));
    }
    if Engine::is_key(input::EnumKey::Up, input::EnumAction::Held) {
      self.m_cameras[0].rotate(math::Vec3::new(&[0.0, 25.0 * time_step as f32, 0.0]));
    }
    if Engine::is_key(input::EnumKey::Left, input::EnumAction::Held) {
      self.m_cameras[0].rotate(math::Vec3::new(&[-25.0 * time_step as f32, 0.0, 0.0]));
    }
    if Engine::is_key(input::EnumKey::Down, input::EnumAction::Held) {
      self.m_cameras[0].rotate(math::Vec3::new(&[0.0, -25.0 * time_step as f32, 0.0]));
    }
    if Engine::is_key(input::EnumKey::Right, input::EnumAction::Held) {
      self.m_cameras[0].rotate(math::Vec3::new(&[25.0 * time_step as f32, 0.0, 0.0]));
    }
    return Ok(());
  }
  
  
  fn on_async_event(&mut self, event: &EnumEvent) -> Result<bool, EnumEngineError> {
    // Process asynchronous events.
    return match event {
      EnumEvent::KeyEvent(key, action, repeat_count, modifiers) => {
        let renderer = self.m_engine.get_renderer_mut();
        match (key, action, repeat_count, modifiers) {
          (input::EnumKey::Minus, input::EnumAction::Pressed, _, _) => {
            let primitive_mode: EnumRendererPrimitiveMode = self.m_wireframe_on.then(|| EnumRendererPrimitiveMode::Wireframe)
              .unwrap_or(EnumRendererPrimitiveMode::SolidWireframe);
            renderer.toggle_primitive_mode(primitive_mode)?;
            self.m_wireframe_on = !self.m_wireframe_on;
            Ok(true)
          }
          (input::EnumKey::Num0, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
            self.m_renderable_assets.get_mut(&"Smooth objects").unwrap().1[0].hide(Some(0));
            Ok(true)
          }
          (input::EnumKey::Num0, input::EnumAction::Pressed, _, &input::EnumModifiers::Shift) => {
            self.m_renderable_assets.get_mut(&"Smooth objects").unwrap().1[0].show(Some(0));
            Ok(true)
          }
          (input::EnumKey::Num1, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
            self.m_renderable_assets.get_mut(&"Smooth objects").unwrap().1[1].hide(None);
            Ok(true)
          }
          (input::EnumKey::Num1, input::EnumAction::Pressed, _, &input::EnumModifiers::Shift) => {
            self.m_renderable_assets.get_mut(&"Smooth objects").unwrap().1[1].show(None);
            Ok(true)
          }
          (input::EnumKey::Num2, input::EnumAction::Pressed, _, &input::EnumModifiers::Alt) => {
            // renderer.toggle(renderer::EnumRendererOption::MSAA(Some(4)))?;
            Ok(true)
          }
          (input::EnumKey::Delete, input::EnumAction::Pressed, _, m) => {
            if m.contains(input::EnumModifiers::Control) {
              for (_, r_assets) in self.m_renderable_assets.values() {
                for r_asset in r_assets.iter() {
                  renderer.dequeue(r_asset.get_uuid(), None)?;
                }
              }
            }
            return Ok(true);
          }
          _ => Ok(false)
        }
      }
      EnumEvent::WindowCloseEvent(_time) => {
        self.free()?;
        Ok(true)
      }
      _ => Ok(false)
    };
  }
  
  fn on_update(&mut self, time_step: f64) -> Result<(), EnumEngineError> {
    if self.m_cameras[0].has_changed() {
      let new_camera_view = self.m_cameras[0].get_view_matrix();
      let new_camera_projection = self.m_cameras[0].get_projection_matrix();
      self.m_engine.get_renderer_mut().update_ubo_camera(new_camera_view, new_camera_projection)?;
    }
    self.m_cameras[0].on_update(time_step);
    return Ok(());
  }
  
  fn on_render(&mut self) -> Result<(), EnumEngineError> {
    return Ok(());
  }
  
  fn free(&mut self) -> Result<(), EnumEngineError> {
    for (linked_shader, _) in self.m_renderable_assets.values_mut() {
      linked_shader.free()?;
    }
    return Ok(());
  }
  
  fn to_string(&self) -> String {
    let mut final_str: String;
    
    final_str = format!("\n{0:115}Assets: [{1}]\n{0:115}", "", self.m_renderable_assets.len());
    
    for (position, (linked_shader, r_asset_vec)) in self.m_renderable_assets.values().enumerate() {
      final_str += &format!("[{1}]:\n{0:117}Associated shader:\n{0:119}{2}\n{0:119}Assets\n{0:121}", "",
        position + 1, linked_shader);
      for r_asset in r_asset_vec.iter() {
        final_str += &format!("[{1}]:\n{0:119}{2}", "", position + 1, r_asset);
      }
    }
    
    return final_str;
  }
}