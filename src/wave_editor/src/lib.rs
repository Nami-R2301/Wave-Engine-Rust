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

use wave_core::{camera, Engine, EnumEngineError, input, layers, TraitApply, TraitHint};
use wave_core::assets::asset_loader::{AssetLoader, EnumAssetHint};
use wave_core::assets::r_assets;
use wave_core::assets::r_assets::EnumSubPrimitivePortion;
#[allow(unused)]
use wave_core::dependencies::chrono;
use wave_core::events::{EnumEvent, EnumEventMask};
use wave_core::graphics::renderer::{EnumRendererBatchMode, EnumRendererHint, EnumRendererRenderPrimitiveAs, Renderer};
use wave_core::graphics::{shader};
use wave_core::graphics::texture::{TextureLoader};
use wave_core::layers::{EnumLayerType, EnumSyncInterval, Layer, TraitLayer};
#[allow(unused)]
use wave_core::layers::imgui_layer::ImguiLayer;
#[allow(unused)]
use wave_core::ui::ui_imgui::Imgui;
use wave_core::utils::macros::logger::*;
use wave_core::window::{EnumWindowHint, EnumWindowMode, Window};

static mut S_EDITOR: Option<*mut Editor> = None;

#[derive(Debug, PartialEq)]
pub enum EnumEditorError {
  InvalidAppLayer,
  LayerError(layers::EnumLayerError),
  EngineError(EnumEngineError)
}

impl From<layers::EnumLayerError> for EnumEditorError {
  fn from(value: layers::EnumLayerError) -> Self {
    log!(EnumLogColor::Red, "ERROR", "[Editor] -->\t Error occurred in layer, Error => {:?}", value);
    return EnumEditorError::LayerError(value);
  }
}

impl From<EnumEngineError> for EnumEditorError {
  fn from(value: EnumEngineError) -> Self {
    log!(EnumLogColor::Red, "ERROR", "[Editor] -->\t Error occurred in engine, Error => {:?}", value);
    return EnumEditorError::EngineError(value);
  }
}

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
  m_r_assets: HashMap<&'static str, (shader::Shader, Vec<r_assets::REntity>)>,
  m_cameras: Vec<camera::Camera>,
  m_wireframe_on: bool,
}

impl Default for Editor {
  fn default() -> Self {
    let mut window = Window::default();  // Apply default window hints.
    let mut renderer = Renderer::default();  // Apply default renderer hints.
    
    window.set_hint(EnumWindowHint::WindowMode(EnumWindowMode::Windowed));  // Force window to start in windowed mode.
    window.set_hint(EnumWindowHint::MSAA(None));  // Force window to discard MSAA for framebuffer.
    // Force each sub primitive to be drawn separately.
    renderer.set_hint(EnumRendererHint::Optimization(EnumRendererBatchMode::OptimizeIndices));
    
    return Editor {
      m_engine: Engine::new(window, renderer, vec![]),
      m_r_assets: HashMap::with_capacity(5),
      m_cameras: Vec::new(),
      m_wireframe_on: true,
    };
  }
}

impl Editor {
  pub fn new(window: Window, renderer: Renderer, app_layers: Vec<Layer>) -> Self {
    return Editor {
      m_engine: Engine::new(window, renderer, app_layers),
      m_r_assets: HashMap::with_capacity(5),
      m_cameras: Vec::new(),
      m_wireframe_on: true,
    };
  }
  
