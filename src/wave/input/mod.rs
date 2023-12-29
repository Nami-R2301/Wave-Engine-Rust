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

use crate::log;
use crate::wave::math::Vec2;
use crate::wave::window::{S_WINDOW, S_WINDOW_CONTEXT};

// Highest glfw key index (348). Glfw key indices => [0, 348], however we don't care about the first 32 indices.
const C_NUM_KEYS: usize = glfw::ffi::KEY_LAST as usize;

// Highest glfw key index (348). Glfw key indices => [0, 7].
const C_NUM_MOUSE_BUTTONS: usize = glfw::ffi::MOUSE_BUTTON_LAST as usize;

static mut S_KEY_STATES: [(EnumAction, EnumAction); C_NUM_KEYS] = [(EnumAction::Release, EnumAction::Release); C_NUM_KEYS];
static mut S_MOUSE_BUTTON_STATES: [(EnumAction, EnumAction); C_NUM_MOUSE_BUTTONS] = [(EnumAction::Release, EnumAction::Release); C_NUM_MOUSE_BUTTONS];

#[derive(Debug, Eq, PartialEq)]
pub enum EnumErrors {
  InvalidWindowContext,
  InvalidKey,
  InvalidMouseButton,
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum EnumKeys {
  Space = glfw::ffi::KEY_SPACE,
  Apostrophe = glfw::ffi::KEY_APOSTROPHE,
  Comma = glfw::ffi::KEY_COMMA,
  Minus = glfw::ffi::KEY_MINUS,
  Period = glfw::ffi::KEY_PERIOD,
  Slash = glfw::ffi::KEY_SLASH,
  Num0 = glfw::ffi::KEY_0,
  Num1 = glfw::ffi::KEY_1,
  Num2 = glfw::ffi::KEY_2,
  Num3 = glfw::ffi::KEY_3,
  Num4 = glfw::ffi::KEY_4,
  Num5 = glfw::ffi::KEY_5,
  Num6 = glfw::ffi::KEY_6,
  Num7 = glfw::ffi::KEY_7,
  Num8 = glfw::ffi::KEY_8,
  Num9 = glfw::ffi::KEY_9,
  Semicolon = glfw::ffi::KEY_SEMICOLON,
  Equal = glfw::ffi::KEY_EQUAL,
  A = glfw::ffi::KEY_A,
  B = glfw::ffi::KEY_B,
  C = glfw::ffi::KEY_C,
  D = glfw::ffi::KEY_D,
  E = glfw::ffi::KEY_E,
  F = glfw::ffi::KEY_F,
  G = glfw::ffi::KEY_G,
  H = glfw::ffi::KEY_H,
  I = glfw::ffi::KEY_I,
  J = glfw::ffi::KEY_J,
  K = glfw::ffi::KEY_K,
  L = glfw::ffi::KEY_L,
  M = glfw::ffi::KEY_M,
  N = glfw::ffi::KEY_N,
  O = glfw::ffi::KEY_O,
  P = glfw::ffi::KEY_P,
  Q = glfw::ffi::KEY_Q,
  R = glfw::ffi::KEY_R,
  S = glfw::ffi::KEY_S,
  T = glfw::ffi::KEY_T,
  U = glfw::ffi::KEY_U,
  V = glfw::ffi::KEY_V,
  W = glfw::ffi::KEY_W,
  X = glfw::ffi::KEY_X,
  Y = glfw::ffi::KEY_Y,
  Z = glfw::ffi::KEY_Z,
  LeftBracket = glfw::ffi::KEY_LEFT_BRACKET,
  Backslash = glfw::ffi::KEY_BACKSLASH,
  RightBracket = glfw::ffi::KEY_RIGHT_BRACKET,
  GraveAccent = glfw::ffi::KEY_GRAVE_ACCENT,
  World1 = glfw::ffi::KEY_WORLD_1,
  World2 = glfw::ffi::KEY_WORLD_2,
  
  Escape = glfw::ffi::KEY_ESCAPE,
  Enter = glfw::ffi::KEY_ENTER,
  Tab = glfw::ffi::KEY_TAB,
  Backspace = glfw::ffi::KEY_BACKSPACE,
  Insert = glfw::ffi::KEY_INSERT,
  Delete = glfw::ffi::KEY_DELETE,
  Right = glfw::ffi::KEY_RIGHT,
  Left = glfw::ffi::KEY_LEFT,
  Down = glfw::ffi::KEY_DOWN,
  Up = glfw::ffi::KEY_UP,
  PageUp = glfw::ffi::KEY_PAGE_UP,
  PageDown = glfw::ffi::KEY_PAGE_DOWN,
  Home = glfw::ffi::KEY_HOME,
  End = glfw::ffi::KEY_END,
  CapsLock = glfw::ffi::KEY_CAPS_LOCK,
  ScrollLock = glfw::ffi::KEY_SCROLL_LOCK,
  NumLock = glfw::ffi::KEY_NUM_LOCK,
  PrintScreen = glfw::ffi::KEY_PRINT_SCREEN,
  Pause = glfw::ffi::KEY_PAUSE,
  F1 = glfw::ffi::KEY_F1,
  F2 = glfw::ffi::KEY_F2,
  F3 = glfw::ffi::KEY_F3,
  F4 = glfw::ffi::KEY_F4,
  F5 = glfw::ffi::KEY_F5,
  F6 = glfw::ffi::KEY_F6,
  F7 = glfw::ffi::KEY_F7,
  F8 = glfw::ffi::KEY_F8,
  F9 = glfw::ffi::KEY_F9,
  F10 = glfw::ffi::KEY_F10,
  F11 = glfw::ffi::KEY_F11,
  F12 = glfw::ffi::KEY_F12,
  F13 = glfw::ffi::KEY_F13,
  F14 = glfw::ffi::KEY_F14,
  F15 = glfw::ffi::KEY_F15,
  F16 = glfw::ffi::KEY_F16,
  F17 = glfw::ffi::KEY_F17,
  F18 = glfw::ffi::KEY_F18,
  F19 = glfw::ffi::KEY_F19,
  F20 = glfw::ffi::KEY_F20,
  F21 = glfw::ffi::KEY_F21,
  F22 = glfw::ffi::KEY_F22,
  F23 = glfw::ffi::KEY_F23,
  F24 = glfw::ffi::KEY_F24,
  F25 = glfw::ffi::KEY_F25,
  Kp0 = glfw::ffi::KEY_KP_0,
  Kp1 = glfw::ffi::KEY_KP_1,
  Kp2 = glfw::ffi::KEY_KP_2,
  Kp3 = glfw::ffi::KEY_KP_3,
  Kp4 = glfw::ffi::KEY_KP_4,
  Kp5 = glfw::ffi::KEY_KP_5,
  Kp6 = glfw::ffi::KEY_KP_6,
  Kp7 = glfw::ffi::KEY_KP_7,
  Kp8 = glfw::ffi::KEY_KP_8,
  Kp9 = glfw::ffi::KEY_KP_9,
  KpDecimal = glfw::ffi::KEY_KP_DECIMAL,
  KpDivide = glfw::ffi::KEY_KP_DIVIDE,
  KpMultiply = glfw::ffi::KEY_KP_MULTIPLY,
  KpSubtract = glfw::ffi::KEY_KP_SUBTRACT,
  KpAdd = glfw::ffi::KEY_KP_ADD,
  KpEnter = glfw::ffi::KEY_KP_ENTER,
  KpEqual = glfw::ffi::KEY_KP_EQUAL,
  LeftShift = glfw::ffi::KEY_LEFT_SHIFT,
  LeftControl = glfw::ffi::KEY_LEFT_CONTROL,
  LeftAlt = glfw::ffi::KEY_LEFT_ALT,
  LeftSuper = glfw::ffi::KEY_LEFT_SUPER,
  RightShift = glfw::ffi::KEY_RIGHT_SHIFT,
  RightControl = glfw::ffi::KEY_RIGHT_CONTROL,
  RightAlt = glfw::ffi::KEY_RIGHT_ALT,
  RightSuper = glfw::ffi::KEY_RIGHT_SUPER,
  Menu = glfw::ffi::KEY_MENU,
  Unknown = glfw::ffi::KEY_UNKNOWN,
}

impl From<glfw::Key> for EnumKeys {
  fn from(value: glfw::Key) -> Self {
    return convert_api_key_to_key(value);
  }
}

impl From<EnumKeys> for glfw::Key {
  fn from(value: EnumKeys) -> Self {
    return convert_key_to_api_key(value);
  }
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum EnumMouseButtons {
  Button1 = glfw::ffi::MOUSE_BUTTON_1,
  Button2 = glfw::ffi::MOUSE_BUTTON_2,
  Button3 = glfw::ffi::MOUSE_BUTTON_3,
  Button4 = glfw::ffi::MOUSE_BUTTON_4,
  Button5 = glfw::ffi::MOUSE_BUTTON_5,
  Button6 = glfw::ffi::MOUSE_BUTTON_6,
  Button7 = glfw::ffi::MOUSE_BUTTON_7,
  Button8 = glfw::ffi::MOUSE_BUTTON_8,
}

impl From<glfw::MouseButton> for EnumMouseButtons {
  fn from(value: glfw::MouseButton) -> Self {
    return convert_api_mouse_btn_to_mouse_btn(value);
  }
}

impl From<EnumMouseButtons> for glfw::MouseButton {
  fn from(value: EnumMouseButtons) -> Self {
    return convert_mouse_btn_to_api_mouse_btn(value);
  }
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum EnumAction {
  Release = glfw::ffi::RELEASE,
  Press = glfw::ffi::PRESS,
  Hold = glfw::ffi::REPEAT,
}

impl From<glfw::Action> for EnumAction {
  fn from(value: glfw::Action) -> Self {
    return convert_api_action_to_action(value);
  }
}

impl From<EnumAction> for glfw::Action {
  fn from(value: EnumAction) -> Self {
    return convert_action_to_api_action(value);
  }
}

fn convert_key_to_api_key(enum_key: EnumKeys) -> glfw::Key {
  return match enum_key {
    EnumKeys::Space => glfw::Key::Space,
    EnumKeys::Apostrophe => glfw::Key::Apostrophe,
    EnumKeys::Comma => glfw::Key::Comma,
    EnumKeys::Minus => glfw::Key::Minus,
    EnumKeys::Period => glfw::Key::Period,
    EnumKeys::Slash => glfw::Key::Slash,
    EnumKeys::Num0 => glfw::Key::Num0,
    EnumKeys::Num1 => glfw::Key::Num1,
    EnumKeys::Num2 => glfw::Key::Num2,
    EnumKeys::Num3 => glfw::Key::Num3,
    EnumKeys::Num4 => glfw::Key::Num4,
    EnumKeys::Num5 => glfw::Key::Num5,
    EnumKeys::Num6 => glfw::Key::Num6,
    EnumKeys::Num7 => glfw::Key::Num7,
    EnumKeys::Num8 => glfw::Key::Num8,
    EnumKeys::Num9 => glfw::Key::Num9,
    EnumKeys::Semicolon => glfw::Key::Semicolon,
    EnumKeys::Equal => glfw::Key::Equal,
    EnumKeys::A => glfw::Key::A,
    EnumKeys::B => glfw::Key::B,
    EnumKeys::C => glfw::Key::C,
    EnumKeys::D => glfw::Key::D,
    EnumKeys::E => glfw::Key::E,
    EnumKeys::F => glfw::Key::F,
    EnumKeys::G => glfw::Key::G,
    EnumKeys::H => glfw::Key::H,
    EnumKeys::I => glfw::Key::I,
    EnumKeys::J => glfw::Key::J,
    EnumKeys::K => glfw::Key::K,
    EnumKeys::L => glfw::Key::L,
    EnumKeys::M => glfw::Key::M,
    EnumKeys::N => glfw::Key::N,
    EnumKeys::O => glfw::Key::O,
    EnumKeys::P => glfw::Key::P,
    EnumKeys::Q => glfw::Key::Q,
    EnumKeys::R => glfw::Key::R,
    EnumKeys::S => glfw::Key::S,
    EnumKeys::T => glfw::Key::T,
    EnumKeys::U => glfw::Key::U,
    EnumKeys::V => glfw::Key::V,
    EnumKeys::W => glfw::Key::W,
    EnumKeys::X => glfw::Key::X,
    EnumKeys::Y => glfw::Key::Y,
    EnumKeys::Z => glfw::Key::Z,
    EnumKeys::LeftBracket => glfw::Key::LeftBracket,
    EnumKeys::Backslash => glfw::Key::Backslash,
    EnumKeys::RightBracket => glfw::Key::RightBracket,
    EnumKeys::GraveAccent => glfw::Key::GraveAccent,
    EnumKeys::World1 => glfw::Key::World1,
    EnumKeys::World2 => glfw::Key::World2,
    EnumKeys::Escape => glfw::Key::Escape,
    EnumKeys::Enter => glfw::Key::Enter,
    EnumKeys::Tab => glfw::Key::Tab,
    EnumKeys::Backspace => glfw::Key::Backspace,
    EnumKeys::Insert => glfw::Key::Insert,
    EnumKeys::Delete => glfw::Key::Delete,
    EnumKeys::Right => glfw::Key::Right,
    EnumKeys::Left => glfw::Key::Left,
    EnumKeys::Down => glfw::Key::Down,
    EnumKeys::Up => glfw::Key::Up,
    EnumKeys::PageUp => glfw::Key::PageUp,
    EnumKeys::PageDown => glfw::Key::PageDown,
    EnumKeys::Home => glfw::Key::Home,
    EnumKeys::End => glfw::Key::End,
    EnumKeys::CapsLock => glfw::Key::CapsLock,
    EnumKeys::ScrollLock => glfw::Key::ScrollLock,
    EnumKeys::NumLock => glfw::Key::NumLock,
    EnumKeys::PrintScreen => glfw::Key::PrintScreen,
    EnumKeys::Pause => glfw::Key::Pause,
    EnumKeys::F1 => glfw::Key::F1,
    EnumKeys::F2 => glfw::Key::F2,
    EnumKeys::F3 => glfw::Key::F3,
    EnumKeys::F4 => glfw::Key::F4,
    EnumKeys::F5 => glfw::Key::F5,
    EnumKeys::F6 => glfw::Key::F6,
    EnumKeys::F7 => glfw::Key::F7,
    EnumKeys::F8 => glfw::Key::F8,
    EnumKeys::F9 => glfw::Key::F9,
    EnumKeys::F10 => glfw::Key::F10,
    EnumKeys::F11 => glfw::Key::F11,
    EnumKeys::F12 => glfw::Key::F12,
    EnumKeys::F13 => glfw::Key::F13,
    EnumKeys::F14 => glfw::Key::F14,
    EnumKeys::F15 => glfw::Key::F15,
    EnumKeys::F16 => glfw::Key::F16,
    EnumKeys::F17 => glfw::Key::F17,
    EnumKeys::F18 => glfw::Key::F18,
    EnumKeys::F19 => glfw::Key::F19,
    EnumKeys::F20 => glfw::Key::F20,
    EnumKeys::F21 => glfw::Key::F21,
    EnumKeys::F22 => glfw::Key::F22,
    EnumKeys::F23 => glfw::Key::F23,
    EnumKeys::F24 => glfw::Key::F24,
    EnumKeys::F25 => glfw::Key::F25,
    EnumKeys::Kp0 => glfw::Key::Kp0,
    EnumKeys::Kp1 => glfw::Key::Kp1,
    EnumKeys::Kp2 => glfw::Key::Kp2,
    EnumKeys::Kp3 => glfw::Key::Kp3,
    EnumKeys::Kp4 => glfw::Key::Kp4,
    EnumKeys::Kp5 => glfw::Key::Kp5,
    EnumKeys::Kp6 => glfw::Key::Kp6,
    EnumKeys::Kp7 => glfw::Key::Kp7,
    EnumKeys::Kp8 => glfw::Key::Kp8,
    EnumKeys::Kp9 => glfw::Key::Kp9,
    EnumKeys::KpDecimal => glfw::Key::KpDecimal,
    EnumKeys::KpDivide => glfw::Key::KpDivide,
    EnumKeys::KpMultiply => glfw::Key::KpMultiply,
    EnumKeys::KpSubtract => glfw::Key::KpSubtract,
    EnumKeys::KpAdd => glfw::Key::KpAdd,
    EnumKeys::KpEnter => glfw::Key::KpEnter,
    EnumKeys::KpEqual => glfw::Key::KpEqual,
    EnumKeys::LeftShift => glfw::Key::LeftShift,
    EnumKeys::LeftControl => glfw::Key::LeftControl,
    EnumKeys::LeftAlt => glfw::Key::LeftAlt,
    EnumKeys::LeftSuper => glfw::Key::LeftSuper,
    EnumKeys::RightShift => glfw::Key::RightShift,
    EnumKeys::RightControl => glfw::Key::RightControl,
    EnumKeys::RightAlt => glfw::Key::RightAlt,
    EnumKeys::RightSuper => glfw::Key::RightSuper,
    EnumKeys::Menu => glfw::Key::Menu,
    _ => glfw::Key::Unknown
  };
}

fn convert_api_key_to_key(api_key: glfw::Key) -> EnumKeys {
  return match api_key {
    glfw::Key::Space => EnumKeys::Space,
    glfw::Key::Apostrophe => EnumKeys::Apostrophe,
    glfw::Key::Comma => EnumKeys::Comma,
    glfw::Key::Minus => EnumKeys::Minus,
    glfw::Key::Period => EnumKeys::Period,
    glfw::Key::Slash => EnumKeys::Slash,
    glfw::Key::Num0 => EnumKeys::Num0,
    glfw::Key::Num1 => EnumKeys::Num1,
    glfw::Key::Num2 => EnumKeys::Num2,
    glfw::Key::Num3 => EnumKeys::Num3,
    glfw::Key::Num4 => EnumKeys::Num4,
    glfw::Key::Num5 => EnumKeys::Num5,
    glfw::Key::Num6 => EnumKeys::Num6,
    glfw::Key::Num7 => EnumKeys::Num7,
    glfw::Key::Num8 => EnumKeys::Num8,
    glfw::Key::Num9 => EnumKeys::Num9,
    glfw::Key::Semicolon => EnumKeys::Semicolon,
    glfw::Key::Equal => EnumKeys::Equal,
    glfw::Key::A => EnumKeys::A,
    glfw::Key::B => EnumKeys::B,
    glfw::Key::C => EnumKeys::C,
    glfw::Key::D => EnumKeys::D,
    glfw::Key::E => EnumKeys::E,
    glfw::Key::F => EnumKeys::F,
    glfw::Key::G => EnumKeys::G,
    glfw::Key::H => EnumKeys::H,
    glfw::Key::I => EnumKeys::I,
    glfw::Key::J => EnumKeys::J,
    glfw::Key::K => EnumKeys::K,
    glfw::Key::L => EnumKeys::L,
    glfw::Key::M => EnumKeys::M,
    glfw::Key::N => EnumKeys::N,
    glfw::Key::O => EnumKeys::O,
    glfw::Key::P => EnumKeys::P,
    glfw::Key::Q => EnumKeys::Q,
    glfw::Key::R => EnumKeys::R,
    glfw::Key::S => EnumKeys::S,
    glfw::Key::T => EnumKeys::T,
    glfw::Key::U => EnumKeys::U,
    glfw::Key::V => EnumKeys::V,
    glfw::Key::W => EnumKeys::W,
    glfw::Key::X => EnumKeys::X,
    glfw::Key::Y => EnumKeys::Y,
    glfw::Key::Z => EnumKeys::Z,
    glfw::Key::LeftBracket => EnumKeys::LeftBracket,
    glfw::Key::Backslash => EnumKeys::Backslash,
    glfw::Key::RightBracket => EnumKeys::RightBracket,
    glfw::Key::GraveAccent => EnumKeys::GraveAccent,
    glfw::Key::World1 => EnumKeys::World1,
    glfw::Key::World2 => EnumKeys::World2,
    glfw::Key::Escape => EnumKeys::Escape,
    glfw::Key::Enter => EnumKeys::Enter,
    glfw::Key::Tab => EnumKeys::Tab,
    glfw::Key::Backspace => EnumKeys::Backspace,
    glfw::Key::Insert => EnumKeys::Insert,
    glfw::Key::Delete => EnumKeys::Delete,
    glfw::Key::Right => EnumKeys::Right,
    glfw::Key::Left => EnumKeys::Left,
    glfw::Key::Down => EnumKeys::Down,
    glfw::Key::Up => EnumKeys::Up,
    glfw::Key::PageUp => EnumKeys::PageUp,
    glfw::Key::PageDown => EnumKeys::PageDown,
    glfw::Key::Home => EnumKeys::Home,
    glfw::Key::End => EnumKeys::End,
    glfw::Key::CapsLock => EnumKeys::CapsLock,
    glfw::Key::ScrollLock => EnumKeys::ScrollLock,
    glfw::Key::NumLock => EnumKeys::NumLock,
    glfw::Key::PrintScreen => EnumKeys::PrintScreen,
    glfw::Key::Pause => EnumKeys::Pause,
    glfw::Key::F1 => EnumKeys::F1,
    glfw::Key::F2 => EnumKeys::F2,
    glfw::Key::F3 => EnumKeys::F3,
    glfw::Key::F4 => EnumKeys::F4,
    glfw::Key::F5 => EnumKeys::F5,
    glfw::Key::F6 => EnumKeys::F6,
    glfw::Key::F7 => EnumKeys::F7,
    glfw::Key::F8 => EnumKeys::F8,
    glfw::Key::F9 => EnumKeys::F9,
    glfw::Key::F10 => EnumKeys::F10,
    glfw::Key::F11 => EnumKeys::F11,
    glfw::Key::F12 => EnumKeys::F12,
    glfw::Key::F13 => EnumKeys::F13,
    glfw::Key::F14 => EnumKeys::F14,
    glfw::Key::F15 => EnumKeys::F15,
    glfw::Key::F16 => EnumKeys::F16,
    glfw::Key::F17 => EnumKeys::F17,
    glfw::Key::F18 => EnumKeys::F18,
    glfw::Key::F19 => EnumKeys::F19,
    glfw::Key::F20 => EnumKeys::F20,
    glfw::Key::F21 => EnumKeys::F21,
    glfw::Key::F22 => EnumKeys::F22,
    glfw::Key::F23 => EnumKeys::F23,
    glfw::Key::F24 => EnumKeys::F24,
    glfw::Key::F25 => EnumKeys::F25,
    glfw::Key::Kp0 => EnumKeys::Kp0,
    glfw::Key::Kp1 => EnumKeys::Kp1,
    glfw::Key::Kp2 => EnumKeys::Kp2,
    glfw::Key::Kp3 => EnumKeys::Kp3,
    glfw::Key::Kp4 => EnumKeys::Kp4,
    glfw::Key::Kp5 => EnumKeys::Kp5,
    glfw::Key::Kp6 => EnumKeys::Kp6,
    glfw::Key::Kp7 => EnumKeys::Kp7,
    glfw::Key::Kp8 => EnumKeys::Kp8,
    glfw::Key::Kp9 => EnumKeys::Kp9,
    glfw::Key::KpDecimal => EnumKeys::KpDecimal,
    glfw::Key::KpDivide => EnumKeys::KpDivide,
    glfw::Key::KpMultiply => EnumKeys::KpMultiply,
    glfw::Key::KpSubtract => EnumKeys::KpSubtract,
    glfw::Key::KpAdd => EnumKeys::KpAdd,
    glfw::Key::KpEnter => EnumKeys::KpEnter,
    glfw::Key::KpEqual => EnumKeys::KpEqual,
    glfw::Key::LeftShift => EnumKeys::LeftShift,
    glfw::Key::LeftControl => EnumKeys::LeftControl,
    glfw::Key::LeftAlt => EnumKeys::LeftAlt,
    glfw::Key::LeftSuper => EnumKeys::LeftSuper,
    glfw::Key::RightShift => EnumKeys::RightShift,
    glfw::Key::RightControl => EnumKeys::RightControl,
    glfw::Key::RightAlt => EnumKeys::RightAlt,
    glfw::Key::RightSuper => EnumKeys::RightSuper,
    glfw::Key::Menu => EnumKeys::Menu,
    glfw::Key::Unknown => EnumKeys::Unknown
  };
}

fn convert_mouse_btn_to_api_mouse_btn(enum_mouse_button: EnumMouseButtons) -> glfw::MouseButton {
  return match enum_mouse_button {
    EnumMouseButtons::Button1 => glfw::MouseButton::Button1,
    EnumMouseButtons::Button2 => glfw::MouseButton::Button2,
    EnumMouseButtons::Button3 => glfw::MouseButton::Button3,
    EnumMouseButtons::Button4 => glfw::MouseButton::Button4,
    EnumMouseButtons::Button5 => glfw::MouseButton::Button5,
    EnumMouseButtons::Button6 => glfw::MouseButton::Button6,
    EnumMouseButtons::Button7 => glfw::MouseButton::Button7,
    EnumMouseButtons::Button8 => glfw::MouseButton::Button8
  };
}

fn convert_api_mouse_btn_to_mouse_btn(api_mouse_button: glfw::MouseButton) -> EnumMouseButtons {
  return match api_mouse_button {
    glfw::MouseButton::Button1 => EnumMouseButtons::Button1,
    glfw::MouseButton::Button2 => EnumMouseButtons::Button2,
    glfw::MouseButton::Button3 => EnumMouseButtons::Button3,
    glfw::MouseButton::Button4 => EnumMouseButtons::Button4,
    glfw::MouseButton::Button5 => EnumMouseButtons::Button5,
    glfw::MouseButton::Button6 => EnumMouseButtons::Button6,
    glfw::MouseButton::Button7 => EnumMouseButtons::Button7,
    glfw::MouseButton::Button8 => EnumMouseButtons::Button8
  };
}

fn convert_action_to_api_action(enum_action: EnumAction) -> glfw::Action {
  match enum_action {
    EnumAction::Release => glfw::Action::Release,
    EnumAction::Press => glfw::Action::Press,
    EnumAction::Hold => glfw::Action::Repeat
  }
}

fn convert_api_action_to_action(api_action: glfw::Action) -> EnumAction {
  match api_action {
    glfw::Action::Release => EnumAction::Release,
    glfw::Action::Press => EnumAction::Press,
    glfw::Action::Repeat => EnumAction::Hold,
  }
}

impl Display for EnumErrors {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Input] -->\t Error encountered with input(s) : {:?}", self)
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Input {}

impl Input {
  pub fn on_update() {
    unsafe {
      if S_WINDOW.is_some() {
        for key in 0..S_KEY_STATES.len() {
          let old_state = S_KEY_STATES[key].1;
          S_KEY_STATES[key] = (old_state, EnumAction::Release);
        }
      }
    }
  }
  
