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
use std::collections::HashMap;
use std::fmt::Display;
use wave_core::{camera, Engine, EnumEngineError, input, TraitBake, TraitOption};
use wave_core::assets::asset_loader::{AssetLoader};
use wave_core::assets::r_assets::{EnumAssetMapMethod, EnumAssetPrimitiveSurface, EnumPrimitiveShading, REntity};
#[allow(unused)]
use wave_core::dependencies::chrono;
use wave_core::events::{EnumEvent};
use wave_core::graphics::renderer::{EnumRendererRenderPrimitiveAs, EnumRendererApi};
use wave_core::graphics::{shader};
use wave_core::graphics::open_gl::renderer::GlContext;
use wave_core::graphics::shader::EnumShaderOption;
use wave_core::graphics::texture::{Texture, TextureArray};
use wave_core::layers::{EnumLayerOption, TraitLayer};
use wave_core::utils::texture_loader::{EnumTextureLoaderOption, TextureLoader};
use wave_core::utils::macros::logger::*;
use wave_core::log::{EnumLogColor, color_to_str, Logger};
use wave_core::utils::Time;

pub struct Editor {
  m_r_assets: HashMap<&'static str, (shader::Shader, Vec<REntity>)>,
  m_cameras: Vec<camera::Camera>,
  m_textures: Vec<Texture>,
}

impl Editor {
  pub fn new() -> Self {
    return Self {
      m_r_assets: Default::default(),
      m_cameras: vec![],
      m_textures: vec![],
    };
  }
}

impl TraitLayer for Editor {
  fn on_bake(&mut self, _options: &mut Vec<EnumLayerOption>, env: &mut Engine, logger: Option<&mut Logger>) -> Result<(), EnumEngineError> {
    let renderer = env.get_renderer_mut::<GlContext>().expect("No active renderer!");
    let logger = logger.expect("No logger!");
    
    logger.log("INFO", "Loading shaders...")?;
    
    let mut shader: shader::Shader = shader::Shader::default();  // Get default smooth shader with 3 stages (vertex, geometry, and fragment).
    shader.set_option(EnumShaderOption::ForceGlslVersion(420));
    // shader.set_option(EnumShaderHint::Api(EnumRendererApi::Vulkan));
    
    // Source and compile the shader program.
    shader.bake()?;
    
    logger.log("INFO", "Loaded shaders successfully")?;
    logger.log("INFO", "Sending textures to GPU...")?;
    
    let mut texture_preset = TextureLoader::new();
    texture_preset.set_option(EnumTextureLoaderOption::FlipUvs(true));
    
    let awp_texture_info = texture_preset.load("res/textures/awp/awp_texture.jpeg")?;
    
    // Load all textures in folders.
    texture_preset.set_option(EnumTextureLoaderOption::FlipUvs(false));
    let mario_textures_info = texture_preset.load_from_folder("res/textures/mario")?;
    let n64_logo_textures_info = texture_preset.load_from_folder("res/textures/n64_logo")?;
    
    // Batch all textures from assets that share the same size to fit them in an appropriate 'texture array bucket' in the shader.
    let mut texture_1024_array = TextureArray::new(EnumRendererApi::OpenGL, vec![awp_texture_info]);
    texture_1024_array.append(mario_textures_info);
    
    let texture_64_array = TextureArray::new(EnumRendererApi::OpenGL, n64_logo_textures_info);
    
    let mut texture_1024_handle = texture_1024_array.get_texture_handle();
    let mut textures_64_handle = texture_64_array.get_texture_handle();
    
    texture_1024_handle.bake()?;
    textures_64_handle.bake()?;
    
    self.m_textures.push(texture_1024_handle);
    self.m_textures.push(textures_64_handle);
    
    logger.log("INFO", "Textures sent to GPU...")?;
    logger.log("INFO", "Sending assets to GPU...")?;
    
    let asset_loader = AssetLoader::new();
    // asset_loader.set_option(EnumAssetHint::VertexDataIs(EnumAssetPrimitiveMode::Plain));
    
    let awp_asset = asset_loader.load("res/assets/awp/awp.obj")?;
    let mario_asset = asset_loader.load("res/assets/mario/mario.obj")?;
    let logo_asset = asset_loader.load("res/assets/n64_logo/n64_logo.obj")?;
    
    let mut awp = REntity::new(awp_asset, EnumPrimitiveShading::default(), "Awp Sniper");
    
    // Map all textures in folder to sub primitives in 1-1 ratio in order.
    awp.map_texture(&texture_1024_array, EnumAssetMapMethod::MultipleForEach(2, 0, 1));
    awp.translate(10.0, -10.0, 50.0);
    awp.rotate(90.0, -90.0, 0.0);
    awp.apply(&mut shader, renderer)?;  // Bake and send the asset.
    awp.show(EnumAssetPrimitiveSurface::Everything, renderer)?;
    
    let mut mario = REntity::new(mario_asset, EnumPrimitiveShading::default(), "Mario");
    
    // Map all textures in folder to sub primitives in 1-1 ratio in order AFTER previous texture depths.
    mario.map_texture(&texture_1024_array, EnumAssetMapMethod::OneForEach(1, texture_1024_array.len()));
    mario.translate(-5.0, -5.0, 15.0);
    mario.apply(&mut shader, renderer)?;  // Bake and send the asset.
    mario.show(EnumAssetPrimitiveSurface::Everything, renderer)?;
    
    let mut logo = REntity::new(logo_asset, EnumPrimitiveShading::default(), "N64 Logo");
    
    // Map all textures in folder to sub primitives in a randomized fashion.
    logo.map_texture(&texture_64_array, EnumAssetMapMethod::Randomized);
    logo.translate(3.0, 0.0, 7.0);
    logo.apply(&mut shader, renderer)?;  // Bake and send the asset.
    logo.show(EnumAssetPrimitiveSurface::Everything, renderer)?;
    
    self.m_r_assets.insert("Smooth assets", (shader, vec![awp, mario, logo]));
    
    logger.log("INFO", "Asset sent to GPU successfully")?;
    
    // Show our window when we are ready to present.
    let window = env.get_window_mut().expect("No window to attach editor to!");
    let aspect_ratio: f32 = window.get_aspect_ratio();
    let main_camera = camera::Camera::new(camera::EnumCameraType::Perspective(75, aspect_ratio, 0.01, 1000.0), None);
    self.m_cameras.push(main_camera);
    
    window.show();
    return Ok(());
  }
  
