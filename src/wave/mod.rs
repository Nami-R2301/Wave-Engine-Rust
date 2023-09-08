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
use crate::wave::camera::PerspectiveCamera;
use crate::wave::graphics::{renderer, renderer::TraitRenderableEntity};
use crate::wave::graphics::shader::GlShader;
use crate::wave::math::Vec3;
use crate::wave::utils::asset_loader::ResLoader;
use crate::wave::utils::Time;
use crate::wave::window::GlfwWindow;

pub mod window;
pub mod math;
pub mod graphics;
pub mod utils;
pub mod assets;
pub mod camera;

static mut S_ENGINE: *mut Engine = std::ptr::null_mut();
static mut S_ACTIVE_WINDOW: *mut GlfwWindow = std::ptr::null_mut();
static S_LOG_FILE_PTR: Lazy<std::fs::File> = Lazy::new(|| utils::logger::init().unwrap());

#[derive(Debug, Copy, Clone, PartialEq)]
enum EnumState {
  NotStarted,
  Starting,
  Running,
  ShuttingDown,
  ShutDown,
}

#[derive(Debug)]
pub enum EnumErrors {
  AppError,
  ShaderError,
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
  m_tick_rate: f64,
  m_state: EnumState,
}

impl Engine {
  /// Setup new engine struct containing an app with the [TraitApp] behavior in order to call
  /// `on_new()`, `on_delete()`, `on_update()`, `on_event()`, and `on_render()`. By default, the
  /// engine uses an OpenGL renderer and GLFW for the context creation and handling.
  ///
  /// # Arguments
  ///
  /// * `app_provided`: A boxed generic app struct `T` which respects the trait [TraitApp].
  ///
  /// # Returns:
  ///   - `Result<GlREntity, EnumErrors>` : Will return a valid Engine if successful, otherwise an [EnumErrors]
  ///     on any error encountered. These include, but are not limited to :
  ///     + [EnumErrors::AppError] : If the app crashes for whatever reason the client may choose.
  ///     + [EnumErrors::RendererError] : If the renderer crashes due to an invalid process loading,
  ///       missing extensions, unsupported version and/or invalid GPU command.
  ///     + [EnumErrors::WindowError] : If the window context crashes due to invalid context creation,
  ///       deletion and/or command (GLX/X11 for Linux, WIN32 for Windows).
  ///
  /// # Examples
  ///
  /// ```text
  /// use wave::{Engine, EnumErrors};
  ///
  /// let my_app: Box<ExampleApp> = Box::new(ExampleApp::new());
  /// // Allocated on the stack -- Use new_shared() to allocate on the heap.
  /// let mut engine = Engine::new(my_app)?;
  ///
  /// // Run `on_new()` for `my_app` prior to running.
  /// engine.on_new()?;
  /// engine.run();
  /// engine.on_delete();
  /// return Ok(());
  /// ```
  pub fn new<T: TraitApp + 'static>(app_provided: Box<T>) -> Result<Engine, EnumErrors> {
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Launching Wave Engine...");
    
    // Setup and launch engine.
    let mut window = GlfwWindow::new();
    
