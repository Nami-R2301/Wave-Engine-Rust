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

use crate::*;
use wave_engine::wave::utils::logger::{init, reset_logs, show_logs};

#[test]
fn test_open_log() {
  assert!(init().is_some())
}

#[test]
fn test_reset_logs() {
  let _option = init().as_ref().unwrap();
  log!("INFO", "Testing");
  
  reset_logs();
  let logs: String = show_logs();
  assert!(!logs.contains("Testing"));
}

#[test]
fn test_show_logs() {
  let _option = init().as_ref().unwrap();
  
  log!("DEBUG", "Testing");
  let logs: String = show_logs();
  assert!(logs.contains("Testing"));
}

pub struct UltraLongStructNameForTesting {}

impl UltraLongStructNameForTesting {
  pub fn long_function_name_for_testing_purposes(_ultra_long_size: u64, _ultra_long_capacity: u64,
                                                 _ultra_long_data: UltraLongStructNameForTesting) -> String {
    function_name!()
  }
}

fn long_function_name_for_testing_purposes() -> String {
  function_name!()
}

#[test]
fn test_function_name_length() {
  let function_str: String = UltraLongStructNameForTesting::long_function_name_for_testing_purposes(8, 8,
    UltraLongStructNameForTesting {});
  
  assert_eq!(function_str.len(), 23);
  assert_eq!(function_str, String::from("long_function_name_f..."));
  
  let function_without_namespace = long_function_name_for_testing_purposes();
  
  assert_eq!(function_without_namespace.len(), 23);
  assert_eq!(function_without_namespace, String::from("long_function_name_f..."));
}
