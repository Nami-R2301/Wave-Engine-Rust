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

use std::collections::HashMap;

use wave_engine::wave_core::EnumError;
use wave_engine::wave_core::input::{EnumAction, EnumKey, EnumModifier, EnumMouseButton, Input};
use wave_engine::wave_core::window::{EnumWindowMode, Window};

fn synchronous_key_inputs_loop(window: &mut Window, keys: &mut HashMap<EnumKey, bool>, action_required: EnumAction,
                               modifier: Option<EnumModifier>) -> Result<(), EnumError> {
  let copy = keys.clone();
  while !window.is_closing() {
    // Migrates new states from last frame to old ones to avoid reading an input multiple times.
    // Needs to be updated every frame. Normally, this is done automatically in the main engine render
    // loop, but here we state it explicitly for testing and clarity purposes.
    Input::reset();
    window.get_api_mut().poll_events();
    
    if Input::get_key_state(window, EnumKey::Escape, EnumAction::Press)? {
      return Ok(());
    }
    
    for (&key, _) in copy.iter() {
      if modifier.is_some() {
        if Input::get_modifier_key_combo(window, key, modifier.unwrap())? && !keys.get(&key).unwrap() {
          keys.insert(key, true);
        }
        continue;
      }
      if Input::get_key_state(window, key, action_required)? && !keys.get(&key).unwrap() {
        keys.insert(key, true);
      }
    }
  }
  return Ok(());
}

fn synchronous_mouse_button_inputs_loop(window: &mut Window, mouse_buttons: &mut HashMap<EnumMouseButton, bool>,
                                        action_required: EnumAction) -> Result<(), EnumError> {
  let copy = mouse_buttons.clone();
  while !window.is_closing() {
    // Migrates new states from last frame to old ones to avoid reading an input multiple times.
    // Needs to be updated every frame. Normally, this is done automatically in the main engine render
    // loop, but here we state it explicitly for testing and clarity purposes.
    Input::reset();
    window.get_api_mut().poll_events();
    
    if Input::get_key_state(window, EnumKey::Escape, EnumAction::Press)? {
      return Ok(());
    }
    
    for (&mouse_button, _) in copy.iter() {
      if Input::get_mouse_button_state(window, mouse_button, action_required)? &&
        !mouse_buttons.get(&mouse_button).unwrap() {
        mouse_buttons.insert(mouse_button, true);
      }
    }
  }
  return Ok(());
}

