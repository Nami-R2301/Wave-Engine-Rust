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
use std::path::PathBuf;

use bitflags::bitflags;

use crate::wave_core::input;
use crate::wave_core::utils::Time;

pub trait TraitEvent {

}

#[derive(Debug, PartialEq)]
pub enum EnumEvent {
  WindowIconifyEvent(bool),
  WindowMaximizeEvent(bool),
  WindowCloseEvent(Time),
  FramebufferEvent(u32, u32),
  WindowPosEvent(i32, i32),
  WindowFocusEvent(bool),
  KeyEvent(input::EnumKey, input::EnumAction, Option<u32>, input::EnumModifiers),
  MouseBtnEvent(input::EnumMouseButton, input::EnumAction, input::EnumModifiers),
  MouseScrollEvent(f64, f64),
  DragAndDrop(Vec<PathBuf>),
  UnknownEvent,
}

impl Display for EnumEvent {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return match self {
      EnumEvent::WindowIconifyEvent(_) => write!(f, "WindowIconifyEvent"),
      EnumEvent::WindowMaximizeEvent(_) => write!(f, "WindowMaximizeEvent"),
      EnumEvent::WindowCloseEvent(_) => write!(f, "WindowCloseEvent"),
      EnumEvent::FramebufferEvent(_, _) => write!(f, "FramebufferEvent"),
      EnumEvent::WindowPosEvent(_, _) => write!(f, "WindowPosEvent"),
      EnumEvent::WindowFocusEvent(_) => write!(f, "WindowFocusEvent"),
      EnumEvent::KeyEvent(_, _, _, _) => write!(f, "KeyEvent"),
      EnumEvent::MouseBtnEvent(_, _, _) => write!(f, "MouseBtnEvent"),
      EnumEvent::MouseScrollEvent(_, _) => write!(f, "MouseScrollEvent"),
      EnumEvent::DragAndDrop(_) => write!(f, "DragAndDrop"),
      EnumEvent::UnknownEvent => write!(f, "UnknownEvent")
    }
  }
}

impl TraitEvent for EnumEvent {
}

impl From<glfw::WindowEvent> for EnumEvent {
  fn from(event: glfw::WindowEvent) -> Self {
    return match event {
      glfw::WindowEvent::Pos(x_pos, y_pos) => EnumEvent::WindowPosEvent(x_pos, y_pos),
      glfw::WindowEvent::Close => EnumEvent::WindowCloseEvent(Time::now()),
      glfw::WindowEvent::Focus(bool) => EnumEvent::WindowFocusEvent(bool),
      glfw::WindowEvent::Iconify(bool) => EnumEvent::WindowFocusEvent(bool),
      glfw::WindowEvent::Maximize(bool) => EnumEvent::WindowFocusEvent(bool),
      glfw::WindowEvent::FramebufferSize(x_size, y_size) => EnumEvent::FramebufferEvent(x_size as u32, y_size as u32),
      glfw::WindowEvent::Key(key, _scancode, action, modifiers) => {
        EnumEvent::KeyEvent(
          input::EnumKey::from(key), input::EnumAction::from(action), input::Input::get_key_repeat(input::EnumKey::from(key)),
          input::EnumModifiers::from(modifiers))
      }
      glfw::WindowEvent::MouseButton(button, action, modifiers) => EnumEvent::MouseBtnEvent(
        input::EnumMouseButton::from(button), input::EnumAction::from(action), input::EnumModifiers::from(modifiers)),
      glfw::WindowEvent::Scroll(x_factor, y_factor) => EnumEvent::MouseScrollEvent(x_factor, y_factor),
      glfw::WindowEvent::FileDrop(path_buffer) => EnumEvent::DragAndDrop(path_buffer),
      _ => EnumEvent::UnknownEvent
    };
  }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EnumError {
  InvalidEventCallback,
  PollingDisabled,
}

impl Display for EnumError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Event] -->\t Error encountered with event handling : {:?}", self)
  }
}

bitflags! {
  #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
  pub struct EnumEventMask: u16 {
    const c_none            = 0b0000000000000000;
    const c_all             = !0;
    
    // Window events.
    const c_window          = 0b1011111100000000;
    const c_window_iconify  = 0b1000000100000000;
    const c_window_maximize = 0b1000001000000000;
    const c_window_focus    = 0b1000010000000000;
    const c_window_close    = 0b1000100000000000;
    const c_window_size     = 0b1001000000000000;
    const c_window_pos      = 0b1010000000000000;
    
    // Input events.
    const c_input           = 0b0000000111111111;
    const c_drag_and_drop   = 0b0000000100000001;
    const c_keyboard        = 0b0000000100000010;
    
    // Mouse events.
    const c_mouse           = 0b0000000100011100;
    const c_cursor_pos      = 0b0000000100000100;
    const c_mouse_btn       = 0b0000000100001000;
    const c_mouse_scroll    = 0b0000000100010000;
  }
}

