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

use std::fmt::{Display, Formatter};

#[cfg(feature = "Vulkan")]
use ash::vk;
use glfw::Context;

use crate::log;
use crate::wave_core::graphics::renderer::{EnumApi, Renderer};
use crate::wave_core::input::{self, Input};

pub(crate) static mut S_WINDOW_CONTEXT: Option<*mut glfw::Glfw> = None;
pub static mut S_WINDOW: Option<*mut Window> = None;

static mut S_PREVIOUS_WIDTH: u32 = 640;
static mut S_PREVIOUS_HEIGHT: u32 = 480;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumState {
  Open,
  Closed,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EnumWindowMode {
  Windowed,
  Borderless,
  Fullscreen,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EnumFeatures {
  VSync(bool),
  MSAA(Option<u32>),
}

impl Display for EnumWindowMode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EnumWindowMode::Windowed => { write!(f, "Windowed") }
      EnumWindowMode::Borderless => { write!(f, "Borderless Window") }
      EnumWindowMode::Fullscreen => { write!(f, "Fullscreen") }
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumError {
  NoContextError,
  InitError,
  ApiError,
  AlreadyInitializedError,
  VulkanIncompatibleError,
  VulkanSurfaceCreationError,
  WindowInputError,
}

impl Display for EnumError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Window] -->\t Error encountered with window context : {:?}", self)
  }
}

impl From<input::EnumError> for EnumError {
  #[allow(unused)]
  fn from(input_error: input::EnumError) -> Self {
    log!(EnumLogColor::Red, "ERROR", "{0}", input_error);
    return EnumError::WindowInputError;
  }
}

impl std::error::Error for EnumError {}

#[cfg(feature = "debug")]
fn glfw_error_callback(error: glfw::Error, message: String) {
  log!(EnumLogColor::Red, "ERROR", "[Window] -->\t GLFW error raised! Error => {0}\n{1:100}Info => \
   {2}", error, "", message);
}

pub struct Window {
  pub m_api_window_events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
  pub m_api_window: glfw::PWindow,
  pub m_vsync: bool,
  pub m_samples: u8,
  pub m_window_resolution: (i32, i32),
  pub m_window_pos: (i32, i32),
  pub m_is_windowed: bool,
  m_window_mode: EnumWindowMode,
  m_render_api: EnumApi,
  m_state: EnumState,
}

impl Window {
  pub fn new(api_preference: Option<EnumApi>, resolution_preference: Option<(i32, i32)>,
             refresh_count_desired: Option<u32>, sample_count_desired: Option<u32>,
             window_mode: EnumWindowMode) -> Result<Self, EnumError> {
    let mut result = glfw::init(glfw::fail_on_errors);
    
    match result {
      Err(glfw::InitError::AlreadyInitialized) => {
        log!(EnumLogColor::Yellow, "WARN",
          "[Window] -->\t GLFW window already initialized! Skipping creation of a new one...");
      }
      Err(glfw::InitError::Internal) => {
        log!(EnumLogColor::Red, "ERROR",
          "[Window] -->\t Failed to create GLFW window due to internal error! Exiting...");
        return Err(EnumError::InitError);
      }
      Ok(_) => {}
    }
    
    let context_ref = result.as_mut().unwrap();
    
    if window_mode == EnumWindowMode::Borderless {
      context_ref.window_hint(glfw::WindowHint::Resizable(false));
      context_ref.window_hint(glfw::WindowHint::Decorated(false));
    } else if window_mode == EnumWindowMode::Windowed {
      context_ref.window_hint(glfw::WindowHint::Resizable(true));
    } else {
      context_ref.window_hint(glfw::WindowHint::Resizable(false));
    }
    
    // Hide window to prevent showing it before needing it.
    context_ref.window_hint(glfw::WindowHint::Visible(false));
    
    // If user has not chosen an api, choose accordingly.
    if api_preference.is_none() {
    } else {
      match api_preference.unwrap() {
        EnumApi::OpenGL => {
          // OpenGL hints.
          context_ref.window_hint(glfw::WindowHint::ContextVersion(4, 6));
          context_ref.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
          context_ref.window_hint(glfw::WindowHint::RefreshRate(refresh_count_desired));
          context_ref.window_hint(glfw::WindowHint::Samples(sample_count_desired));
          
          #[cfg(feature = "debug")]
          context_ref.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
        }
        EnumApi::Vulkan => {
          context_ref.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        }
      }
    }
    
    unsafe {
      S_WINDOW_CONTEXT = Some(context_ref);
    }
    
    let resolution = resolution_preference.unwrap_or((640, 480));
    
    match context_ref.create_window(resolution.0 as u32, resolution.1 as u32,
      "Wave Engine (Rust)",
      glfw::WindowMode::Windowed) {
      None => {
        log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Unable to create GLFW window");
        Err(EnumError::InitError)
      }
      Some((mut window, events)) => {
        
        // Set input polling rate.
        window.set_sticky_keys(true);
        window.set_sticky_mouse_buttons(true);
        
        // Set glfw events.
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_pos_polling(true);
        
        // Set GLFW error callback.
        #[cfg(feature = "debug")]
        window.glfw.set_error_callback(glfw_error_callback);
        
        let bounds = window.get_size();
        let initial_position = window.get_pos();
        unsafe {
          S_PREVIOUS_WIDTH = bounds.0 as u32;
          S_PREVIOUS_HEIGHT = bounds.1 as u32;
        }
        window.set_aspect_ratio(bounds.0 as u32, bounds.1 as u32);
        
        let mut glfw_window = Window {
          m_state: EnumState::Open,
          m_api_window: window,
          m_api_window_events: events,
          m_render_api: api_preference.is_some().then(|| {
            return api_preference.unwrap();
          }).unwrap_or_else(|| {
            #[cfg(not(feature = "Vulkan"))]
            return EnumApi::OpenGL;
            #[cfg(feature = "Vulkan")]
            return EnumApi::Vulkan;
          }),
          m_window_mode: window_mode,
          m_vsync: true,
          m_samples: sample_count_desired.unwrap_or(1) as u8,
          m_window_resolution: bounds,
          m_window_pos: initial_position,
          m_is_windowed: window_mode == EnumWindowMode::Windowed,
        };
        
        // Toggle on fullscreen if requested.
        return if window_mode != EnumWindowMode::Windowed {
          let result =
            context_ref.with_primary_monitor(|_, monitor| -> Result<Self, EnumError> {
              if monitor.is_none() {
                log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Cannot identify primary monitor!");
                return Err(EnumError::InitError);
              }
              
              let mode: glfw::VidMode = monitor.as_ref().unwrap().get_video_mode().unwrap();
              
              match window_mode {
                EnumWindowMode::Windowed => {}
                EnumWindowMode::Borderless => {
                  glfw_window.m_api_window.set_monitor(glfw::WindowMode::Windowed,
                    0, 0, mode.width, mode.height, refresh_count_desired);
                }
                EnumWindowMode::Fullscreen => {
                  glfw_window.m_api_window.set_monitor(glfw::WindowMode::FullScreen(monitor.unwrap()),
                    0, 0, mode.width, mode.height, Some(mode.refresh_rate));
                }
              }
              return Ok(glfw_window);
            });
          result
        } else {
          Ok(glfw_window)
        };
      }
    }
  }
  
