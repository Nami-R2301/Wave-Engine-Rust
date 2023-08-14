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

extern crate num;

use chrono::DateTime;

pub mod asset_loader;

///
/// Convenience utility for displaying caller's function name, without the path prefix,
/// useful for debugging and logging. This is equivalent to `__FUNC__` or `__FUNCTION__` C macros.
/// \
/// \
/// **Returns** : `String`
///
/// A static copy of the function name.
/// \
/// \
/// **Example** :
/// ```text
/// fn init () {
///   let result = wave_engine::wave::utils::logger::init();  // Full function path.
///   if !result.is_some() {
///     log_callback!("Error in {0}! Logger init failed unexpectedly!", function_name!());
///   }
/// }
///
/// --> "Error in init()! Logger init failed unexpectedly"
/// ```
///
#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> String {
          let mut _string = String::from(
            std::any::type_name::<T>()
            .strip_suffix("::f").unwrap_or("").to_string() + "()");
          
          if _string.starts_with("wave_engine::") {
            _string = _string.strip_prefix("wave_engine::").unwrap_or("").to_string();
            
            if _string.starts_with("wave::") {
              _string = _string.strip_prefix("wave::").unwrap_or("").to_string();
            }
          }
          let function_start_index = _string.rfind(':').unwrap_or(0);
          
          // Truncate string to minimize text length when logging.
          if _string.len() > 25 {
            
            // If the function declaration is too long.
            if _string.len() - function_start_index >= 24 {
              _string = String::from(_string.strip_prefix(&_string[0 .. function_start_index + 1]).unwrap());
              let function_param_start_index = _string.find('(').unwrap_or(0);
              
               // If the function name is too long.
              if function_param_start_index >= 23 {
                // super_long_name_for_...()
                //                     ^
                //                     |
                //                  truncate from here.
                _string.replace_range(20..function_param_start_index, "...");
              }
            } else {
                _string = String::from(_string.strip_prefix(&_string[0 .. _string.len() - 25]).unwrap());
                _string.replace_range(0..3, "...");
            }
          }
          _string.replace_range(_string.len() - 2.._string.len(), "");
          
          return _string;
        }
        type_name_of(f)
    }};
}

///
/// Convenience utility for displaying caller's file name, without the path prefix,
/// useful for debugging and logging. This is equivalent to `__FILE__` or `__FILENAME__` C macros.
/// \
/// \
/// **Returns** : `String`
///
/// A static copy of the file name.
/// \
/// \
/// **Example** :
/// ```text
/// wave_engine/wave/utils/logger.rs :
///
/// fn init () {
///   let result = wave_engine::wave::utils::logger::init();  // Full function path.
///   if !result.is_some() {
///     log_callback!("Error in {0}! Logger init failed unexpectedly!", file_name!());
///   }
/// }
///
/// --> "Error in logger.rs! Logger init failed unexpectedly!"
/// ```
///
#[macro_export]
macro_rules! file_name {
    () => {{
        use std::path::Path;
        let this_file = file!();
        let mut path = Path::new(this_file);
        
        if path.starts_with("src/") {
          path = path.strip_prefix("src/").unwrap_or(path);
        }
        
        let mut path_str = String::from(path.to_str().unwrap_or(""));
        
        // Truncate string to minimize text length when logging.
          if path_str.len() > 25 {
            path_str = String::from(path_str.strip_prefix(&path_str[0 .. path_str.len() - 25]).unwrap());
            path_str.replace_range(0..3, "...");
          }
        
        path_str
    }};
}

