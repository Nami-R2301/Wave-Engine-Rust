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
use std::path::PathBuf;

#[cfg(feature = "vulkan")]
use glfw::Context;

use crate::utils::macros::logger::*;
use crate::{Engine, TraitApply, TraitFree, TraitHint};
use crate::events::{EnumEvent, EnumEventMask};
use crate::graphics::renderer::EnumRendererApi;
use crate::input::{self, EnumAction, EnumKey, EnumModifiers, EnumMouseButton};
use crate::utils::Time;

pub(crate) static mut S_WINDOW_CONTEXT: Option<glfw::Glfw> = None;

pub(crate) static mut S_PREVIOUS_WIDTH: u32 = 640;
pub(crate) static mut S_PREVIOUS_HEIGHT: u32 = 480;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumWindowState {
  ContextReady,
  Created,
  Visible,
  Hidden,
  Closed,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EnumWindowHint {
  WindowMode(EnumWindowMode),
  Resolution(u32, u32),
  Visible(bool),
  Resizable(bool),
  Position(u32, u32),
  Focused(bool),
  Maximized(bool),
  Decorated(bool),
  VSync(bool),
  MSAA(Option<u32>),
  DebugApi(bool),
  RefreshRate(Option<u32>),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EnumWindowMode {
  Windowed,
  Borderless,
  Fullscreen,
}

impl Default for EnumWindowMode {
  fn default() -> Self {
    return EnumWindowMode::Windowed;
  }
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
pub enum EnumWindowError {
  NoContext,
  InitError,
  InvalidWindowOption,
  ApiError,
  InvalidEventMask,
  InvalidEventCallback,
  AlreadyInitializedError,
  VulkanIncompatibleError,
  VulkanSurfaceCreationError,
  WindowInputError,
}

impl Display for EnumWindowError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Window] -->\t Error encountered with window context : {:?}", self)
  }
}

impl From<input::EnumInputError> for EnumWindowError {
  #[allow(unused)]
  fn from(input_error: input::EnumInputError) -> Self {
    log!(EnumLogColor::Red, "ERROR", "{0}", input_error);
    return EnumWindowError::WindowInputError;
  }
}

#[cfg(feature = "debug")]
fn glfw_error_callback(error: glfw::Error, message: String) {
  log!(EnumLogColor::Red, "ERROR", "[Window] -->\t GLFW error raised! Error => {0}\n{1:100}Info => \
   {2}", error, "", message);
}

pub struct Window {
  pub(crate) m_state: EnumWindowState,
  pub(crate) m_api_window_events: Option<glfw::GlfwReceiver<(f64, glfw::WindowEvent)>>,
  pub(crate) m_api_window: Option<glfw::PWindow>,
  pub(crate) m_vsync: bool,
  pub(crate) m_refresh_count_desired: Option<u32>,
  pub(crate) m_samples: u32,
  pub(crate) m_window_resolution: Option<(u32, u32)>,
  pub(crate) m_window_pos: (i32, i32),
  pub(crate) m_is_windowed: bool,
  m_window_mode: EnumWindowMode,
  m_render_api: EnumRendererApi,
}

impl Default for Window {
  fn default() -> Self {
    let mut result = glfw::init(glfw::fail_on_errors);
    
    match result {
      Err(glfw::InitError::AlreadyInitialized) => {
        log!(EnumLogColor::Yellow, "WARN",
          "[Window] -->\t GLFW window already initialized! Skipping creation of a new one...");
      }
      Err(glfw::InitError::Internal) => {
        log!(EnumLogColor::Red, "ERROR",
          "[Window] -->\t Failed to create GLFW window due to internal error! Exiting...");
        panic!("[Window] -->\t Cannot init glfw library for window context, Error => {0}", glfw::InitError::Internal)
      }
      Ok(_) => {}
    }
    
    let context_ref = result.as_mut().unwrap();
    
    // Set default window behavior.
    context_ref.window_hint(glfw::WindowHint::Visible(false));
    context_ref.window_hint(glfw::WindowHint::Decorated(true));
    context_ref.window_hint(glfw::WindowHint::Maximized(true));
    context_ref.window_hint(glfw::WindowHint::Resizable(true));
    context_ref.window_hint(glfw::WindowHint::RefreshRate(None));
    context_ref.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGl));
    
    unsafe { S_WINDOW_CONTEXT = Some(result.unwrap()); }
    
    return Self {
      m_api_window_events: None,
      m_api_window: None,
      m_vsync: true,
      m_refresh_count_desired: None,
      m_samples: 1,
      m_window_resolution: None,
      m_window_pos: (0, 0),
      m_is_windowed: true,
      m_window_mode: EnumWindowMode::default(),  // Default to Fullscreen.
      m_render_api: EnumRendererApi::default(),
      m_state: EnumWindowState::ContextReady,
    };
  }
}

