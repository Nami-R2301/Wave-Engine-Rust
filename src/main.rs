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

pub mod wave;
use wave::*;

fn main()
{
  let log_file = logger::open_log();
  let mut app: Engine = Engine::create_app();
  let mut vector: math::Vec2<f32> = math::Vec2 {x: 0.0, y: 1.0} + math::Vec2 {x: 0.0, y: 1.0};
  vector += math::Vec2 {x: 2.1, y: 1.8};
  
  log_info!("{0}", vector);
  
  app.run();
  let exit_status: i64 = Engine::destroy_app(app);
  
  if exit_status != 0 {
    log_error!("App exited with code {0}", exit_status);
  }
  logger::close_log(&log_file);
}
