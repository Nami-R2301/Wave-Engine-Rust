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

use wave_engine::wave::EnumErrors;
use wave_engine::wave::input::{EnumAction, EnumKeys, Input};
use wave_engine::wave::window::{EnumWindowMode, S_WINDOW, Window};

fn synchronous_inputs_loop(window: &mut Window, keys: &mut HashMap<EnumKeys, bool>, action_required: EnumAction) -> Result<(), EnumErrors> {
  let copy = keys.clone();
  while !window.is_closing() {
    window.m_api_window.glfw.poll_events();
    // Migrates new states from last frame to old ones to avoid reading an input multiple times.
    // Needs to be updated every frame. Normally, this is done automatically in the main engine render
    // loop, but here we state it explicitly for testing and clarity purposes.
    Input::on_update();
    
    if Input::get_key_state(EnumKeys::Escape, EnumAction::Press)? {
      return Ok(());
    }
    
    for (&key, _) in copy.iter() {
      Input::get_key_state(key, action_required)?;
      keys.insert(key, true);
    }
  }
  return Ok(());
}

#[test]
fn test_single_key_inputs() -> Result<(), EnumErrors> {
  let mut window = Window::new(None, Some(1024), Some(768), None,
    None, EnumWindowMode::Windowed)?;
  unsafe {
    S_WINDOW = Some(&mut window);
  }
  
  // Check if PRESS input events work properly.
  {
    window.set_title("[Test] : Press keys : [A, B, C, D, E] in any order before exiting");
    window.show();
    let mut keys_tracked: HashMap<EnumKeys, bool> = HashMap::from([
      (EnumKeys::A, false), (EnumKeys::B, false), (EnumKeys::C, false), (EnumKeys::D, false),
      (EnumKeys::E, false)]);
    
    synchronous_inputs_loop(&mut window, &mut keys_tracked, EnumAction::Press)?;
    
    assert!(keys_tracked.iter().all(|(_, &was_pressed)| was_pressed));
    window.hide();
  }
  
  // Check if HOLD input events work properly.
  {
    window.set_title("[Test] : Press and hold keys : [F, G, H, I, J] in any order before exiting");
    window.show();
    let mut keys_tracked: HashMap<EnumKeys, bool> = HashMap::from([
      (EnumKeys::F, false), (EnumKeys::G, false), (EnumKeys::H, false), (EnumKeys::I, false),
      (EnumKeys::J, false)]);
    
    synchronous_inputs_loop(&mut window, &mut keys_tracked, EnumAction::Hold)?;
    
    assert!(keys_tracked.iter().all(|(_, &was_held)| was_held));
    window.hide();
  }
  
  // Check if RELEASE input events work properly.
  {
    window.set_title("[Test] : Press and quickly release keys : [K, L, M, N, O] in any order before exiting");
    window.show();
    let mut keys_tracked: HashMap<EnumKeys, bool> = HashMap::from([
      (EnumKeys::K, false), (EnumKeys::L, false), (EnumKeys::M, false), (EnumKeys::N, false),
      (EnumKeys::O, false)]);
    
    synchronous_inputs_loop(&mut window, &mut keys_tracked, EnumAction::Release)?;
    
    assert!(keys_tracked.iter().all(|(_, &was_released)| was_released));
    window.hide();
  }
  
  return Ok(());
}

#[test]
fn test_multiple_key_inputs() -> Result<(), EnumErrors> {
  let mut window = Window::new(None, Some(1024), Some(768), None,
    None, EnumWindowMode::Windowed)?;
  unsafe {
    S_WINDOW = Some(&mut window);
  }
  
  // Check for combination of PRESS input events.
  {
    window.set_title("[Test] : Hold any of the two keys : [LEFT-SHIFT + A] while pressing the other one before exiting");
    window.show();
    let mut keys_tracked: HashMap<EnumKeys, bool> = HashMap::from([
      (EnumKeys::LeftShift, false), (EnumKeys::A, false)]);
    
    synchronous_inputs_loop(&mut window, &mut keys_tracked, EnumAction::Press)?;
    
    // Check if either one was pressed (The other will undoubtedly be held).
    assert!(*keys_tracked.get(&EnumKeys::LeftShift).unwrap() ||
      *keys_tracked.get(&EnumKeys::A).unwrap());
    window.hide();
  }
  
  return Ok(());
}