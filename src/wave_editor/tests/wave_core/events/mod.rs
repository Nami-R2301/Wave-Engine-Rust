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

use wave_editor::wave_core::events::EnumEventMask;

#[test]
fn test_event_masking() {
  let window = EnumEventMask::Window;
  let inputs = EnumEventMask::Input;
  let keys = EnumEventMask::Keyboard;
  
  assert!(inputs.contains(keys));
  assert!(keys.intersects(inputs));
  
  assert_eq!(keys & EnumEventMask::None, keys.intersection(EnumEventMask::None));
  assert_eq!(keys | inputs, keys.union(inputs));
  assert_eq!(inputs & !keys, inputs.difference(keys));
  
  assert_ne!(keys.union(inputs), window);
}