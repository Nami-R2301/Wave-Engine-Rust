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
pub enum EnumError {
  InvalidWindowContext,
  InvalidKey,
  InvalidMouseButton,
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum EnumKey {
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

impl From<glfw::Key> for EnumKey {
  fn from(value: glfw::Key) -> Self {
    return convert_api_key_to_key(value);
  }
}

impl From<EnumKey> for glfw::Key {
  fn from(value: EnumKey) -> Self {
    return convert_key_to_api_key(value);
  }
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum EnumMouseButton {
  LeftButton = glfw::ffi::MOUSE_BUTTON_1,
  RightButton = glfw::ffi::MOUSE_BUTTON_2,
  MiddleButton = glfw::ffi::MOUSE_BUTTON_3,
  Button4 = glfw::ffi::MOUSE_BUTTON_4,
  Button5 = glfw::ffi::MOUSE_BUTTON_5,
  Button6 = glfw::ffi::MOUSE_BUTTON_6,
  Button7 = glfw::ffi::MOUSE_BUTTON_7,
  Button8 = glfw::ffi::MOUSE_BUTTON_8,
}

impl From<glfw::MouseButton> for EnumMouseButton {
  fn from(value: glfw::MouseButton) -> Self {
    return convert_api_mouse_btn_to_mouse_btn(value);
  }
}

impl From<EnumMouseButton> for glfw::MouseButton {
  fn from(value: EnumMouseButton) -> Self {
    return convert_mouse_btn_to_api_mouse_btn(value);
  }
}

#[repr(i32)]
#[doc = "Key events for each key input."]
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


#[doc = "Key modifiers (e.g., Shift, Control, Alt, Super)."]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum EnumModifier {
  Shift,
  Control,
  Alt,
  Super,
  CapsLock,
  NumLock
}

fn convert_key_to_api_key(enum_key: EnumKey) -> glfw::Key {
  return match enum_key {
    EnumKey::Space => glfw::Key::Space,
    EnumKey::Apostrophe => glfw::Key::Apostrophe,
    EnumKey::Comma => glfw::Key::Comma,
    EnumKey::Minus => glfw::Key::Minus,
    EnumKey::Period => glfw::Key::Period,
    EnumKey::Slash => glfw::Key::Slash,
    EnumKey::Num0 => glfw::Key::Num0,
    EnumKey::Num1 => glfw::Key::Num1,
    EnumKey::Num2 => glfw::Key::Num2,
    EnumKey::Num3 => glfw::Key::Num3,
    EnumKey::Num4 => glfw::Key::Num4,
    EnumKey::Num5 => glfw::Key::Num5,
    EnumKey::Num6 => glfw::Key::Num6,
    EnumKey::Num7 => glfw::Key::Num7,
    EnumKey::Num8 => glfw::Key::Num8,
    EnumKey::Num9 => glfw::Key::Num9,
    EnumKey::Semicolon => glfw::Key::Semicolon,
    EnumKey::Equal => glfw::Key::Equal,
    EnumKey::A => glfw::Key::A,
    EnumKey::B => glfw::Key::B,
    EnumKey::C => glfw::Key::C,
    EnumKey::D => glfw::Key::D,
    EnumKey::E => glfw::Key::E,
    EnumKey::F => glfw::Key::F,
    EnumKey::G => glfw::Key::G,
    EnumKey::H => glfw::Key::H,
    EnumKey::I => glfw::Key::I,
    EnumKey::J => glfw::Key::J,
    EnumKey::K => glfw::Key::K,
    EnumKey::L => glfw::Key::L,
    EnumKey::M => glfw::Key::M,
    EnumKey::N => glfw::Key::N,
    EnumKey::O => glfw::Key::O,
    EnumKey::P => glfw::Key::P,
    EnumKey::Q => glfw::Key::Q,
    EnumKey::R => glfw::Key::R,
    EnumKey::S => glfw::Key::S,
    EnumKey::T => glfw::Key::T,
    EnumKey::U => glfw::Key::U,
    EnumKey::V => glfw::Key::V,
    EnumKey::W => glfw::Key::W,
    EnumKey::X => glfw::Key::X,
    EnumKey::Y => glfw::Key::Y,
    EnumKey::Z => glfw::Key::Z,
    EnumKey::LeftBracket => glfw::Key::LeftBracket,
    EnumKey::Backslash => glfw::Key::Backslash,
    EnumKey::RightBracket => glfw::Key::RightBracket,
    EnumKey::GraveAccent => glfw::Key::GraveAccent,
    EnumKey::World1 => glfw::Key::World1,
    EnumKey::World2 => glfw::Key::World2,
    EnumKey::Escape => glfw::Key::Escape,
    EnumKey::Enter => glfw::Key::Enter,
    EnumKey::Tab => glfw::Key::Tab,
    EnumKey::Backspace => glfw::Key::Backspace,
    EnumKey::Insert => glfw::Key::Insert,
    EnumKey::Delete => glfw::Key::Delete,
    EnumKey::Right => glfw::Key::Right,
    EnumKey::Left => glfw::Key::Left,
    EnumKey::Down => glfw::Key::Down,
    EnumKey::Up => glfw::Key::Up,
    EnumKey::PageUp => glfw::Key::PageUp,
    EnumKey::PageDown => glfw::Key::PageDown,
    EnumKey::Home => glfw::Key::Home,
    EnumKey::End => glfw::Key::End,
    EnumKey::CapsLock => glfw::Key::CapsLock,
    EnumKey::ScrollLock => glfw::Key::ScrollLock,
    EnumKey::NumLock => glfw::Key::NumLock,
    EnumKey::PrintScreen => glfw::Key::PrintScreen,
    EnumKey::Pause => glfw::Key::Pause,
    EnumKey::F1 => glfw::Key::F1,
    EnumKey::F2 => glfw::Key::F2,
    EnumKey::F3 => glfw::Key::F3,
    EnumKey::F4 => glfw::Key::F4,
    EnumKey::F5 => glfw::Key::F5,
    EnumKey::F6 => glfw::Key::F6,
    EnumKey::F7 => glfw::Key::F7,
    EnumKey::F8 => glfw::Key::F8,
    EnumKey::F9 => glfw::Key::F9,
    EnumKey::F10 => glfw::Key::F10,
    EnumKey::F11 => glfw::Key::F11,
    EnumKey::F12 => glfw::Key::F12,
    EnumKey::F13 => glfw::Key::F13,
    EnumKey::F14 => glfw::Key::F14,
    EnumKey::F15 => glfw::Key::F15,
    EnumKey::F16 => glfw::Key::F16,
    EnumKey::F17 => glfw::Key::F17,
    EnumKey::F18 => glfw::Key::F18,
    EnumKey::F19 => glfw::Key::F19,
    EnumKey::F20 => glfw::Key::F20,
    EnumKey::F21 => glfw::Key::F21,
    EnumKey::F22 => glfw::Key::F22,
    EnumKey::F23 => glfw::Key::F23,
    EnumKey::F24 => glfw::Key::F24,
    EnumKey::F25 => glfw::Key::F25,
    EnumKey::Kp0 => glfw::Key::Kp0,
    EnumKey::Kp1 => glfw::Key::Kp1,
    EnumKey::Kp2 => glfw::Key::Kp2,
    EnumKey::Kp3 => glfw::Key::Kp3,
    EnumKey::Kp4 => glfw::Key::Kp4,
    EnumKey::Kp5 => glfw::Key::Kp5,
    EnumKey::Kp6 => glfw::Key::Kp6,
    EnumKey::Kp7 => glfw::Key::Kp7,
    EnumKey::Kp8 => glfw::Key::Kp8,
    EnumKey::Kp9 => glfw::Key::Kp9,
    EnumKey::KpDecimal => glfw::Key::KpDecimal,
    EnumKey::KpDivide => glfw::Key::KpDivide,
    EnumKey::KpMultiply => glfw::Key::KpMultiply,
    EnumKey::KpSubtract => glfw::Key::KpSubtract,
    EnumKey::KpAdd => glfw::Key::KpAdd,
    EnumKey::KpEnter => glfw::Key::KpEnter,
    EnumKey::KpEqual => glfw::Key::KpEqual,
    EnumKey::LeftShift => glfw::Key::LeftShift,
    EnumKey::LeftControl => glfw::Key::LeftControl,
    EnumKey::LeftAlt => glfw::Key::LeftAlt,
    EnumKey::LeftSuper => glfw::Key::LeftSuper,
    EnumKey::RightShift => glfw::Key::RightShift,
    EnumKey::RightControl => glfw::Key::RightControl,
    EnumKey::RightAlt => glfw::Key::RightAlt,
    EnumKey::RightSuper => glfw::Key::RightSuper,
    EnumKey::Menu => glfw::Key::Menu,
    _ => glfw::Key::Unknown
  };
}

fn convert_api_key_to_key(api_key: glfw::Key) -> EnumKey {
  return match api_key {
    glfw::Key::Space => EnumKey::Space,
    glfw::Key::Apostrophe => EnumKey::Apostrophe,
    glfw::Key::Comma => EnumKey::Comma,
    glfw::Key::Minus => EnumKey::Minus,
    glfw::Key::Period => EnumKey::Period,
    glfw::Key::Slash => EnumKey::Slash,
    glfw::Key::Num0 => EnumKey::Num0,
    glfw::Key::Num1 => EnumKey::Num1,
    glfw::Key::Num2 => EnumKey::Num2,
    glfw::Key::Num3 => EnumKey::Num3,
    glfw::Key::Num4 => EnumKey::Num4,
    glfw::Key::Num5 => EnumKey::Num5,
    glfw::Key::Num6 => EnumKey::Num6,
    glfw::Key::Num7 => EnumKey::Num7,
    glfw::Key::Num8 => EnumKey::Num8,
    glfw::Key::Num9 => EnumKey::Num9,
    glfw::Key::Semicolon => EnumKey::Semicolon,
    glfw::Key::Equal => EnumKey::Equal,
    glfw::Key::A => EnumKey::A,
    glfw::Key::B => EnumKey::B,
    glfw::Key::C => EnumKey::C,
    glfw::Key::D => EnumKey::D,
    glfw::Key::E => EnumKey::E,
    glfw::Key::F => EnumKey::F,
    glfw::Key::G => EnumKey::G,
    glfw::Key::H => EnumKey::H,
    glfw::Key::I => EnumKey::I,
    glfw::Key::J => EnumKey::J,
    glfw::Key::K => EnumKey::K,
    glfw::Key::L => EnumKey::L,
    glfw::Key::M => EnumKey::M,
    glfw::Key::N => EnumKey::N,
    glfw::Key::O => EnumKey::O,
    glfw::Key::P => EnumKey::P,
    glfw::Key::Q => EnumKey::Q,
    glfw::Key::R => EnumKey::R,
    glfw::Key::S => EnumKey::S,
    glfw::Key::T => EnumKey::T,
    glfw::Key::U => EnumKey::U,
    glfw::Key::V => EnumKey::V,
    glfw::Key::W => EnumKey::W,
    glfw::Key::X => EnumKey::X,
    glfw::Key::Y => EnumKey::Y,
    glfw::Key::Z => EnumKey::Z,
    glfw::Key::LeftBracket => EnumKey::LeftBracket,
    glfw::Key::Backslash => EnumKey::Backslash,
    glfw::Key::RightBracket => EnumKey::RightBracket,
    glfw::Key::GraveAccent => EnumKey::GraveAccent,
    glfw::Key::World1 => EnumKey::World1,
    glfw::Key::World2 => EnumKey::World2,
    glfw::Key::Escape => EnumKey::Escape,
    glfw::Key::Enter => EnumKey::Enter,
    glfw::Key::Tab => EnumKey::Tab,
    glfw::Key::Backspace => EnumKey::Backspace,
    glfw::Key::Insert => EnumKey::Insert,
    glfw::Key::Delete => EnumKey::Delete,
    glfw::Key::Right => EnumKey::Right,
    glfw::Key::Left => EnumKey::Left,
    glfw::Key::Down => EnumKey::Down,
    glfw::Key::Up => EnumKey::Up,
    glfw::Key::PageUp => EnumKey::PageUp,
    glfw::Key::PageDown => EnumKey::PageDown,
    glfw::Key::Home => EnumKey::Home,
    glfw::Key::End => EnumKey::End,
    glfw::Key::CapsLock => EnumKey::CapsLock,
    glfw::Key::ScrollLock => EnumKey::ScrollLock,
    glfw::Key::NumLock => EnumKey::NumLock,
    glfw::Key::PrintScreen => EnumKey::PrintScreen,
    glfw::Key::Pause => EnumKey::Pause,
    glfw::Key::F1 => EnumKey::F1,
    glfw::Key::F2 => EnumKey::F2,
    glfw::Key::F3 => EnumKey::F3,
    glfw::Key::F4 => EnumKey::F4,
    glfw::Key::F5 => EnumKey::F5,
    glfw::Key::F6 => EnumKey::F6,
    glfw::Key::F7 => EnumKey::F7,
    glfw::Key::F8 => EnumKey::F8,
    glfw::Key::F9 => EnumKey::F9,
    glfw::Key::F10 => EnumKey::F10,
    glfw::Key::F11 => EnumKey::F11,
    glfw::Key::F12 => EnumKey::F12,
    glfw::Key::F13 => EnumKey::F13,
    glfw::Key::F14 => EnumKey::F14,
    glfw::Key::F15 => EnumKey::F15,
    glfw::Key::F16 => EnumKey::F16,
    glfw::Key::F17 => EnumKey::F17,
    glfw::Key::F18 => EnumKey::F18,
    glfw::Key::F19 => EnumKey::F19,
    glfw::Key::F20 => EnumKey::F20,
    glfw::Key::F21 => EnumKey::F21,
    glfw::Key::F22 => EnumKey::F22,
    glfw::Key::F23 => EnumKey::F23,
    glfw::Key::F24 => EnumKey::F24,
    glfw::Key::F25 => EnumKey::F25,
    glfw::Key::Kp0 => EnumKey::Kp0,
    glfw::Key::Kp1 => EnumKey::Kp1,
    glfw::Key::Kp2 => EnumKey::Kp2,
    glfw::Key::Kp3 => EnumKey::Kp3,
    glfw::Key::Kp4 => EnumKey::Kp4,
    glfw::Key::Kp5 => EnumKey::Kp5,
    glfw::Key::Kp6 => EnumKey::Kp6,
    glfw::Key::Kp7 => EnumKey::Kp7,
    glfw::Key::Kp8 => EnumKey::Kp8,
    glfw::Key::Kp9 => EnumKey::Kp9,
    glfw::Key::KpDecimal => EnumKey::KpDecimal,
    glfw::Key::KpDivide => EnumKey::KpDivide,
    glfw::Key::KpMultiply => EnumKey::KpMultiply,
    glfw::Key::KpSubtract => EnumKey::KpSubtract,
    glfw::Key::KpAdd => EnumKey::KpAdd,
    glfw::Key::KpEnter => EnumKey::KpEnter,
    glfw::Key::KpEqual => EnumKey::KpEqual,
    glfw::Key::LeftShift => EnumKey::LeftShift,
    glfw::Key::LeftControl => EnumKey::LeftControl,
    glfw::Key::LeftAlt => EnumKey::LeftAlt,
    glfw::Key::LeftSuper => EnumKey::LeftSuper,
    glfw::Key::RightShift => EnumKey::RightShift,
    glfw::Key::RightControl => EnumKey::RightControl,
    glfw::Key::RightAlt => EnumKey::RightAlt,
    glfw::Key::RightSuper => EnumKey::RightSuper,
    glfw::Key::Menu => EnumKey::Menu,
    glfw::Key::Unknown => EnumKey::Unknown
  };
}

fn convert_mouse_btn_to_api_mouse_btn(enum_mouse_button: EnumMouseButton) -> glfw::MouseButton {
  return match enum_mouse_button {
    EnumMouseButton::LeftButton => glfw::MouseButton::Button1,
    EnumMouseButton::RightButton => glfw::MouseButton::Button2,
    EnumMouseButton::MiddleButton => glfw::MouseButton::Button3,
    EnumMouseButton::Button4 => glfw::MouseButton::Button4,
    EnumMouseButton::Button5 => glfw::MouseButton::Button5,
    EnumMouseButton::Button6 => glfw::MouseButton::Button6,
    EnumMouseButton::Button7 => glfw::MouseButton::Button7,
    EnumMouseButton::Button8 => glfw::MouseButton::Button8
  };
}

fn convert_api_mouse_btn_to_mouse_btn(api_mouse_button: glfw::MouseButton) -> EnumMouseButton {
  return match api_mouse_button {
    glfw::MouseButton::Button1 => EnumMouseButton::LeftButton,
    glfw::MouseButton::Button2 => EnumMouseButton::RightButton,
    glfw::MouseButton::Button3 => EnumMouseButton::MiddleButton,
    glfw::MouseButton::Button4 => EnumMouseButton::Button4,
    glfw::MouseButton::Button5 => EnumMouseButton::Button5,
    glfw::MouseButton::Button6 => EnumMouseButton::Button6,
    glfw::MouseButton::Button7 => EnumMouseButton::Button7,
    glfw::MouseButton::Button8 => EnumMouseButton::Button8
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

impl Display for EnumError {
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
        
        for mouse_button in 0..S_MOUSE_BUTTON_STATES.len() {
          let old_state = S_MOUSE_BUTTON_STATES[mouse_button].1;
          S_MOUSE_BUTTON_STATES[mouse_button] = (old_state, EnumAction::Release);
        }
      }
    }
  }
  
  // KEY QUERY FUNCTIONS.
  pub fn get_key_state(key_code: EnumKey, key_action: EnumAction) -> Result<bool, EnumError> {
    let api_key = convert_key_to_api_key(key_code);
    let old_state: EnumAction = unsafe {
      S_KEY_STATES[api_key as usize].0
    };
    
    if unsafe { S_WINDOW.is_none() } {
      return Err(EnumError::InvalidWindowContext);
    }
    
    let new_state = unsafe {
      (*S_WINDOW.unwrap()).m_api_window.get_key(api_key)
    };
    
    unsafe { S_KEY_STATES[api_key as usize] = (old_state, EnumAction::from(new_state)) };
    
    return match key_action {
      EnumAction::Release => {
        Ok(old_state == EnumAction::Press &&
          new_state == glfw::Action::Release)
      }
      EnumAction::Press => {
        Ok(old_state == EnumAction::Release &&
          (new_state == glfw::Action::Press && new_state != glfw::Action::Repeat))
      }
      EnumAction::Hold => {
        Ok((old_state == EnumAction::Press || old_state == EnumAction::Hold) &&
          (new_state == glfw::Action::Press || new_state == glfw::Action::Repeat))
      }
    };
  }
  
  pub fn get_key_name(key_code: EnumKey) -> Result<String, EnumError> {
    let api_key = convert_key_to_api_key(key_code);
    if api_key.get_name().is_some() {
      return Ok(api_key.get_name().unwrap());
    }
    log!(EnumLogColor::Red, "ERROR", "[Input] -->\t Cannot retrieve key name from {:?} : \
    Invalid key code!", api_key);
    return Err(EnumError::InvalidKey);
  }
  
  pub fn get_modifier_key_combo(first_key: EnumKey, second_key: EnumModifier) -> Result<bool, EnumError> {
    return match second_key {
        EnumModifier::Shift => {
            // Pressed down first key while holding down second key.
            Ok((Input::get_key_state(first_key, EnumAction::Press)? &&
              Input::get_key_state(EnumKey::LeftShift, EnumAction::Hold)?) ||
              // Pressed down second key while holding down first key.
              (Input::get_key_state(EnumKey::LeftShift, EnumAction::Press)? &&
                Input::get_key_state(first_key, EnumAction::Hold)?) ||
              // Right version.
              (Input::get_key_state(first_key, EnumAction::Press)? &&
                Input::get_key_state(EnumKey::RightShift, EnumAction::Hold)?) ||
              // Pressed down second key while holding down first key.
              (Input::get_key_state(EnumKey::RightShift, EnumAction::Press)? &&
                Input::get_key_state(first_key, EnumAction::Hold)?))
        }
        EnumModifier::Control => {
          // Pressed down first key while holding down second key.
          Ok((Input::get_key_state(first_key, EnumAction::Press)? &&
            Input::get_key_state(EnumKey::LeftControl, EnumAction::Hold)?) ||
            // Pressed down second key while holding down first key.
            (Input::get_key_state(EnumKey::LeftControl, EnumAction::Press)? &&
              Input::get_key_state(first_key, EnumAction::Hold)?) ||
            // Right version.
            (Input::get_key_state(first_key, EnumAction::Press)? &&
              Input::get_key_state(EnumKey::RightControl, EnumAction::Hold)?) ||
            // Pressed down second key while holding down first key.
            (Input::get_key_state(EnumKey::RightControl, EnumAction::Press)? &&
              Input::get_key_state(first_key, EnumAction::Hold)?))
        }
        EnumModifier::Alt => {
          // Pressed down first key while holding down second key.
          Ok((Input::get_key_state(first_key, EnumAction::Press)? &&
            Input::get_key_state(EnumKey::LeftAlt, EnumAction::Hold)?) ||
            // Pressed down second key while holding down first key.
            (Input::get_key_state(EnumKey::LeftAlt, EnumAction::Press)? &&
              Input::get_key_state(first_key, EnumAction::Hold)?) ||
            // Right version.
            (Input::get_key_state(first_key, EnumAction::Press)? &&
              Input::get_key_state(EnumKey::RightAlt, EnumAction::Hold)?) ||
            // Pressed down second key while holding down first key.
            (Input::get_key_state(EnumKey::RightAlt, EnumAction::Press)? &&
              Input::get_key_state(first_key, EnumAction::Hold)?))
        }
        EnumModifier::Super => {
          // Pressed down first key while holding down second key.
          Ok((Input::get_key_state(first_key, EnumAction::Press)? &&
            Input::get_key_state(EnumKey::LeftSuper, EnumAction::Hold)?) ||
            // Pressed down second key while holding down first key.
            (Input::get_key_state(EnumKey::LeftSuper, EnumAction::Press)? &&
              Input::get_key_state(first_key, EnumAction::Hold)?) ||
            // Right version.
            (Input::get_key_state(first_key, EnumAction::Press)? &&
              Input::get_key_state(EnumKey::RightSuper, EnumAction::Hold)?) ||
            // Pressed down second key while holding down first key.
            (Input::get_key_state(EnumKey::RightSuper, EnumAction::Press)? &&
              Input::get_key_state(first_key, EnumAction::Hold)?))
        }
        EnumModifier::CapsLock => {
          // Pressed down first key while holding down second key.
          Ok((Input::get_key_state(first_key, EnumAction::Press)? &&
            Input::get_key_state(EnumKey::CapsLock, EnumAction::Hold)?) ||
            // Pressed down second key while holding down first key.
            (Input::get_key_state(EnumKey::CapsLock, EnumAction::Press)? &&
              Input::get_key_state(first_key, EnumAction::Hold)?))
        }
        EnumModifier::NumLock => {
          // Pressed down first key while holding down second key.
          Ok((Input::get_key_state(first_key, EnumAction::Press)? &&
            Input::get_key_state(EnumKey::NumLock, EnumAction::Hold)?) ||
            // Pressed down second key while holding down first key.
            (Input::get_key_state(EnumKey::NumLock, EnumAction::Press)? &&
              Input::get_key_state(first_key, EnumAction::Hold)?))
        }
      }
  }
  
  // MOUSE BUTTON QUERY FUNCTIONS.
  pub fn get_mouse_button_state(mouse_button: EnumMouseButton, mouse_button_action: EnumAction) -> Result<bool, EnumError> {
    let api_mouse_button = convert_mouse_btn_to_api_mouse_btn(mouse_button);
    
    if unsafe { S_WINDOW.is_none() } {
      return Err(EnumError::InvalidWindowContext);
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
  
  pub fn get_mouse_cursor_attribute() -> Result<glfw::CursorMode, EnumError> {
    unsafe {
      if S_WINDOW_CONTEXT.is_some() {
        return Ok((*S_WINDOW.unwrap()).m_api_window.get_cursor_mode());
      }
    }
    return Err(EnumError::InvalidWindowContext);
  }
  
  pub fn set_mouse_cursor_attribute(cursor_mode: glfw::CursorMode) -> Result<(), EnumError> {
    unsafe {
      if S_WINDOW.is_some() {
        return Ok((*S_WINDOW.unwrap()).m_api_window.set_cursor_mode(cursor_mode));
      }
    }
    return Err(EnumError::InvalidWindowContext);
  }
  
  pub fn set_mouse_cursor_position(cursor_position: Vec2<f32>) -> Result<(), EnumError> {
    unsafe {
      if S_WINDOW_CONTEXT.is_some() {
        return Ok((*S_WINDOW.unwrap()).m_api_window
          .set_cursor_pos(cursor_position.x as f64, cursor_position.y as f64));
      }
    }
    return Err(EnumError::InvalidWindowContext);
  }
}