///
/// Utility for displaying the caller's truncated filename, function name, and line number.
/// Useful for debugging and logging. This is combination of `utils::file_name()`,
/// `utils::function_name`, and `line!()`.
/// \
/// \
/// **Returns** : `String`
///
/// A static copy of the file name, followed by the function name, and finally followed by the line
/// number name. => | <*filename*>**:**<*function name*>**:**<*line number*> |
/// \
/// \
/// **Example** :
/// ```text
/// wave_engine/wave/utils/logger.rs :
///
/// fn init () {
///   let result = wave_engine::wave::utils::logger::init();  // Full function path.
///   if !result.is_some() {
///     log_callback!("Error in {0}! Logger init failed unexpectedly!", trace!());
///   }
/// }
///
/// --> "Error in [wave/utils/mod.rs    | ...::logger::init    | 47]! Logger init failed unexpectedly!"
/// ```
///
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! trace {
    () => {{
        let format = format!("[{0:25} | {1:25} | {2}]", file_name!(), function_name!(), line!());
        format
    }};
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace {
    () => {{ "" }};
}

///
/// Convenience macro for creating custom generic vectors of dynamic sizes without copying
/// boilerplate code for traits and operator overloading. This macro is intended for internal
/// structure creation, NOT for client use.
/// \
/// \
/// **Returns** : `Nothing`
/// \
/// \
/// **Example** :
/// *src/wave/math/mod.rs*
/// ```text
///
///   create_vec!(Color<T, 4> { r, g, b, a, }, false)
///
/// --> #[derive(Color, Clone)]
///     struct Color<T> {
///       r: T,
///       g: T,
///       b: T,
///       a: T,
///     }
///
///     impl<T: num::Zero> Color<T> where T: Copy {
///       pub fn new() -> Color<T> {...}
///       pub fn new_shared() -> Box<Color<T>> {...}
///       pub fn from(array: [T; 4]) -> Color<T> {...}
///       pub fn delete(&mut self) {...}
///
///     ...
/// ```
///
/// Additionally, basic traits implementations for Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div,
/// DivAssign, Display, Debug, and PartialEq will be automatically implemented.
#[macro_export]
macro_rules! create_vec {
    ($struct_name: ident<$struct_type: ident> { $($struct_item: ident,)* }) => {
      #[derive(Clone)]
      pub struct $struct_name<$struct_type> {
        $(pub $struct_item: $struct_type,)*
      }
      
      impl<$struct_type: num::Zero> $struct_name<$struct_type> where $struct_type: Copy {
         pub fn new() -> Self {
           return $struct_name { $($struct_item: $struct_type::zero(),)* };
         }
         pub fn new_shared() -> Box<Self> {
           return Box::new($struct_name::new());
         }
         pub fn from(array: &[$struct_type])  -> Self {
           let mut new = $struct_name { $($struct_item: $struct_type::zero(),)* };
           for i in (0..array.len()) {
             new[i] = array[i];
           }
           return new;
         }
         pub fn delete(&mut self) -> () {
           $(self.$struct_item = $struct_type::zero();)*
         }
      }
      
      ///////////////////// ARITHMETIC ////////////////////////
      
      impl<$struct_type> std::ops::Add for $struct_name<$struct_type>
      where $struct_type: std::ops::Add<$struct_type, Output=$struct_type> {
        type Output = $struct_name<$struct_type>;
        fn add(self, other: $struct_name<$struct_type>) -> $struct_name<$struct_type> {
          return $struct_name {
            $($struct_item: self.$struct_item + other.$struct_item,)+
          };
        }
      }
      impl<$struct_type> std::ops::AddAssign for $struct_name<$struct_type>
      where for<'a> &'a $struct_type: std::ops::Add<&'a $struct_type, Output=$struct_type> {
        fn add_assign(&mut self, other: $struct_name<$struct_type>) {
          $(self.$struct_item = &self.$struct_item + &other.$struct_item;)+
        }
      }
      impl<$struct_type> std::ops::Sub for $struct_name<$struct_type>
      where $struct_type: std::ops::Sub<$struct_type, Output=$struct_type> {
        type Output = $struct_name<$struct_type>;
        fn sub(self, other: $struct_name<$struct_type>) -> $struct_name<$struct_type> {
          return $struct_name {
            $($struct_item: self.$struct_item - other.$struct_item,)+
          };
        }
      }
      impl<$struct_type> std::ops::SubAssign for $struct_name<$struct_type>
      where for<'a> &'a $struct_type: std::ops::Sub<&'a $struct_type, Output=$struct_type> {
        fn sub_assign(&mut self, other: $struct_name<$struct_type>) {
          $(self.$struct_item = &self.$struct_item - &other.$struct_item;)+
        }
      }
      impl<$struct_type> std::ops::Mul for $struct_name<$struct_type>
      where $struct_type: std::ops::Mul<$struct_type, Output=$struct_type> {
        type Output = $struct_name<$struct_type>;
        fn mul(self, other: $struct_name<$struct_type>) -> $struct_name<$struct_type> {
          return $struct_name {
            $($struct_item: self.$struct_item * other.$struct_item,)+
          };
        }
      }
      impl<$struct_type> std::ops::MulAssign for $struct_name<$struct_type>
      where for<'a> &'a $struct_type: std::ops::Mul<&'a $struct_type, Output=$struct_type> {
        fn mul_assign(&mut self, other: $struct_name<$struct_type>) {
          $(self.$struct_item = &self.$struct_item * &other.$struct_item;)+
        }
      }
      impl<$struct_type> std::ops::Div for $struct_name<$struct_type>
      where $struct_type: std::ops::Div<$struct_type, Output=$struct_type> {
        type Output = $struct_name<$struct_type>;
        fn div(self, other: $struct_name<$struct_type>) -> $struct_name<$struct_type> {
          return $struct_name {
            $($struct_item: self.$struct_item / other.$struct_item,)+
          };
        }
      }
      impl<$struct_type> std::ops::DivAssign for $struct_name<$struct_type>
      where for<'a> &'a $struct_type: std::ops::Div<&'a $struct_type, Output=$struct_type> {
        fn div_assign(&mut self, other: $struct_name<$struct_type>) {
          $(self.$struct_item = &self.$struct_item / &other.$struct_item;)+
        }
      }
      /////////////////////// EQUALITY ////////////////////////
      
      impl<$struct_type> PartialEq for $struct_name<$struct_type>
      where for<'a> &'a $struct_type: PartialEq {
        fn eq(&self, other: &Self) -> bool {
          if (self as *const $struct_name<$struct_type> == other as *const $struct_name<$struct_type>) {
            return true;
          }
          let mut equal: bool = true;
          $(equal &= &self.$struct_item == &other.$struct_item;)+
          return equal;
        }
        
        fn ne(&self, other: &$struct_name<$struct_type>) -> bool {
          return !&self.eq(&other);
        }
      }
      
      ///////////////////// DEBUG ////////////////////////

      impl<$struct_type: std::fmt::Debug> std::fmt::Debug for $struct_name<$struct_type> {
        fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          let struct_name: String = String::from(stringify!([$struct_name])) + " -->";
          format.debug_struct(&struct_name)
          $(.field(stringify!($struct_item), &self.$struct_item))*
          .finish()
          .expect(&(struct_name + " Could not print (debug) struct!"));
          return Ok(());
        }
      }
      
      ///////////////////// DISPLAY ////////////////////////

      impl<$struct_type: std::fmt::Display> std::fmt::Display for $struct_name<$struct_type> {
        fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          let struct_name: String = String::from(stringify!([$struct_name])) + " --> Could not print struct!";
          
          write!(format, "[{0}] --> ", stringify!($struct_name))
            .expect(&struct_name);
          $(write!(format, "{0}: {1:.3}, ", stringify!($struct_item), &self.$struct_item)
            .expect(&struct_name);)+
          return Ok(());
        }
      }
    };
}

