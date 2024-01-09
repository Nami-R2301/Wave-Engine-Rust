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
use crate::wave::assets::renderable_assets::{REntity, TraitRenderableEntity};
use crate::wave::camera::PerspectiveCamera;
use crate::wave::graphics::renderer::{self, EnumApi, Renderer, S_RENDERER};
use crate::wave::graphics::shader::{self, EnumShaderSource, EnumShaderType, Shader, ShaderStage};
use crate::wave::input::{EnumKey, EnumModifier, Input};
use crate::wave::math::Vec3;
use crate::wave::utils::asset_loader::ResLoader;
use crate::wave::utils::Time;
use crate::wave::window::{EnumWindowMode, S_WINDOW, Window};

pub mod window;
pub mod math;
pub mod graphics;
pub mod utils;
pub mod assets;
pub mod camera;
pub mod input;
mod events;

static mut S_ENGINE: *mut Engine = std::ptr::null_mut();

static S_LOG_FILE_PTR: Lazy<std::fs::File> = Lazy::new(|| utils::logger::init().unwrap());

#[derive(Debug, Copy, Clone, PartialEq)]
enum EnumState {
  NotStarted,
  Starting,
  Started,
  Running,
  Deleting,
  Deleted,
  ShutDown,
}

#[derive(Debug)]
pub enum EnumError {
  UndefinedError,
  AppError,
  ResourceError(utils::asset_loader::EnumError),
  ShaderError(shader::EnumError),
  RendererError(renderer::EnumError),
  WindowError(window::EnumError),
  InputError(input::EnumError)
}

macro_rules! impl_enum_error {
  ($error_type: ty, $resulting_error: expr) => {
    impl From<$error_type> for EnumError {
      fn from(err: $error_type) -> EnumError {
        log!(EnumLogColor::Red, "ERROR", "{0}", err);
        return $resulting_error(err);
      }
    }
  }
}

// Convert renderer error to wave::EnumError.
impl_enum_error!(renderer::EnumError, EnumError::RendererError);

// Convert resource loader error to wave::EnumError.
impl_enum_error!(utils::asset_loader::EnumError, EnumError::ResourceError);

// Convert shader error to wave::EnumError.
impl_enum_error!(shader::EnumError, EnumError::ShaderError);

// Convert window error to wave::EnumError.
impl_enum_error!(window::EnumError, EnumError::WindowError);

// Convert input error to wave::EnumError.
impl_enum_error!(input::EnumError, EnumError::InputError);

pub trait TraitApp {
  fn on_new(&mut self) -> Result<(), EnumError>;
  fn on_delete(&mut self) -> Result<(), EnumError>;
  
  fn on_event(&mut self) -> bool;
  fn on_update(&mut self, time_step: f64) -> Result<bool, EnumError>;
  fn on_render(&mut self) -> Result<(), EnumError>;
}

pub struct Engine {
  m_app: Box<dyn TraitApp>,
  m_window: Window,
  m_renderer: Renderer,
  m_time_step: f64,
  m_tick_rate: f32,
  m_state: EnumState,
}

