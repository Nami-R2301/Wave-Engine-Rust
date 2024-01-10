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

use wave_engine::wave::{EmptyApp, Engine, EnumError};
use wave_engine::wave::graphics::shader::{EnumShaderSource, EnumShaderType, Shader, ShaderStage};

use wave_engine::wave::math::Mat4;

#[test]
fn test_shader_send() -> Result<(), EnumError> {
  let mut engine = Engine::new(None, Some(Box::new(EmptyApp::new())))?;
  engine.on_new()?;
  
  let vertex_shader = ShaderStage::new(EnumShaderType::Vertex,
    EnumShaderSource::FromFile(String::from("res/shaders/test.vert")));
  let fragment_shader = ShaderStage::new(EnumShaderType::Fragment,
    EnumShaderSource::FromFile(String::from("res/shaders/test.frag")));
  
  let mut result = Shader::new(vec![vertex_shader, fragment_shader])?;
  
  // Sourcing and compilation.
  return match result.submit() {
    Ok(_) => { Ok(()) }
    Err(err) => { Err(EnumError::from(err)) }
  };
}

#[test]
fn test_load_uniforms() -> Result<(), EnumError> {
  let mut engine = Engine::new(None, Some(Box::new(EmptyApp::new())))?;
  engine.on_new()?;
  
  let vertex_shader = ShaderStage::new(EnumShaderType::Vertex,
    EnumShaderSource::FromFile(String::from("res/shaders/test.vert")));
  let fragment_shader = ShaderStage::new(EnumShaderType::Fragment,
    EnumShaderSource::FromFile(String::from("res/shaders/test.frag")));
  
  let mut new_shader = Shader::new(vec![vertex_shader, fragment_shader])?;
  
  // Sourcing and compilation.
  new_shader.submit()?;
  
  // Load uniforms.
  return match  new_shader.upload_data("u_model_matrix",
    &Mat4::new(1.0)) {
    Ok(_) => { Ok(()) }
    Err(err) => { Err(EnumError::from(err)) }
  };
}