/*
///////////////////////////////////   LOGGER  ///////////////////////////////////
///////////////////////////////////           ///////////////////////////////////
///////////////////////////////////           ///////////////////////////////////
 */

pub mod logger {
  use std::fs::File;
  
  pub enum EnumLogColor {
    White,
    Yellow,
    Red,
    Blue,
    Green,
    Purple,
  }
  
  #[cfg(not(feature = "logging"))]
  #[macro_export]
  macro_rules! log {
    () => {};
    
    ($log_type: literal, $($format_and_arguments:tt)*) =>{{}};
    ($log_color: expr, $log_type: literal, $($format_and_arguments:tt)*) =>{{}};
  }
  
  ///
  /// Macros for displaying string formatted messages (**message_format**) in a given file stream
  /// (**log_output**).
  ///
  /// ## Parameters
  /// - *log_output*: File  --> Output stream for the log message.
  /// - *log_color*: utils:logger::EnumLogColor  --> Output color shown in terminal for the whole message.
  /// - *log_type*: &str  --> Alert type of the message, changing this will change the prefix of the log message.
  /// - *format*: &str  --> String format for subsequent arguments provided, much like format!().
  /// - *format_arguments*: Arguments<'_>)  --> Formatted arguments provided after a **format** was given.
  ///
  /// All possible macro matches :
  /// ```text
  /// log!(log_type, format + arguments)
  /// log!(log_color, log_type, format + arguments)
  /// ```
  /// \
  /// \
  /// **Returns** : `Nothing`
  /// \
  /// \
  /// **Example** :
  ///
  /// wave_engine/wave/math/mod.rs :
  ///
  /// ```text
  /// fn add () {
  ///   let result = 6 + 8;
  ///
  ///   // Colored logging.
  ///   log!(utils::logger::EnumLogColor::Yellow, "DEBUG",
  ///             "[Math] --> Addition of {0} AND {1} = {2}", 6, 8, result)
  ///
  ///   // Regular logging.
  ///   log!("INFO", "[Math] --> Addition of {0} AND {1} = {2}", 6, 8, result)
  /// }
  ///
  /// --> "\x1b[33m[DEBUG]  [2023-07-17 23:26:44] | mod.rs::add()::47 | [Math] --> Addition of 6 AND 8 = 14"
  /// --> "[INFO]  [2023-07-17 23:26:45] | mod.rs::add()::47 | [Math] --> Addition of 6 AND 8 = 14"
  /// ```
  ///
  #[cfg(feature = "logging")]
  #[macro_export]
  macro_rules! log {
    () => {
      print!("\n");
    };

    ($log_type: literal, $($format_and_arguments:tt)*) => {{
      use std::io::Write;
      use crate::wave::Engine;
      use crate::{trace, file_name, function_name};

      let current_time = chrono::Local::now();

      let format_string: String = format!("\x1b[0m[{0}]\t[{1:19}] {2:<60}\t",
                                           $log_type, &current_time.to_string()[0..19], trace!());

      let log_message: String = format!($($format_and_arguments)*);
      let mut log_file_ptr = Engine::get_log_file();
      writeln!(log_file_ptr, "{0}", format_string.clone() + &log_message).
                          expect("\x1b[31m[Logger] --> Unable to log statement!");
      writeln!(std::io::stdout(), "{0}", format_string + &log_message).
                          expect("\x1b[31m[Logger] --> Unable to log statement!");
    }};

    ($log_color: expr, $log_type: literal, $($format_and_arguments:tt)*) =>{{
      use std::io::Write;
      use crate::wave::Engine;
      use crate::wave::utils;
      use crate::wave::utils::logger::EnumLogColor;
      use crate::{trace, file_name, function_name};

      let current_time = chrono::Local::now();

      let log_color: &str = utils::logger::color_to_str($log_color);
      let format_string: String = format!("{0}[{1}]\t[{2:19}] {3:<60}\t",
                                          log_color, $log_type, &current_time.to_string()[0..19],
                                          trace!());

      let log_message: String = format!($($format_and_arguments)*);
      let mut log_file_ptr = Engine::get_log_file();
      writeln!(log_file_ptr, "{0}", format_string.clone() + &log_message).
                          expect("\x1b[31m[Logger] --> Unable to log statement!");
      writeln!(std::io::stdout(), "{0}", format_string + &log_message).
                          expect("\x1b[31m[Logger] --> Unable to log statement!");
    }};
  }
  