  pub fn show(&mut self) {
    self.m_api_window.show();
  }
  
  pub fn hide(&mut self) {
    self.m_api_window.hide();
  }
  
  pub fn init_opengl_surface(&mut self) {
    // Make the window's context current
    self.m_api_window.make_current();
    
    // Set v-sync.
    self.m_api_window.glfw.set_swap_interval(glfw::SwapInterval::Sync(self.m_vsync
      .then(|| { return 1; })
      .unwrap_or(0)));
  }
  
  #[cfg(feature = "Vulkan")]
  pub fn init_vulkan_surface(&self, vk_instance: &ash::Instance, vk_surface_khr: &mut vk::SurfaceKHR) {
    let result = self.m_api_window.create_window_surface(vk_instance.handle(),
      std::ptr::null_mut(), vk_surface_khr).result();
    
    if result.is_err() {
      log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Failed to create Vulkan surface!");
      panic!();
    }
  }
  
  pub fn on_update(&mut self) -> Result<bool, EnumError> {
    return Ok(true);
  }
  
  pub fn on_event(&mut self) -> Result<bool, EnumError> {
    self.m_api_window.glfw.poll_events();
    let mut window_event_happened = false;
    for (_, event) in glfw::flush_messages(&self.m_api_window_events) {
      window_event_happened = true;
      // Asynchronous event polling.
      match event {
        glfw::WindowEvent::Key(key, _scancode, action, mods) => {
          match (key, action, mods) {
            (glfw::Key::Escape, glfw::Action::Press, _) => {
              self.m_api_window.set_should_close(true);
              log!(EnumLogColor::Yellow, "WARN", "[Window] -->\t User requested to close the window");
            }
            (glfw::Key::R, glfw::Action::Press, _) => {
              // Resize should force the window to "refresh"
              let (window_width, window_height) = self.m_api_window.get_size();
              self.m_api_window.set_size(window_width + 1, window_height);
              self.m_api_window.set_size(window_width, window_height);
            }
            (glfw::Key::Enter, glfw::Action::Press, glfw::Modifiers::Alt) => {}
            _ => {}
          }
        }
        glfw::WindowEvent::MouseButton(_btn, _action, _mods) => {
          // log!("INFO", "Button: {:?}, Action: {:?}, Modifiers: [{:?}]", btn, action, mods);
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
          log!("INFO", "[Window] -->\t Framebuffer size: ({0}, {1})", width, height);
          let renderer = Renderer::get()
            .expect("Cannot process window event (Framebuffer size) : No active renderer context!");
          
          unsafe {
            (*renderer).on_events(glfw::WindowEvent::FramebufferSize(width, height))
              .expect("Error while processing events for renderer!");
            
            S_PREVIOUS_WIDTH = self.m_window_resolution.0 as u32;
            S_PREVIOUS_HEIGHT = self.m_window_resolution.1 as u32;
          }
          self.m_window_resolution = (width, height);
        }
        glfw::WindowEvent::Pos(pos_x, pos_y) => {
          if self.m_is_windowed {
            self.m_window_pos = (pos_x, pos_y);
          }
        }
        _ => {}
      };
    };
    if window_event_happened {
      Input::on_update();
      return Ok(true);
    }
    return Ok(false);
  }
  
