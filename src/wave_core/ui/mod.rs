/*
 MIT License

 Copyright (c) 2024 Nami Reghbati

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

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumError {
  InvalidContext,
  InvalidUiOptions,
  ApiError,
}

impl Display for EnumError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Ui] -->\t Error encountered with Ui element(s) : {:?}", self)
  }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum EnumUiType {
  Docked,
  Floating,
}

pub mod ui_imgui {
  use std::ffi::CStr;
  use std::os::raw::c_void;
  
  /// Taken from https://docs.rs/imgui-glfw-rs/latest/src/imgui_glfw_rs/lib.rs.html#101-232
  
  /// Use the reexported glfw crate to avoid version conflicts.
  pub use glfw;
  use glfw::{Context, Key, StandardCursor};
  use glfw::ffi::GLFWwindow;
  use imgui::{Condition, ConfigFlags, Key as ImGuiKey, MouseCursor};
  use imgui_opengl_renderer::Renderer;
  
  #[allow(unused)]
  use crate::log;
  use crate::wave_core::events::EnumEvent;
  use crate::wave_core::graphics::renderer::EnumApi;
  use crate::wave_core::input::{EnumAction, EnumModifiers, EnumMouseButton};
  use crate::wave_core::ui::EnumError;
  use crate::wave_core::utils::Time;
  use crate::wave_core::window::Window;
  
  struct GlfwClipboardBackend(*mut c_void);
  
  impl imgui::ClipboardBackend for GlfwClipboardBackend {
    fn get(&mut self) -> Option<String> {
      let char_ptr = unsafe { glfw::ffi::glfwGetClipboardString(self.0 as *mut GLFWwindow) };
      let c_str = unsafe { CStr::from_ptr(char_ptr) };
      Some(c_str.to_str().unwrap().parse().unwrap())
    }
    
    fn set(&mut self, value: &str) {
      unsafe {
        glfw::ffi::glfwSetClipboardString(self.0 as *mut GLFWwindow, value.as_ptr().cast());
      };
    }
  }
  
  trait TraitUi {
    fn on_event(&mut self, event: &EnumEvent) -> bool;
    fn on_update(&mut self);
    fn on_render(&mut self);
    fn on_delete(&mut self) -> Result<(), EnumError>;
  }
  
  
  pub struct Imgui {
    m_api: Box<dyn TraitUi>,
  }
  
  impl Imgui {
    pub fn new(api_choice: EnumApi, window: &mut Window) -> Self {
      return match api_choice {
        EnumApi::OpenGL => Imgui {
          m_api: Box::new(GlImgui::new(window))
        },
        EnumApi::Vulkan => {
          todo!()
        }
      };
    }
    
    pub fn on_event(&mut self, event: &EnumEvent) -> bool {
      return self.m_api.on_event(event);
    }
    
    pub fn on_update(&mut self) {
      return self.m_api.on_update();
    }
    
    pub fn on_render(&mut self) {
      return self.m_api.on_render();
    }
  }
  
  impl Drop for Imgui {
    fn drop(&mut self) {
      match self.m_api.on_delete() {
        Ok(_) => {}
        Err(_) => {}
      }
    }
  }
  
  pub(crate) struct GlImgui {
    m_last_frame: Time,
    m_mouse_press: [bool; 5],
    #[allow(unused)]
    m_cursor_pos: (f64, f64),
    m_cursor: (MouseCursor, Option<StandardCursor>),
    m_imgui_handle: imgui::Context,
    m_ui_handle: *mut imgui::Ui,
    m_window_handle: *mut Window,
    m_renderer: Renderer,
  }
  
  impl TraitUi for GlImgui {
    fn on_event(&mut self, event: &EnumEvent) -> bool {
      return match event {
        EnumEvent::MouseBtnEvent(mouse_btn, action, _modifiers) => {
          let index = match mouse_btn {
            EnumMouseButton::LeftButton => 0,
            EnumMouseButton::RightButton => 1,
            EnumMouseButton::MiddleButton => 2,
            EnumMouseButton::Button4 => 3,
            EnumMouseButton::Button5 => 4,
            _ => 0,
          };
          self.m_mouse_press[index] = action == &EnumAction::Pressed;
          
          self.m_imgui_handle.io_mut().mouse_down[index] = action != &EnumAction::Released;
          true
        }
        // WindowEvent::CursorPos(w, h) => {
        //   self.m_imgui_handle.io_mut().mouse_pos = [w as f32, h as f32];
        //   self.m_cursor_pos = (w, h);
        //   true
        // }
        EnumEvent::MouseScrollEvent(_x, y) => {
          self.m_imgui_handle.io_mut().mouse_wheel = *y as f32;
          true
        }
        // WindowEvent::Char(character) => {
        //   self.m_imgui_handle.io_mut().add_input_character(character);
        //   true
        // }
        EnumEvent::KeyEvent(key, action, _repeat_count, modifier) => {
          // GLFW modifiers.
          self.m_imgui_handle.io_mut().key_ctrl = modifier.intersects(EnumModifiers::Control);
          self.m_imgui_handle.io_mut().key_alt = modifier.intersects(EnumModifiers::Alt);
          self.m_imgui_handle.io_mut().key_shift = modifier.intersects(EnumModifiers::Shift);
          self.m_imgui_handle.io_mut().key_super = modifier.intersects(EnumModifiers::Super);
          
          self.m_imgui_handle.io_mut().keys_down[*key as usize] = action != &EnumAction::Released;
          false
        }
        _ => false
      };
    }
    
    fn on_update(&mut self) {
      let io = self.m_imgui_handle.io_mut();
      
      let now = Time::now();
      let delta = now - self.m_last_frame;
      self.m_last_frame = now;
      io.delta_time = delta.to_secs() as f32;
      
      let window_size = unsafe { (*self.m_window_handle).m_window_resolution };
      io.display_size = [window_size.0 as f32, window_size.1 as f32];
      
      self.m_ui_handle = self.m_imgui_handle.new_frame();
      unsafe {
        (*self.m_ui_handle).window("Example Ui")
          .menu_bar(true)
          .size([window_size.0 as f32, window_size.1 as f32], Condition::FirstUseEver)
          .build(|| {
            (*self.m_ui_handle).text_colored([1.0, 0.0, 0.0, 1.0], "Example text");
          });
      }
    }
    
    fn on_render(&mut self) {
      unsafe {
        let io = (*self.m_ui_handle).io();
        if !io.config_flags.contains(ConfigFlags::NO_MOUSE_CURSOR_CHANGE) {
          match (*self.m_ui_handle).mouse_cursor() {
            Some(mouse_cursor) if !io.mouse_draw_cursor => {
              (*self.m_window_handle).m_api_window.set_cursor_mode(glfw::CursorMode::Normal);
              
              let cursor = match mouse_cursor {
                MouseCursor::TextInput => StandardCursor::IBeam,
                MouseCursor::ResizeNS => StandardCursor::VResize,
                MouseCursor::ResizeEW => StandardCursor::HResize,
                MouseCursor::Hand => StandardCursor::Hand,
                _ => StandardCursor::Arrow,
              };
              (*self.m_window_handle).m_api_window.set_cursor(Some(glfw::Cursor::standard(cursor)));
              
              if self.m_cursor.1 != Some(cursor) {
                self.m_cursor.1 = Some(cursor);
                self.m_cursor.0 = mouse_cursor;
              }
            }
            _ => {
              self.m_cursor.0 = MouseCursor::Arrow;
              self.m_cursor.1 = None;
              (*self.m_window_handle).m_api_window.set_cursor_mode(glfw::CursorMode::Hidden);
            }
          }
        }
        
        self.m_renderer.render(&mut self.m_imgui_handle);
        self.m_imgui_handle.update_platform_windows();
      }
    }
    
    fn on_delete(&mut self) -> Result<(), EnumError> {
      return Ok(());
    }
  }
  
  impl GlImgui {
    pub fn new(window: *mut Window) -> Self {
      let mut context = imgui::Context::create();
      unsafe {
        let window_ptr = (*window).m_api_window.window_ptr() as *mut c_void;
        context.set_clipboard_backend(GlfwClipboardBackend(window_ptr));
      }
      
      let io_mut = context.io_mut();
      Self::glfw_to_imgui(io_mut);
      context.set_renderer_name(String::from("OpenGL"));
      
      let renderer = Renderer::new(&mut context, |s| unsafe {
        (*window).m_api_window.get_proc_address(s) as _
      });
      
      Self {
        m_last_frame: Time::new(),
        m_mouse_press: [false; 5],
        m_cursor_pos: (0., 0.),
        m_cursor: (MouseCursor::Arrow, None),
        m_imgui_handle: context,
        m_ui_handle: std::ptr::null_mut(),
        m_window_handle: window,
        m_renderer: renderer,
      }
    }
    
    fn glfw_to_imgui(imgui: &mut imgui::Io) {
      // GLFW keys.
      imgui.key_map[ImGuiKey::Tab as usize] = Key::Tab as u32;
      imgui.key_map[ImGuiKey::LeftArrow as usize] = Key::Left as u32;
      imgui.key_map[ImGuiKey::RightArrow as usize] = Key::Right as u32;
      imgui.key_map[ImGuiKey::UpArrow as usize] = Key::Up as u32;
      imgui.key_map[ImGuiKey::DownArrow as usize] = Key::Down as u32;
      imgui.key_map[ImGuiKey::PageUp as usize] = Key::PageUp as u32;
      imgui.key_map[ImGuiKey::PageDown as usize] = Key::PageDown as u32;
      imgui.key_map[ImGuiKey::Home as usize] = Key::Home as u32;
      imgui.key_map[ImGuiKey::End as usize] = Key::End as u32;
      imgui.key_map[ImGuiKey::Insert as usize] = Key::Insert as u32;
      imgui.key_map[ImGuiKey::Delete as usize] = Key::Delete as u32;
      imgui.key_map[ImGuiKey::Backspace as usize] = Key::Backspace as u32;
      imgui.key_map[ImGuiKey::Space as usize] = Key::Space as u32;
      imgui.key_map[ImGuiKey::Enter as usize] = Key::Enter as u32;
      imgui.key_map[ImGuiKey::Escape as usize] = Key::Escape as u32;
      imgui.key_map[ImGuiKey::A as usize] = Key::A as u32;
      imgui.key_map[ImGuiKey::C as usize] = Key::C as u32;
      imgui.key_map[ImGuiKey::V as usize] = Key::V as u32;
      imgui.key_map[ImGuiKey::X as usize] = Key::X as u32;
      imgui.key_map[ImGuiKey::Y as usize] = Key::Y as u32;
      imgui.key_map[ImGuiKey::Z as usize] = Key::Z as u32;
    }
  }
}