    match window {
      Ok(_) => unsafe { S_ACTIVE_WINDOW = window.as_mut().unwrap(); }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Window] -->\t Error creating GLFW window! Exiting... \
         Error code => {:?}", err);
        return Err(EnumErrors::WindowError);
      }
    }
    
    // Setup basic renderer features.
    let renderer = renderer::GlRenderer::new();
    
    match renderer {
      Ok(_) => {
        log!(EnumLogColor::Yellow, "INFO", "[Renderer] -->\t {0}", renderer::GlRenderer::get_renderer_info());
        log!(EnumLogColor::Yellow, "INFO", "[Renderer] -->\t {:?}", renderer::GlRenderer::get_api_info());
        log!(EnumLogColor::Yellow, "INFO", "[Renderer] -->\t {0}", renderer::GlRenderer::get_shading_info());
        
        let _ = renderer::GlRenderer::toggle_feature(renderer::EnumFeature::DepthTest(true));
        let _ = renderer::GlRenderer::toggle_feature(renderer::EnumFeature::CullFacing(true, gl::BACK));
        let _ = renderer::GlRenderer::toggle_feature(renderer::EnumFeature::Debug(true));
        let _ = renderer::GlRenderer::toggle_feature(renderer::EnumFeature::Wireframe(true));
        let _ = renderer::GlRenderer::toggle_feature(renderer::EnumFeature::MSAA(true));
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Error creating OpenGL context! \
        Exiting... Error code => {:?}", err);
        return Err(EnumErrors::RendererError);
      }
    }
    
    Ok({
      log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Launched Wave Engine successfully");
      Engine {
        m_app: app_provided,
        m_window: window.unwrap(),
        m_time_step: 0.0,
        m_tick_rate: 0.0,
        m_state: EnumState::NotStarted,
      }
    })
  }
  
  /// Teardown engine. This effectively first shuts down the renderer and then flags the window to
  /// be closed on drop.
  ///
  ///
  /// # Returns:
  ///   - `Result<GlREntity, EnumErrors>` : Will return a valid Engine if successful, otherwise an [EnumErrors]
  ///     on any error encountered. These include, but are not limited to :
  ///     + [EnumErrors::AppError] : If the app crashes for whatever reason the client may choose.
  ///     + [EnumErrors::RendererError] : If the renderer crashes due to an invalid process loading,
  ///       missing extensions, unsupported version and/or invalid GPU command.
  ///     + [EnumErrors::WindowError] : If the window context crashes due to invalid context creation,
  ///       deletion and/or command (GLX/X11 for Linux, WIN32 for Windows).
  ///
  /// # Examples
  ///
  /// ```text
  /// use wave::{Engine, EnumErrors};
  ///
  /// let my_app: Box<ExampleApp> = Box::new(ExampleApp::new());
  /// // Allocated on the stack -- Use new_shared() to allocate on the heap.
  /// let mut engine = Engine::new(my_app)?;
  ///
  /// // Run `on_new()` for `my_app` prior to running.
  /// engine.on_new()?;
  /// engine.run();
  /// engine.on_delete();
  /// return Ok(());
  /// ```
  pub fn shutdown(&mut self) -> Result<(), renderer::EnumErrors> {
    if self.m_state == EnumState::ShutDown {
      log!(EnumLogColor::Yellow, "WARN", "[Engine] -->\t Engine already shut down! Not shutting down twice...");
      return Ok(());
    }
    
    self.m_state = EnumState::ShuttingDown;
    let result = renderer::GlRenderer::shutdown();
    match result {
      Ok(_) => {}
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Error when trying to shut down renderer! \
         Error code => {:?}", err);
        return Err(err);
      }
    }
    self.m_window.close();
    self.m_state = EnumState::ShutDown;
    return Ok(());
  }
  
  pub fn get() -> *mut Engine {
    unsafe { return S_ENGINE; }
  }
  
  pub fn get_active_window() -> *mut GlfwWindow {
    unsafe { return S_ACTIVE_WINDOW; }
  }
  
  pub fn on_new(&mut self) -> Result<(), EnumErrors> {
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Starting app...");
    self.m_state = EnumState::Starting;
    unsafe {
      S_ENGINE = self;
      S_ACTIVE_WINDOW = &mut self.m_window;
    }
    
    match renderer::GlRenderer::new() {
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
        match self.on_delete() {
          Ok(_) => {}
          Err(err) => {
            log!("ERROR", "[Renderer] -->\t Error deleting renderer context! Exiting... Error code => \
              {:?}", err);
            return Err(EnumErrors::RendererError);
          }
        }
        return Err(err);
      }
    }
    return Ok(());
  }
  
  pub fn on_delete(&mut self) -> Result<(), EnumErrors> {
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Shutting down app...");
    self.m_app.on_delete();
    // Destroy app first.
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Shut down app successfully");
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Shutting down engine...");
    match self.shutdown() {
      Ok(_) => {}
      Err(err) => {
        log!("ERROR", "[Renderer] -->\t Error deleting renderer context! Exiting... Error code => \
              {:?}", err);
        return Err(EnumErrors::RendererError);
      }
    }
    // Then, destroy engine specific data.
    log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Engine shut down successfully");
    return Ok(());
  }
  
  pub fn run(&mut self) {
    if self.m_state != EnumState::Starting {
      log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Engine has not started up correctly! Exiting...");
      return;
    }
    self.m_state = EnumState::Running;
    
    // For time step.
    let mut frame_start: Time = Time::from(chrono::Utc::now());
    
    // For up time and fps.
    let mut frame_counter: u16 = 0;
    let mut runtime: Time = Time::new();
    
    // Loop until the user closes the window
    while !self.m_window.is_closing() {
      self.m_time_step = Time::get_delta(&frame_start,
        &Time::from(chrono::Utc::now())).to_secs();
      frame_start = Time::from(chrono::Utc::now());
      
      self.on_event();
      self.on_update(self.m_time_step);
      self.on_render();
      
      // Sync to engine tick rate.
      Time::wait_for(self.m_tick_rate);
      
      self.m_window.refresh();  // Refresh window.
      frame_counter += 1;
      
      if Time::get_delta(&runtime, &Time::from(chrono::Utc::now())).to_secs() >= 1.0 {
        let title_format: String = format!("Wave Engine (Rust) | OpenGL | {0} FPS", &frame_counter);
        self.m_window.set_title(&title_format);
        frame_counter = 0;
        runtime = Time::from(chrono::Utc::now());
      }
    }
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
    return &S_LOG_FILE_PTR;
  }
}


