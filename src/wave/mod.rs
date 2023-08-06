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

use crate::{log, trace};
#[cfg(feature = "trace")]
use crate::{file_name, function_name};


use crate::wave::graphics::renderer::{EnumFeature, GlRenderer};
use crate::wave::utils::Time;
use crate::wave::utils::logger::EnumLogColor;
use crate::wave::window::{GlWindow};

pub mod window;
pub mod math;
pub mod graphics;
pub mod utils;

pub trait TraitApp {
  fn on_new(&mut self) -> ();
  fn on_delete(&mut self) -> ();
  fn on_destroy(&mut self) -> () {}
  
  fn on_event(&mut self) -> bool;
  fn on_update(&mut self, time_step: f64);
  fn on_render(&self);
}

#[derive(PartialEq)]
enum EnumState {
  Starting,
  Running,
  ShuttingDown,
  ShutDown
}

pub struct Engine {
  m_state: EnumState,
  m_app: Box<dyn TraitApp>,
  m_log_file_ptr: std::fs::File,
  m_window: GlWindow,
  m_exit_status: i64,
  m_time_step: f64,
}

impl Engine {
  pub fn new<T: TraitApp + 'static>(app_provided: Box<T>) -> Result<Engine, String> {
    let mut file_ptr = utils::logger::init().unwrap();
    
    log!(file_ptr, "INFO", "[Engine] -->\t Launching Wave Engine...");
    
    // Setup and launch engine.
    let window = GlWindow::new();
    
    match window {
      Ok(_) => {
        log!(file_ptr, "INFO", "[Engine] -->\t Created GLFW window successfully");
      }
      Err(_) => {
        log!(file_ptr, EnumLogColor::Red, "ERROR", "[Window] -->\t Error creating GLFW window! Exiting...");
        return Err("Error creating GLFW context! Exiting...".parse().unwrap());
      }
    }
    
    // Setup basic renderer features.
    let renderer = GlRenderer::new();
    
    match renderer {
      Ok(_) => {
        log!(file_ptr, EnumLogColor::Yellow, "INFO", "[Renderer] -->\t {0}", unsafe { GlRenderer::get_renderer_info() });
        log!(file_ptr, EnumLogColor::Yellow, "INFO", "[Renderer] -->\t {:?}", unsafe { GlRenderer::get_api_info() });
        log!(file_ptr, EnumLogColor::Yellow, "INFO", "[Renderer] -->\t {0}", unsafe { GlRenderer::get_shading_info() });
        log!(file_ptr, "INFO", "[Renderer] -->\t Created OpenGL context successfully");
        GlRenderer::toggle_feature(EnumFeature::DepthTest(true));
        GlRenderer::toggle_feature(EnumFeature::CullFacing(true, gl::BACK));
      }
      Err(_) => {
        log!(file_ptr, EnumLogColor::Red, "ERROR", "[Renderer] -->\t Error creating OpenGL context! Exiting...");
        return Err("Error creating OpenGL context! Exiting...".parse().unwrap());
      }
    }
    
    Ok({
      log!(file_ptr, "INFO", "[Engine] -->\t Launched Wave Engine successfully");
      Engine {
        m_state: EnumState::Starting,
        m_app: app_provided,
        m_log_file_ptr: file_ptr,
        m_window: window.unwrap(),
        m_exit_status: 0,
        m_time_step: 0.0,
      }
    })
  }
  
  pub fn shutdown(engine: &mut Engine) -> () {
    if engine.m_state == EnumState::ShutDown {
      return;
    }
    
    engine.m_state = EnumState::ShuttingDown;
    let result = GlRenderer::shutdown();
    
    match result {
      Ok(_) => {
        log!("INFO", "[Renderer] -->\t Renderer shut down successfully");
      }
      Err(_) => {
        log!("ERROR", "[Renderer] -->\t Error when trying to shut down renderer! Check logs for more info...");
      }
    }
    
    let exit_status: i64 = engine.get_exit_status();
    if exit_status != 0 {
      log!(utils::logger::EnumLogColor::Red,
                "ERROR", "[App] -->\t App exited with code {:#x}",
                exit_status);
    }
  }
  
  pub fn on_new(&mut self) -> () {
    let mut file_ptr = &self.m_log_file_ptr;
    log!(file_ptr, "INFO", "[App] -->\t Starting app...");
    
    match GlRenderer::new() {
      Ok(_) => {}
      Err(_) => {
        log!(file_ptr, "ERROR", "[Renderer] -->\t Error creating renderer context! Exiting...");
        return;
      }
    }
    self.m_app.on_new();
    
    log!(file_ptr, "INFO", "[App] -->\t Started app successfully");
  }
  
  pub fn on_delete(&mut self) -> () {
    let mut file_ptr = &self.m_log_file_ptr;
    log!(file_ptr, "INFO", "[App] -->\t Shutting down app...");
    self.m_app.on_delete();
    // Destroy app first.
    log!(file_ptr, "INFO", "[App] -->\t Shut down app successfully");
    
    Engine::shutdown(self);  // Then, destroy engine specific data.
  }
  
  pub fn run(&mut self) {
    self.m_state = EnumState::Running;
    
    // For time step.
    let mut _frame_start: Time = Time::from(chrono::Utc::now());
    
    // For up time and fps.
    let mut _frame_counter: u32 = 0;
    let mut runtime: Time = Time::new();
    
    // Loop until the user closes the window
    while !self.m_window.is_closing() {
      self.m_time_step = Time::get_delta(&_frame_start, &Time::from(chrono::Utc::now())).to_secs();
      _frame_start = Time::from(chrono::Utc::now());
      
      self.on_event();
      self.on_update(self.m_time_step);
      self.on_render();
      
      // Sync to engine tick rate.
      Time::wait_for(1.0 / 60.0);
      
      self.m_window.refresh();  // Refresh window.
      _frame_counter += 1;
      
      if Time::get_delta(&runtime, &Time::from(chrono::Utc::now())).to_secs() >= 1.0 {
        let title_format: String = format!("Wave Engine (Rust) | OpenGL | {0} FPS", &_frame_counter);
        self.m_window.set_title(&title_format);
        _frame_counter = 0;
        runtime = Time::from(chrono::Utc::now());
      }
    }
    self.m_state = EnumState::ShuttingDown;
  }
  
  pub fn on_event(&mut self) -> bool {
    return self.m_window.on_event();
  }
  
  pub fn on_update(&mut self, _time_step: f64) {}
  
  pub fn on_render(&self) {
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }
  }
  
  pub fn get_exit_status(&self) -> i64 {
    return self.m_exit_status;
  }
  
  pub fn get_log_file(&self) -> &std::fs::File {
    return &self.m_log_file_ptr;
  }
}


pub struct ExampleApp {}

impl TraitApp for ExampleApp {
  fn on_new(&mut self) -> () {}
  
  fn on_delete(&mut self) -> () {}
  
  fn on_event(&mut self) -> bool {
    return false;
  }
  
  fn on_update(&mut self, _time_step: f64) -> () {}
  
  fn on_render(&self) -> () {}
}