  pub fn on_delete(&mut self) -> Result<(), EnumError> {
    if self.m_state == EnumState::Closed {
      return Ok(());
    }
    self.m_state = EnumState::Closed;
    unsafe { S_WINDOW_CONTEXT = None };
    return Ok(());
  }
  
  pub fn refresh(&mut self) {
    if self.m_render_api == EnumApi::OpenGL {
      self.m_api_window.swap_buffers();
    }
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
  
  pub fn get_api_ref(&self) -> &glfw::Glfw {
    return &self.m_api_window.glfw;
  }
  
  pub fn get_api_mut(&mut self) -> &mut glfw::Glfw {
    return &mut self.m_api_window.glfw;
  }
  
  pub fn set_title(&mut self, title: &str) {
    return self.m_api_window.set_title(title);
  }
  
  pub fn toggle_vsync(&mut self) {
    self.m_vsync = !self.m_vsync;
    self.m_api_window.glfw.set_swap_interval(glfw::SwapInterval::Sync(self.m_vsync as u32));
    log!(EnumLogColor::Blue, "INFO", "[Window] -->\t VSync {0}", self.m_vsync);
  }
  
  pub fn toggle_fullscreen(&mut self) -> Result<(), EnumError> {
    unsafe {
      if S_WINDOW_CONTEXT.is_none() {
        log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Cannot toggle fullscreen : \
        No active window context!");
        return Err(EnumError::NoContextError);
      };
      
      (*S_WINDOW_CONTEXT.unwrap()).with_primary_monitor(|_, monitor| {
        let mode: glfw::VidMode = monitor.as_ref().unwrap().get_video_mode().unwrap();
        
        if !self.m_is_windowed {
          self.m_api_window.set_resizable(true);
          
          if self.m_window_mode == EnumWindowMode::Borderless {
            self.m_api_window.set_decorated(true);
            self.m_api_window.set_size(S_PREVIOUS_WIDTH as i32, S_PREVIOUS_HEIGHT as i32);
          } else {
            self.m_api_window.set_monitor(glfw::WindowMode::Windowed,
              self.m_window_pos.0, self.m_window_pos.1,
              S_PREVIOUS_WIDTH, S_PREVIOUS_HEIGHT, None);
          }
          log!("INFO", "[Window] -->\t Window mode : Windowed");
        } else {
          match self.m_window_mode {
            EnumWindowMode::Borderless => {
              self.m_api_window.set_decorated(false);
              self.m_api_window.set_size(mode.width as i32, mode.height as i32);
              log!("INFO", "[Window] -->\t Window mode : Borderless");
            }
            _ => {
              self.m_api_window.set_resizable(false);
              self.m_api_window.set_monitor(glfw::WindowMode::FullScreen(monitor.unwrap()),
                self.m_window_pos.0, self.m_window_pos.1, mode.width, mode.height,
                Some(mode.refresh_rate));
              log!("INFO", "[Window] -->\t Window mode : Fullscreen");
            }
          }
        }
        self.m_is_windowed = !self.m_is_windowed;
      });
    }
    return Ok(());
  }
  
  pub fn get_framebuffer_size(&mut self) -> (u32, u32) {
    if self.m_window_mode != EnumWindowMode::Windowed {
      return self.m_api_window.glfw.with_primary_monitor(|_, primary_monitor| {
        let mode: glfw::VidMode = primary_monitor.as_ref().unwrap().get_video_mode().unwrap();
        return (mode.width, mode.height);
      });
    }
    return (self.m_window_resolution.0 as u32, self.m_window_resolution.1 as u32);
  }
  
  pub fn get() -> Option<*mut Window> {
    unsafe { return S_WINDOW; };
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    unsafe {
      if S_WINDOW_CONTEXT.is_some() {
        log!(EnumLogColor::Purple, "INFO", "[Window] -->\t Dropping window...");
        match self.on_delete() {
          Ok(_) => {
            log!(EnumLogColor::Green, "INFO", "[Window] -->\t Dropped window successfully");
          }
          #[allow(unused)]
          Err(err) => {
            log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Error while dropping window : Error => {:?}",
        err);
            log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Dropped window unsuccessfully");
          }
        }
      }
      S_WINDOW_CONTEXT = None;
    }
  }
}