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

use wave_engine::wave::graphics::renderer::GlRenderer;
use wave_engine::wave::graphics::shader;
use wave_engine::wave::graphics::shader::{GlShader};
use wave_engine::wave::window;
use wave_engine::wave::window::GlfwWindow;

#[test]
fn test_shader_send() {
  // Setup window context in order to use gl functions.
  let window = GlfwWindow::new();
  match window.as_ref() {
    Ok(_) => {}
    Err(window::EnumErrors::AlreadyInitialized) => {}
    Err(_) => { return assert!(false); }
  }
  
  // Setup renderer in order to use gl functions.
  let renderer = GlRenderer::new();
  assert!(renderer.is_ok());
  
  let mut result = GlShader::new("res/shaders/default_3D.vert",
    "res/shaders/default_3D.frag");
  let new_shader: &mut GlShader;
  
  match result {
    Ok(_) => {
      new_shader = result.as_mut().unwrap();
    }
    Err(_) => { return assert!(false); }
  }
  assert_ne!(new_shader.m_vertex_str, "");
  assert_ne!(new_shader.m_fragment_str, "");
  
  // Sourcing and compilation.
  let result = unsafe { new_shader.send() };
  assert!(result.is_ok());
}

#[test]
fn test_load_uniforms() {
  let window = GlfwWindow::new();
  match window.as_ref() {
    Ok(_) => {}
    Err(window::EnumErrors::AlreadyInitialized) => {}
    Err(err) => {
      println!("[Window] --> Cannot create window! Error => {:?}", err);
      return assert!(false);
    }
  }
  
  // Setup renderer in order to use gl functions.
  let renderer = GlRenderer::new();
  assert!(renderer.is_ok());
  
  let mut new_shader = GlShader::new("res/shaders/default_3D.vert",
    "res/shaders/default_3D.frag");
  
  assert!(new_shader.as_ref().is_ok());
  assert_ne!(new_shader.as_ref().unwrap().m_vertex_str, "");
  assert_ne!(new_shader.as_ref().unwrap().m_fragment_str, "");
  
  // Sourcing and compilation.
  let result = unsafe { new_shader.as_mut().unwrap().send() };
  assert!(result.is_ok());
  
  match new_shader.as_ref().unwrap().bind(){
    Ok(_) => {}
    Err(_) => { return assert!(false)}
  }
  // Load uniforms.
  let uniform = new_shader.as_mut().unwrap().load_uniform("u_has_texture",
    1i32);
  
  match uniform {
    Ok(_) => {}
    Err(shader::EnumErrors::GlError(err)) => {
      println!("[Window] --> Cannot load uniform {0}! Error => 0x{1:x}", "u_has_texture", err);
      return assert!(false);
    }
    _ => {}
  }
}