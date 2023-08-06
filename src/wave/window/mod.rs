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

extern crate imgui_glfw_rs;

use crate::{log, trace};
#[cfg(feature = "trace")]
use crate::{file_name, function_name};
use crate::wave::utils;
use crate::wave::utils::logger::EnumLogColor;

pub use imgui_glfw_rs::{glfw, glfw::Context};
pub use imgui_glfw_rs::glfw::ffi::{glfwGetPrimaryMonitor, glfwSetWindowMonitor};

static mut S_CONTEXT: glfw::Glfw = glfw::Glfw {};
static mut S_WINDOW_IS_FULLSCREEN: bool = false;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumErrors {
  Ok,
  NoContext,
  InvalidCreation,
  InvalidApi,
}

pub struct GlWindow {
  m_window: glfw::Window,
  m_window_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl GlWindow {
  pub fn new() -> Result<Self, EnumErrors> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    glfw.window_hint(glfw::WindowHint::Samples(None));
    glfw.window_hint(glfw::WindowHint::RefreshRate(None));
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
    
    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(1920, 1080,
      "Wave Engine (Rust)", glfw::WindowMode::Windowed)
      .expect("[Window] -->\t Unable to create GLFW window");
    
    // Set input polling rate.
    window.set_sticky_keys(true);
    window.set_sticky_mouse_buttons(true);
    
    // Set glfw events.
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_framebuffer_size_polling(true);
    
    // Make the window's context current
    window.make_current();
    
    // Set v-sync.
    window.glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
    
    unsafe { S_CONTEXT = window.glfw };
    
    Ok(GlWindow {
      m_window: window,
      m_window_events: events,
    })
  }
  
  pub fn delete(&mut self) {}
  
  pub fn on_event(&mut self) -> bool {
    self.m_window.glfw.poll_events();
    for (_, event) in glfw::flush_messages(&self.m_window_events) {
      return match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
          self.m_window.set_should_close(true);
          log!(EnumLogColor::Yellow, "WARN", "[Window] -->\t User requested to close the window");
          true
        }
        glfw::WindowEvent::Key(glfw::Key::Enter, _, glfw::Action::Press, glfw::Modifiers::Alt) => unsafe {
          S_WINDOW_IS_FULLSCREEN.then_some({
            glfwSetWindowMonitor(self.m_window.window_ptr(), std::ptr::null_mut(),
              0, 0, 1920, 1016, -1);
          }).unwrap_or_else(|| {
            glfwSetWindowMonitor(self.m_window.window_ptr(), glfwGetPrimaryMonitor(),
              0, 0, 1920, 1080, -1);
          });
          
          log!("INFO", "[Window] -->\t Fullscreen {0}",
            S_WINDOW_IS_FULLSCREEN.then_some("ON").unwrap_or("OFF"));
          
          S_WINDOW_IS_FULLSCREEN = !S_WINDOW_IS_FULLSCREEN;
          gl::Viewport(0, 0, 1920, 1080);
          true
        }
        glfw::WindowEvent::Key(key, _scancode, action, _mods) => {
          // log!("INFO", "Key: {:?}, ScanCode: {:?}, Action: {:?}, Modifiers: [{:?}]",
          // key, scancode, action, mods);
          
          match (key, action) {
            (glfw::Key::R, glfw::Action::Press) => {
              // Resize should cause the window to "refresh"
              let (window_width, window_height) = self.m_window.get_size();
              self.m_window.set_size(window_width + 1, window_height);
              self.m_window.set_size(window_width, window_height);
              false
            }
            _ => false
          }
        }
        glfw::WindowEvent::MouseButton(_btn, _action, _mods) => {
          // log!("INFO", "Button: {:?}, Action: {:?}, Modifiers: [{:?}]", btn, action, mods);
          false
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
          log!("INFO", "[Window] -->\t Framebuffer size: ({:?}, {:?})", width, height);
          false
        }
        _ => false
      }
    };
    return false;
  }
  
  pub fn refresh(&mut self) {
    self.m_window.swap_buffers();
  }
  
  pub fn is_closing(&self) -> bool {
    return self.m_window.should_close();
  }
  
  pub fn get_api_ptr(&self) -> &glfw::Glfw {
    return &self.m_window.glfw;
  }
  
  pub fn set_title(&mut self, title: &str) {
    self.m_window.set_title(title);
  }
  
  pub unsafe fn get_current_window() -> glfw::Glfw {
    return S_CONTEXT;
  }
}