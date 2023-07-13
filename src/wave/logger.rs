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

use std::fs::File;
use std::io::{Write};

#[macro_export]
macro_rules! log_info {
  () => {
    print!("\n");
  };
  
  ($($arg:tt)*) =>{{
    let log_type : &str = "[INFO]";
    let current_time = chrono::Utc::now();
    let current_time_str: String = "[".to_string() + &current_time.to_string() + &"]".to_string();
    let truncated_time = &current_time_str[0..20];
    
    let final_time_str = truncated_time.to_string() + "]";
    print!("\x1b[0m{0:<10}{1:20}\t\t", log_type, &final_time_str);
    println!($($arg)*);
    }};
}

#[macro_export]
macro_rules! log_warning {
  () => {
    print!("\n");
  };
  
  ($($arg:tt)*) =>{{
    let log_type : &str = "[WARN]";
    let current_time = chrono::Utc::now();
    let current_time_str: String = "[".to_string() + &current_time.to_string() + &"]".to_string();
    let truncated_time = &current_time_str[0..20];
    
    let final_time_str = truncated_time.to_string() + "]";
    print!("\x1b[33m{0:<10}{1:20}\t\t", log_type, &final_time_str);
    println!($($arg)*);
    }};
}

#[macro_export]
macro_rules! log_error {
  () => {
    print!("\n");
  };
  
  ($($arg:tt)*) =>{{
    let log_type : &str = "[ERROR]";
    let current_time = chrono::Utc::now();
    let current_time_str: String = "[".to_string() + &current_time.to_string() + &"]".to_string();
    let truncated_time = &current_time_str[0..20];
    
    let final_time_str = truncated_time.to_string() + "]";
    print!("\x1b[31m{0:<10}{1:20}\t\t", log_type, &final_time_str);
    println!($($arg)*);
    }};
}

#[inline(always)]
pub fn open_log() -> Option<File> {
  return File::open("log.txt").ok();
}

#[inline(always)]
pub fn close_log(file_ptr: &Option<File>) {
  assert!(file_ptr.is_some());
  file_ptr.as_ref().unwrap().flush().expect("Not all bytes could be written due to I/O errors or EOF reached");
}