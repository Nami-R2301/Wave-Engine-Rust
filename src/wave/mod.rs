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

use once_cell::sync::Lazy;

use crate::log;
use crate::wave::assets::renderable_assets::GlREntity;
use crate::wave::graphics::renderer::{EnumFeature, GlRenderer, TraitSendableEntity};
use crate::wave::graphics::shader::GlShader;
use crate::wave::utils::asset_loader::ResLoader;
use crate::wave::utils::Time;
use crate::wave::window::GlfwWindow;

pub mod window;
pub mod math;
pub mod graphics;
pub mod utils;
pub mod assets;
pub mod camera;

static mut S_LOG_FILE_PTR: Lazy<std::fs::File> = Lazy::new(|| utils::logger::init()
  .expect("[Logger] --> Cannot open log file!"));
static mut S_STATE: EnumState = EnumState::NotStarted;

#[derive(PartialEq)]
enum EnumState {
  NotStarted,
  Starting,
  Running,
  ShuttingDown,
  ShutDown,
}

#[derive(Debug)]
pub enum EnumErrors {
  Ok,
  ShaderError,
  AppError,
  RendererError,
  WindowError,
}

pub trait TraitApp {
  fn on_new(&mut self) -> Result<(), EnumErrors>;
  fn on_delete(&mut self) -> ();
  fn on_destroy(&mut self) -> () {}
  
  fn on_event(&mut self) -> bool;
  fn on_update(&mut self, time_step: f64);
  fn on_render(&self);
}

pub struct Engine {
  m_app: Box<dyn TraitApp>,
  m_window: GlfwWindow,
  m_time_step: f64,
}

impl Engine {
  pub fn new<T: TraitApp + 'static>(app_provided: Box<T>) -> Result<Engine, EnumErrors> {
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Launching Wave Engine...");
    
    // Setup and launch engine.
    let window = GlfwWindow::new();
    
    match window {
      Ok(_) => {
        log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Created GLFW window successfully");
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Error creating GLFW window! Exiting... \
         Error code => {:?}", err);
        return Err(EnumErrors::WindowError);
      }
    }
    
    // Setup basic renderer features.
    let renderer = GlRenderer::new();
    
    match renderer {
      Ok(_) => {
        log!(EnumLogColor::Yellow, "INFO", "[Renderer] -->\t {0}", GlRenderer::get_renderer_info());
        log!(EnumLogColor::Yellow, "INFO", "[Renderer] -->\t {:?}", GlRenderer::get_api_info());
        log!(EnumLogColor::Yellow, "INFO", "[Renderer] -->\t {0}", GlRenderer::get_shading_info());
        log!(EnumLogColor::Green, "INFO", "[Renderer] -->\t Created OpenGL context successfully");
        
        GlRenderer::toggle_feature(EnumFeature::DepthTest(true));
        GlRenderer::toggle_feature(EnumFeature::CullFacing(true, gl::BACK));
        GlRenderer::toggle_feature(EnumFeature::Debug(true));
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Error creating OpenGL context! \
        Exiting... Error code => {:?}", err);
        return Err(EnumErrors::RendererError);
      }
    }
    
