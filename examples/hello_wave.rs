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

use std::process::Command;
use wave_core::{EnumEngineError, EnumCommandError, Engine, log};
use wave_core::dependencies::chrono;
use wave_core::graphics::renderer::{EnumRendererApi, Renderer};
use wave_core::utils::macros::logger::{trace, file_name, color_to_str, function_name, EnumLogColor};

static HELP_STR: &str = "Usage: wave_engine [OPTIONS] COMMANDS...

Description:
   Run subprocesses with basic tooling provided such as, benchmarking, logging,
     asynchronous running, and event handling. Note that subprocesses will run in parallel
     unless the thread limit is reached, resulting in a sequential, queue-like behavior.

Options:
    -h, --help            Show this message and exit.
    -v, --version         Show the version information and exit.
    -c, --commands        Provide a list of commands with their args provided to run in parallel.
    -l, --logs PATH       Specify the path to the log file to read/write to.
    -s, --show-logs       Print current logs from log file and exit.
    -q, --quiet           Suppress output (only show errors).
    -t, --timeout INT     Set the timeout for the operation (in seconds).
    
Examples:
    wave_engine --logs /path/to/logs.txt
    wave_engine --show-logs
    wave_engine --commands 'my-script.sh' 'echo Testing'
    wave_engine --timeout 120 --commands 'my-script.sh'";

fn main() -> Result<(), EnumEngineError> {
  let args: Vec<String> = std::env::args().collect();
  Engine::init_logs();
  
  if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) || args.len() == 1 {
    log!("INFO", "\n{0}", HELP_STR);
    return Ok(());
  }
  
  if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
    log!("INFO", "Version: {0}", env!("CARGO_PKG_VERSION"));
    return Ok(());
  }
  
  if args.contains(&"-l".to_string()) || args.contains(&"--logs".to_string()) {
    let path = args.iter().position(|arg| arg == "--logs");
    if path.is_none() || args.len() <= path.unwrap() + 1 {
      return Err(EnumCommandError::InvalidCommand.into());
    }

    let path = wave_core::utils::macros::logger::reset_logs(args[path.unwrap() + 1].clone());
    log!(EnumLogColor::Green, "INFO", "Successfully set logs file directory to {0}", path);
  }
  
  if args.contains(&"-s".to_string()) || args.contains(&"--show-logs".to_string()) {
    log!("INFO", "Logs: \n{0}", wave_core::utils::macros::logger::show_logs());
    return Ok(());
  }
  
  let mut engine = Engine::new(vec![]);
  engine.run()?;
  
  if args.contains(&"-c".to_string()) || args.contains(&"--commands".to_string()) {
    let commands = resolve_commands(args);
    commands.iter().for_each(|command| {
      log!("INFO", "{:?}", String::from_utf8(Command::new("sh")
      .arg("-c")
      .arg(command)
      .output()
      .expect("Failed to execute command").stdout).unwrap());
    });
  }
  
  return Ok(());
}

fn resolve_commands(args: Vec<String>) -> Vec<String> {
  let mut commands: Vec<String> = vec![];
  
  for arg in args.into_iter().skip(2) {
      commands.push(arg);
  }
  return commands;
}
