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

extern crate gl;
extern crate glfw;

use glfw::{Context, Window};

pub mod utils;
pub mod math;

pub trait App {
  fn on_new(&self);
  fn on_delete(&self);
  
  fn on_event(&self);
  fn on_update(&self);
  fn on_render(&self);
}

#[derive(Debug)]
pub struct Engine {
  m_window: Window,
  m_exit_status: i64,
}

impl Engine {
  pub fn new() -> Engine {
    let window: Window = Engine::init();
    return Engine { m_window: window, m_exit_status: 0 };
  }
  
  pub fn delete(&mut self) -> i64 {
    self.on_destroy();
    return self.get_exit_status();
  }
  
  pub fn run(&mut self) {
    self.m_exit_status = 0;
    // Loop until the user closes the window
    while !self.m_window.should_close() {
      // Poll for and process events
      self.m_window.glfw.poll_events();
      
      unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); };
      
      // Swap front and back buffers
      self.m_window.swap_buffers();
    }
  }
  
  pub fn get_exit_status(&self) -> i64
  {
    return self.m_exit_status;
  }
  
  fn init() -> Window {
    // Setup and launch engine.
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    // Create a windowed mode window and its OpenGL context
    let (mut window, _events) = glfw.create_window(1920, 1080, "Wave Engine",
      glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window.");
    
    window.glfw.window_hint(glfw::WindowHint::Samples(Option::from(8u32)));
    window.glfw.window_hint(glfw::WindowHint::RefreshRate(Option::from(144u32)));
    window.glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
    window.glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3u32));
    window.glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3u32));
    
    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    
    gl::load_with(|f_name| window.get_proc_address(f_name));
    
    unsafe { gl::Viewport(0, 0, 1920, 1080); }
    unsafe { gl::ClearColor(0.15, 0.15, 0.15, 1.0); };
    return window;
  }
  
  fn on_destroy(&mut self) {}
}

impl App for Engine {
  fn on_new(&self) {
    todo!()
  }
  
  fn on_delete(&self) {
    todo!()
  }
  
  fn on_event(&self) {
    todo!()
  }
  
  fn on_update(&self) {
    todo!()
  }
  
  fn on_render(&self) {
    todo!()
  }
}