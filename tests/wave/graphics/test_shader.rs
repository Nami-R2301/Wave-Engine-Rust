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
use wave_engine::wave::graphics::shader::Shader;

use wave_engine::wave::math::Mat4;

#[test]
fn test_shader_send() -> Result<(), EnumError> {
  let mut engine = Engine::new(None, Some(Box::new(EmptyApp::new())))?;
  engine.on_new()?;
  
  let mut result = Shader::new("res/shaders/default_3D.vert",
    "res/shaders/default_3D.frag")?;
  
  // Check if shader str is empty.
  assert_ne!(result.to_string(), "Vertex shader :\n\nFragment shader : \n");
  
  // Sourcing and compilation.
  return match result.send() {
    Ok(_) => { Ok(()) }
    Err(err) => { Err(EnumError::from(err)) }
  };
}

#[test]
fn test_load_uniforms() -> Result<(), EnumError> {
  let mut engine = Engine::new(None, Some(Box::new(EmptyApp::new())))?;
  engine.on_new()?;
  
  let mut new_shader = Shader::new("res/shaders/default_3D.vert",
    "res/shaders/default_3D.frag")?;
  
  // Check if shader str is empty.
  assert_ne!(new_shader.to_string(), "[Shader] -->\t\nVertex shader :\n\
  \nFragment shader : \n");
  
  // Sourcing and compilation.
  new_shader.send()?;
  
  // Load uniforms.
  return match  new_shader.upload_data("u_model_matrix",
    &Mat4::new(1.0)) {
    Ok(_) => { Ok(()) }
    Err(err) => { Err(EnumError::from(err)) }
  };
}