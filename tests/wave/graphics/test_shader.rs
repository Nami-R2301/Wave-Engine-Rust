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

use wave_engine::wave::graphics::renderer::{VkRenderer, GlRenderer, Renderer};
use wave_engine::wave::graphics::shader;
use wave_engine::wave::graphics::shader::{GlShader, GlslShader, VkShader};
use wave_engine::wave::math::Mat4;
use wave_engine::wave::window;
use wave_engine::wave::window::GlfwWindow;

#[test]
#[ignore]
fn test_shader_send() {
  // Setup window context in order to use gl functions.
  let window = GlfwWindow::new();
  match window.as_ref() {
    Ok(_) => {}
    Err(window::EnumErrors::AlreadyInitializedError) => {}
    Err(_) => { return assert!(false); }
  }
  
  // Setup renderer in order to use gl functions.
  #[cfg(feature = "OpenGL")]
  let renderer = Renderer::<GlRenderer>::new(&mut window.unwrap());
  
  #[cfg(feature = "Vulkan")]
    let renderer = Renderer::<VkRenderer>::new(&mut window.unwrap());
  
  assert!(renderer.is_ok());
  
  #[cfg(feature = "Vulkan")]
  let result = GlslShader::<VkShader>::new("res/shaders/default_3D.vert",
    "res/shaders/default_3D.frag");
  
  #[cfg(feature = "OpenGL")]
    let result = GlslShader::<GlShader>::new("res/shaders/default_3D.vert",
    "res/shaders/default_3D.frag");
  
  if result.is_err() {
    return assert!(false);
  }
  
  assert_ne!(result.as_ref().unwrap().get_api_data().m_vertex_str, "");
  assert_ne!(result.as_ref().unwrap().get_api_data().m_fragment_str, "");
  
  // Sourcing and compilation.
  let result = result.unwrap().send();
  assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_load_uniforms() {
  let window = GlfwWindow::new();
  match window.as_ref() {
    Ok(_) => {}
    Err(window::EnumErrors::AlreadyInitializedError) => {}
    Err(err) => {
      println!("[Window] --> Cannot create window! Error => {:?}", err);
      return assert!(false);
    }
  }
  
  // Setup renderer in order to use gl functions.
  #[cfg(feature = "OpenGL")]
    let renderer = Renderer::<GlRenderer>::new(&mut window.unwrap());
  
  #[cfg(feature = "Vulkan")]
    let renderer = Renderer::<VkRenderer>::new(&mut window.unwrap());
  assert!(renderer.is_ok());
  
  #[cfg(feature = "Vulkan")]
    let mut new_shader = GlslShader::<VkShader>::new("res/shaders/default_3D.vert",
    "res/shaders/default_3D.frag");
  
  #[cfg(feature = "OpenGL")]
    let mut new_shader = GlslShader::<GlShader>::new("res/shaders/default_3D.vert",
    "res/shaders/default_3D.frag");
  
  if new_shader.is_err() {
    return assert!(false);
  }
  
  assert!(new_shader.as_ref().is_ok());
  assert_ne!(new_shader.as_ref().unwrap().get_api_data().m_vertex_str, "");
  assert_ne!(new_shader.as_ref().unwrap().get_api_data().m_fragment_str, "");
  
  // Sourcing and compilation.
  let result = new_shader.as_mut().unwrap().send();
  assert!(result.is_ok());
  
  #[cfg(feature = "OpenGL")]
  match new_shader.as_mut().unwrap().get_api_data().bind() {
    Ok(_) => {}
    Err(_) => { return assert!(false); }
  }
  
  // Load uniforms.
  let uniform = new_shader.as_mut().unwrap().upload_data("u_model_matrix",
    &Mat4::new(1.0));
  
  match uniform {
    Ok(_) => {}
    Err(shader::EnumErrors::GlError(err)) => {
      println!("[Window] --> Cannot load uniform {0}! Error => 0x{1:x}", "u_has_texture", err);
      return assert!(false);
    }
    _ => {}
  }
}