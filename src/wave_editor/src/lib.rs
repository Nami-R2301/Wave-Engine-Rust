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

use wave_core::{camera, Engine, EnumEngineError, input, layers, TraitApply, TraitFree, TraitHint};
use wave_core::assets::asset_loader::{AssetLoader};
use wave_core::assets::r_assets::{EnumAssetMapMethod, EnumAssetPrimitiveSurface, EnumPrimitiveShading, REntity};
#[allow(unused)]
use wave_core::dependencies::chrono;
use wave_core::events::{EnumEvent, EnumEventMask};
use wave_core::graphics::renderer::{Renderer, EnumRendererRenderPrimitiveAs, EnumRendererHint, EnumRendererOptimizationMode, EnumRendererApi, EnumRendererCallCheckingMode};
use wave_core::graphics::{shader};
use wave_core::graphics::shader::EnumShaderHint;
use wave_core::graphics::texture::{Texture, TextureArray};
use wave_core::utils::texture_loader::{EnumTextureLoaderHint, TextureLoader};
use wave_core::layers::{EnumLayerType, EnumSyncInterval, Layer, TraitLayer};
#[allow(unused)]
use wave_core::layers::imgui_layer::ImguiLayer;
#[allow(unused)]
use wave_core::ui::ui_imgui::Imgui;
use wave_core::utils::macros::logger::*;
use wave_core::window::{EnumWindowHint, Window};

static mut S_EDITOR: Option<*mut Editor> = None;

#[derive(Debug)]
pub enum EnumEditorError {
  InvalidAppLayer,
  IoError(std::io::Result<()>),
  LayerError(layers::EnumLayerError),
  EngineError(EnumEngineError),
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
  m_r_assets: HashMap<&'static str, (shader::Shader, Vec<REntity>)>,
  m_cameras: Vec<camera::Camera>,
  m_textures: Vec<Texture>,
}

impl Default for Editor {
  fn default() -> Self {
    let mut window = Window::default();  // Apply default window hints.
    let mut renderer = Renderer::default();  // Apply default renderer hints.
    
    // window.set_hint(EnumWindowHint::WindowApi(EnumRendererApi::Vulkan));  // Select Vulkan client api.
    window.set_hint(EnumWindowHint::MSAA(Some(8)));  // Enable MSAA.
    
    // Enable all optimizations.
    renderer.set_hint(EnumRendererHint::ApiCallChecking(EnumRendererCallCheckingMode::SyncAndAsync));
    renderer.set_hint(EnumRendererHint::Optimization(EnumRendererOptimizationMode::All));
    renderer.set_hint(EnumRendererHint::MSAA(Some(8)));  // Enable MSAA.
    // renderer.set_hint(EnumRendererHint::ContextApi(EnumRendererApi::Vulkan));  // Select Vulkan context api.
    
    return Editor {
      m_engine: Engine::new(window, renderer, vec![]),
      m_r_assets: HashMap::with_capacity(5),
      m_cameras: Vec::with_capacity(1),
      m_textures: Vec::with_capacity(5),
    };
  }
}

impl Editor {
  pub fn new(window: Window, renderer: Renderer, app_layers: Vec<Layer>) -> Self {
    return Editor {
      m_engine: Engine::new(window, renderer, app_layers),
      m_r_assets: HashMap::new(),
      m_cameras: Vec::new(),
      m_textures: Vec::new(),
    };
  }
  
