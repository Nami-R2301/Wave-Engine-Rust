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

use events::{EnumEvent};
use graphics::renderer::{self, Renderer};
use graphics::shader::{self};
use input::{EnumAction, EnumKey, EnumMouseButton, Input};
use layers::{EnumLayerType, Layer, TraitLayer};
use layers::renderer_layer::RendererLayer;
use layers::window_layer::WindowLayer;
use once_cell::sync::Lazy;
#[cfg(feature = "debug")]
use utils::macros::logger::{color_to_str, EnumLogColor};
use utils::Time;
use window::Window;
use crate::events::EnumEventMask;

pub mod dependencies;
pub mod ui;
pub mod window;
pub mod math;
pub mod graphics;
pub mod utils;
pub mod assets;
pub mod camera;
pub mod input;
pub mod events;
pub mod layers;

static mut S_ENGINE: Option<*mut Engine> = None;
pub(crate) static S_LOG_FILE_PTR: Lazy<std::fs::File> = Lazy::new(|| utils::macros::logger::init().unwrap());

#[derive(Debug, Copy, Clone, PartialEq)]
enum EnumEngineState {
  NotStarted,
  Starting,
  Started,
  Running,
  Deleting,
  Deleted,
  ShutDown,
}

#[derive(Debug, PartialEq)]
pub enum EnumEngineError {
  NoActiveEngine,
  UndefinedError,
  AppError,
  LayerError(layers::EnumLayerError),
  ResourceError(assets::asset_loader::EnumAssetError),
  ShaderError(shader::EnumShaderError),
  RendererError(renderer::EnumRendererError),
  WindowError(window::EnumWindowError),
  InputError(input::EnumInputError),
  UiError(ui::EnumUIError),
  EventError(events::EnumEventError),
}

macro_rules! impl_enum_error {
  ($error_type: ty, $resulting_error: expr) => {
    impl From<$error_type> for EnumEngineError {
      fn from(err: $error_type) -> EnumEngineError {
        log!(EnumLogColor::Red, "ERROR", "{0}", err);
        return $resulting_error(err);
      }
    }
  }
}

// Convert layer error to wave_core::EnumError.
impl_enum_error!(layers::EnumLayerError, EnumEngineError::LayerError);

// Convert renderer error to wave_core::EnumError.
impl_enum_error!(renderer::EnumRendererError, EnumEngineError::RendererError);

// Convert resource loader error to wave_core::EnumError.
impl_enum_error!(assets::asset_loader::EnumAssetError, EnumEngineError::ResourceError);

// Convert shader error to wave_core::EnumError.
impl_enum_error!(shader::EnumShaderError, EnumEngineError::ShaderError);

// Convert window error to wave_core::EnumError.
impl_enum_error!(window::EnumWindowError, EnumEngineError::WindowError);

// Convert input error to wave_core::EnumError.
impl_enum_error!(input::EnumInputError, EnumEngineError::InputError);

// Convert ui errors to wave_core::EnumError.
impl_enum_error!(ui::EnumUIError, EnumEngineError::UiError);

impl_enum_error!(events::EnumEventError, EnumEngineError::EventError);

pub struct Engine {
  m_layers: Vec<Layer>,
  m_window: Window,
  m_renderer: Renderer,
  m_time_step: f64,
  m_tick_rate: f32,
  m_state: EnumEngineState,
}

