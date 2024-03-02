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

use std::path::PathBuf;
use glfw::Action;
use crate::wave_core::input;
use crate::wave_core::utils::Time;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum EnumEvent {
  WindowIconifyEvent(bool),
  WindowMaximizeEvent(bool),
  WindowCloseEvent(Time),
  WindowResizeEvent(u32, u32),
  WindowMoveEvent(i32, i32),
  WindowFocusEvent(bool),
  KeyPressedEvent(input::EnumKey, glfw::Modifiers),
  KeyHeldEvent(input::EnumKey, glfw::Modifiers),
  KeyReleasedEvent(input::EnumKey, glfw::Modifiers),
  MouseBtnPressedEvent(input::EnumMouseButton, glfw::Modifiers),
  MouseBtnHeldEvent(input::EnumMouseButton, glfw::Modifiers),
  MouseBtnReleasedEvent(input::EnumMouseButton, glfw::Modifiers),
  MouseScrollEvent(f64, f64),
  FileDropEvent(Vec<PathBuf>),
  UnknownEvent
}

impl From<glfw::WindowEvent> for EnumEvent {
  fn from(event: glfw::WindowEvent) -> Self {
    return match event {
      glfw::WindowEvent::Pos(x_pos, y_pos) => EnumEvent::WindowMoveEvent(x_pos, y_pos),
      glfw::WindowEvent::Close => EnumEvent::WindowCloseEvent(Time::now()),
      glfw::WindowEvent::Focus(bool) => EnumEvent::WindowFocusEvent(bool),
      glfw::WindowEvent::Iconify(bool) => EnumEvent::WindowFocusEvent(bool),
      glfw::WindowEvent::Maximize(bool) => EnumEvent::WindowFocusEvent(bool),
      glfw::WindowEvent::FramebufferSize(x_size, y_size) => EnumEvent::WindowResizeEvent(x_size as u32, y_size as u32),
      glfw::WindowEvent::Key(key, _scancode, action, modifiers) => {
        return match action {
          Action::Release => EnumEvent::KeyReleasedEvent(input::EnumKey::from(key), modifiers),
          Action::Press => EnumEvent::KeyPressedEvent(input::EnumKey::from(key), modifiers),
          Action::Repeat => EnumEvent::KeyHeldEvent(input::EnumKey::from(key), modifiers),
        }
      },
      glfw::WindowEvent::MouseButton(button, action, modifiers) => {
        return match action {
          Action::Release => EnumEvent::MouseBtnReleasedEvent(input::EnumMouseButton::from(button), modifiers),
          Action::Press => EnumEvent::MouseBtnPressedEvent(input::EnumMouseButton::from(button), modifiers),
          Action::Repeat => EnumEvent::MouseBtnHeldEvent(input::EnumMouseButton::from(button), modifiers),
        }
      },
      glfw::WindowEvent::Scroll(x_factor, y_factor) => EnumEvent::MouseScrollEvent(x_factor, y_factor),
      glfw::WindowEvent::FileDrop(path_buffer) => EnumEvent::FileDropEvent(path_buffer),
      _ => EnumEvent::UnknownEvent
    }
  }
}

#[derive(Debug)]
pub struct Event {
  pub m_event: EnumEvent,
}

impl Event {
  pub fn new(event: EnumEvent) -> Self {
    return Self {
      m_event: event,
    }
  }
}