  fn on_event(&mut self, event: &EnumEvent, env: &mut Engine) -> Result<bool, EnumEngineError> {
    // Process asynchronous events.
    self.m_cameras[0].on_event(event)?;
    
    return match event {
      EnumEvent::KeyEvent(key, action, repeat_count, modifiers) => {
        match (key, action, repeat_count, modifiers) {
          (input::EnumKey::Escape, input::EnumAction::Pressed, _, _) => {
            log!(EnumLogColor::Yellow, "EVENT", "[Window] -->\t Window close event");
            env.on_async_event(&EnumEvent::WindowCloseEvent(Time::now()));
            return Ok(true);
          },
          (input::EnumKey::Minus, input::EnumAction::Pressed, _, _) => {
            for asset in self.m_r_assets.values_mut() {
              for primitive in asset.1.iter_mut() {
                primitive.toggle_primitive_mode((primitive.get_primitive_mode() == EnumRendererRenderPrimitiveAs::SolidWireframe)
                  .then(|| EnumRendererRenderPrimitiveAs::Filled)
                  .unwrap_or(EnumRendererRenderPrimitiveAs::SolidWireframe));
                primitive.reapply(env.get_renderer_mut::<GlContext>().unwrap())?;
              }
            }
            Ok(true)
          }
          (input::EnumKey::Num0, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[0].hide(EnumAssetPrimitiveSurface::Everything,
              env.get_renderer_mut::<GlContext>().unwrap())?;
            Ok(true)
          }
          (input::EnumKey::Num0, input::EnumAction::Pressed, _, &input::EnumModifiers::Shift) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[0].show(EnumAssetPrimitiveSurface::Everything,
              env.get_renderer_mut::<GlContext>().unwrap())?;
            Ok(true)
          }
          (input::EnumKey::Num1, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[1].hide(EnumAssetPrimitiveSurface::Everything,
              env.get_renderer_mut::<GlContext>().unwrap())?;
            Ok(true)
          }
          (input::EnumKey::Num1, input::EnumAction::Pressed, _, &input::EnumModifiers::Shift) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[1].show(EnumAssetPrimitiveSurface::Everything,
              env.get_renderer_mut::<GlContext>().unwrap())?;
            Ok(true)
          }
          (input::EnumKey::Num2, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[2].hide(EnumAssetPrimitiveSurface::Everything,
              env.get_renderer_mut::<GlContext>().unwrap())?;
            Ok(true)
          }
          (input::EnumKey::Num2, input::EnumAction::Pressed, _, &input::EnumModifiers::Shift) => {
            self.m_r_assets.get_mut(&"Smooth assets").unwrap().1[2].show(EnumAssetPrimitiveSurface::Everything,
              env.get_renderer_mut::<GlContext>().unwrap())?;
            Ok(true)
          }
          (input::EnumKey::Num2, input::EnumAction::Pressed, _, &input::EnumModifiers::Alt) => {
            // renderer.toggle_msaa(Some(4))?;
            Ok(true)
          }
          (input::EnumKey::Delete, input::EnumAction::Pressed, _, &input::EnumModifiers::Control) => {
            for (_, r_assets) in self.m_r_assets.values_mut() {
              for r_asset in r_assets.iter_mut() {
                r_asset.free::<GlContext>()?;
              }
            }
            return Ok(true);
          }
          _ => Ok(false)
        }
      }
      EnumEvent::WindowCloseEvent(_time) => {
        self.on_free()?;
        Ok(true)
      }
      _ => Ok(false)
    };
  }
  
  fn on_frame(&mut self, env: &mut Engine) -> Result<(), EnumEngineError> {
    // Process synchronous events.
    let time_step = env.get_time_step() as f32;
    let mut rotate = [0.0, 0.0];
    
    if env.is_key(input::EnumKey::Up, input::EnumAction::Held) {
      rotate[1] +=  25.0 * time_step;
    }
    if env.is_key(input::EnumKey::Left, input::EnumAction::Held) {
      rotate[0] -= 25.0 * time_step;
    }
    if env.is_key(input::EnumKey::Down, input::EnumAction::Held) {
      rotate[1] -= 25.0 * time_step;
    }
    if env.is_key(input::EnumKey::Right, input::EnumAction::Held) {
      rotate[0] += 25.0 * time_step;
    }
    
    // Apply all movement recorded.
    for asset in self.m_r_assets.values_mut() {
      for primitive in asset.1.iter_mut() {
        primitive.rotate(rotate[0], rotate[1], 0.0);
        primitive.reapply(env.get_renderer_mut::<GlContext>().unwrap())?;
      }
    }
    
    return self.m_cameras[0].on_frame::<GlContext>(env).map_err(|err| EnumEngineError::from(err));
  }
  
  fn on_free(&mut self) -> Result<(), EnumEngineError> {
    for asset in self.m_r_assets.values_mut() {
      log!(EnumLogColor::Purple, "INFO", "[App] -->\t Freeing game assets for shader [{0}]...",
        asset.0.get_id());
      for primitive in asset.1.iter_mut() {
        primitive.free::<GlContext>()?;
      }
      log!(EnumLogColor::Green, "INFO", "[App] -->\t Freed game assets for shader [{0}]",
      asset.0.get_id());
      asset.0.free()?;
    }
    
    
    for texture in self.m_textures.iter_mut() {
      texture.free()?;
    }
    _engine_log!(EnumLogColor::Green, "INFO", "[App] -->\t Freed textures successfully");
    return Ok(());
  }
}

impl Display for Editor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut final_str: String = Default::default();
    
    for (linked_shader, r_asset_vec) in self.m_r_assets.values() {
      final_str += &format!("\n{0:115}Assets: ({1})", "", r_asset_vec.len());
      
      for (position, r_asset) in r_asset_vec.iter().enumerate() {
        final_str += &format!("\n{0:117}[{1}]:\n{0:119}Associated shader:\n{0:121}{2}\n{0:119}Meshes:\
        \n{0:121}{3}", "", position + 1, linked_shader, r_asset);
      }
    }
    return write!(f, "{}", final_str);
  }
}