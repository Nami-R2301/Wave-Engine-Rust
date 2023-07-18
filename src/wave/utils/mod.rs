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
        return String::from(std::any::type_name::<T>().strip_suffix("::f")
                    .expect("[Utils] --> Could not strip f suffix!")
                    .to_string() + "()");
      }
      type_name_of(f)
    }}
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
      Path::new(this_file).file_name().and_then(|s| s.to_str()).unwrap()
    }}
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
/// --> "Error in | logger::init()::47 |! Logger init failed unexpectedly!"
/// ```
///

#[macro_export]
macro_rules! trace {
    () => {{
      let format = format!("| {0}::{1}::{2} |", file_name!(), function_name!(), line!());
      format
    }}
}

/*
///////////////////////////////////   LOGGER  ///////////////////////////////////
///////////////////////////////////           ///////////////////////////////////
///////////////////////////////////           ///////////////////////////////////
 */

pub mod logger {
  use std::fs::File;
  use std::io::Write;
  
  pub enum EnumLogColor {
    White,
    Yellow,
    Red,
    Blue,
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
  /// log!(log_output, log_type, format + arguments)
  /// log!(log_color, log_type, format + arguments)
  /// log!(log_output, log_color, log_type, format + arguments)
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
  #[macro_export]
  macro_rules! log {
    () => {
      print!("\n");
    };
    
    ($log_type: literal, $($format_and_arguments:tt)*) =>{{
      use std::io::Write;
      
      let current_time = chrono::Local::now();
      
      let format_string: String = format!("\x1b[0m[{0}]\t[{1:19}] {2}{3:<10}",
                                           $log_type, &current_time.to_string()[0..19], trace!(), "");
    
      let log_message: String = format!($($format_and_arguments)*);
      writeln!(std::io::stdout(), "{0}", format_string + &log_message).
                          expect("\x1b[31m[Logger] --> Unable to log statement!");
    }};
    
    ($log_output: ident, $log_type: literal, $($format_and_arguments:tt)*) =>{{
      use std::io::Write;
      
      let current_time = chrono::Local::now();
      
      let format_string: String = format!("\x1b[0m[{0}]\t[{1:19}] {2}{3:<10}",
                                           $log_type, &current_time.to_string()[0..19], trace!(), "");
    
      let log_message: String = format!($($format_and_arguments)*);
      writeln!($log_output, "{0}", format_string + &log_message).
                          expect("\x1b[31m[Logger] --> Unable to log statement!");
    }};
    
    ($log_color: expr, $log_type: literal, $($format_and_arguments:tt)*) =>{{
      use std::io::Write;
      
      let current_time = chrono::Local::now();
    
      let log_color: &str = utils::logger::color_to_str($log_color);
      let format_string: String = format!("{0}[{1}]\t[{2:19}] {3}{4:<10}",
                                          log_color, $log_type, &current_time.to_string()[0..19], trace!(), "");
    
      let log_message: String = format!($($format_and_arguments)*);
      writeln!(std::io::stdout(), "{0}", format_string + &log_message).
                          expect("\x1b[31m[Logger] --> Unable to log statement!");
    }};
  
    ($log_output: ident, $log_color: expr, $log_type: literal, $($format_and_arguments:tt)*) =>{{
      use std::io::Write;
      
      let current_time = chrono::Local::now();
    
      let log_color: &str = utils::logger::color_to_str($log_color);
      let format_string: String = format!("{0}[{1}]\t[{2:19}] {3}{4:<10}",
                                          log_color, $log_type, &current_time.to_string()[0..19], trace!(), "");
    
      let log_message: String = format!($($format_and_arguments)*);
      writeln!($log_output, "{0}", format_string + &log_message).
                          expect("\x1b[31m[Logger] --> Unable to log statement!");
    }};
  }
  
  #[inline(always)]
  pub fn init() -> Option<File> {
    let file = std::fs::OpenOptions::new().append(true).create(true).open("wave-engine.log");
    return file.ok();
  }
  
  #[inline(always)]
  pub fn color_to_str(log_color: EnumLogColor) -> &'static str {
    return match log_color {
      EnumLogColor::White => { "\x1b[0m" }
      EnumLogColor::Yellow => { "\x1b[33m" }
      EnumLogColor::Red => { "\x1b[31m" }
      EnumLogColor::Blue => { "\x1b[34m" }
    }
  }
  
  #[inline(always)]
  pub fn show_logs() -> String {
    let logs: String = std::fs::read_to_string("wave-engine.log")
      .expect("[Logger] --> Unable to show logs, due to error opening file!");
    println!("-----------------Start of Logs------------------------\n{}\
           ------------------End of Logs--------------------------\n", logs);
    return logs;
  }
  
  #[inline(always)]
  pub fn reset_logs() {
    std::fs::OpenOptions::new().write(true).truncate(true).open("wave-engine.log")
      .expect("[Logger] --> Could not reset file, due to error opening file!");
  }
  
  #[inline(always)]
  pub fn shutdown() {
    std::io::stdout().flush()
      .expect("[Logger] --> Could not flush stdout when shutting down logger!");
    let mut file = std::fs::OpenOptions::new().write(true).open("wave-engine.log")
      .expect("[Logger] --> Could not open file!");
    file.flush().expect("[Logger] --> Cannot flush log file!");
  }
}