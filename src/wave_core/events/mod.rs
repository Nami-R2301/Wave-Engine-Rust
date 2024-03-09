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
  pub struct EnumEventMask: u8 {
    const c_none = 0b00000000;
    const c_all = !0;
    
    // Window events.
    const c_window = 0b01111111;
    const c_window_iconify = 0b01000001;
    const c_window_maximize = 0b01000010;
    const c_window_focus = 0b01000100;
    const c_window_close = 0b01001000;
    const c_window_size = 0b01010000;
    const c_window_pos = 0b01100000;
    
    // Input events.
    const c_input = 0b00111111;
    const c_drag_and_drop = 0b00100001;
    const c_keyboard = 0b00100010;
    
    // Mouse events.
    const c_mouse = 0b00111100;
    const c_cursor_pos = 0b00100100;
    const c_mouse_btn = 0b00101000;
    const c_mouse_scroll = 0b00110000;
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