  #[inline(always)]
  pub fn init() -> Option<File> {
    let file = std::fs::OpenOptions::new()
      .append(true)
      .create(true)
      .open("wave-engine.log");
    
    return file.ok();
  }
  
  #[inline(always)]
  pub fn color_to_str(log_color: EnumLogColor) -> &'static str {
    return match log_color {
      EnumLogColor::White => "\x1b[0m",
      EnumLogColor::Yellow => "\x1b[33m",
      EnumLogColor::Red => "\x1b[31m",
      EnumLogColor::Blue => "\x1b[34m",
      EnumLogColor::Green => "\x1b[32m",
      EnumLogColor::Purple => "\x1b[0;35m",
    };
  }
  
  #[inline(always)]
  pub fn show_logs() -> String {
    let logs: String = std::fs::read_to_string("wave-engine.log")
      .expect("[Logger] --> Unable to show logs, due to error opening file!");
    println!(
      "-----------------Start of Logs------------------------\n{}\
           ------------------End of Logs--------------------------\n",
      logs
    );
    return logs;
  }
  
  #[inline(always)]
  pub fn reset_logs() {
    std::fs::OpenOptions::new()
      .write(true)
      .truncate(true)
      .open("wave-engine.log")
      .expect("[Logger] --> Could not reset file, due to error opening file!");
  }
}