impl<'a> Engine {
  /// Setup new engine struct containing an app with the [TraitApp] behavior in order to call
  /// `on_new()`, `free()`, `on_update()`, `on_event()`, and `on_render()`. By default, the
  /// engine uses an OpenGL renderer and GLFW for the context creation and handling.
  ///
  /// ### Arguments:
  ///
  /// * `app_provided`: A boxed generic app struct `T` which respects the trait [TraitApp].
  ///
  /// ### Returns:
  ///   - `Result<GlREntity, EnumError>` : Will return a valid Engine if successful, otherwise an [EnumEngineError]
  ///     on any error encountered. These include, but are not limited to :
  ///     + [EnumEngineError::AppError] : If the app crashes for whatever reason the client may choose.
  ///     + [EnumEngineError::RendererError] : If the renderer crashes due to an invalid process loading,
  ///       missing extensions, unsupported version and/or invalid GPU command.
  ///     + [EnumEngineError::WindowError] : If the window context crashes due to invalid context creation,
  ///       deletion and/or command (GLX/X11 for Linux, WIN32 for Windows).
  ///
  /// ### Examples
  ///
  /// ```text
  /// use wave_core::{Engine, EnumError};
  ///
  /// let my_app = Box::new(ExampleApp::new());
  /// // Allocated on the stack -- Use new_shared() to allocate on the heap.
  /// let mut engine = Box::new(Engine::new(my_app)?);
  ///
  /// // Run `on_new()` for `my_app` prior to running.
  /// engine.on_new()?;
  /// engine.run();
  /// engine.free();
  /// return Ok(());
  /// ```
  pub fn new(window: Window, renderer: Renderer, app_layers: Vec<Layer>) -> Result<Self, EnumEngineError> {
    Ok(Engine {
      m_layers: app_layers,
      m_window: window,
      m_renderer: renderer,
      m_time_step: 0.0,
      m_tick_rate: 0.0,
      m_state: EnumEngineState::NotStarted,
    })
  }
  
  pub fn apply(&mut self) -> Result<(), EnumEngineError> {
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Launching Wave Engine...");
    if self.m_state != EnumEngineState::NotStarted {
      log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Cannot instantiate engine : Engine already started!");
      return Err(EnumEngineError::AppError);
    }
    
    self.m_state = EnumEngineState::Starting;
    let mut window_layer = Layer::new("Window Layer", WindowLayer::new(&mut self.m_window));
    let mut renderer_layer = Layer::new("Renderer Layer", RendererLayer::new(&mut self.m_renderer));
    
    window_layer.enable_async_polling_for(EnumEventMask::WindowClose | EnumEventMask::WindowSize
      | EnumEventMask::Keyboard);
    renderer_layer.enable_async_polling_for(EnumEventMask::WindowClose | EnumEventMask::WindowSize
      | EnumEventMask::Keyboard);
    
    // Setup window context for polling to ba available when pushing subsequent layers.
    self.m_window.apply()?;
    
    self.m_layers.push(window_layer);
    self.m_layers.push(renderer_layer);
    self.m_layers.sort_unstable();
    
    Engine::set_singleton(self);
    
    for layer in self.m_layers.iter_mut() {
      Self::enable_async_polling_for(layer);
      layer.apply()?;
    }
    
    self.m_state = EnumEngineState::Started;
    log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Launched Wave Engine successfully");
    return Ok(());
  }
  
  pub fn run(&mut self) -> Result<(), EnumEngineError> {
    self.apply()?;
    
    if self.m_state != EnumEngineState::Started {
      log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Cannot run : Engine has not started up correctly!");
      return Err(EnumEngineError::AppError);
    }
    
    self.m_state = EnumEngineState::Running;
    
    // For time step.
    let mut frame_start: Time = Time::from(chrono::Utc::now());
    
    // For uptime and fps.
    let mut frame_counter: u32 = 0;
    // For keeping track of previous logged fps.
    let mut same_frame_counter: u32 = 0;
    let mut runtime: Time = Time::new();
    
    let title_cache: String = format!("Wave Engine (Rust) | {0:?}", self.m_renderer.m_type);
    self.m_window.set_title(&title_cache);
    
    // Loop until the user closes the window or an error occurs.
    while !self.m_window.is_closed() {
      self.m_time_step = Time::get_delta(frame_start, Time::from(chrono::Utc::now())).to_secs();
      frame_start = Time::from(chrono::Utc::now());
      
      self.m_window.poll_events();
      
      // Sync event polling.
      let mut result: Result<(), EnumEngineError> = Ok(());
      self.m_layers.iter_mut().rev()
        .filter(|layer| {
          if !layer.is_sync_enabled() {
            return false;
          }
          layer.get_sync_interval() == 0 || frame_counter % layer.get_sync_interval() == 0
        })
        .all(|matching_layer| {
          result = matching_layer.on_sync_event();
          return result.is_ok();
        });
      
      // Exit function if an error occurred.
      result?;
      
      // Update layers.
      for layer in self.m_layers.iter_mut().rev() {
        layer.on_update(self.m_time_step)?;
      }
      
      // Render layers.
      for layer in self.m_layers.iter_mut().rev() {
        layer.on_render()?;
      }
      
      // Sync to engine tick rate.
      Time::wait_for(self.m_tick_rate as f64);
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
  
  pub fn get_window_ref(&self) -> &Window {
    return &self.m_window;
  }
  
  pub fn get_window_mut(&mut self) -> &mut Window {
    return &mut self.m_window;
  }
  
  pub fn get_renderer_ref(&self) -> &Renderer {
    return &self.m_renderer;
  }
  
  pub fn get_renderer_mut(&mut self) -> &mut Renderer {
    return &mut self.m_renderer;
  }
  
  pub fn free(&mut self) -> Result<(), EnumEngineError> {
    self.m_state = EnumEngineState::Deleting;
    
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Shutting down layers...");
    
    // Free all layers in reverse.
    for layer in self.m_layers.iter_mut().rev() {
      layer.free()?;
    }
    
    log!(EnumLogColor::Green, "INFO", "[App] -->\t Shut down layers successfully");
    
    self.m_state = EnumEngineState::Deleted;
    return Ok(());
  }
  
  pub fn panic_shutdown(mut self, error: EnumEngineError) {
    log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Dropping engine...");
    
    match self.free() {
      Ok(_) => {
        self.m_state = EnumEngineState::ShutDown;
        log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Dropped engine successfully");
      }
      #[allow(unused)]
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Error while dropping engine : Engine \
        returned with error => {:?} while trying to delete app!", err);
      }
    }
    panic!("{}", format!("Fatal error occurred : {0:?}", error))
  }
  
  pub fn push_layer(&mut self, mut new_layer: Layer, apply_on_push: bool) -> Result<(), EnumEngineError> {
    if apply_on_push {
      new_layer.apply()?;
    }
    
    log!("INFO", "[Engine] -->\t Pushed layer: {0}", new_layer);
    self.m_layers.push(new_layer);
    self.m_layers.sort_unstable();
    return Ok(());
  }
  
  pub fn pop_layer(&mut self) -> Result<Option<Layer>, EnumEngineError> {
    if self.m_layers.is_empty() {
      return Ok(None);
    }
    
    log!("INFO", "[Engine] -->\t Popping layer: {0}", self.m_layers.last().unwrap().m_name);
    let layer_popped = self.m_layers.pop();
    self.m_layers.sort_unstable();
    
    return Ok(layer_popped);
  }
  
  pub fn get_time_step(&self) -> f64 {
    return self.m_time_step;
  }
  
  pub fn is_key(key: EnumKey, state: EnumAction) -> bool {
    let engine = unsafe { &mut *S_ENGINE.expect("Cannot retrieve active engine!") };
    return Input::get_key_state(&engine.m_window, key, state);
  }
  
  pub fn is_mouse_btn_from(button: EnumMouseButton, state: EnumAction) -> bool {
    let engine = unsafe { &mut *S_ENGINE.expect("Cannot retrieve active engine!") };
    return Input::get_mouse_button_state(&engine.m_window, button, state);
  }
  
  pub fn get_log_file() -> &'a std::fs::File {
    return &S_LOG_FILE_PTR;
  }
  
  ////////////////////////////// PRIVATE FUNCTIONS ////////////////////////////////
  
  pub(crate) fn enable_async_polling_for(requested_layer: &mut Layer) {
    let engine = unsafe { &mut *S_ENGINE.expect("Cannot push layer, engine not active!") };
    
    // If a window context exists at this moment in order to enable polling for it.
    if !engine.m_window.is_applied() {
      log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Cannot enable polling for {0} in window, No active window!",
        requested_layer.get_poll_mask());
      return;
    }
    let poll_mask = requested_layer.get_poll_mask();
    
    if engine.m_layers.iter().any(|layer| layer.poll_includes(poll_mask)) {
      engine.m_window.enable_polling_for(poll_mask);
      engine.m_window.enable_callback_for(poll_mask);
    }
  }
  
  #[allow(unused)]
  pub(crate) fn disable_async_polling_for(poll_mask: EnumEventMask) {
    let engine = unsafe { &mut *S_ENGINE.expect("Cannot push layer, engine not active!") };
    
    // If a window context exists at this moment in order to enable polling for it.
    if !engine.m_window.is_applied() {
      log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Cannot enable polling for {0} in window, No active window!", poll_mask);
      return;
    }
    
    // If no other layer polls for the specific poll mask, remove polling for those types of events.
    if engine.m_layers.iter().all(|layer| !layer.poll_includes(poll_mask)) {
      engine.m_window.disable_polling(poll_mask);
    }
  }
  
  pub(crate) fn on_async_event(event: &EnumEvent) {
    let engine = unsafe { &mut *S_ENGINE.expect("Cannot push layer, engine not active!") };
    
    // Async event polling.
    let mut each_result: Result<bool, EnumEngineError> = Ok(false);
    let _result = engine.m_layers.iter_mut().rev()
      .filter(|layer| layer.polls(&event))
      .all(|matching_layer| {
        // Mandatory event handling, ignoring if the event has been processed or not.
        match event {
          EnumEvent::WindowCloseEvent(_) | EnumEvent::FramebufferEvent(_, _) => {
            each_result = matching_layer.on_async_event(event);
            return each_result.is_ok();
          }
          _ => {}
        }
        each_result = matching_layer.on_async_event(&event);
        // Short circuit if an error occurred or if the event has been processed.
        return each_result.is_ok() && !each_result.as_ref().unwrap();
      });
    
    if each_result.is_err() {
      log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Error while processing async event: {0:?}", each_result.err().unwrap());
    }
  }
  
  pub(crate) fn get_active_renderer() -> &'a mut Renderer {
    let engine = unsafe { &mut *S_ENGINE.expect("Cannot retrieve active engine!") };
    return &mut engine.m_renderer;
  }
  
  pub(crate) fn get_active_window() -> &'a mut Window {
    let engine = unsafe { &mut *S_ENGINE.expect("Cannot retrieve active engine!") };
    return &mut engine.m_window;
  }
  
  fn set_singleton(engine: &mut Engine) -> () {
    unsafe { S_ENGINE = Some(engine) };
  }
}

