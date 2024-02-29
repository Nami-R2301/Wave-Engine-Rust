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
  use glfw::WindowEvent;
  use once_cell::sync::Lazy;
  
  use graphics::renderer::{self, EnumApi, Renderer, S_RENDERER};
  #[cfg(feature = "debug")]
  use graphics::shader::{self};
  use input::{EnumKey, EnumModifier, Input};
  use utils::Time;
  use window::{EnumWindowMode, S_WINDOW, Window};
  
  use crate::log;
  use crate::wave_core::layers::{EnumLayerType, Layer};
  use crate::wave_core::layers::app_layer::AppLayer;
  #[cfg(feature = "imgui")]
  use crate::wave_core::layers::imgui_layer::ImguiLayer;
  #[cfg(feature = "imgui")]
  use crate::wave_core::ui::ui_imgui::Imgui;
  
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
  pub mod layers;
  mod events;
  
  static mut S_ENGINE: Option<*mut Engine> = None;
  
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
  
  #[derive(Debug, PartialEq)]
  pub enum EnumError {
    UndefinedError,
    AppError,
    ResourceError(assets::asset_loader::EnumError),
    ShaderError(shader::EnumError),
    RendererError(renderer::EnumError),
    WindowError(window::EnumError),
    InputError(input::EnumError),
    UiError(ui::EnumError),
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
  
  pub trait TraitApp {
    fn on_new(&mut self) -> Result<(), EnumError>;
    fn on_event(&mut self, window_event: &WindowEvent) -> Result<bool, EnumError>;
    fn on_update(&mut self, time_step: f64) -> Result<(), EnumError>;
    fn on_render(&mut self) -> Result<(), EnumError>;
    fn on_delete(&mut self) -> Result<(), EnumError>;
  }
  
  pub struct Engine {
    m_app: Box<dyn TraitApp>,
    m_layers: Vec<Layer>,
    m_window: Box<Window>,
    m_renderer: Box<Renderer>,
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
    /// use wave_core::{Engine, EnumError};
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
      let mut window = Box::new(Window::new(api_preference, Some((1920, 1080)),
        None, None, EnumWindowMode::Windowed)?);
      log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Opened window successfully");
      
      log!(EnumLogColor::Purple, "INFO", "[Engine] -->\t Starting renderer...");
      // Create graphics context.
      let renderer = Box::new(Renderer::new(api_preference, &mut window)?);
      log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Started renderer successfully");
      
      let layers: Vec<Layer> = vec![];
      
      Ok({
        log!(EnumLogColor::Green, "INFO", "[Engine] -->\t Launched Wave Engine successfully");
        Engine {
          m_app: app_provided.unwrap_or(Box::new(EmptyApp::default())),
          m_layers: layers,
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
      
      let window_layer: Layer = Layer::new("Main Window", EnumLayerType::Window, WindowLayer::new(self.m_window.as_mut()));
      let renderer_layer: Layer = Layer::new("Renderer", EnumLayerType::Renderer, RendererLayer::new(self.m_renderer.as_mut()));
      
      #[cfg(feature = "imgui")]
      let imgui_layer: Layer = Layer::new("Imgui", EnumLayerType::Imgui,
        ImguiLayer::new(Imgui::new(self.m_renderer.m_type, self.m_window.as_mut())));
      
      let app_layer: Layer = Layer::new("App", EnumLayerType::App, AppLayer::new(self.m_app.as_mut()));
      
      self.m_layers.push(window_layer);
      #[cfg(feature = "imgui")]
      self.m_layers.push(imgui_layer);
      self.m_layers.push(renderer_layer);
      self.m_layers.push(app_layer);
      
      Engine::set_singleton(self);
      
      for layer in self.m_layers.iter_mut() {
        layer.on_new()?;
      }
      
      self.m_state = EnumState::Started;
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
      
      // For uptime and fps.
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
        
        // Engine routine.
        self.process_events()?;
        
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
    
    fn process_events(&mut self) -> Result<(), EnumError> {
      if self.m_state != EnumState::Running {
        log!(EnumLogColor::Red, "ERROR", "[Engine] -->\t Cannot process events : Engine not started! Make sure to call `on_new()` before");
        return Err(EnumError::AppError);
      }
      
      self.m_window.get_api_mut().poll_events();
      
      let mut event_processed: bool = false;
      for (_, event) in glfw::flush_messages(&self.m_window.m_api_window_events) {
        for layer in self.m_layers.iter_mut().rev() {
          if layer.on_event(&event)? {
            event_processed = true;
            break;
          }
        }
      }
      
      if !event_processed {
        // If an event happened and Input::on_update() has been called, process custom inputs.
        Input::reset();
        if Input::get_modifier_key_combo(&self.m_window, EnumKey::V, EnumModifier::Alt)? {
          self.m_window.toggle_vsync();
        }
        
        if Input::get_modifier_key_combo(&self.m_window, EnumKey::Enter, EnumModifier::Alt)? {
          self.m_window.toggle_fullscreen()?;
        }
      }
      return Ok(());
    }
    
    pub fn on_delete(&mut self) -> Result<(), EnumError> {
      self.m_state = EnumState::Deleting;
      
      log!(EnumLogColor::Purple, "INFO", "[App] -->\t Shutting down app...");
      // Free app first.
      self.m_app.on_delete()?;
      log!(EnumLogColor::Green, "INFO", "[App] -->\t Shut down app successfully");
      
      self.m_renderer.on_delete()?;
      self.m_window.on_delete()?;
      
      self.m_state = EnumState::Deleted;
      return Ok(());
    }
    
    pub fn push_layer(&mut self, new_layer: Layer) -> () {
      self.m_layers.push(new_layer);
    }
    
    pub fn pop_layer(&mut self, layer_type: EnumLayerType) -> bool {
      // Reverse iterator to get the last layer corresponding to the requested layer to remove.
      if self.m_layers.iter().rev().any(|layer| layer.is(layer_type)) {
        return true;
      }
      return false;
    }
    
    pub fn get_window(&mut self) -> &mut Window {
      return &mut self.m_window;
    }
    
    pub fn get_renderer(&mut self) -> &mut Renderer {
      return &mut self.m_renderer;
    }
    
    pub fn get_renderer_api(&self) -> EnumApi {
      return self.m_renderer.m_type;
    }
    
    pub fn get_log_file() -> &'static std::fs::File {
      return &S_LOG_FILE_PTR;
    }
    
    
    pub fn get() -> *mut Engine {
      unsafe {
        return S_ENGINE.expect("[Engine] -->\t Cannot retrieve engine : Engine is not initialized!");
      }
    }
    
    pub fn set_singleton(engine: &mut Engine) -> () {
      unsafe {
        S_ENGINE = Some(engine);
        S_RENDERER = Some((*S_ENGINE.unwrap()).m_renderer.as_mut());
        S_WINDOW = Some((*S_ENGINE.unwrap()).m_window.as_mut());
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
  
  impl TraitApp for EmptyApp {
    fn on_new(&mut self) -> Result<(), EnumError> {
      return Ok(());
    }
    
    fn on_event(&mut self, _window_event: &WindowEvent) -> Result<bool, EnumError> {
      return Ok(false);
    }
    
    fn on_update(&mut self, _time_step: f64) -> Result<(), EnumError> {
      return Ok(());
    }
    
    fn on_render(&mut self) -> Result<(), EnumError> {
      return Ok(());
    }
    
    fn on_delete(&mut self) -> Result<(), EnumError> {
      return Ok(());
    }
  }
}