  pub fn run(&mut self) -> Result<(), EnumEditorError> {
    let mut editor_layer = Layer::new("Editor Layer", EditorLayer::new(self));
    
    // Making editor poll input events during async call.
    editor_layer.enable_async_polling_for(EnumEventMask::Input | EnumEventMask::WindowClose);
    // Make editor synchronously poll on each frame interval (after async), for movement and spontaneous event handling.
    editor_layer.enable_sync_polling();
    editor_layer.set_sync_interval(EnumSyncInterval::EveryFrame)?;
    
    self.m_engine.push_layer(editor_layer, false)?;
    
    unsafe { S_EDITOR = Some(self) };
    
    return self.m_engine.run().map_err(|err| EnumEditorError::from(err));
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
    
    let vertex_shader = shader::ShaderStage::default_for(shader::EnumShaderStageType::Vertex);
    let fragment_shader = shader::ShaderStage::default_for(shader::EnumShaderStageType::Fragment);
    let geometry_shader = shader::ShaderStage::default_for(shader::EnumShaderStageType::Geometry);
    
    let mut shader = shader::Shader::default();  // Apply default shader hints.
    
    shader.push_stage(vertex_shader)?;
    shader.push_stage(geometry_shader)?;
    shader.push_stage(fragment_shader)?;
    
    shader.apply()?;  // Source and compile the shader program.
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Loaded shaders successfully");
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending assets to GPU...");
    
    let mut asset_loader = AssetLoader::default(); // Apply default asset loader hints.
    asset_loader.set_hint(EnumAssetHint::ReduceMeshes(false));  // Keep sub primitives for Editor view of sub meshes.
    
    let awp_asset = asset_loader.load("awp.obj")?;
    let mario_asset = asset_loader.load("Mario.obj")?;
    let logo_asset = asset_loader.load("n64logo.obj")?;
    
    let mut awp = r_assets::REntity::new(awp_asset, r_assets::EnumPrimitive::Mesh(r_assets::EnumMaterial::Smooth));
    
    awp.translate(10.0, -10.0, 50.0);
    awp.rotate(90.0, -90.0, 0.0);
    awp.apply(&mut shader)?;  // Bake and send the asset.
    awp.show(EnumSubPrimitivePortion::Everything);
    
    let mut mario = r_assets::REntity::new(mario_asset, r_assets::EnumPrimitive::Mesh(r_assets::EnumMaterial::Smooth));
    
    mario.translate(-5.0, -5.0, 15.0);
    mario.rotate(0.0, 0.0, 0.0);
    mario.apply(&mut shader)?;  // Bake and send the asset.
    mario.show(EnumSubPrimitivePortion::Everything);
    
    let mut logo = r_assets::REntity::new(logo_asset, r_assets::EnumPrimitive::Mesh(r_assets::EnumMaterial::Smooth));
    
    logo.translate(5.0, 0.0, 20.0);
    logo.rotate(0.0, 0.0, 0.0);
    logo.apply(&mut shader)?;  // Bake and send the asset.
    logo.show(EnumSubPrimitivePortion::Everything);
    
    self.m_r_assets.insert("Smooth assets", (shader, vec![awp, mario, logo]));
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending textures to GPU...");
    
    let mut texture_preset = TextureLoader::default();  // Apply default texture loader hints.
    let mut texture = texture_preset.load("res/textures/mario-atlas.png")?;  // Load texture.
    texture.apply()?;  // Bake and send the texture.
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Textures sent to GPU successfully");
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Asset sent to GPU successfully");
    
    let renderer = self.m_engine.get_renderer_mut();
    self.m_cameras.push(camera::Camera::new(camera::EnumCameraType::Perspective(75, aspect_ratio, 0.01, 1000.0), None));
    renderer.update_ubo_camera(self.m_cameras[0].get_view_matrix(), self.m_cameras[0].get_projection_matrix())?;
    
    // let mut imgui_layer: Layer = Layer::new("Imgui",
    //   ImguiLayer::new(Imgui::new(self.m_engine.get_renderer_mut().get_type(), self.m_engine.get_window_mut())));
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
      self.m_cameras[0].translate(0.0, 0.0, -10.0 * time_step as f32);
    }
    if Engine::is_key(input::EnumKey::A, input::EnumAction::Held) {
      self.m_cameras[0].translate(-10.0 * time_step as f32, 0.0, 0.0);
    }
    if Engine::is_key(input::EnumKey::S, input::EnumAction::Held) {
      self.m_cameras[0].translate(0.0, 0.0, 10.0 * time_step as f32);
    }
    if Engine::is_key(input::EnumKey::D, input::EnumAction::Held) {
      self.m_cameras[0].translate(10.0 * time_step as f32, 0.0, 0.0);
    }
    if Engine::is_key(input::EnumKey::Up, input::EnumAction::Held) {
      self.m_cameras[0].rotate(0.0, 25.0 * time_step as f32, 0.0);
    }
    if Engine::is_key(input::EnumKey::Left, input::EnumAction::Held) {
      self.m_cameras[0].rotate(-25.0 * time_step as f32, 0.0, 0.0);
    }
    if Engine::is_key(input::EnumKey::Down, input::EnumAction::Held) {
      self.m_cameras[0].rotate(0.0, -25.0 * time_step as f32, 0.0);
    }
    if Engine::is_key(input::EnumKey::Right, input::EnumAction::Held) {
      self.m_cameras[0].rotate(25.0 * time_step as f32, 0.0, 0.0);
    }
    return Ok(());
  }
  
  fn on_async_event(&mut self, event: &EnumEvent) -> Result<bool, EnumEngineError> {
    // Process asynchronous events.
    return match event {
      EnumEvent::KeyEvent(key, action, repeat_count, modifiers) => {
        let renderer = self.m_engine.get_renderer_mut();
        match (key, action, repeat_count, modifiers) {
          (input::EnumKey::Minus, input::EnumAction::Pressed, _, &modifier) => {
            let mut primitive_mode: EnumRendererRenderPrimitiveAs = EnumRendererRenderPrimitiveAs::default();
            if modifier.contains(input::EnumModifiers::Shift) {
              primitive_mode = EnumRendererRenderPrimitiveAs::Filled;
              self.m_wireframe_on = false;
            } else if modifier.contains(input::EnumModifiers::Control) {
              primitive_mode = EnumRendererRenderPrimitiveAs::SolidWireframe;
              self.m_wireframe_on = true;
            } else if modifier.is_empty() {
              primitive_mode = EnumRendererRenderPrimitiveAs::Wireframe;
              self.m_wireframe_on = true;
            }
            
            renderer.toggle_primitive_mode(primitive_mode)?;
            Ok(true)
          }
          (input::EnumKey::Num0, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[0].hide(EnumSubPrimitivePortion::Everything);
            Ok(true)
          }
          (input::EnumKey::Num0, input::EnumAction::Pressed, _, &input::EnumModifiers::Shift) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[0].show(EnumSubPrimitivePortion::Some(0));
            Ok(true)
          }
          (input::EnumKey::Num1, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[1].hide(EnumSubPrimitivePortion::Everything);
            Ok(true)
          }
          (input::EnumKey::Num1, input::EnumAction::Pressed, _, &input::EnumModifiers::Shift) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[1].show(EnumSubPrimitivePortion::Some(1));
            Ok(true)
          }
          (input::EnumKey::Num2, input::EnumAction::Pressed, _, &input::EnumModifiers::Alt) => {
            renderer.toggle_msaa(Some(4))?;
            Ok(true)
          }
          (input::EnumKey::Delete, input::EnumAction::Pressed, _, m) => {
            if m.contains(input::EnumModifiers::Control) {
              for (_, r_assets) in self.m_r_assets.values() {
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
    return Ok(());
  }
  
  fn to_string(&self) -> String {
    let mut final_str: String;
    
    final_str = format!("\n{0:115}Assets: [{1}]\n{0:115}", "", self.m_r_assets.len());
    
    for (position, (linked_shader, r_asset_vec)) in self.m_r_assets.values().enumerate() {
      final_str += &format!("[{1}]:\n{0:117}Associated shader:\n{0:119}{2}\n{0:119}Assets\n{0:121}", "",
        position + 1, linked_shader);
      for r_asset in r_asset_vec.iter() {
        final_str += &format!("[{1}]:\n{0:119}{2}", "", position + 1, r_asset);
      }
    }
    
    return final_str;
  }
}