impl From<&EnumEvent> for EnumEventMask {
  fn from(value: &EnumEvent) -> Self {
    return match value {
      EnumEvent::WindowIconifyEvent(_) => EnumEventMask::c_window_iconify,
      EnumEvent::WindowMaximizeEvent(_) => EnumEventMask::c_window_maximize,
      EnumEvent::WindowCloseEvent(_) => EnumEventMask::c_window_close,
      EnumEvent::FramebufferEvent(_, _) => EnumEventMask::c_window_size,
      EnumEvent::WindowPosEvent(_, _) => EnumEventMask::c_window_pos,
      EnumEvent::WindowFocusEvent(_) => EnumEventMask::c_window_focus,
      EnumEvent::KeyEvent(_, _, _, _) => EnumEventMask::c_keyboard,
      EnumEvent::MouseBtnEvent(_, _, _) => EnumEventMask::c_mouse_btn,
      EnumEvent::MouseScrollEvent(_, _) => EnumEventMask::c_mouse_scroll,
      EnumEvent::DragAndDrop(_) => EnumEventMask::c_drag_and_drop,
      EnumEvent::UnknownEvent => EnumEventMask::empty()
    };
  }
}

impl Display for EnumEventMask {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut mask_count = 0;
    
    write!(f, "[{1:016b}]\n{0:115}Events: ", "", self.0.0)?;
    
    if self == &EnumEventMask::c_none {
      return write!(f, "Nothing ({0:016b})", EnumEventMask::c_none);
    }
    if self == &EnumEventMask::c_all {
      return write!(f, "Everything ({0:016b})", EnumEventMask::c_all);
    }
    
    if self.contains(EnumEventMask::c_window) {
      mask_count += 1;
      write!(f, "All window ({0:016b}) ", EnumEventMask::c_window)?;
    }
    
    if !self.contains(EnumEventMask::c_window) && self.contains(EnumEventMask::c_window_size) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Window framebuffer ({0:016b}) ", EnumEventMask::c_window_size)?;
      } else {
        write!(f, "Window framebuffer ({0:016b}) ", EnumEventMask::c_window_size)?;
      }
    }
    if !self.contains(EnumEventMask::c_window) && self.contains(EnumEventMask::c_window_close) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Window close ({0:016b}) ", EnumEventMask::c_window_close)?;
      } else {
        write!(f, "Window close ({0:016b}) ", EnumEventMask::c_window_close)?;
      }
    }
    if !self.contains(EnumEventMask::c_window) && self.contains(EnumEventMask::c_window_iconify) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Window iconify ({0:016b}) ", EnumEventMask::c_window_iconify)?;
      } else {
        write!(f, "Window iconify ({0:016b}) ", EnumEventMask::c_window_iconify)?;
      }
    }
    if !self.contains(EnumEventMask::c_window) && self.contains(EnumEventMask::c_window_maximize) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Window maximize ({0:016b}) ", EnumEventMask::c_window_maximize)?;
      } else {
        write!(f, "Window maximize ({0:016b}) ", EnumEventMask::c_window_maximize)?;
      }
    }
    if !self.contains(EnumEventMask::c_window) && self.contains(EnumEventMask::c_window_focus) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Window focus ({0:016b}) ", EnumEventMask::c_window_focus)?;
      } else {
        write!(f, "Window focus ({0:016b}) ", EnumEventMask::c_window_focus)?;
      }
    }
    if !self.contains(EnumEventMask::c_window) && self.contains(EnumEventMask::c_window_pos) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Window position ({0:016b}) ", EnumEventMask::c_window_pos)?;
      } else {
        write!(f, "Window position ({0:016b}) ", EnumEventMask::c_window_pos)?;
      }
    }
    
    if self.contains(EnumEventMask::c_input) {
      mask_count += 1;
      if mask_count > 1 {
        return write!(f, "| All input ({0:016b})", EnumEventMask::c_input);
      } else {
        return write!(f, "All input ({0:016b})", EnumEventMask::c_input);
      }
    }
    
    if self.contains(EnumEventMask::c_mouse) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| All Mouse ({0:016b}) ", EnumEventMask::c_mouse)?;
      } else {
        write!(f, "All Mouse ({0:016b}) ", EnumEventMask::c_mouse)?;
      }
    }
    
    if !self.contains(EnumEventMask::c_mouse) && self.contains(EnumEventMask::c_mouse_btn) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Mouse button ({0:016b}) ", EnumEventMask::c_mouse_btn)?;
      } else {
        write!(f, "Mouse button ({0:016b}) ", EnumEventMask::c_mouse_btn)?;
      }
    }
    if !self.contains(EnumEventMask::c_mouse) && self.contains(EnumEventMask::c_mouse_scroll) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Mouse scroll ({0:016b}) ", EnumEventMask::c_mouse_scroll)?;
      } else {
        write!(f, "Mouse scroll ({0:016b}) ", EnumEventMask::c_mouse_scroll)?;
      }
    }
    
    if self.contains(EnumEventMask::c_keyboard) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Keyboard ({0:016b}) ", EnumEventMask::c_keyboard)?;
      } else {
        write!(f, "Keyboard ({0:016b}) ", EnumEventMask::c_keyboard)?;
      }
    }
    if self.contains(EnumEventMask::c_drag_and_drop) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Drag and drop ({0:016b})", EnumEventMask::c_drag_and_drop)?;
      } else {
        write!(f, "Drag and drop ({0:016b})", EnumEventMask::c_drag_and_drop)?;
      }
    }
    return Ok(());
  }
}