  // KEY QUERY FUNCTIONS.
  pub fn get_key_state(key_code: EnumKeys, key_action: EnumAction) -> Result<bool, EnumErrors> {
    let api_key = convert_key_to_api_key(key_code);
    let old_state: EnumAction = unsafe {
      S_KEY_STATES[api_key as usize].0
    };
    
    if unsafe { S_WINDOW.is_none() } {
      return Err(EnumErrors::InvalidWindowContext);
    }
    
    let new_state = unsafe {
      (*S_WINDOW.unwrap()).m_api_window.get_key(api_key)
    };
    
    unsafe { S_KEY_STATES[api_key as usize] = (old_state, EnumAction::from(new_state)) };
    
    return match key_action {
      EnumAction::Release => {
        Ok(old_state == EnumAction::Press &&
          new_state == glfw::Action::from(EnumAction::Release))
      }
      EnumAction::Press => {
        Ok(old_state == EnumAction::Release &&
          new_state == glfw::Action::from(EnumAction::Press))
      }
      EnumAction::Hold => {
        Ok(old_state == EnumAction::Press &&
          new_state == glfw::Action::from(EnumAction::Press))
      }
    };
  }
  
  pub fn get_key_name(key_code: EnumKeys) -> Result<String, EnumErrors> {
    let api_key = convert_key_to_api_key(key_code);
    if api_key.get_name().is_some() {
      return Ok(api_key.get_name().unwrap());
    }
    log!(EnumLogColor::Red, "ERROR", "[Input] -->\t Cannot retrieve key name from {:?} : \
    Invalid key code!", api_key);
    return Err(EnumErrors::InvalidKey);
  }
  