#[ignore]
#[test]
fn test_synchronous_key_inputs() -> Result<(), EnumError> {
  let mut window = Window::new(None, Some((1024, 768)),
    None, None, EnumWindowMode::Windowed)?;
  
  // Check if PRESS input events work properly.
  {
    window.set_title("[Test] : Press keys : [A, B, C, D, E] in any order before exiting");
    window.show();
    let mut keys_tracked: HashMap<EnumKey, bool> = HashMap::from([
      (EnumKey::A, false), (EnumKey::B, false), (EnumKey::C, false), (EnumKey::D, false),
      (EnumKey::E, false)]);
    
    synchronous_key_inputs_loop(&mut window, &mut keys_tracked, EnumAction::Press, None)?;
    
    assert!(keys_tracked.into_iter().all(|(_, was_pressed)| was_pressed));
    window.hide();
  }
  
  // Check if HOLD input events work properly.
  {
    window.set_title("[Test] : Press and hold keys : [F, G, H, I, J] in any order before exiting");
    window.show();
    let mut keys_tracked: HashMap<EnumKey, bool> = HashMap::from([
      (EnumKey::F, false), (EnumKey::G, false), (EnumKey::H, false), (EnumKey::I, false),
      (EnumKey::J, false)]);
    
    synchronous_key_inputs_loop(&mut window, &mut keys_tracked, EnumAction::Hold, None)?;
    
    assert!(keys_tracked.into_iter().all(|(_, was_held)| was_held));
    window.hide();
  }
  
  // Check if RELEASE input events work properly.
  {
    window.set_title("[Test] : Press and quickly release keys : [K, L, M, N, O] in any order before exiting");
    window.show();
    let mut keys_tracked: HashMap<EnumKey, bool> = HashMap::from([
      (EnumKey::K, false), (EnumKey::L, false), (EnumKey::M, false), (EnumKey::N, false),
      (EnumKey::O, false)]);
    
    synchronous_key_inputs_loop(&mut window, &mut keys_tracked, EnumAction::Release, None)?;
    
    assert!(keys_tracked.into_iter().all(|(_, was_released)| was_released));
    window.hide();
  }
  
  // Check for combination of input events.
  {
    window.set_title("[Test] : Hold SHIFT and press the following keys : [A] in any order before exiting");
    window.show();
    let mut keys_tracked: HashMap<EnumKey, bool> = HashMap::from([(EnumKey::A, false)]);
    
    synchronous_key_inputs_loop(&mut window, &mut keys_tracked, EnumAction::Press,
      Some(EnumModifier::Shift))?;
    
    assert!(keys_tracked.into_iter().all(|(_key, value)| value));
    window.hide();
  }
  {
    window.set_title("[Test] : Hold ALT and press the following keys : [B, D] in any order before exiting");
    window.show();
    let mut keys_tracked: HashMap<EnumKey, bool> = HashMap::from([(EnumKey::B, false),
      (EnumKey::D, false)]);
    
    synchronous_key_inputs_loop(&mut window, &mut keys_tracked, EnumAction::Hold,
      Some(EnumModifier::Alt))?;
    
    assert!(keys_tracked.into_iter().all(|(_key, value)| value));
    window.hide();
  }
  {
    window.set_title("[Test] : Hold CONTROL and press the following keys : [SPACE] in any order before exiting");
    window.show();
    let mut keys_tracked: HashMap<EnumKey, bool> = HashMap::from([(EnumKey::Space, false)]);
    
    synchronous_key_inputs_loop(&mut window, &mut keys_tracked, EnumAction::Release,
      Some(EnumModifier::Control))?;
    
    assert!(keys_tracked.into_iter().all(|(_key, value)| value));
    window.hide();
  }
  
  return Ok(());
}

#[ignore]
#[test]
fn test_synchronous_mouse_button_inputs() -> Result<(), EnumError> {
  let mut window = Window::new(None, Some((1024, 768)),
    None, None, EnumWindowMode::Windowed)?;
  
  // Check if PRESS input events work properly.
  {
    window.set_title("[Test] : Press mouse button : [M1, M2, M3, M4, M5] in any order before exiting");
    window.show();
    let mut mouse_buttons_tracked: HashMap<EnumMouseButton, bool> = HashMap::from([
      (EnumMouseButton::LeftButton, false), (EnumMouseButton::RightButton, false),
      (EnumMouseButton::MiddleButton, false), (EnumMouseButton::Button4, false),
      (EnumMouseButton::Button5, false)]);
    
    synchronous_mouse_button_inputs_loop(&mut window, &mut mouse_buttons_tracked, EnumAction::Press)?;
    
    assert!(mouse_buttons_tracked.into_iter().all(|(_, was_pressed)| was_pressed));
    window.hide();
  }
  
  // Check if HOLD input events work properly.
  {
    window.set_title("[Test] : Press and hold mouse button : [M1, M2] in any order before exiting");
    window.show();
    let mut mouse_buttons_tracked: HashMap<EnumMouseButton, bool> = HashMap::from([
      (EnumMouseButton::LeftButton, false), (EnumMouseButton::RightButton, false)]);
    
    synchronous_mouse_button_inputs_loop(&mut window, &mut mouse_buttons_tracked, EnumAction::Hold)?;
    
    assert!(mouse_buttons_tracked.into_iter().all(|(_, was_held)| was_held));
    window.hide();
  }
  
  // Check if RELEASE input events work properly.
  {
    window.set_title("[Test] : Press and release mouse button : [M2, M4] in any order before exiting");
    window.show();
    let mut mouse_buttons_tracked: HashMap<EnumMouseButton, bool> = HashMap::from([
      (EnumMouseButton::RightButton, false), (EnumMouseButton::Button4, false)]);
    
    synchronous_mouse_button_inputs_loop(&mut window, &mut mouse_buttons_tracked, EnumAction::Release)?;
    
    assert!(mouse_buttons_tracked.into_iter().all(|(_, was_released)| was_released));
    window.hide();
  }
  
  return Ok(());
}