impl Engine {
  /// Setup new engine struct containing an app with the [TraitApp] behavior in order to call
  /// `on_new()`, `on_delete()`, `on_update()`, `on_event()`, and `on_render()`. By default, the
  /// engine uses an OpenGL renderer and GLFW for the context creation and handling.
  ///
  /// ### Arguments:
  ///
  /// * `app_provided`: A boxed generic app struct `T` which respects the trait [TraitApp].
  ///
  /// ### Returns:
  ///   - `Result<GlREntity, EnumError>` : Will return a valid Engine if successful, otherwise an [EnumError]
  ///     on any error encountered. These include, but are not limited to :
  ///     + [EnumError::AppError] : If the app crashes for whatever reason the client may choose.
  ///     + [EnumError::RendererError] : If the renderer crashes due to an invalid process loading,
  ///       missing extensions, unsupported version and/or invalid GPU command.
  ///     + [EnumError::WindowError] : If the window context crashes due to invalid context creation,
  ///       deletion and/or command (GLX/X11 for Linux, WIN32 for Windows).
  ///
  /// ### Examples
  ///
  /// ```text
  /// use wave::{Engine, EnumError};
  ///
  /// let my_app = Box::new(ExampleApp::new());
  /// // Allocated on the stack -- Use new_shared() to allocate on the heap.
  /// let mut engine = Box::new(Engine::new(my_app)?);
  ///
  /// // Run `on_new()` for `my_app` prior to running.
  /// engine.on_new()?;
  /// engine.run();
  /// engine.on_delete();
  /// return Ok(());
  /// ```
  pub fn new(api_preference: Option<EnumApi>, app_provided: Option<Box<dyn TraitApp>>) -> Result<Self, EnumError> {
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Launching Wave Engine...");
    
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Opening window...");
    // Setup window context.
    let mut window = Window::new(api_preference, Some((1920, 1080)),
      None, None, EnumWindowMode::Windowed)?;
    log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Opened window successfully");
    
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Starting renderer...");
    // Create graphics context.
    let renderer = Renderer::new(api_preference, &mut window)?;
    log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Started renderer successfully");
    
    Ok({
      log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Launched Wave Engine successfully");
      Engine {
        m_app: app_provided.unwrap_or(Box::new(ExampleApp::default())),
        m_window: window,
        m_renderer: renderer,
        m_time_step: 0.0,
        m_tick_rate: 0.0,
        m_state: EnumState::NotStarted,
      }
    })
  }
  
  pub fn on_new(&mut self) -> Result<(), EnumError> {
    if self.m_state != EnumState::NotStarted {
      log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Cannot instantiate engine : Engine already started!");
      return Err(EnumError::AppError);
    }
    
    self.m_state = EnumState::Starting;
    
    Engine::set_singleton(self);
    
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Setting up renderer...");
    // Enable features BEFORE finalizing context.
    #[cfg(not(feature = "Vulkan"))]
    self.m_renderer.renderer_hint(renderer::EnumFeature::CullFacing(Some(gl::BACK as i64)));
    
    self.m_renderer.renderer_hint(renderer::EnumFeature::DepthTest(true));
    self.m_renderer.renderer_hint(renderer::EnumFeature::Debug(true));
    self.m_renderer.renderer_hint(renderer::EnumFeature::Wireframe(true));
    self.m_renderer.renderer_hint(renderer::EnumFeature::MSAA(None));
    
    // Finalize graphics context with all hinted features to prepare for frame presentation.
    self.m_renderer.submit()?;
    log!(EnumLogColor::White, "INFO", "[Renderer] -->\t {0}", self.m_renderer);
    log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Setup renderer successfully");
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Starting app...");
    self.m_app.on_new()?;
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Started app successfully");
    
    self.m_state = EnumState::Started;
    return Ok(());
  }
  
  pub fn on_delete(&mut self) -> Result<(), EnumError> {
    self.m_state = EnumState::Deleting;
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Shutting down app...");
    // Destroy app first.
    self.m_app.on_delete()?;
    self.m_renderer.on_delete()?;
    self.m_window.on_delete()?;
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Shut down app successfully");
    
    self.m_state = EnumState::Deleted;
    return Ok(());
  }
  