impl TraitHint<EnumWindowHint> for Window {
  fn set_hint(&mut self, hint: EnumWindowHint) {
    match hint {
      EnumWindowHint::WindowMode(window_mode) => {
        self.m_window_mode = window_mode;
        self.m_is_windowed = window_mode == EnumWindowMode::Windowed;
      }
      EnumWindowHint::Resolution(x_res, y_res) => {
        self.m_window_resolution = Some((x_res, y_res));
      }
      EnumWindowHint::Visible(flag) => unsafe {
        (*S_WINDOW_CONTEXT.as_mut().unwrap()).window_hint(glfw::WindowHint::Visible(flag));
      }
      EnumWindowHint::Resizable(flag) => unsafe {
        if self.m_window_mode == EnumWindowMode::Borderless {
          (*S_WINDOW_CONTEXT.as_mut().unwrap()).window_hint(glfw::WindowHint::Resizable(false));
          (*S_WINDOW_CONTEXT.as_mut().unwrap()).window_hint(glfw::WindowHint::Decorated(false));
        } else if self.m_window_mode == EnumWindowMode::Fullscreen {
          (*S_WINDOW_CONTEXT.as_mut().unwrap()).window_hint(glfw::WindowHint::Resizable(false));
        } else {
          (*S_WINDOW_CONTEXT.as_mut().unwrap()).window_hint(glfw::WindowHint::Resizable(flag));
        }
      }
      EnumWindowHint::Position(x_pos, y_pos) => {
        self.m_window_pos = (x_pos as i32, y_pos as i32);
      }
      EnumWindowHint::Focused(flag) => unsafe {
        (*S_WINDOW_CONTEXT.as_mut().unwrap()).window_hint(glfw::WindowHint::Focused(flag));
      }
      EnumWindowHint::Maximized(flag) => unsafe {
        (*S_WINDOW_CONTEXT.as_mut().unwrap()).window_hint(glfw::WindowHint::Maximized(flag));
      }
      EnumWindowHint::Decorated(flag) => unsafe {
        (*S_WINDOW_CONTEXT.as_mut().unwrap()).window_hint(glfw::WindowHint::Decorated(flag));
      }
      EnumWindowHint::VSync(flag) => {
        self.m_vsync = flag;
      }
      EnumWindowHint::MSAA(sample_rate_desired) => unsafe {
        (*S_WINDOW_CONTEXT.as_mut().unwrap()).window_hint(glfw::WindowHint::Samples(sample_rate_desired));
        self.m_samples = sample_rate_desired.unwrap_or(1);
      }
      EnumWindowHint::DebugApi(flag) => unsafe {
        (*S_WINDOW_CONTEXT.as_mut().unwrap()).window_hint(glfw::WindowHint::OpenGlDebugContext(flag));
        // Set GLFW error callback.
        #[cfg(feature = "debug")]
        (*S_WINDOW_CONTEXT.as_mut().unwrap()).set_error_callback(glfw_error_callback);
      }
      EnumWindowHint::RefreshRate(refresh_count_desired) => unsafe {
        (*S_WINDOW_CONTEXT.as_mut().unwrap()).window_hint(glfw::WindowHint::RefreshRate(refresh_count_desired));
        self.m_refresh_count_desired = refresh_count_desired;
      }
    }
  }
  
  fn reset_hints(&mut self) {
    let context_ref = unsafe { &mut *S_WINDOW_CONTEXT.as_mut().unwrap() };
    
    // Set default window behavior.
    context_ref.window_hint(glfw::WindowHint::Visible(false));
    context_ref.window_hint(glfw::WindowHint::Decorated(true));
    context_ref.window_hint(glfw::WindowHint::Maximized(true));
    context_ref.window_hint(glfw::WindowHint::Resizable(true));
    context_ref.window_hint(glfw::WindowHint::RefreshRate(None));
    context_ref.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGl));
    context_ref.window_hint(glfw::WindowHint::OpenGlDebugContext(false));
    
    self.m_vsync = true;
    self.m_render_api = EnumRendererApi::default();
    self.m_window_resolution = None;
    self.m_window_mode = EnumWindowMode::default();
    self.m_refresh_count_desired = None;
    self.m_window_pos = (0, 0);
    self.m_is_windowed = false;
  }
}

