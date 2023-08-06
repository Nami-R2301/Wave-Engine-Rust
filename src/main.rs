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
/// impl TraitApp for ExampleApp {
///   // Create app-specific assets before entering the game loop.
///   fn on_new(&mut self) {
///     todo!()
///   }
///
///   // Delete app-specific assets before going out of scope and dropping.
///   fn on_delete(&mut self) {
///     todo!()
///   }
///
///   // Process app-specific events.
///   fn on_event(&mut self) {
///     todo!()
///   }
///
///   // Update app-specific data.
///   fn on_update(&mut self, time_step: f64) {
///     todo!()
///   }
///
///   /* App-specific directives before the window refresh (window swapping) in the main loop.
///    * Note, that any additional rendering in this function will only take effect after window swapping,
///    * and that the render color and depth buffers of the window are automatically cleared
///    * prior to this function call.
///   */
///   fn on_render(&self) {
///     todo!()
///   }
/// }
/// ```

fn main() {
  let my_app = Box::new(ExampleApp {});
  
  // Allocated on the stack -- Use new_shared() to allocate on the heap.
  let mut engine = Engine::new(my_app)
    .expect("[Engine] --> Fatal error occurred when running app! Exiting...");
  
  engine.on_new();  // Run `on_new()` for `my_app` prior to running.
  engine.run();
  engine.on_delete();  // Run `on_delete()` for `my_app` prior to dropping.
}