  pub fn run(&mut self) -> Result<(), EnumError> {
    self.on_new()?;
    
    if self.m_state != EnumState::Started {
      log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Cannot run : Engine has not started up correctly!");
      return Err(EnumError::AppError);
    }
    
    self.m_state = EnumState::Running;
    
    // For time step.
    let mut frame_start: Time = Time::from(chrono::Utc::now());
    
    // For up time and fps.
    let mut frame_counter: u32 = 0;
    // For keeping track of previous logged fps.
    let mut same_frame_counter: u32 = 0;
    let mut runtime: Time = Time::new();
    
    let title_cache: String = format!("Wave Engine (Rust) | {0:?}", self.m_renderer.m_type);
    self.m_window.set_title(&title_cache);
    
    // Loop until the user closes the window
    while !self.m_window.is_closing() {
      self.m_time_step = Time::get_delta(frame_start, Time::from(chrono::Utc::now())).to_secs();
      frame_start = Time::from(chrono::Utc::now());
      
      self.on_event()?;
      self.on_update(self.m_time_step)?;
      self.on_render()?;
      
      // Sync to engine tick rate.
      Time::wait_for(self.m_tick_rate as f64);
      
      self.m_window.refresh();  // Refresh window.
      frame_counter += 1;
      
      // If a second passed, display fps counter and reset it.
      if Time::get_delta(runtime, Time::from(chrono::Utc::now())).to_secs() >= 1.0 {
        if same_frame_counter != frame_counter {
          // Only display differing framerate to avoid output clutter for logging and displaying the
          // same fps several times.
          self.m_window.set_title(&format!("{0} | {1} FPS", title_cache, &frame_counter));
          #[cfg(feature = "debug")]
          log!(EnumLogColor::White, "INFO", "Framerate : {0}", &frame_counter);
        }
        
        same_frame_counter = frame_counter;
        frame_counter = 0;
        runtime = Time::from(chrono::Utc::now());
      }
    }
    return Ok(());
  }
  
  pub fn on_event(&mut self) -> Result<bool, EnumError> {
    if self.m_state != EnumState::Running {
      log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Cannot process events : Engine not started! Make sure to call `on_new()` before");
      return Err(EnumError::AppError);
    }
    
    if self.m_window.on_event()? {
      // If an event happened and Input::on_update() has been called, process custom inputs.
      if Input::get_modifier_key_combo(EnumKey::V, EnumModifier::Alt)? {
        self.m_window.toggle_vsync();
      }
      
      if Input::get_modifier_key_combo(EnumKey::Enter, EnumModifier::Alt)? {
        self.m_window.toggle_fullscreen()?;
      }
    }
    
    return Ok(false);
  }
  
  pub fn on_update(&mut self, time_step: f64) -> Result<bool, EnumError> {
    if self.m_state != EnumState::Running {
      log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Cannot update : Engine not started! Make sure to call `on_new()` before");
      return Err(EnumError::AppError);
    }
    
    self.m_window.on_update()?;
    return self.m_app.on_update(time_step);
  }
  
  pub fn on_render(&mut self) -> Result<(), EnumError> {
    if self.m_state != EnumState::Running {
      log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Cannot render : Engine not started! Make sure to call `on_new()` before");
      return Err(EnumError::AppError);
    }
    
    return self.m_app.on_render();
  }
  
  pub fn get_window(&mut self) -> &mut Window {
    return &mut self.m_window;
  }
  
  pub fn get_log_file() -> &'static std::fs::File {
    return &S_LOG_FILE_PTR;
  }
  
  
  pub fn get() -> *mut Engine {
    unsafe { return S_ENGINE; }
  }
  
  pub fn set_singleton(engine: &mut Engine) -> () {
    unsafe {
      S_ENGINE = engine;
      S_RENDERER = Some(&mut (*S_ENGINE).m_renderer);
      S_WINDOW = Some(&mut (*S_ENGINE).m_window);
    }
  }
}

impl Drop for Engine {
  fn drop(&mut self) {
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Dropping engine...");
    
    match self.on_delete() {
      Ok(_) => {
        self.m_state = EnumState::ShutDown;
        log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Dropped engine successfully");
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Error while dropping engine : Engine \
        returned with error => {:?} while trying to delete app!", err);
        return;
      }
    }
  }
}


/*
///////////////////////////////////   App       ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
 */

pub struct ExampleApp {
  m_shaders: Vec<Shader>,
  m_renderable_assets: Vec<REntity>,
  m_cameras: Vec<PerspectiveCamera>,
}

