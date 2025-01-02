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
use std::fmt::{Display, Formatter};
use wave_core::{TraitBake};
use wave_core::graphics::renderer::{EnumRendererApi, TraitContext};
use wave_core::{Engine, EnumEngineError};
use wave_core::events::EnumEventMask;
use wave_core::graphics::open_gl::renderer::GlContext;
use wave_core::graphics::open_gl::shader::GlShader;
use wave_core::graphics::renderer::{Renderer};
use wave_core::graphics::shader::{EnumShaderSource, EnumShaderStageType, Shader, ShaderStage};
use wave_core::layers::{EnumLayerType, Layer};

use wave_core::math::Mat4;
use wave_core::window::Window;

struct EmptyApp {
}

impl Display for EmptyApp {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "EmptyApp")
  }
}

#[ignore]
#[test]
fn test_shader_send() -> Result<(), EnumEngineError> {
  let layer = Layer::new("Shader send", Box::new(EmptyApp {}), EnumLayerType::Log, EnumEventMask::empty());
  let window = Window::new("Test Shader Send", EnumRendererApi::OpenGL);
  let renderer = Renderer::new(GlContext::new());
  
  let window_layer = Layer::new_window_layer("Window", window);
  let renderer_layer = Layer::new_renderer_layer("Renderer", renderer);
  
  let mut engine = Engine::new(vec![window_layer, renderer_layer, layer]);
  engine.bake()?;
  
  let vertex_shader = ShaderStage::new(EnumShaderStageType::Vertex,
    EnumShaderSource::FromFile(String::from("res/shaders/test.vert")));
  let fragment_shader = ShaderStage::new(EnumShaderStageType::Fragment,
    EnumShaderSource::FromFile(String::from("res/shaders/test.frag")));
  
  let mut shader: Shader<GlShader> = Shader::default();
  
  shader.push_stage(vertex_shader)?;
  shader.push_stage(fragment_shader)?;
  
  let result = shader.bake();
  
  // Sourcing and compilation.
  return match result {
    Ok(_) => { Ok(()) }
    Err(err) => { Err(EnumEngineError::from(err)) }
  };
}

#[ignore]
#[test]
fn test_load_uniforms() -> Result<(), EnumEngineError> {
  let layer = Layer::new("Shader send", Box::new(EmptyApp {}), EnumLayerType::Log, EnumEventMask::empty());
  let window = Window::new("Test Shader Send", EnumRendererApi::OpenGL);
  let renderer = Renderer::new(GlContext::new());
  
  let window_layer = Layer::new_window_layer("Window", window);
  let renderer_layer = Layer::new_renderer_layer("Renderer", renderer);
  
  let mut engine = Engine::new(vec![window_layer, renderer_layer, layer]);
  engine.bake()?;
  
  let vertex_shader = ShaderStage::new(EnumShaderStageType::Vertex,
    EnumShaderSource::FromFile(String::from("res/shaders/test.vert")));
  let fragment_shader = ShaderStage::new(EnumShaderStageType::Fragment,
    EnumShaderSource::FromFile(String::from("res/shaders/test.frag")));
  
  let mut shader: Shader<GlShader> = Shader::default();
  
  shader.push_stage(vertex_shader)?;
  shader.push_stage(fragment_shader)?;
  
  shader.bake()?;
  
  // Load uniforms.
  return match  shader.upload_data("u_model_matrix",
    &Mat4::new(1.0)) {
    Ok(_) => { Ok(()) }
    Err(err) => { Err(EnumEngineError::from(err)) }
  };
}