impl TraitApply<EnumWindowError> for Window {
  fn apply(&mut self) -> Result<(), EnumWindowError> {
    unsafe {
      (*S_WINDOW_CONTEXT.as_mut().unwrap()).with_primary_monitor(|_, monitor| -> Result<(), EnumWindowError> {
        let primary_monitor = monitor.expect("Cannot apply window context, cannot retrieve primary monitor!");
        let vid_mode = primary_monitor.get_video_mode()
          .expect("Cannot apply window context, cannot retrieve video mode of primary monitor!");
        
        match (*S_WINDOW_CONTEXT.as_mut().unwrap()).create_window(vid_mode.width, vid_mode.height,
          "Wave Engine (Rust)",
          match self.m_window_mode {
            EnumWindowMode::Fullscreen => glfw::WindowMode::FullScreen(&primary_monitor),
            EnumWindowMode::Windowed => glfw::WindowMode::Windowed,
            EnumWindowMode::Borderless => glfw::WindowMode::Windowed
          }) {
          None => {
            log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Unable to create GLFW window");
            return Err(EnumWindowError::InitError);
          }
          Some((mut window, events)) => {
            
            // Set input polling rate.
            window.set_sticky_keys(true);
            window.set_sticky_mouse_buttons(true);
            
            let bounds = window.get_size();
            S_PREVIOUS_WIDTH = bounds.0 as u32;
            S_PREVIOUS_HEIGHT = bounds.1 as u32;
            window.set_aspect_ratio(bounds.0 as u32, bounds.1 as u32);
            
            self.m_state = EnumWindowState::Created;
            self.m_window_pos = window.get_pos();
            self.m_is_windowed = self.m_window_mode != EnumWindowMode::Fullscreen;
            self.m_api_window = Some(window);
            self.m_api_window_events = Some(events);
            self.m_window_resolution = Some((bounds.0 as u32, bounds.1 as u32));
          }
        };
        return Ok(());
      }).map_err(|err| return err)?
    }
    // Toggle on fullscreen if requested.
    if self.m_window_mode != EnumWindowMode::Windowed {
      unsafe {
        (*S_WINDOW_CONTEXT.as_mut().unwrap()).with_primary_monitor(|_, monitor| {
          if monitor.is_none() {
            log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Cannot identify primary monitor!");
            return;
          }
          
          let mode: glfw::VidMode = monitor.as_ref().unwrap().get_video_mode().unwrap();
          
          match self.m_window_mode {
            EnumWindowMode::Windowed => {}
            EnumWindowMode::Borderless => {
              self.m_api_window.as_mut().unwrap().set_monitor(glfw::WindowMode::Windowed,
                self.m_window_pos.0, self.m_window_pos.1, mode.width, mode.height, self.m_refresh_count_desired);
            }
            EnumWindowMode::Fullscreen => {
              self.m_api_window.as_mut().unwrap().set_monitor(glfw::WindowMode::FullScreen(monitor.unwrap()),
                self.m_window_pos.0, self.m_window_pos.1, mode.width, mode.height, Some(mode.refresh_rate));
            }
          }
        });
      }
    }
    return Ok(());
  }
}

impl TraitFree<EnumWindowError> for Window {
  fn free(&mut self) -> Result<(), EnumWindowError> {
    if self.m_state == EnumWindowState::Closed {
      return Ok(());
    }
    self.m_state = EnumWindowState::Closed;
    unsafe { S_WINDOW_CONTEXT = None };
    return Ok(());
  }
}