impl ExampleApp {
  pub fn default() -> Self {
    return ExampleApp {
      m_shaders: Vec::new(),
      m_renderable_assets: Vec::new(),
      m_cameras: Vec::new(),
    };
  }
}

pub struct EmptyApp {}

impl EmptyApp {
  pub fn new() -> Self {
    return EmptyApp {};
  }
}

impl TraitApp for EmptyApp {
  fn on_new(&mut self) -> Result<(), EnumError> {
    return Ok(());
  }
  
  fn on_delete(&mut self) -> Result<(), EnumError> {
    return Ok(());
  }
  
  fn on_event(&mut self) -> bool {
    return true;
  }
  
  fn on_update(&mut self, _time_step: f64) -> Result<bool, EnumError> {
    return Ok(false);
  }
  
  fn on_render(&mut self) -> Result<(), EnumError> {
    return Ok(());
  }
}

impl TraitApp for ExampleApp {
  fn on_new(&mut self) -> Result<(), EnumError> {
    let window = Window::get().expect("Cannot retrieve active window context!");
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Loading shaders...");
    
    let vertex_shader = ShaderStage::new(EnumShaderType::Vertex,
      EnumShaderSource::FromFile(String::from("res/shaders/test.vert")), true);
    let fragment_shader = ShaderStage::new(EnumShaderType::Fragment,
      EnumShaderSource::FromFile(String::from("res/shaders/test.frag")), true);
    
    let shader = Shader::new(vec![vertex_shader, fragment_shader])?;
    
    log!("INFO", "{0}", shader);
    
    self.m_shaders.push(shader);
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Loaded shaders successfully");
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending shaders to GPU...");
    // Sourcing and compilation.
    self.m_shaders[0].submit()?;
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Shaders sent to GPU successfully");
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Uploading camera view and projection to the GPU...");
    let aspect_ratio: f32 = unsafe {
      (*window).m_window_resolution.0 as f32 /
        (*window).m_window_resolution.1 as f32
    };
    
    self.m_cameras.push(PerspectiveCamera::from(75.0, aspect_ratio, 0.01, 1000.0));
    let renderer = Renderer::get().expect("Cannot retrieve active renderer!");
    unsafe { (*renderer).batch(&self.m_cameras[0])? };
    // self.m_cameras[0].set_view_projection();
    // self.m_shaders[0].upload_data("u_view_projection", self.m_cameras[0].get_matrix())?;
    // log!(EnumLogColor::Green, "INFO", "[App] -->\t Camera view and projection uploaded to GPU successfully");
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Sending asset 'awp.obj' to GPU...");
    self.m_renderable_assets.push(REntity::from(ResLoader::new("awp.obj")?));
    self.m_renderable_assets[0].translate(Vec3::new(&[10.0, -10.0, 50.0]));
    self.m_renderable_assets[0].rotate(Vec3::new(&[-90.0, 90.0, 0.0]));
    // self.m_shaders[0].upload_data("u_model", &self.m_renderable_assets[0].get_matrix())?;
    // log!("INFO", "[Shader] -->\t Uniform 'u_model_matrix' uploaded to GPU successfully");
    
    self.m_renderable_assets[0].send(&mut self.m_shaders[0])?;
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Asset sent to GPU successfully");
    
    // Show our window when we are done.
    unsafe { (*window).show() };
    return Ok(());
  }
  
  fn on_delete(&mut self) -> Result<(), EnumError> {
    return Ok(());
  }
  
  fn on_event(&mut self) -> bool {
    return false;
  }
  
  fn on_update(&mut self, _time_step: f64) -> Result<bool, EnumError> {
    return Ok(false);
  }
  
  fn on_render(&mut self) -> Result<(), EnumError> {
    let renderer = Renderer::get().expect("Cannot retrieve active renderer!");
    unsafe { (*renderer).on_render()? };
    
    return Ok(());
  }
}
