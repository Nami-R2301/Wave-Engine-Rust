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

use wave::*;

pub mod wave;

///
/// Example entrypoint to the application **executable** for the client. Substitute this out with
/// your own app.
///
/// ### Returns : Nothing
///
/// ## Example :
/// ```text
/// pub struct ExampleApp {}
///
/// impl App for ExampleApp {
///   fn on_new(&self) {
///
///   }
///
///   fn on_delete(&self) {
///     todo!()
///   }
///
///   fn on_event(&self) {
///     todo!()
///   }
///
///   fn on_update(&self) {
///     todo!()
///   }
///
///   fn on_render(&self) {
///     todo!()
///   }
/// }
/// ```

fn main()
{
  let mut file_ptr = utils::logger::init().unwrap();
  let mut stderr = std::io::stderr();
  
  log!(file_ptr, "INFO", "[App] --> Initialising App...");
  // Allocated on the stack -- Use new_shared() to allocate on the heap.
  let mut app: Engine = Engine::new();
  
  log!(file_ptr, "INFO", "[App] --> Starting App...");
  app.run();
  
  log!(file_ptr, "INFO", "[App] --> Destroying App...");
  let exit_status: i64 = app.delete();  // Ability to explicitly drop app.
  if exit_status != 0 {
    log!(stderr, utils::logger::EnumLogColor::Red, "ERROR", "[App] --> App exited with code {:#x}",
      exit_status);
  }
  
  utils::logger::shutdown();  // Safely flush and close file.
}