impl Drop for Engine {
  fn drop(&mut self) {
    log!(EnumLogColor::Purple, "INFO", "[App] -->\t Dropping engine...");
    
    match self.free() {
      Ok(_) => {
        self.m_state = EnumEngineState::ShutDown;
        log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Dropped engine successfully");
      }
      #[allow(unused)]
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

pub struct EmptyApp {}

impl EmptyApp {
  pub fn default() -> Self {
    return EmptyApp {};
  }
}

impl TraitLayer for EmptyApp {
  fn get_type(&self) -> EnumLayerType {
    return EnumLayerType::App;
  }
  
  fn on_apply(&mut self) -> Result<(), EnumEngineError> {
    return Ok(());
  }
  
  fn on_sync_event(&mut self) -> Result<(), EnumEngineError> {
    todo!()
  }
  
  fn on_async_event(&mut self, _event: &EnumEvent) -> Result<bool, EnumEngineError> {
    return Ok(false);
  }
  
  fn on_update(&mut self, _time_step: f64) -> Result<(), EnumEngineError> {
    return Ok(());
  }
  
  fn on_render(&mut self) -> Result<(), EnumEngineError> {
    return Ok(());
  }
  
  fn free(&mut self) -> Result<(), EnumEngineError> {
    return Ok(());
  }
  
  fn to_string(&self) -> String {
    return String::from("[Empty App]");
  }
}