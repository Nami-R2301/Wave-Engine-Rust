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

extern crate glfw;

#[cfg(feature = "Vulkan")]
use ash::vk;

#[cfg(feature = "OpenGL")]
use glfw::{Context};

use crate::log;

#[cfg(feature = "OpenGL")]
use crate::wave::graphics::buffer::GLsizei;

use crate::wave::math::Vec2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumState {
  Open,
  Closed,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumErrors {
  NoContextError,
  InitError,
  ApiError,
  AlreadyInitializedError,
  VulkanIncompatibleError,
  VulkanSurfaceCreationError,
}

#[cfg(feature = "debug")]
fn glfw_error_callback(error: glfw::Error, message: String) {
  log!(EnumLogColor::Red, "ERROR", "[Window] -->\t GLFW error raised! Error => {0}\n{1:100}Info => \
   {2}", error, "", message);
}

pub struct GlfwWindow {
  m_state: EnumState,
  m_api_window: glfw::PWindow,
  m_api_window_events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
  m_fullscreen: bool,
  pub m_vsync: bool,
  pub m_vulkan_compatible: bool,
  pub m_window_bounds: Vec2<i32>,
}

impl GlfwWindow {
  pub fn new() -> Result<Self, EnumErrors> {
    let mut result = glfw::init(glfw::fail_on_errors);
    
    match result {
      Err(glfw::InitError::AlreadyInitialized) => {
        log!(EnumLogColor::Yellow, "WARN",
          "[Window] -->\t GLFW window already initialized! Skipping \
         creation of a new one...");
      }
      Err(glfw::InitError::Internal) => {
        log!(EnumLogColor::Red, "ERROR",
          "[Window] -->\t Failed to create GLFW window due to internal \
         error! Exiting...");
        return Err(EnumErrors::InitError);
      }
      Ok(_) => {}
    }
    
    let context_ref = result.as_mut().unwrap();
    
    context_ref.window_hint(glfw::WindowHint::Samples(Some(8)));
    context_ref.window_hint(glfw::WindowHint::RefreshRate(None));
    
    #[cfg(feature = "Vulkan")]
    {
      context_ref.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    }
    
    #[cfg(all(feature = "OpenGL", feature = "debug"))]
    {
      context_ref.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
    }
    
    // Create a windowed mode window and its OpenGL context
    match context_ref.create_window(1920, 1080,
      "Wave Engine (Rust)", glfw::WindowMode::Windowed) {
      None => {
        log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Unable to create GLFW window");
        return Err(EnumErrors::InitError);
      }
      Some((mut window, events)) => {
        
        // Set input polling rate.
        window.set_sticky_keys(true);
        window.set_sticky_mouse_buttons(true);
        
        // Set glfw events.
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);
        
        // Set GLFW error callback.
        #[cfg(feature = "debug")]
        window.glfw.set_error_callback(glfw_error_callback);
        
        let bounds = Vec2::from(&[window.get_size().0, window.get_size().1]);
        
        Ok(GlfwWindow {
          m_state: EnumState::Open,
          m_vulkan_compatible: window.glfw.vulkan_supported(),
          m_api_window: window,
          m_api_window_events: events,
          m_fullscreen: false,
          m_vsync: true,
          m_window_bounds: bounds,
        })
      }
    }
  }
  
  #[cfg(feature = "OpenGL")]
  pub fn init_opengl_surface(&mut self) {
    // Make the window's context current
    self.m_api_window.make_current();
    
    // Set v-sync.
    self.m_api_window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
  }
  
  #[cfg(feature = "Vulkan")]
  pub fn init_vulkan_surface(&mut self, vk_instance: &ash::Instance, vk_surface_khr: &mut vk::SurfaceKHR) {
    
    if !self.m_vulkan_compatible {
      log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Cannot create Vulkan surface,\
       Vulkan API is not supported! Make sure the Vulkan SDK and drivers \
             are installed or select another renderer and re-run app! Exiting...");
      panic!();
    }
    
    let result = self.m_api_window.create_window_surface(vk_instance.handle(),
      std::ptr::null_mut(), vk_surface_khr).result();
    
    if result.is_err() {
      log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Failed to create Vulkan surface!");
      panic!();
    }
  }
  
  pub fn on_event(&mut self) -> bool {
    self.m_api_window.glfw.poll_events();
    for (_, event) in glfw::flush_messages(&self.m_api_window_events) {
      return match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
          self.m_api_window.set_should_close(true);
          log!(EnumLogColor::Yellow, "WARN", "[Window] -->\t User requested to close the window");
          true
        }
        glfw::WindowEvent::Key(glfw::Key::Enter, _, glfw::Action::Press, glfw::Modifiers::Alt) => {
          let window: *mut glfw::PWindow = &mut self.m_api_window;
          self.m_api_window.glfw.with_primary_monitor(|_, monitor| {
            let mode: glfw::VidMode = monitor.as_ref().unwrap().get_video_mode().unwrap();
            if !self.m_fullscreen {
              unsafe {
                (*window).set_monitor(glfw::WindowMode::FullScreen(monitor.unwrap()),
                  0, 0, mode.width, mode.height, Some(mode.refresh_rate));
              }
              log!("INFO", "[Window] -->\t Fullscreen ON");
              #[cfg(feature = "OpenGL")]
              unsafe {
                gl::Viewport(0, 0, mode.width as GLsizei, mode.height as GLsizei);
              }
              self.m_fullscreen = true;
            } else {
              unsafe {
                (*window).set_monitor(glfw::WindowMode::Windowed,
                  0, 0, self.m_window_bounds.x as u32, self.m_window_bounds.y as u32,
                  Some(mode.refresh_rate));
              }
              log!("INFO", "[Window] -->\t Fullscreen OFF");
              #[cfg(feature = "OpenGL")]
              unsafe {
                gl::Viewport(0, 0, self.m_window_bounds.x as GLsizei,
                  self.m_window_bounds.y as GLsizei);
              }
              self.m_fullscreen = false;
            }
          });
          true
        }
        glfw::WindowEvent::Key(glfw::Key::V, _, glfw::Action::Press, glfw::Modifiers::Alt) => {
          if self.m_vsync {
            self.m_api_window.glfw.set_swap_interval(glfw::SwapInterval::None);
            self.m_vsync = false;
          } else {
            self.m_api_window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
            self.m_vsync = true;
          }
          return false;
        }
        glfw::WindowEvent::Key(key, _scancode, action, _mods) => {
          // log!("INFO", "Key: {:?}, ScanCode: {:?}, Action: {:?}, Modifiers: [{:?}]",
          // key, scancode, action, mods);
          
          match (key, action) {
            (glfw::Key::R, glfw::Action::Press) => {
              // Resize should force the window to "refresh"
              let (window_width, window_height) = self.m_api_window.get_size();
              self.m_api_window.set_size(window_width + 1, window_height);
              self.m_api_window.set_size(window_width, window_height);
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
          log!("INFO", "[Window] -->\t Framebuffer size: ({0}, {1})", width, height);
          false
        }
        _ => false
      };
    };
    
    return false;
  }
  
  pub fn refresh(&mut self) {
    #[cfg(feature = "OpenGL")]
    self.m_api_window.swap_buffers();
  }
  
  pub fn is_closing(&self) -> bool {
    return self.m_api_window.should_close();
  }
  
  pub fn close(&mut self) {
    self.m_api_window.set_should_close(true);
    self.m_state = EnumState::Closed;
  }
  
  pub fn get_state(&self) -> EnumState {
    return self.m_state;
  }
  
  pub fn get_api_ptr(&self) -> &glfw::Glfw {
    return &self.m_api_window.glfw;
  }
  
  pub fn get_api_window(&mut self) -> &mut glfw::PWindow {
    return &mut self.m_api_window;
  }
  
  pub fn set_title(&mut self, title: &str) {
    return self.m_api_window.set_title(title);
  }
  
  pub fn get_size(&self) -> &Vec2<i32> {
    return &self.m_window_bounds;
  }
}