  pub fn are_keys_pressed(first_key: EnumKeys, second_key: EnumKeys, third_key: Option<EnumKeys>) -> Result<bool, EnumErrors> {
    if third_key.is_some() {
      return
        // Pressed down first key while holding down second key and third key.
        Ok((Input::get_key_state(first_key, EnumAction::Press)? &&
          Input::get_key_state(second_key, EnumAction::Hold)? &&
          Input::get_key_state(third_key.unwrap(), EnumAction::Hold)?) ||
          // Pressed down second key while holding down first key and third key.
          (Input::get_key_state(second_key, EnumAction::Press)? &&
            Input::get_key_state(first_key, EnumAction::Hold)? &&
            Input::get_key_state(third_key.unwrap(), EnumAction::Hold)?) ||
          // Pressed down third key while holding down first key and second key.
          (Input::get_key_state(third_key.unwrap(), EnumAction::Press)? &&
            Input::get_key_state(first_key, EnumAction::Hold)? &&
            Input::get_key_state(second_key, EnumAction::Hold)?));
    }
    return
      // Pressed down first key while holding down second key.
      Ok((Input::get_key_state(first_key, EnumAction::Press)? &&
        Input::get_key_state(second_key, EnumAction::Hold)?) ||
        // Pressed down second key while holding down first key.
        (Input::get_key_state(second_key, EnumAction::Press)? &&
          Input::get_key_state(first_key, EnumAction::Hold)?));
  }
  
