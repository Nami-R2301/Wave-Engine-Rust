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
use crate::wave_core;
use crate::wave_core::{input};
use crate::wave_core::layers::Layer;
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
  UnknownEvent
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
      glfw::WindowEvent::Key(key, _scancode, action, modifiers) =>  {
        EnumEvent::KeyEvent(
            input::EnumKey::from(key), input::EnumAction::from(action), input::Input::get_key_repeat(input::EnumKey::from(key)),
            input::EnumModifiers::from(modifiers))
      },
      glfw::WindowEvent::MouseButton(button, action, modifiers) => EnumEvent::MouseBtnEvent(
        input::EnumMouseButton::from(button), input::EnumAction::from(action), input::EnumModifiers::from(modifiers)),
      glfw::WindowEvent::Scroll(x_factor, y_factor) => EnumEvent::MouseScrollEvent(x_factor, y_factor),
      glfw::WindowEvent::FileDrop(path_buffer) => EnumEvent::DragAndDrop(path_buffer),
      _ => EnumEvent::UnknownEvent
    }
  }
}

bitflags! {
  #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
  pub struct EnumEventMask: ::std::os::raw::c_int {
    const c_none = 0x0000;
    const c_all = 0x0001;
    
    // Window events.
    const c_window = 0x0002;
    const c_window_iconify = 0x0004;
    const c_window_maximize = 0x0008;
    const c_window_focus = 0x0010;
    const c_window_close = 0x0020;
    const c_window_size = 0x0040;
    const c_window_pos = 0x0080;
    
    // Input events.
    const c_input = 0x0100;
    const c_drag_and_drop = 0x0200;
    const c_key = 0x0400;
    
    // Mouse events.
    const c_mouse = 0x0800;
    const c_mouse_btn = 0x1000;
    const cursor_pos = 0x2000;
    const c_mouse_scroll = 0x4000;
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

pub struct AsyncCallback {
  pub(crate) m_callback: &'static dyn std::any::Any
}

impl AsyncCallback {
  pub fn new(callback_type: EnumEventMask, glfw_callback: &'static dyn std::any::Any) -> Result<Self, EnumError> {
    match callback_type {
      EnumEventMask::c_key => {
        glfw_callback.downcast_ref::<fn(&mut glfw::Window, glfw::Key, glfw::Scancode, glfw::Action, glfw::Modifiers)>()
          .ok_or(EnumError::InvalidEventCallback)?;
      }
      EnumEventMask::c_mouse_btn => {
        glfw_callback.downcast_ref::<fn(&mut glfw::Window, glfw::MouseButton, glfw::Action, glfw::Modifiers)>()
          .ok_or(EnumError::InvalidEventCallback)?;
      }
      EnumEventMask::c_mouse_scroll => {
        glfw_callback.downcast_ref::<fn(&mut glfw::Window, glfw::MouseButton, f64, f64)>()
          .ok_or(EnumError::InvalidEventCallback)?;
      }
      EnumEventMask::c_window_close => {
        glfw_callback.downcast_ref::<fn(&mut glfw::Window)>().ok_or(EnumError::InvalidEventCallback)?;
      }
      EnumEventMask::c_window_focus => {
        glfw_callback.downcast_ref::<fn(&mut glfw::Window, bool)>().ok_or(EnumError::InvalidEventCallback)?;
      }
      EnumEventMask::c_window_maximize => {
        glfw_callback.downcast_ref::<fn(&mut glfw::Window, bool)>().ok_or(EnumError::InvalidEventCallback)?;
      }
      EnumEventMask::c_window_iconify => {
        glfw_callback.downcast_ref::<fn(&mut glfw::Window, bool)>().ok_or(EnumError::InvalidEventCallback)?;
      }
      EnumEventMask::c_window_pos => {
        glfw_callback.downcast_ref::<fn(&mut glfw::Window, i32, i32)>().ok_or(EnumError::InvalidEventCallback)?;
      }
      EnumEventMask::c_window_size => {
        glfw_callback.downcast_ref::<fn(&mut glfw::Window, i32, i32)>().ok_or(EnumError::InvalidEventCallback)?;
      }
      EnumEventMask::c_drag_and_drop => {
        glfw_callback.downcast_ref::<fn(&mut glfw::Window, Vec<PathBuf>)>().ok_or(EnumError::InvalidEventCallback)?;
      }
      _ => return Err(EnumError::InvalidEventCallback)
    }
    return Ok(Self {
      m_callback: glfw_callback
    });
  }
}

pub struct SyncCallback {
  m_callback: fn(&mut Layer) -> Result<bool, wave_core::EnumError>
}

impl SyncCallback {
  pub fn new(event_callback: fn(&mut Layer) -> Result<bool, wave_core::EnumError>) -> Self {
    return Self {
      m_callback: event_callback
    }
  }
  
  pub fn call(&mut self, data: &mut Layer) -> Result<bool, wave_core::EnumError> {
    return (self.m_callback)(data);
  }
}