  pub fn run(&mut self) -> Result<(), EnumEditorError> {
    let mut editor_layer = Layer::new("Editor Layer", EditorLayer::new(self));
    
    // Making editor poll input events during async call.
    editor_layer.enable_async_polling_for(EnumEventMask::Input | EnumEventMask::WindowClose | EnumEventMask::WindowSize);
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
    
    let mut shader = shader::Shader::default();  // Get default smooth shader with 3 stages (vertex, geometry, and fragment).
    shader.set_hint(EnumShaderHint::ForceGlslVersion(420));
    // shader.set_hint(EnumShaderHint::Api(EnumRendererApi::Vulkan));
    
    // Source and compile the shader program.
    shader.apply()?;
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Loaded shaders successfully");
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending textures to GPU...");
    
    let mut texture_preset = TextureLoader::new();
    texture_preset.set_hint(EnumTextureLoaderHint::FlipUvs(true));
    
    let awp_texture_info = texture_preset.load("res/textures/awp/awp_texture.jpeg")?;
    
    // Load all textures in folders.
    texture_preset.set_hint(EnumTextureLoaderHint::FlipUvs(false));
    let mario_textures_info = texture_preset.load_from_folder("res/textures/mario")?;
    let n64_logo_textures_info = texture_preset.load_from_folder("res/textures/n64_logo")?;
    
    // Batch all textures from assets that share the same size to fit them in an appropriate 'texture array bucket' in the shader.
    let mut texture_1024_array = TextureArray::new(EnumRendererApi::OpenGL, vec![awp_texture_info]);
    texture_1024_array.append(mario_textures_info);
    
    let texture_64_array = TextureArray::new(EnumRendererApi::OpenGL, n64_logo_textures_info);
    
    let mut texture_1024_handle = texture_1024_array.get_texture_handle();
    let mut textures_64_handle = texture_64_array.get_texture_handle();
    
    texture_1024_handle.apply()?;
    textures_64_handle.apply()?;
    
    self.m_textures.push(texture_1024_handle);
    self.m_textures.push(textures_64_handle);
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Textures sent to GPU...");
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending assets to GPU...");
    
    let asset_loader = AssetLoader::new();
    // asset_loader.set_hint(EnumAssetHint::VertexDataIs(EnumAssetPrimitiveMode::Plain));
    
    let awp_asset = asset_loader.load("res/assets/awp/awp.obj")?;
    let mario_asset = asset_loader.load("res/assets/mario/mario.obj")?;
    let logo_asset = asset_loader.load("res/assets/n64_logo/n64_logo.obj")?;
    
    let mut awp = REntity::new(awp_asset, EnumPrimitiveShading::default(), "Awp Sniper");
    
    // Map all textures in folder to sub primitives in 1-1 ratio in order.
    awp.map_texture(&texture_1024_array, EnumAssetMapMethod::MultipleForEach(2, 0, 1));
    awp.translate(10.0, -10.0, 50.0);
    awp.rotate(90.0, -90.0, 0.0);
    awp.apply(&mut shader)?;  // Bake and send the asset.
    awp.show(EnumAssetPrimitiveSurface::Everything);
    
    let mut mario = REntity::new(mario_asset, EnumPrimitiveShading::default(), "Mario");
    
    // Map all textures in folder to sub primitives in 1-1 ratio in order AFTER previous texture depths.
    mario.map_texture(&texture_1024_array, EnumAssetMapMethod::OneForEach(1, texture_1024_array.len()));
    mario.translate(-5.0, -5.0, 15.0);
    mario.apply(&mut shader)?;  // Bake and send the asset.
    mario.show(EnumAssetPrimitiveSurface::Everything);
    
    let mut logo = REntity::new(logo_asset, EnumPrimitiveShading::default(), "N64 Logo");
    
    // Map all textures in folder to sub primitives in a randomized fashion.
    logo.map_texture(&texture_64_array, EnumAssetMapMethod::Randomized);
    logo.translate(3.0, 0.0, 7.0);
    logo.apply(&mut shader)?;  // Bake and send the asset.
    logo.show(EnumAssetPrimitiveSurface::Everything);
    
    self.m_r_assets.insert("Smooth assets", (shader, vec![awp, mario, logo]));
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Asset sent to GPU successfully");
    
    let mut main_camera = camera::Camera::new(camera::EnumCameraType::Perspective(75, aspect_ratio, 0.01, 1000.0), None);
    main_camera.on_update(self.m_engine.get_time_step());
    self.m_cameras.push(main_camera);
    
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
    
    if Engine::is_key(input::EnumKey::Up, input::EnumAction::Held) {
      for asset in self.m_r_assets.values_mut() {
        for primitive in asset.1.iter_mut() {
          primitive.rotate(0.0, 25.0 * time_step as f32, 0.0);
          primitive.reapply()?;
        }
      }
    }
    if Engine::is_key(input::EnumKey::Left, input::EnumAction::Held) {
      for asset in self.m_r_assets.values_mut() {
        for primitive in asset.1.iter_mut() {
          primitive.rotate(-25.0 * time_step as f32, 0.0, 0.0);
          primitive.reapply()?;
        }
      }
    }
    if Engine::is_key(input::EnumKey::Down, input::EnumAction::Held) {
      for asset in self.m_r_assets.values_mut() {
        for primitive in asset.1.iter_mut() {
          primitive.rotate(0.0, -25.0 * time_step as f32, 0.0);
          primitive.reapply()?;
        }
      }
    }
    if Engine::is_key(input::EnumKey::Right, input::EnumAction::Held) {
      for asset in self.m_r_assets.values_mut() {
        for primitive in asset.1.iter_mut() {
          primitive.rotate(25.0 * time_step as f32, 0.0, 0.0);
          primitive.reapply()?;
        }
      }
    }
    return Ok(());
  }
  
  fn on_async_event(&mut self, event: &EnumEvent) -> Result<bool, EnumEngineError> {
    // Process asynchronous events.
    self.m_cameras[0].on_event(event)?;
    
    return match event {
      EnumEvent::KeyEvent(key, action, repeat_count, modifiers) => {
        match (key, action, repeat_count, modifiers) {
          (input::EnumKey::Minus, input::EnumAction::Pressed, _, _) => {
            for asset in self.m_r_assets.values_mut() {
              for primitive in asset.1.iter_mut() {
                primitive.toggle_primitive_mode((primitive.get_primitive_mode() == EnumRendererRenderPrimitiveAs::SolidWireframe)
                  .then(|| EnumRendererRenderPrimitiveAs::Filled)
                  .unwrap_or(EnumRendererRenderPrimitiveAs::SolidWireframe));
                primitive.reapply()?;
              }
            }
            Ok(true)
          }
          (input::EnumKey::Num0, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[0].hide(EnumAssetPrimitiveSurface::Everything);
            Ok(true)
          }
          (input::EnumKey::Num0, input::EnumAction::Pressed, _, &input::EnumModifiers::Shift) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[0].show(EnumAssetPrimitiveSurface::Everything);
            Ok(true)
          }
          (input::EnumKey::Num1, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[1].hide(EnumAssetPrimitiveSurface::Everything);
            Ok(true)
          }
          (input::EnumKey::Num1, input::EnumAction::Pressed, _, &input::EnumModifiers::Shift) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[1].show(EnumAssetPrimitiveSurface::Everything);
            Ok(true)
          }
          (input::EnumKey::Num2, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[2].hide(EnumAssetPrimitiveSurface::Everything);
            Ok(true)
          }
          (input::EnumKey::Num2, input::EnumAction::Pressed, _, &input::EnumModifiers::Shift) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[2].show(EnumAssetPrimitiveSurface::Everything);
            Ok(true)
          }
          (input::EnumKey::Num2, input::EnumAction::Pressed, _, &input::EnumModifiers::Alt) => {
            // renderer.toggle_msaa(Some(4))?;
            Ok(true)
          }
          (input::EnumKey::Delete, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
            for (_, r_assets) in self.m_r_assets.values_mut() {
              for r_asset in r_assets.iter_mut() {
                r_asset.free()?;
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