pub struct ExampleApp {
  m_shaders: Vec<GlShader>,
  m_renderable_assets: Vec<GlREntity>,
}

impl ExampleApp {
  pub fn new() -> Self {
    return ExampleApp {
      m_shaders: Vec::new(),
      m_renderable_assets: Vec::new(),
    };
  }
}

impl TraitApp for ExampleApp {
  fn on_new(&mut self) -> Result<(), EnumErrors> {
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Loading GLSL shaders...");
    let result = GlShader::new("res/shaders/test_vert.glsl",
      "res/shaders/test_frag.glsl");
    match result {
      Ok(gl_shader) => { self.m_shaders.push(gl_shader); }
      Err(_) => {
        return Err(EnumErrors::ShaderError);
      }
    }
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Loaded GLSL shaders successfully");
    
    // Sourcing and compilation.
    let result = self.m_shaders[0].send();
    match result {
      Ok(_) => { log!("INFO", "[Shader] -->\t Shader sent to GPU successfully"); }
      Err(_) => { return Err(EnumErrors::ShaderError); }
    }
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Uploading camera view and projection to the GPU...");
    let mut camera: PerspectiveCamera = PerspectiveCamera::from(65.0, 0.1, 1000.0);
    camera.set_view_projection();
    match self.m_shaders[0].upload_uniform("u_view_projection", camera.get_matrix()) {
      Ok(_) => {}
      Err(_) => { return Err(EnumErrors::RendererError); }
    }
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Camera view and projection uploaded to GPU successfully");
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending asset 'awp.obj' to GPU...");
    let result = ResLoader::new("awp.obj");
    match result {
      Ok(gl_vertices) => {
        log!("INFO", "[ResLoader] -->\t Asset {0} loaded successfully", "awp.obj");
        self.m_renderable_assets.push(GlREntity::from(gl_vertices));
        self.m_renderable_assets[0].translate(Vec3::from(&[10.0, -10.0, 30.0]));
        self.m_renderable_assets[0].rotate(Vec3::from(&[-90.0, 90.0, 0.0]));
        match self.m_shaders[0].upload_uniform("u_model_matrix", self.m_renderable_assets[0].get_matrix()) {
          Ok(_) => { log!("INFO", "[Shader] -->\t Uniform 'u_model_matrix' uploaded to GPU successfully"); }
          Err(_) => {}
        }
        
        let result = self.m_renderable_assets[0].send(&mut self.m_shaders[0]);
        match result {
          Ok(_) => { log!(EnumLogColor::Green, "INFO", "[App] -->\t Asset sent to GPU successfully"); }
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
    let result = renderer::GlRenderer::draw();
    match result {
      Ok(_) => {}
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot draw all assets! Error => {0:?}",
          err);
      }
    }
  }
}
