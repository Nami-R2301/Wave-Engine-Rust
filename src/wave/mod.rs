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

extern crate gl;
extern crate imgui_glfw_rs;

use imgui_glfw_rs::{glfw, glfw::Context};
use imgui_glfw_rs::glfw::ffi::{glfwGetPrimaryMonitor, glfwSetWindowMonitor};

use crate::{log, trace};
use crate::wave::utils::{Time};

#[cfg(feature = "trace")]
use crate::{file_name, function_name};
use crate::wave::utils::logger::EnumLogColor;

pub mod math;
pub mod utils;

pub trait TraitApp {
  fn on_new(&mut self) -> ();
  fn on_delete(&mut self) -> ();
  fn on_destroy(&mut self) -> () {}

  fn on_event(&mut self) -> bool;
  fn on_update(&mut self, time_step: f64);
  fn on_render(&self);
}

static mut S_WINDOW_IS_FULLSCREEN: bool = false;

pub struct Engine<T> {
  m_app: T,
  m_log_file_ptr: std::fs::File,
  m_window: glfw::Window,
  m_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
  m_exit_status: i64,
  m_time_step: f64,
}

impl<T: TraitApp> Engine<T> {
  pub fn new(app_provided: T) -> Result<Engine<T>, String> {
    let mut file_ptr = utils::logger::init().unwrap();
    log!(file_ptr, "INFO", "[Engine] --> Launching Wave Engine...");
    
    // Setup and launch engine.
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    glfw.window_hint(glfw::WindowHint::Samples(None));
    glfw.window_hint(glfw::WindowHint::RefreshRate(None));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    
    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(1920, 1080,
      "Wave Engine (Rust)", glfw::WindowMode::Windowed)
      .expect("[Window] --> Failed to create GLFW window!");
    
    // Set input polling rate.
    window.set_sticky_keys(true);
    window.set_sticky_mouse_buttons(true);
    
    // Set glfw events.
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_framebuffer_size_polling(true);
    
    // Make the window's context current
    window.make_current();
    
    // Set v-sync.
    window.glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
    
    gl::load_with(|f_name| window.get_proc_address(f_name));
    
    unsafe { gl::Viewport(0, 0, 1920, 1080); }
    unsafe { gl::ClearColor(0.15, 0.15, 0.15, 1.0); }
    
    log!(file_ptr, "INFO", "[Engine] --> Launched Wave Engine Successfully");
    
    Ok(Engine {
      m_app: app_provided,  // App layer provided to run.
      m_log_file_ptr: file_ptr,  // Setup log file stream.
      m_window: window,
      m_events: events,
      m_exit_status: 0,
      m_time_step: 0.0,
    })
  }
  
  pub fn shutdown(engine: &mut Engine<T>) -> () {
    utils::logger::shutdown(); // Safely flush and close file.
    
    let exit_status: i64 = engine.get_exit_status();
    if exit_status != 0 {
      log!(utils::logger::EnumLogColor::Red,
                "ERROR", "[App] --> App exited with code {:#x}",
                exit_status);
    }
  }
  
  pub fn on_new(&mut self) -> () {
    let mut file_ptr = &self.m_log_file_ptr;
    log!(file_ptr, "INFO", "[App] --> Starting app...");
    self.m_app.on_new();
    log!(file_ptr, "INFO", "[App] --> Started app successfully");
  }
  
  pub fn on_delete(&mut self) -> () {
    let mut file_ptr = &self.m_log_file_ptr;
    log!(file_ptr, "INFO", "[App] --> Shutting down app...");
    self.m_app.on_delete();  // Destroy app first.
    log!(file_ptr, "INFO", "[App] --> Shut down app successfully");
    
    Engine::shutdown(self);  // Then, destroy engine specific data.
  }
  
  pub fn run(&mut self) {
    self.m_exit_status = 0;
    
    // For time step.
    let mut _frame_start: Time = Time::from(chrono::Utc::now());
    
    // For up time and fps.
    let mut _frame_counter: u32 = 0;
    let mut runtime: Time = Time::new();
    
    // Loop until the user closes the window
    while !self.m_window.should_close() {
      self.m_time_step = Time::get_delta(&_frame_start, &Time::from(chrono::Utc::now())).to_secs();
      _frame_start = Time::from(chrono::Utc::now());
      
      self.on_event();
      self.on_update(self.m_time_step);
      self.on_render();
      
      // Sync to engine tick rate.
      Time::wait_for(1.0 / 60.0);
      
      self.m_window.swap_buffers();  // Refresh window.
      _frame_counter += 1;
      
      if Time::get_delta(&runtime, &Time::from(chrono::Utc::now())).to_secs() >= 1.0 {
        let title_format: String = format!("Wave Engine (Rust) | OpenGL | {0} FPS", &_frame_counter);
        self.m_window.set_title(&title_format);
        _frame_counter = 0;
        runtime = Time::from(chrono::Utc::now());
      }
    }
  }
  
  pub fn on_event(&mut self) -> bool {
    self.m_window.glfw.poll_events();
    for (_, event) in glfw::flush_messages(&self.m_events) {
      return match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
          self.m_window.set_should_close(true);
          log!(EnumLogColor::Yellow, "WARN", "[Engine] --> User requested to close the window");
          true
        }
        glfw::WindowEvent::Key(glfw::Key::Enter, _, glfw::Action::Press, glfw::Modifiers::Alt) => unsafe {
          S_WINDOW_IS_FULLSCREEN.then_some({
            glfwSetWindowMonitor(self.m_window.window_ptr(), std::ptr::null_mut(),
              0, 0, 1920, 1016, -1);
          }).unwrap_or_else(|| {
            glfwSetWindowMonitor(self.m_window.window_ptr(), glfwGetPrimaryMonitor(),
              0, 0, 1920, 1080, -1);
          });
          
          log!(EnumLogColor::Yellow, "WARN", "[Engine] --> Fullscreen {0}",
            S_WINDOW_IS_FULLSCREEN.then_some("ON").unwrap_or("OFF"));
          
          S_WINDOW_IS_FULLSCREEN = !S_WINDOW_IS_FULLSCREEN;
          gl::Viewport(0, 0, 1920, 1080);
          true
        }
        glfw::WindowEvent::Key(key, _scancode, action, _mods) => {
          // log!("INFO", "Key: {:?}, ScanCode: {:?}, Action: {:?}, Modifiers: [{:?}]",
          // key, scancode, action, mods);
          
          match (key, action) {
            (glfw::Key::R, glfw::Action::Press) => {
              // Resize should cause the window to "refresh"
              let (window_width, window_height) = self.m_window.get_size();
              self.m_window.set_size(window_width + 1, window_height);
              self.m_window.set_size(window_width, window_height);
              false
            }
            _ => false
          }
        }
        glfw::WindowEvent::MouseButton(_btn, _action, _mods) => {
          // log!("INFO", "Button: {:?}, Action: {:?}, Modifiers: [{:?}]", btn, action, mods);
          false
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
          log!(EnumLogColor::White, "INFO", "[Engine] --> Framebuffer size: ({:?}, {:?})", width, height);
          false
        }
        _ => self.m_app.on_event()
      };
    }
    return false;
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