/*
///////////////////////////////////   TIME    ///////////////////////////////////
///////////////////////////////////           ///////////////////////////////////
///////////////////////////////////           ///////////////////////////////////
 */

const CONST_TIME_NANO: f64 = 1000000000.0;
const CONST_TIME_MICRO: f64 = 1000000.0;
const CONST_TIME_MILLI: f64 = 1000.0;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Time {
  pub m_nano_seconds: f64,
}

impl Time {
  pub fn new() -> Self {
    return Time {
      m_nano_seconds: 0.0,
    };
  }
  
  pub fn from(local_time: DateTime<chrono::Utc>) -> Self {
    return Time {
      m_nano_seconds: local_time.timestamp_nanos() as f64
    };
  }
  
  pub fn get_delta(start_time: &Time, end_time: &Time) -> Time {
    return Time {
      m_nano_seconds: (&end_time.m_nano_seconds - &start_time.m_nano_seconds).abs(),
    };
  }
  
  pub fn wait_for(seconds: f64) -> () {
    if seconds <= 0.0 {
      return;
    }
    let end_time: f64 = Time::from(chrono::Utc::now()).m_nano_seconds + (seconds * CONST_TIME_NANO);
    while Time::from(chrono::Utc::now()).m_nano_seconds < end_time {}
  }
  
  pub fn wait_between(start_time: &Time, end_time: &Time) -> () {
    if start_time == end_time {
      return;
    }
    while &Time::from(chrono::Utc::now()) >= &start_time &&
      Time::from(chrono::Utc::now()) < *end_time {}
  }
  
  pub fn reset(&mut self) {
    self.m_nano_seconds = 0.0;
  }
  
  pub fn to_secs(&self) -> f64 {
    return self.m_nano_seconds / CONST_TIME_NANO;
  }
  
  pub fn to_micros(&self) -> f64 {
    return self.m_nano_seconds / CONST_TIME_MILLI;
  }
  
  pub fn to_millis(&self) -> f64 {
    return self.m_nano_seconds / CONST_TIME_MICRO;
  }
}

///////////////////////////////////   ARITHMETIC    ///////////////////////////////////

impl std::ops::Add for Time {
  type Output = Time;
  
  fn add(self, rhs: Self) -> Time {
    return Time {
      m_nano_seconds: self.m_nano_seconds - rhs.m_nano_seconds,
    };
  }
}

impl std::ops::Sub for Time {
  type Output = Time;
  
  fn sub(self, rhs: Time) -> Time {
    return Time {
      m_nano_seconds: self.m_nano_seconds - rhs.m_nano_seconds,
    };
  }
}