  pub fn are_keys_held(first_key: EnumKeys, second_key: EnumKeys, third_key: Option<EnumKeys>) -> Result<bool, EnumErrors> {
    return Ok(Input::get_key_state(first_key, EnumAction::Hold)? &&
      Input::get_key_state(second_key, EnumAction::Hold)? &&
      third_key.is_none().then(|| {
        return true;
      }).unwrap_or(Input::get_key_state(third_key.unwrap(), EnumAction::Hold)?));
  }
  
  pub fn are_keys_released(first_key: EnumKeys, second_key: EnumKeys, third_key: Option<EnumKeys>) -> Result<bool, EnumErrors> {
    return Ok(Input::get_key_state(first_key, EnumAction::Release)? &&
      Input::get_key_state(second_key, EnumAction::Release)? &&
      third_key.is_none().then(|| {
        return true;
      }).unwrap_or(Input::get_key_state(third_key.unwrap(), EnumAction::Release)?));
  }
  
  // MOUSE BUTTON QUERY FUNCTIONS.
  pub fn get_mouse_button_state(mouse_button: EnumMouseButtons, mouse_button_action: EnumAction) -> Result<bool, EnumErrors> {
    let api_mouse_button = convert_mouse_btn_to_api_mouse_btn(mouse_button);
    
    if unsafe { S_WINDOW.is_none() } {
      return Err(EnumErrors::InvalidWindowContext);
    }
    
    let old_state = unsafe {
      S_MOUSE_BUTTON_STATES[api_mouse_button as usize].0
    };
    let new_state = unsafe {
      (*S_WINDOW.unwrap()).m_api_window.get_mouse_button(api_mouse_button)
    };
    
    unsafe { S_MOUSE_BUTTON_STATES[api_mouse_button as usize] = (old_state, EnumAction::from(new_state)) };
    
    return match mouse_button_action {
      EnumAction::Release => {
        Ok(old_state == EnumAction::Press &&
          new_state == glfw::Action::from(EnumAction::Release))
      }
      EnumAction::Press => {
        Ok(old_state == EnumAction::Release &&
          new_state == glfw::Action::from(EnumAction::Press))
      }
      EnumAction::Hold => {
        Ok(old_state == EnumAction::Press &&
          new_state == glfw::Action::from(EnumAction::Press))
      }
    };
  }
  
