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

pub mod wave_core {
  use once_cell::sync::Lazy;
  
  use graphics::renderer::{self, Renderer};
  use graphics::shader::{self};
  use utils::Time;
  use window::{Window};
  
  use crate::log;
  use crate::wave_core::events::{EnumEvent, EnumEventMask};
  use crate::wave_core::input::{EnumAction, EnumKey, EnumMouseButton, Input};
  use crate::wave_core::layers::{EnumLayerType, Layer, TraitLayer};
  use crate::wave_core::layers::renderer_layer::RendererLayer;
  use crate::wave_core::layers::window_layer::WindowLayer;
  
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
  pub(crate) static S_LOG_FILE_PTR: Lazy<std::fs::File> = Lazy::new(|| utils::logger::init().unwrap());
  
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
  
  #[derive(Debug, PartialEq)]
  pub enum EnumError {
    NoActiveEngine,
    UndefinedError,
    AppError,
    ResourceError(assets::asset_loader::EnumError),
    ShaderError(shader::EnumError),
    RendererError(renderer::EnumError),
    WindowError(window::EnumError),
    InputError(input::EnumError),
    UiError(ui::EnumError),
    EventError(events::EnumError),
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
  
  // Convert renderer error to wave_core::EnumError.
  impl_enum_error!(renderer::EnumError, EnumError::RendererError);
  
  // Convert resource loader error to wave_core::EnumError.
  impl_enum_error!(assets::asset_loader::EnumError, EnumError::ResourceError);
  
  // Convert shader error to wave_core::EnumError.
  impl_enum_error!(shader::EnumError, EnumError::ShaderError);
  
  // Convert window error to wave_core::EnumError.
  impl_enum_error!(window::EnumError, EnumError::WindowError);
  
  // Convert input error to wave_core::EnumError.
  impl_enum_error!(input::EnumError, EnumError::InputError);
  
  // Convert ui errors to wave_core::EnumError.
  impl_enum_error!(ui::EnumError, EnumError::UiError);
  
  impl_enum_error!(events::EnumError, EnumError::EventError);
  
  pub struct Engine {
    m_layers_vec: Vec<Layer>,
    m_window: Window,
    m_renderer: Renderer,
    m_time_step: f64,
    m_tick_rate: f32,
    m_state: EnumState,
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
    pub fn new(window: Window, renderer: Renderer, app_layer: Layer) -> Result<Self, EnumError> {
      log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Launching Wave Engine...");
      
      // Setup basic layers.
      let layers = vec![app_layer];
      
      Ok({
        log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Launched Wave Engine successfully");
        Engine {
          m_layers_vec: layers,
          m_window: window,
          m_renderer: renderer,
          m_time_step: 0.0,
          m_tick_rate: 0.0,
          m_state: EnumState::NotStarted,
        }
      })
    }
    
    pub fn submit(&mut self) -> Result<(), EnumError> {
      if self.m_state != EnumState::NotStarted {
        log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Cannot instantiate engine : Engine already started!");
        return Err(EnumError::AppError);
      }
      
      self.m_state = EnumState::Starting;
      
      let mut window_layer: Layer = Layer::new("Main Window", WindowLayer::new(&mut self.m_window));
      let mut renderer_layer: Layer = Layer::new("Renderer", RendererLayer::new(&mut self.m_renderer));
      
      window_layer.enable_async_polling_for(EnumEventMask::c_window_close | EnumEventMask::c_window_size
        | EnumEventMask::c_keyboard);
      renderer_layer.enable_async_polling_for(EnumEventMask::c_window_close | EnumEventMask::c_window_size
        | EnumEventMask::c_keyboard);
      
      Engine::set_singleton(self);
      
      Self::push_layer(window_layer)?;
      Self::push_layer(renderer_layer)?;
      self.m_window.enable_callback(EnumEventMask::c_window_size | EnumEventMask::c_window_close | EnumEventMask::c_keyboard);
      
      for index in 2..self.m_layers_vec.len() {
        self.m_layers_vec[index].submit()?;
      }
      
      self.m_state = EnumState::Started;
      return Ok(());
    }
    
