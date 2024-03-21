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

use crate::input;
use crate::utils::Time;

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
pub enum EnumEventError {
  InvalidEventCallback,
  PollingDisabled,
}

impl Display for EnumEventError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Event] -->\t Error encountered with event handling : {:?}", self)
  }
}

bitflags! {
  #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
  pub struct EnumEventMask: u16 {
    const None            = 0b0000000000000000;
    const All             = !0;
    
    // Window events.
    const Window          = 0b1011111100000000;
    const WindowIconify  = 0b1000000100000000;
    const WindowMaximize = 0b1000001000000000;
    const WindowFocus    = 0b1000010000000000;
    const WindowClose    = 0b1000100000000000;
    const WindowSize     = 0b1001000000000000;
    const WindowPos      = 0b1010000000000000;
    
    // Input events.
    const Input           = 0b0000000111111111;
    const DragAndDrop   = 0b0000000100000001;
    const Keyboard        = 0b0000000100000010;
    
    // Mouse events.
    const Mouse           = 0b0000000100011100;
    const CursorPos      = 0b0000000100000100;
    const MouseBtn       = 0b0000000100001000;
    const MouseScroll    = 0b0000000100010000;
  }
}

impl From<&EnumEvent> for EnumEventMask {
  fn from(value: &EnumEvent) -> Self {
    return match value {
      EnumEvent::WindowIconifyEvent(_) => EnumEventMask::WindowIconify,
      EnumEvent::WindowMaximizeEvent(_) => EnumEventMask::WindowMaximize,
      EnumEvent::WindowCloseEvent(_) => EnumEventMask::WindowClose,
      EnumEvent::FramebufferEvent(_, _) => EnumEventMask::WindowSize,
      EnumEvent::WindowPosEvent(_, _) => EnumEventMask::WindowPos,
      EnumEvent::WindowFocusEvent(_) => EnumEventMask::WindowFocus,
      EnumEvent::KeyEvent(_, _, _, _) => EnumEventMask::Keyboard,
      EnumEvent::MouseBtnEvent(_, _, _) => EnumEventMask::MouseBtn,
      EnumEvent::MouseScrollEvent(_, _) => EnumEventMask::MouseScroll,
      EnumEvent::DragAndDrop(_) => EnumEventMask::DragAndDrop,
      EnumEvent::UnknownEvent => EnumEventMask::empty()
    };
  }
}

impl Display for EnumEventMask {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut mask_count = 0;
    
    write!(f, "[{1:016b}]\n{0:115}Events: ", "", self.0.0)?;
    
    if self == &EnumEventMask::None {
      return write!(f, "Nothing ({0:016b})", EnumEventMask::None);
    }
    if self == &EnumEventMask::All {
      return write!(f, "Everything ({0:016b})", EnumEventMask::All);
    }
    
    if self.contains(EnumEventMask::Window) {
      mask_count += 1;
      write!(f, "All window ({0:016b}) ", EnumEventMask::Window)?;
    }
    
    if !self.contains(EnumEventMask::Window) && self.contains(EnumEventMask::WindowSize) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Window framebuffer ({0:016b}) ", EnumEventMask::WindowSize)?;
      } else {
        write!(f, "Window framebuffer ({0:016b}) ", EnumEventMask::WindowSize)?;
      }
    }
    if !self.contains(EnumEventMask::Window) && self.contains(EnumEventMask::WindowClose) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Window close ({0:016b}) ", EnumEventMask::WindowClose)?;
      } else {
        write!(f, "Window close ({0:016b}) ", EnumEventMask::WindowClose)?;
      }
    }
    if !self.contains(EnumEventMask::Window) && self.contains(EnumEventMask::WindowIconify) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Window iconify ({0:016b}) ", EnumEventMask::WindowIconify)?;
      } else {
        write!(f, "Window iconify ({0:016b}) ", EnumEventMask::WindowIconify)?;
      }
    }
    if !self.contains(EnumEventMask::Window) && self.contains(EnumEventMask::WindowMaximize) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Window maximize ({0:016b}) ", EnumEventMask::WindowMaximize)?;
      } else {
        write!(f, "Window maximize ({0:016b}) ", EnumEventMask::WindowMaximize)?;
      }
    }
    if !self.contains(EnumEventMask::Window) && self.contains(EnumEventMask::WindowFocus) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Window focus ({0:016b}) ", EnumEventMask::WindowFocus)?;
      } else {
        write!(f, "Window focus ({0:016b}) ", EnumEventMask::WindowFocus)?;
      }
    }
    if !self.contains(EnumEventMask::Window) && self.contains(EnumEventMask::WindowPos) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Window position ({0:016b}) ", EnumEventMask::WindowPos)?;
      } else {
        write!(f, "Window position ({0:016b}) ", EnumEventMask::WindowPos)?;
      }
    }
    
    if self.contains(EnumEventMask::Input) {
      mask_count += 1;
      if mask_count > 1 {
        return write!(f, "| All input ({0:016b})", EnumEventMask::Input);
      } else {
        return write!(f, "All input ({0:016b})", EnumEventMask::Input);
      }
    }
    
    if self.contains(EnumEventMask::Mouse) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| All Mouse ({0:016b}) ", EnumEventMask::Mouse)?;
      } else {
        write!(f, "All Mouse ({0:016b}) ", EnumEventMask::Mouse)?;
      }
    }
    
    if !self.contains(EnumEventMask::Mouse) && self.contains(EnumEventMask::MouseBtn) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Mouse button ({0:016b}) ", EnumEventMask::MouseBtn)?;
      } else {
        write!(f, "Mouse button ({0:016b}) ", EnumEventMask::MouseBtn)?;
      }
    }
    if !self.contains(EnumEventMask::Mouse) && self.contains(EnumEventMask::MouseScroll) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Mouse scroll ({0:016b}) ", EnumEventMask::MouseScroll)?;
      } else {
        write!(f, "Mouse scroll ({0:016b}) ", EnumEventMask::MouseScroll)?;
      }
    }
    
    if self.contains(EnumEventMask::Keyboard) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Keyboard ({0:016b}) ", EnumEventMask::Keyboard)?;
      } else {
        write!(f, "Keyboard ({0:016b}) ", EnumEventMask::Keyboard)?;
      }
    }
    if self.contains(EnumEventMask::DragAndDrop) {
      mask_count += 1;
      if mask_count > 1 {
        write!(f, "| Drag and drop ({0:016b})", EnumEventMask::DragAndDrop)?;
      } else {
        write!(f, "Drag and drop ({0:016b})", EnumEventMask::DragAndDrop)?;
      }
    }
    return Ok(());
  }
}