  // MOUSE MOVEMENT STATIC FUNCTIONS.
  pub fn get_mouse_cursor_position() {
    todo!()
  }
  
  pub fn get_mouse_cursor_attribute() -> Result<glfw::CursorMode, EnumErrors> {
    unsafe {
      if S_WINDOW_CONTEXT.is_some() {
        return Ok((*S_WINDOW.unwrap()).m_api_window.get_cursor_mode());
      }
    }
    return Err(EnumErrors::InvalidWindowContext);
  }
  
  pub fn set_mouse_cursor_attribute(cursor_mode: glfw::CursorMode) -> Result<(), EnumErrors> {
    unsafe {
      if S_WINDOW.is_some() {
        return Ok((*S_WINDOW.unwrap()).m_api_window.set_cursor_mode(cursor_mode));
      }
    }
    return Err(EnumErrors::InvalidWindowContext);
  }
  
  pub fn set_mouse_cursor_position(cursor_position: Vec2<f32>) -> Result<(), EnumErrors> {
    unsafe {
      if S_WINDOW_CONTEXT.is_some() {
        return Ok((*S_WINDOW.unwrap()).m_api_window
          .set_cursor_pos(cursor_position.x as f64, cursor_position.y as f64));
      }
    }
    return Err(EnumErrors::InvalidWindowContext);
  }
}