    unsafe { S_STATE = EnumState::Starting; }
    Ok({
      log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Launched Wave Engine successfully");
      Engine {
        m_app: app_provided,
        m_window: window.unwrap(),
        m_time_step: 0.0,
      }
    })
  }
  
  pub fn shutdown() -> () {
    if unsafe { S_STATE == EnumState::ShutDown } {
      return;
    }
    
    unsafe { S_STATE = EnumState::ShuttingDown; }
    let result = GlRenderer::shutdown();
    match result {
      Ok(_) => {
        log!(EnumLogColor::Green, "INFO", "[Renderer] -->\t Renderer shut down successfully");
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Error when trying to shut down renderer! \
         Error code => {:?}", err);
      }
    }
  }
  
  pub fn on_new(&mut self) -> Result<(), EnumErrors> {
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Starting app...");
    
    match GlRenderer::new() {
      Ok(_) => {}
      Err(err) => {
        log!("ERROR", "[Renderer] -->\t Error creating renderer context! Exiting... Error code => \
         {:?}", err);
        return Err(EnumErrors::RendererError);
      }
    }
    match self.m_app.on_new() {
      Ok(_) => {
        log!(EnumLogColor::Green, "INFO", "[App] -->\t Started app successfully");
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[App] -->\t Started app unsuccessfully! Error => {:?}",
          err);
        self.on_delete();
        Engine::shutdown();
        return Err(err);
      }
    }
    return Ok(());
  }
  
  pub fn on_delete(&mut self) -> () {
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Shutting down app...");
    self.m_app.on_delete();
    // Destroy app first.
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Shut down app successfully");
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Shutting down engine...");
    Engine::shutdown();
    // Then, destroy engine specific data.
    log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Engine shut down successfully");
  }
  
  pub fn run(&mut self) {
    if unsafe { S_STATE != EnumState::Starting } {
      return;
    }
    unsafe { S_STATE = EnumState::Running; }
    
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
    unsafe { S_STATE = EnumState::ShuttingDown; }
  }
  
  pub fn on_event(&mut self) -> bool {
    return self.m_window.on_event();
  }
  
  pub fn on_update(&mut self, time_step: f64) {
    self.m_app.on_update(time_step);
  }
  
  pub fn on_render(&self) {
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }
    self.m_app.on_render();
  }
  
  pub fn get_log_file() -> &'static std::fs::File {
    unsafe { return &S_LOG_FILE_PTR; }
  }
}


pub struct ExampleApp {
  m_shaders: Vec<GlShader>,
  m_assets: Vec<GlREntity>,
}

impl ExampleApp {
  pub fn new() -> Self {
    return ExampleApp {
      m_shaders: Vec::with_capacity(5),
      m_assets: Vec::with_capacity(10),
    }
  }
}

impl TraitApp for ExampleApp {
  fn on_new(&mut self) -> Result<(), EnumErrors> {
    if self.m_shaders.is_empty() {
      self.m_shaders = Vec::new();
    }
    
    let result = GlShader::new("res/shaders/test_vert.glsl",
      "res/shaders/test_frag.glsl");
    match result {
      Ok(gl_shader) => { self.m_shaders.push(gl_shader); }
      Err(_) => {
        return Err(EnumErrors::ShaderError);
      }
    }
    
    // Sourcing and compilation.
    let result = self.m_shaders[0].send();
    match result {
      Ok(_) => { log!(EnumLogColor::Green, "INFO", "[Shader] -->\t Shader sent to GPU successfully"); }
      Err(_) => { return Err(EnumErrors::ShaderError); }
    }
    
    let _ = self.m_shaders[0].bind();
    
    let result = ResLoader::new("res/assets/cube.obj");
    match result {
      Ok(gl_vertices) => {
        log!(EnumLogColor::Green, "INFO", "[ResLoader] -->\t Asset {0} loaded successfully", "cube.obj");
        self.m_assets.push(GlREntity::from(gl_vertices));
        
        let result = self.m_assets.last_mut().unwrap().send();
        match result {
          Ok(_) => { log!(EnumLogColor::Green, "INFO", "[Renderer] -->\t Asset sent to GPU successfully"); }
          Err(_) => { return Err(EnumErrors::RendererError); }
        }
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[ResLoader] -->\t Asset {0} loaded unsuccessfully! \
          Error => {1:?}", "cube.obj", err);
      }
    }
    
    return Ok(());
  }
  
  fn on_delete(&mut self) -> () {}
  
  fn on_event(&mut self) -> bool {
    return false;
  }
  
  fn on_update(&mut self, _time_step: f64) -> () {}
  
  fn on_render(&self) -> () {
    let result = GlRenderer::draw();
    match result {
      Ok(_) => {}
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot draw all assets! Error => {0:?}",
          err);
      }
    }
  }
}