    pub fn run(&mut self) -> Result<(), EnumError> {
      self.submit()?;
      
      if self.m_state != EnumState::Started {
        log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Cannot run : Engine has not started up correctly!");
        return Err(EnumError::AppError);
      }
      
      self.m_state = EnumState::Running;
      
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
      while !self.m_window.is_closing() {
        self.m_time_step = Time::get_delta(frame_start, Time::from(chrono::Utc::now())).to_secs();
        frame_start = Time::from(chrono::Utc::now());
        
        self.m_window.poll_events();
        
        // Sync event polling.
        let mut result: Result<(), EnumError> = Ok(());
        self.m_layers_vec.iter_mut().rev()
          .filter(|layer| layer.is_sync_enabled())
          .all(|matching_layer| {
            result = matching_layer.on_sync_event();
            return result.is_ok();
          });
        
        // Exit function if an error occurred.
        result?;
        
        // Update layers.
        for layer in self.m_layers_vec.iter_mut().rev() {
          layer.on_update(self.m_time_step)?;
        }
        
        // Render layers.
        for layer in self.m_layers_vec.iter_mut().rev() {
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
    
    pub(crate) fn on_async_event(event: &EnumEvent) {
      let engine = unsafe { &mut *S_ENGINE.expect("Cannot push layer, engine not active!") };
      
      // Async event polling.
      let mut each_result: Result<bool, EnumError> = Ok(false);
      let _result = engine.m_layers_vec.iter_mut().rev()
        .filter(|layer| layer.polls(&event))
        .all(|matching_layer| {
          each_result = matching_layer.on_async_event(&event);
          // Short circuit if an error occurred or if the event has been processed.
          return each_result.is_ok() && !each_result.as_ref().unwrap();
        });
      
      // Mandatory event handling by the window context ignoring if the event has been processed or not.
      match event {
        EnumEvent::WindowCloseEvent(_) | EnumEvent::FramebufferEvent(_, _) => {
          engine.m_window.on_event(event);
        }
        _ => {}
      }
      
      if each_result.is_err() {
        log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Error while processing async event: {0:?}", each_result.err().unwrap());
      }
    }
    
    pub fn free(&mut self) -> Result<(), EnumError> {
      self.m_state = EnumState::Deleting;
      
      log!(EnumLogColor::Purple, "INFO", "[App] -->\t Shutting down app layers...");
      
      // Free all layers expect renderer and window in reverse.
      let matched_layers =
        self.m_layers_vec.iter_mut().rev().filter(|layer| !layer.is_type(EnumLayerType::Renderer) &&
          !layer.is_type(EnumLayerType::Window));
      
      for layer in matched_layers {
        log!("INFO", "[Engine] -->\t Deleting layer: {0}", layer);
        layer.free()?;
      }
      
      log!(EnumLogColor::Green, "INFO", "[App] -->\t Shut down app layers successfully");
      
      self.m_renderer.free()?;
      self.m_window.free()?;
      
      self.m_state = EnumState::Deleted;
      return Ok(());
    }
    
    pub fn panic_shutdown(error: EnumError) {
      let engine = unsafe { &mut *S_ENGINE.expect("Cannot push layer, engine not active!") };
      log!(EnumLogColor::Purple, "INFO", "[App] -->\t Dropping engine...");
      
      match engine.free() {
        Ok(_) => {
          engine.m_state = EnumState::ShutDown;
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
    
    pub fn push_layer(mut new_layer: Layer) -> Result<(), EnumError> {
      let engine = unsafe { &mut *S_ENGINE.expect("Cannot push layer, engine not active!") };
      
      new_layer.submit()?;
      // If this was the first layer requesting polling with this poll mask, enable polling for this type of event.
      if engine.m_layers_vec.iter().all(|layer| !layer.poll_includes(new_layer.get_poll_mask())) {
        engine.m_window.enable_polling(new_layer.get_poll_mask());
      }
      
      log!("INFO", "[Engine] -->\t Pushed layer: {0}", new_layer);
      engine.m_layers_vec.push(new_layer);
      engine.m_layers_vec.sort_unstable();
      return Ok(());
    }
    
    pub fn pop_layer(popped_layer: &mut Layer) -> Result<bool, EnumError> {
      let engine = unsafe { &mut *S_ENGINE.expect("Cannot push layer, engine not active!") };
      
      let position_found = engine.m_layers_vec.iter_mut().position(|layer| layer == popped_layer);
      if position_found.is_some() {
        log!("INFO", "[Engine] -->\t Popping layer: {0}", popped_layer.m_name);
        engine.m_layers_vec.remove(position_found.unwrap());
        engine.m_layers_vec.sort_unstable();
        
        // If no other layer polls for the specific poll mask, remove polling for those types of events.
        if engine.m_layers_vec.iter().all(|layer| !layer.poll_includes(layer.get_poll_mask())) {
          engine.m_window.disable_polling(popped_layer.get_poll_mask());
        }
        return Ok(true);
      }
      return Ok(false);
    }
    
    pub fn get_log_file() -> &'a std::fs::File {
      return &S_LOG_FILE_PTR;
    }
    
    pub fn get_time_step() -> f64 {
      let engine = unsafe { &mut *S_ENGINE.expect("Cannot retrieve active engine!") };
      return engine.m_time_step;
    }
    
    pub fn get_active_renderer() -> &'a mut Renderer {
      let engine = unsafe { &mut *S_ENGINE.expect("Cannot retrieve active engine!") };
      return &mut engine.m_renderer;
    }
    
    pub fn get_active_window() -> &'a mut Window {
      let engine = unsafe { &mut *S_ENGINE.expect("Cannot retrieve active engine!") };
      return &mut engine.m_window;
    }
    
    pub fn is_key(key: EnumKey, state: EnumAction) -> bool {
      let engine = unsafe { &mut *S_ENGINE.expect("Cannot retrieve active engine!") };
      return Input::get_key_state(&engine.m_window, key, state);
    }
    
    pub fn is_mouse_btn(button: EnumMouseButton, state: EnumAction) -> bool {
      let engine = unsafe { &mut *S_ENGINE.expect("Cannot retrieve active engine!") };
      return Input::get_mouse_button_state(&engine.m_window, button, state);
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
          self.m_state = EnumState::ShutDown;
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
    
    fn on_submit(&mut self) -> Result<(), EnumError> {
      return Ok(());
    }
    
    fn on_sync_event(&mut self) -> Result<(), EnumError> {
      todo!()
    }
    
    fn on_async_event(&mut self, _event: &EnumEvent) -> Result<bool, EnumError> {
      return Ok(false);
    }
    
    fn on_update(&mut self, _time_step: f64) -> Result<(), EnumError> {
      return Ok(());
    }
    
    fn on_render(&mut self) -> Result<(), EnumError> {
      return Ok(());
    }
    
    fn on_free(&mut self) -> Result<(), EnumError> {
      return Ok(());
    }
    
    fn to_string(&self) -> String {
      return String::from("[Empty App]");
    }
  }
}