impl<'a> Window {
  pub fn new(context_api_chosen: EnumRendererApi) -> Self {
    let result = glfw::init(glfw::fail_on_errors);
    
    match result {
      Err(glfw::InitError::AlreadyInitialized) => {
        log!(EnumLogColor::Yellow, "WARN",
          "[Window] -->\t GLFW window already initialized! Skipping creation of a new one...");
      }
      Err(glfw::InitError::Internal) => {
        log!(EnumLogColor::Red, "ERROR",
          "[Window] -->\t Failed to create GLFW window due to internal error! Exiting...");
        panic!("[Window] -->\t Cannot init glfw library for window context, Error => {0}", glfw::InitError::Internal)
      }
      Ok(mut context_ref) => {
        // Set default window behavior.
        context_ref.window_hint(glfw::WindowHint::Visible(false));
        context_ref.window_hint(glfw::WindowHint::Decorated(true));
        context_ref.window_hint(glfw::WindowHint::Maximized(true));
        context_ref.window_hint(glfw::WindowHint::Resizable(true));
        context_ref.window_hint(glfw::WindowHint::RefreshRate(None));
        context_ref.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        unsafe { S_WINDOW_CONTEXT = Some(context_ref); }
      }
    }
    
    return Self {
      m_api_window_events: None,
      m_api_window: None,
      m_vsync: true,
      m_refresh_count_desired: None,
      m_samples: 1,
      m_window_resolution: None,
      m_window_pos: (0, 0),
      m_is_windowed: true,
      m_window_mode: EnumWindowMode::default(),
      m_render_api: context_api_chosen,
      m_state: EnumWindowState::ContextReady,
    };
  }
  
  pub fn is_applied(&self) -> bool {
    return self.m_api_window.is_some();
  }
  
  pub fn show(&mut self) {
    self.m_api_window.as_mut().unwrap().show();
    self.m_state = EnumWindowState::Visible;
  }
  
  pub fn hide(&mut self) {
    self.m_api_window.as_mut().unwrap().hide();
    self.m_state = EnumWindowState::Hidden;
  }
  
  pub fn init_opengl_surface(&mut self) {
    // Make the window's context current
    self.m_api_window.as_mut().unwrap().make_current();
    
    // Set v-sync.
    self.m_api_window.as_mut().unwrap().glfw.set_swap_interval(glfw::SwapInterval::Sync(self.m_vsync
      .then(|| { return 1; })
      .unwrap_or(0)));
  }
  
  #[cfg(feature = "vulkan")]
  pub fn init_vulkan_surface(&self, vk_instance: &ash::Instance, vk_surface_khr: &mut ash::vk::SurfaceKHR) {
    let result = self.m_api_window.as_ref().unwrap().create_window_surface(vk_instance.handle(),
      std::ptr::null_mut(), vk_surface_khr).result();
    
    if result.is_err() {
      log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Failed to create Vulkan surface!");
      panic!();
    }
  }
  
  pub fn on_update(&mut self) -> Result<(), EnumWindowError> {
    return Ok(());
  }
  
  pub fn poll_events(&mut self) {
    self.m_api_window.as_mut().unwrap().glfw.poll_events();
  }
  
  pub fn on_event(&mut self, event: &EnumEvent) -> bool {
    return match event {
      EnumEvent::KeyEvent(key, action, _repeat_count, modifiers) => {
        return match (key, action, modifiers) {
          (EnumKey::Escape, EnumAction::Pressed, _) => {
            self.close();
            log!(EnumLogColor::Yellow, "EVENT", "[Window] -->\t User requested to close the window");
            true
          }
          (EnumKey::Enter, EnumAction::Pressed, &EnumModifiers::Alt) => {
            self.toggle_fullscreen();
            true
          }
          (EnumKey::V, EnumAction::Pressed, &EnumModifiers::Alt) => {
            self.toggle_vsync();
            true
          }
          _ => false
        };
      }
      EnumEvent::FramebufferEvent(width, height) => {
        log!(EnumLogColor::Blue, "EVENT", "[Window] -->\t Framebuffer resized to: ({0}, {1})", width, height);
        unsafe {
          S_PREVIOUS_WIDTH = self.m_window_resolution.unwrap().0;
          S_PREVIOUS_HEIGHT = self.m_window_resolution.unwrap().1;
        }
        self.m_window_resolution = Some((*width, *height));
        true
      }
      EnumEvent::WindowCloseEvent(_time) => {
        match self.free() {
          Err(_err) => {
            log!(EnumLogColor::Red, "EVENT", "[Window] -->\t Error while freeing resources during close event, Error => {0}", _err);
          }
          _ => {}
        }
        self.m_state = EnumWindowState::Closed;
        true
      }
      EnumEvent::WindowPosEvent(pos_x, pos_y) => {
        if self.m_is_windowed {
          self.m_window_pos = (*pos_x, *pos_y);
        }
        true
      }
      _ => false
    };
  }
  
  pub fn enable_polling_for(&mut self, event_mask: EnumEventMask) {
    if event_mask.contains(EnumEventMask::Window) {
      self.m_api_window.as_mut().unwrap().set_close_polling(true);
      self.m_api_window.as_mut().unwrap().set_iconify_polling(true);
      self.m_api_window.as_mut().unwrap().set_maximize_polling(true);
      self.m_api_window.as_mut().unwrap().set_focus_polling(true);
      self.m_api_window.as_mut().unwrap().set_framebuffer_size_polling(true);
      self.m_api_window.as_mut().unwrap().set_pos_polling(true);
    }
    if event_mask.contains(EnumEventMask::Input) {
      self.m_api_window.as_mut().unwrap().set_key_polling(true);
      self.m_api_window.as_mut().unwrap().set_mouse_button_polling(true);
      self.m_api_window.as_mut().unwrap().set_scroll_polling(true);
      self.m_api_window.as_mut().unwrap().set_drag_and_drop_polling(true);
    }
    if event_mask.contains(EnumEventMask::WindowClose) {
      self.m_api_window.as_mut().unwrap().set_close_polling(true);
    }
    if event_mask.contains(EnumEventMask::WindowIconify) {
      self.m_api_window.as_mut().unwrap().set_iconify_polling(true);
    }
    if event_mask.contains(EnumEventMask::WindowMaximize) {
      self.m_api_window.as_mut().unwrap().set_maximize_polling(true);
    }
    if event_mask.contains(EnumEventMask::WindowFocus) {
      self.m_api_window.as_mut().unwrap().set_focus_polling(true);
    }
    if event_mask.contains(EnumEventMask::WindowSize) {
      self.m_api_window.as_mut().unwrap().set_framebuffer_size_polling(true);
    }
    if event_mask.contains(EnumEventMask::WindowPos) {
      self.m_api_window.as_mut().unwrap().set_pos_polling(true);
    }
    if event_mask.contains(EnumEventMask::Keyboard) {
      self.m_api_window.as_mut().unwrap().set_key_polling(true);
    }
    if event_mask.contains(EnumEventMask::Mouse) {
      self.m_api_window.as_mut().unwrap().set_mouse_button_polling(true);
      self.m_api_window.as_mut().unwrap().set_scroll_polling(true);
    }
    if event_mask.contains(EnumEventMask::MouseBtn) {
      self.m_api_window.as_mut().unwrap().set_mouse_button_polling(true);
    }
    if event_mask.contains(EnumEventMask::MouseScroll) {
      self.m_api_window.as_mut().unwrap().set_scroll_polling(true);
    }
    if event_mask.contains(EnumEventMask::DragAndDrop) {
      self.m_api_window.as_mut().unwrap().set_drag_and_drop_polling(true);
    }
  }
  
  pub fn disable_polling(&mut self, event_mask: EnumEventMask) {
    if event_mask.contains(EnumEventMask::Window) {
      self.m_api_window.as_mut().unwrap().unset_close_callback();
      self.m_api_window.as_mut().unwrap().set_close_polling(false);
      self.m_api_window.as_mut().unwrap().unset_iconify_callback();
      self.m_api_window.as_mut().unwrap().set_iconify_polling(false);
      self.m_api_window.as_mut().unwrap().unset_maximize_callback();
      self.m_api_window.as_mut().unwrap().set_maximize_polling(false);
      self.m_api_window.as_mut().unwrap().unset_focus_callback();
      self.m_api_window.as_mut().unwrap().set_focus_polling(false);
      self.m_api_window.as_mut().unwrap().unset_framebuffer_size_callback();
      self.m_api_window.as_mut().unwrap().set_framebuffer_size_polling(false);
      self.m_api_window.as_mut().unwrap().unset_pos_callback();
      self.m_api_window.as_mut().unwrap().set_pos_polling(false);
    }
    if event_mask.contains(EnumEventMask::Input) {
      self.m_api_window.as_mut().unwrap().unset_key_callback();
      self.m_api_window.as_mut().unwrap().set_key_polling(false);
      self.m_api_window.as_mut().unwrap().unset_mouse_button_callback();
      self.m_api_window.as_mut().unwrap().set_mouse_button_polling(false);
      self.m_api_window.as_mut().unwrap().unset_scroll_callback();
      self.m_api_window.as_mut().unwrap().set_scroll_polling(false);
      self.m_api_window.as_mut().unwrap().unset_drag_and_drop_callback();
      self.m_api_window.as_mut().unwrap().set_drag_and_drop_polling(false);
    }
    if event_mask.contains(EnumEventMask::WindowClose) {
      self.m_api_window.as_mut().unwrap().unset_close_callback();
      self.m_api_window.as_mut().unwrap().set_close_polling(false);
    }
    if event_mask.contains(EnumEventMask::WindowIconify) {
      self.m_api_window.as_mut().unwrap().unset_iconify_callback();
      self.m_api_window.as_mut().unwrap().set_iconify_polling(false);
    }
    if event_mask.contains(EnumEventMask::WindowMaximize) {
      self.m_api_window.as_mut().unwrap().unset_maximize_callback();
      self.m_api_window.as_mut().unwrap().set_maximize_polling(false);
    }
    if event_mask.contains(EnumEventMask::WindowFocus) {
      self.m_api_window.as_mut().unwrap().unset_focus_callback();
      self.m_api_window.as_mut().unwrap().set_focus_polling(false);
    }
    if event_mask.contains(EnumEventMask::WindowSize) {
      self.m_api_window.as_mut().unwrap().unset_framebuffer_size_callback();
      self.m_api_window.as_mut().unwrap().set_framebuffer_size_polling(false);
    }
    if event_mask.contains(EnumEventMask::WindowPos) {
      self.m_api_window.as_mut().unwrap().unset_pos_callback();
      self.m_api_window.as_mut().unwrap().set_pos_polling(false);
    }
    if event_mask.contains(EnumEventMask::Keyboard) {
      self.m_api_window.as_mut().unwrap().unset_key_callback();
      self.m_api_window.as_mut().unwrap().set_key_polling(false);
    }
    if event_mask.contains(EnumEventMask::Mouse) {
      self.m_api_window.as_mut().unwrap().unset_mouse_button_callback();
      self.m_api_window.as_mut().unwrap().set_mouse_button_polling(false);
      self.m_api_window.as_mut().unwrap().unset_scroll_callback();
      self.m_api_window.as_mut().unwrap().set_scroll_polling(false);
    }
    if event_mask.contains(EnumEventMask::MouseBtn) {
      self.m_api_window.as_mut().unwrap().unset_mouse_button_callback();
      self.m_api_window.as_mut().unwrap().set_mouse_button_polling(false);
    }
    if event_mask.contains(EnumEventMask::MouseScroll) {
      self.m_api_window.as_mut().unwrap().unset_scroll_callback();
      self.m_api_window.as_mut().unwrap().set_scroll_polling(false);
    }
    if event_mask.contains(EnumEventMask::DragAndDrop) {
      self.m_api_window.as_mut().unwrap().unset_drag_and_drop_callback();
      self.m_api_window.as_mut().unwrap().set_drag_and_drop_polling(false);
    }
  }
  
  pub fn enable_callback_for(&mut self, event_mask: EnumEventMask) {
    if event_mask.contains(EnumEventMask::Window) {
      self.m_api_window.as_mut().unwrap().set_close_callback(Self::window_close_callback);
      self.m_api_window.as_mut().unwrap().set_iconify_callback(Self::window_iconify_callback);
      self.m_api_window.as_mut().unwrap().set_maximize_callback(Self::window_maximize_callback);
      self.m_api_window.as_mut().unwrap().set_focus_callback(Self::window_focus_callback);
      self.m_api_window.as_mut().unwrap().set_framebuffer_size_callback(Self::window_size_callback);
      self.m_api_window.as_mut().unwrap().set_pos_callback(Self::window_pos_callback);
    }
    if event_mask.contains(EnumEventMask::Input) {
      self.m_api_window.as_mut().unwrap().set_key_callback(Self::key_callback);
      self.m_api_window.as_mut().unwrap().set_mouse_button_callback(Self::mouse_btn_callback);
      self.m_api_window.as_mut().unwrap().set_scroll_callback(Self::scroll_callback);
      self.m_api_window.as_mut().unwrap().set_drag_and_drop_callback(Self::drag_and_drop_callback);
    }
    if event_mask.contains(EnumEventMask::WindowClose) {
      self.m_api_window.as_mut().unwrap().set_close_callback(Self::window_close_callback);
    }
    if event_mask.contains(EnumEventMask::WindowIconify) {
      self.m_api_window.as_mut().unwrap().set_iconify_callback(Self::window_iconify_callback);
    }
    if event_mask.contains(EnumEventMask::WindowMaximize) {
      self.m_api_window.as_mut().unwrap().set_maximize_callback(Self::window_maximize_callback);
    }
    if event_mask.contains(EnumEventMask::WindowFocus) {
      self.m_api_window.as_mut().unwrap().set_focus_callback(Self::window_focus_callback);
    }
    if event_mask.contains(EnumEventMask::WindowSize) {
      self.m_api_window.as_mut().unwrap().set_framebuffer_size_callback(Self::window_size_callback);
    }
    if event_mask.contains(EnumEventMask::WindowPos) {
      self.m_api_window.as_mut().unwrap().set_pos_callback(Self::window_pos_callback);
    }
    if event_mask.contains(EnumEventMask::Keyboard) {
      self.m_api_window.as_mut().unwrap().set_key_callback(Self::key_callback);
    }
    if event_mask.contains(EnumEventMask::Mouse) {
      self.m_api_window.as_mut().unwrap().set_mouse_button_callback(Self::mouse_btn_callback);
      self.m_api_window.as_mut().unwrap().set_scroll_callback(Self::scroll_callback);
    }
    if event_mask.contains(EnumEventMask::MouseBtn) {
      self.m_api_window.as_mut().unwrap().set_mouse_button_callback(Self::mouse_btn_callback);
    }
    if event_mask.contains(EnumEventMask::MouseScroll) {
      self.m_api_window.as_mut().unwrap().set_scroll_callback(Self::scroll_callback);
    }
    if event_mask.contains(EnumEventMask::DragAndDrop) {
      self.m_api_window.as_mut().unwrap().set_drag_and_drop_callback(Self::drag_and_drop_callback);
    }
  }
  
  pub fn disable_callback_for(&mut self, event_mask: EnumEventMask) {
    if event_mask.contains(EnumEventMask::Window) {
      self.m_api_window.as_mut().unwrap().unset_close_callback();
      self.m_api_window.as_mut().unwrap().unset_iconify_callback();
      self.m_api_window.as_mut().unwrap().unset_maximize_callback();
      self.m_api_window.as_mut().unwrap().unset_focus_callback();
      self.m_api_window.as_mut().unwrap().unset_framebuffer_size_callback();
      self.m_api_window.as_mut().unwrap().unset_pos_callback();
    }
    if event_mask.contains(EnumEventMask::Input) {
      self.m_api_window.as_mut().unwrap().unset_key_callback();
      self.m_api_window.as_mut().unwrap().unset_mouse_button_callback();
      self.m_api_window.as_mut().unwrap().unset_scroll_callback();
      self.m_api_window.as_mut().unwrap().unset_drag_and_drop_callback();
    }
    if event_mask.contains(EnumEventMask::WindowClose) {
      self.m_api_window.as_mut().unwrap().unset_close_callback();
    }
    if event_mask.contains(EnumEventMask::WindowIconify) {
      self.m_api_window.as_mut().unwrap().unset_iconify_callback();
    }
    if event_mask.contains(EnumEventMask::WindowMaximize) {
      self.m_api_window.as_mut().unwrap().unset_maximize_callback();
    }
    if event_mask.contains(EnumEventMask::WindowFocus) {
      self.m_api_window.as_mut().unwrap().unset_focus_callback();
    }
    if event_mask.contains(EnumEventMask::WindowSize) {
      self.m_api_window.as_mut().unwrap().unset_framebuffer_size_callback();
    }
    if event_mask.contains(EnumEventMask::WindowPos) {
      self.m_api_window.as_mut().unwrap().unset_pos_callback();
    }
    if event_mask.contains(EnumEventMask::Keyboard) {
      self.m_api_window.as_mut().unwrap().unset_key_callback();
    }
    if event_mask.contains(EnumEventMask::Mouse) {
      self.m_api_window.as_mut().unwrap().unset_mouse_button_callback();
      self.m_api_window.as_mut().unwrap().unset_scroll_callback();
    }
    if event_mask.contains(EnumEventMask::MouseBtn) {
      self.m_api_window.as_mut().unwrap().unset_mouse_button_callback();
    }
    if event_mask.contains(EnumEventMask::MouseScroll) {
      self.m_api_window.as_mut().unwrap().unset_scroll_callback();
    }
    if event_mask.contains(EnumEventMask::DragAndDrop) {
      self.m_api_window.as_mut().unwrap().unset_drag_and_drop_callback();
    }
  }
  
  pub fn refresh(&mut self) {
    if self.m_render_api == EnumRendererApi::OpenGL {
      self.m_api_window.as_mut().unwrap().swap_buffers();
    }
  }
  
  pub fn is_closing(&self) -> bool {
    return self.m_api_window.as_ref().unwrap().should_close();
  }
  
  pub fn is_closed(&self) -> bool {
    return self.m_state == EnumWindowState::Closed;
  }
  
  pub fn close(&mut self) {
    self.m_api_window.as_mut().unwrap().set_should_close(true);
    self.m_state = EnumWindowState::Closed;
  }
  
  pub fn get_aspect_ratio(&self) -> f32 {
    return self.m_window_resolution.unwrap().0 as f32 / self.m_window_resolution.unwrap().1 as f32;
  }
  
  pub fn get_state(&self) -> EnumWindowState {
    return self.m_state;
  }
  
  pub fn get_api_ref(&self) -> &glfw::Glfw {
    return unsafe { &*S_WINDOW_CONTEXT.as_ref().unwrap() };
  }
  
  pub fn get_api_mut(&mut self) -> &mut glfw::Glfw {
    return unsafe { &mut *S_WINDOW_CONTEXT.as_mut().unwrap() };
  }
  
  pub fn set_title(&mut self, title: &str) {
    return self.m_api_window.as_mut().unwrap().set_title(title);
  }
  
  pub fn toggle_vsync(&mut self) {
    self.m_vsync = !self.m_vsync;
    self.m_api_window.as_mut().unwrap().glfw.set_swap_interval(glfw::SwapInterval::Sync(self.m_vsync as u32));
    log!(EnumLogColor::Blue, "EVENT", "[Window] -->\t VSync {0}", self.m_vsync);
  }
  
  pub fn toggle_fullscreen(&mut self) {
    unsafe {
      if S_WINDOW_CONTEXT.is_none() {
        log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Cannot toggle fullscreen : \
        No active window context!");
        panic!("[Window] -->\t Cannot toggle fullscreen : No active window context!");
      };
      
      (*S_WINDOW_CONTEXT.as_mut().unwrap()).with_primary_monitor(|_, monitor| {
        let mode: glfw::VidMode = monitor.as_ref().unwrap().get_video_mode().unwrap();
        
        if !self.m_is_windowed {
          self.m_api_window.as_mut().unwrap().set_resizable(true);
          
          if self.m_window_mode == EnumWindowMode::Borderless {
            self.m_api_window.as_mut().unwrap().set_decorated(true);
            self.m_api_window.as_mut().unwrap().set_size(S_PREVIOUS_WIDTH as i32, S_PREVIOUS_HEIGHT as i32);
          } else {
            self.m_api_window.as_mut().unwrap().set_decorated(true);
            self.m_api_window.as_mut().unwrap().set_resizable(true);
            self.m_api_window.as_mut().unwrap().set_monitor(glfw::WindowMode::Windowed,
              self.m_window_pos.0, self.m_window_pos.1, S_PREVIOUS_WIDTH, S_PREVIOUS_HEIGHT, None);
          }
          log!(EnumLogColor::Blue, "EVENT", "[Window] -->\t Window mode : Windowed");
        } else {
          match self.m_window_mode {
            EnumWindowMode::Borderless => {
              self.m_api_window.as_mut().unwrap().set_decorated(false);
              self.m_api_window.as_mut().unwrap().set_size(mode.width as i32, mode.height as i32);
              log!(EnumLogColor::Blue, "EVENT", "[Window] -->\t Window mode : Borderless");
            }
            _ => {
              self.m_api_window.as_mut().unwrap().set_resizable(false);
              self.m_api_window.as_mut().unwrap().set_monitor(glfw::WindowMode::FullScreen(monitor.unwrap()),
                self.m_window_pos.0, self.m_window_pos.1, mode.width, mode.height,
                Some(mode.refresh_rate));
              log!(EnumLogColor::Blue, "EVENT", "[Window] -->\t Window mode : Fullscreen");
            }
          }
        }
        self.m_is_windowed = !self.m_is_windowed;
      });
    }
  }
  
  pub fn get_framebuffer_size(&mut self) -> (u32, u32) {
    if self.m_window_mode != EnumWindowMode::Windowed {
      return self.m_api_window.as_mut().unwrap().glfw.with_primary_monitor(|_, primary_monitor| {
        let mode: glfw::VidMode = primary_monitor.as_ref().unwrap().get_video_mode().unwrap();
        return (mode.width, mode.height);
      });
    }
    return (self.m_window_resolution.unwrap().0, self.m_window_resolution.unwrap().1);
  }
  
  pub fn window_close_callback(_window: &mut glfw::Window) {
    Engine::on_async_event(&EnumEvent::WindowCloseEvent(Time::now()));
  }
  
  pub fn window_iconify_callback(_window: &mut glfw::Window, flag: bool) {
    Engine::on_async_event(&EnumEvent::WindowIconifyEvent(flag));
  }
  
  pub fn window_focus_callback(_window: &mut glfw::Window, flag: bool) {
    Engine::on_async_event(&EnumEvent::WindowFocusEvent(flag));
  }
  
  pub fn window_maximize_callback(_window: &mut glfw::Window, flag: bool) {
    Engine::on_async_event(&EnumEvent::WindowMaximizeEvent(flag));
  }
  
  pub fn window_pos_callback(_window: &mut glfw::Window, pos_x: i32, pos_y: i32) {
    Engine::on_async_event(&EnumEvent::WindowPosEvent(pos_x, pos_y));
  }
  
  pub fn window_size_callback(_window: &mut glfw::Window, size_x: i32, size_y: i32) {
    Engine::on_async_event(&EnumEvent::FramebufferEvent(size_x as u32, size_y as u32));
  }
  
  pub fn key_callback(_window: &mut glfw::Window, key: glfw::Key, _scancode: glfw::Scancode, action: glfw::Action,
                      modifiers: glfw::Modifiers) {
    Engine::on_async_event(&EnumEvent::KeyEvent(EnumKey::from(key), EnumAction::from(action), None, EnumModifiers::from(modifiers)));
  }
  
  pub fn mouse_btn_callback(_window: &mut glfw::Window, mouse_btn: glfw::MouseButton, action: glfw::Action, modifiers: glfw::Modifiers) {
    Engine::on_async_event(&EnumEvent::MouseBtnEvent(EnumMouseButton::from(mouse_btn), EnumAction::from(action), EnumModifiers::from(modifiers)));
  }
  
  pub fn scroll_callback(_window: &mut glfw::Window, delta_x: f64, delta_y: f64) {
    Engine::on_async_event(&EnumEvent::MouseScrollEvent(delta_x, delta_y));
  }
  
  pub fn drag_and_drop_callback(_window: &mut glfw::Window, path: Vec<PathBuf>) {
    Engine::on_async_event(&EnumEvent::DragAndDrop(path));
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    unsafe {
      if S_WINDOW_CONTEXT.is_some() {
        log!(EnumLogColor::Purple, "INFO", "[Window] -->\t Dropping window...");
        match self.free() {
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