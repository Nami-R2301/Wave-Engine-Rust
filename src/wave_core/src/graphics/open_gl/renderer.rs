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

use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::mem::size_of;

use gl46::GlFns;

use crate::{Engine, S_ENGINE};
use crate::assets::renderable_assets::{EnumPrimitiveType, EnumVertexMemberOffset, REntity};
use crate::events::EnumEvent;
use crate::graphics::{open_gl, renderer};
use crate::graphics::open_gl::buffer::{EnumAttributeType, EnumUboType, EnumUboTypeSize, GLchar, GLenum, GlIbo, GLsizei, GlUbo, GLuint, GlVao, GlVbo, GlVertexAttribute};
use crate::graphics::renderer::{EnumCallCheckingType, EnumRendererError, EnumRendererOption, EnumRendererState, TraitContext};
use crate::graphics::shader::{EnumShaderLanguage, Shader};
use crate::math::Mat4;
use crate::utils::macros::logger::*;
use crate::window::Window;

/*
///////////////////////////////////   OpenGL renderer   ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
 */

pub(crate) static mut S_GL_4_6: Option<GlFns> = None;

#[macro_export]
macro_rules! check_gl_call {
    () => {};
    ($name:literal, $gl_function:expr) => {
      let engine = unsafe { &mut *S_ENGINE.expect("Cannot log, no active Engine!") };
        if engine.m_renderer.m_options.contains(&crate::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::graphics::renderer::EnumCallCheckingType::None)) ||
          engine.m_renderer.m_options.contains(&crate::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::graphics::renderer::EnumCallCheckingType::Async))
         {
          unsafe { $gl_function };
        } else if engine.m_renderer.m_options.contains(&crate::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::graphics::renderer::EnumCallCheckingType::Sync)) ||
        engine.m_renderer.m_options.contains(&crate::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::graphics::renderer::EnumCallCheckingType::SyncAndAsync)) {
          unsafe { $gl_function };
          let error = unsafe { gl::GetError() };
          if error != gl::NO_ERROR {
            log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t OpenGL call error :\nCall =>\t\t {1:?}\n\
            Code =>\t\t 0x{2:x}\nWhere =>\t {3}", $name, stringify!($gl_function), error, trace!());
            return Err(crate::graphics::open_gl::renderer::EnumOpenGLError::InvalidOperation(error).into());
          }
        } else {
          unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
          unsafe { $gl_function };
          let error = unsafe { gl::GetError() };
          if error != gl::NO_ERROR {
            log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t OpenGL call error :\nCall =>\t\t {1:?}\n\
            Code =>\t\t 0x{2:x}\nWhere =>\t {3}", $name, stringify!($gl_function), error, trace!());
            return Err(crate::graphics::open_gl::renderer::EnumOpenGLError::InvalidOperation(error).into());
          }
        }
    };
    ($name:literal, let $var:ident: $var_type:ty = $gl_function:expr) => {
      let $var:$var_type;
      let engine = unsafe { &mut *S_ENGINE.expect("Cannot log, no active Engine!") };
        if engine.m_renderer.m_options.contains(&crate::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::graphics::renderer::EnumCallCheckingType::None)) ||
          engine.m_renderer.m_options.contains(&crate::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::graphics::renderer::EnumCallCheckingType::Async))
         {
          $var = unsafe { $gl_function };
        } else if engine.m_renderer.m_options.contains(&crate::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::graphics::renderer::EnumCallCheckingType::Sync)) ||
        engine.m_renderer.m_options.contains(&crate::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::graphics::renderer::EnumCallCheckingType::SyncAndAsync)) {
          $var = unsafe { $gl_function };
          let error = unsafe { gl::GetError() };
          if error != gl::NO_ERROR {
            log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t OpenGL call error :\nCall =>\t\t {1:?}\n\
            Code =>\t\t 0x{2:x}\nWhere =>\t {3}", $name, stringify!($gl_function), error, trace!());
            return Err(crate::graphics::open_gl::renderer::EnumOpenGLError::InvalidOperation(error).into());
          }
        } else {
        unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
        $var = unsafe { $gl_function };
        let error = unsafe { gl::GetError() };
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t OpenGL call error :\nCall =>\t\t {1:?}\n\
            Code =>\t\t 0x{2:x}\nWhere =>\t {3}", $name, stringify!($gl_function), error, trace!());
          return Err(crate::graphics::open_gl::renderer::EnumOpenGLError::InvalidOperation(error).into());
        }
      }
    };
    ($name:literal, $var:ident = $gl_function:expr) => {
      $var = Default::default();
     let engine = unsafe { &mut *S_ENGINE.expect("Cannot log, no active Engine!") };
        if engine.m_renderer.m_options.contains(&crate::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::graphics::renderer::EnumCallCheckingType::None)) ||
          engine.m_renderer.m_options.contains(&crate::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::graphics::renderer::EnumCallCheckingType::Async))
         {
          $var = unsafe { $gl_function };
        } else if engine.m_renderer.m_options.contains(&crate::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::graphics::renderer::EnumCallCheckingType::Sync)) ||
        engine.m_renderer.m_options.contains(&crate::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::graphics::renderer::EnumCallCheckingType::SyncAndAsync)) {
          $var = unsafe { $gl_function };
          let error = unsafe { gl::GetError() };
          if error != gl::NO_ERROR {
            log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t OpenGL call error :\nCall =>\t\t {1:?}\n\
            Code =>\t\t 0x{2:x}\nWhere =>\t {3}", $name, stringify!($gl_function), error, trace!());
            return Err(crate::graphics::open_gl::renderer::EnumOpenGLError::InvalidOperation(error).into());
          }
        } else {
        unsafe { while gl::GetError() != gl::NO_ERROR {} };
        $var = unsafe { $gl_function };
        let error = unsafe { gl::GetError() };
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t OpenGL call error :\nCall =>\t\t {1:?}\n\
            Code =>\t\t 0x{2:x}\nWhere =>\t {3}", $name, stringify!($gl_function), error, trace!());
          return Err(crate::graphics::open_gl::renderer::EnumOpenGLError::InvalidOperation(error).into());
        }
      }
    };
}

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
pub enum EnumOpenGLError {
  CStringError,
  ApiFunctionLoadingError,
  UnsupportedApiFunction,
  InvalidContext,
  InvalidOperation(GLenum),
  MSAAError,
  EntityUUIDNotFound,
  InvalidEntityType,
  InvalidBufferOperation(open_gl::buffer::EnumGlBufferError),
  InvalidShaderOperation(open_gl::shader::EnumError),
}

impl From<open_gl::buffer::EnumGlBufferError> for EnumOpenGLError {
  fn from(value: open_gl::buffer::EnumGlBufferError) -> Self {
    return EnumOpenGLError::InvalidBufferOperation(value);
  }
}

impl From<open_gl::shader::EnumError> for EnumOpenGLError {
  fn from(value: open_gl::shader::EnumError) -> Self {
    return EnumOpenGLError::InvalidShaderOperation(value);
  }
}

#[derive(Clone)]
struct GlShaderInfo {
  m_version: u16,
  m_id: u32,
}

#[derive(Clone)]
struct GlPrimitiveInfo {
  m_uuid: u64,
  m_linked_shader: GlShaderInfo,
  m_vao_index: usize,
  m_vbo_index: usize,
  m_ibo_index: usize,
  m_vbo_length: usize,
  m_ibo_length: usize,
  m_vbo_count: usize,
  // Vertex count for the primitive in vbo.
  m_ibo_count: usize,
  // Size per vertex for the primitive in vbo.
  m_size_per_vertex: usize,
  m_visible: bool,  // Make primitive appear or disappear upon request from the user
}

struct GlBatchPrimitives {
  m_primitives: Vec<GlPrimitiveInfo>,
  m_vao_buffers: Vec<GlVao>,
  m_vbo_buffers: Vec<GlVbo>,
  m_ibo_buffers: Vec<GlIbo>,
  m_ubo_buffers: Vec<GlUbo>,
}

impl GlBatchPrimitives {
  pub fn new() -> Self {
    return GlBatchPrimitives {
      m_primitives: Vec::new(),
      m_vao_buffers: Vec::new(),
      m_vbo_buffers: Vec::new(),
      m_ibo_buffers: Vec::new(),
      m_ubo_buffers: Vec::new(),
    };
  }
}

pub struct GlContext {
  pub(crate) m_ext: HashMap<String, ()>,
  pub(crate) m_state: EnumRendererState,
  m_batch: GlBatchPrimitives,
  m_debug_callback: gl::types::GLDEBUGPROC,
  m_wireframe_enabled: bool,
}

impl TraitContext for GlContext {
  fn default() -> Self {
    return Self {
      m_state: EnumRendererState::NotCreated,
      m_ext: HashMap::new(),
      m_batch: GlBatchPrimitives::new(),
      m_debug_callback: Some(gl_error_callback),
      m_wireframe_enabled: false,
    };
  }
  
  fn on_new(window: &mut Window) -> Result<Self, EnumRendererError> {
    // Init context.
    window.init_opengl_surface();
    gl::load_with(|f_name| window.get_api_ref().get_proc_address_raw(f_name));
    unsafe {
      match GlFns::load_from(&|f_name| {
        let string = std::ffi::CStr::from_ptr(f_name as *const std::ffi::c_char);
        window.get_api_ref().get_proc_address_raw(string.to_str().unwrap())
      }) {
        Ok(gl_fns) => {
          S_GL_4_6 = Some(gl_fns);
        }
        Err(_err) => {
          log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot load one or more OpenGL API \
          functions! Error => {_err}");
          return Err(renderer::EnumRendererError::from(EnumOpenGLError::ApiFunctionLoadingError));
        }
      }
    }
    let extensions = GlContext::load_extensions()?;
    let mut hash_map = HashMap::with_capacity(extensions.len());
    for ext in extensions.into_iter() {
      hash_map.insert(ext, ());
    }
    
    return Ok(GlContext {
      m_ext: hash_map,
      m_state: EnumRendererState::Created,
      m_batch: GlBatchPrimitives::new(),
      m_debug_callback: Some(gl_error_callback),
      m_wireframe_enabled: false,
    });
  }
  
  fn get_api_handle(&mut self) -> &mut dyn Any {
    return self;
  }
  
  fn get_api_version(&self) -> f32 {
    let version: Vec<&str> = unsafe {
      std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
        .to_str().unwrap_or("Cannot retrieve api version information!")
        .split(" ")
        .collect()
    };
    
    let api_version: &str = version.first().unwrap_or(&"Unknown api version!");
    let api_major_minor_only = api_version.get(0..3).unwrap().to_string();
    let to_float: f32 = api_major_minor_only.parse::<f32>().unwrap_or(-1.0);
    return to_float;
  }
  
  fn get_max_shader_version_available(&self) -> u16 {
    let shading_info: Vec<&str> = unsafe {
      std::ffi::CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8)
        .to_str().unwrap_or("Cannot retrieve shading info information!")
        .split(" ")
        .collect()
    };
    let shading_language_version_str = shading_info.first().unwrap();
    return (shading_language_version_str.parse::<f32>().unwrap_or(0.0) * 100.0) as u16;
  }
  
  fn check_extension(&self, desired_extension: &str) -> bool {
    let str = String::from(desired_extension);
    return self.m_ext.contains_key(&str);
  }
  
  fn on_event(&mut self, event: &EnumEvent) -> Result<bool, EnumRendererError> {
    return match event {
      EnumEvent::FramebufferEvent(width, height) => {
        check_gl_call!("GlContext", gl::Viewport(0, 0, *width as GLsizei, *height as GLsizei));
        Ok(true)
      }
      _ => Ok(false)
    };
  }
  
  fn on_render(&mut self) -> Result<(), EnumRendererError> {
    if self.m_state == EnumRendererState::Submitted {
      check_gl_call!("GlContext", gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));
      
      // If we are rendering the same material type, don't make unnecessary bindings.
      // Also keep track of the vbo and ibo offsets relevant.
      let mut previous_shader_id: i32 = -1;
      let mut base_vertex_counter: usize = 0;
      let mut ibo_offset_counter: usize = 0;
      
      
      for primitive in self.m_batch.m_primitives.iter() {
        if primitive.m_linked_shader.m_id != previous_shader_id as u32 {
          check_gl_call!("GlContext", gl::UseProgram(primitive.m_linked_shader.m_id));
          
          self.m_batch.m_vao_buffers[primitive.m_vao_index].bind()?;
          self.m_batch.m_ibo_buffers[primitive.m_ibo_index].bind()?;
          
          previous_shader_id = primitive.m_linked_shader.m_id as i32;
          base_vertex_counter = 0;
        }
        
        if primitive.m_visible && primitive.m_ibo_count == 0 {
          check_gl_call!("GlContext", gl::DrawArrays(gl::TRIANGLES, base_vertex_counter as i32, primitive.m_vbo_count as GLsizei));
        } else if primitive.m_visible {
          check_gl_call!("GlContext", gl::DrawElementsBaseVertex(gl::TRIANGLES, primitive.m_ibo_count as GLsizei,
              gl::UNSIGNED_INT, ibo_offset_counter as *const _, base_vertex_counter as i32));
        }
        
        ibo_offset_counter += primitive.m_ibo_length;
        base_vertex_counter += primitive.m_vbo_count;
      }
    }
    return Ok(());
  }
  
  fn apply(&mut self, window: &mut Window, features: &HashSet<EnumRendererOption>) -> Result<(), EnumRendererError> {
    // Enable or disable features AFTER context creation since we need a context to load our openGL
    // functions.
    for feature in features {
      self.toggle(*feature)?;
    }
    
    let window_framebuffer_size = window.get_framebuffer_size();
    check_gl_call!("GlContext", gl::Viewport(0, 0, window_framebuffer_size.0 as i32, window_framebuffer_size.1 as i32));
    check_gl_call!("GlContext", gl::ClearColor(0.05, 0.05, 0.05, 1.0));
    check_gl_call!("GLContext", gl::FrontFace(gl::CW));
    
    self.m_state = EnumRendererState::Submitted;
    return Ok(());
  }
  
  fn toggle_visibility_of(&mut self, entity_uuid: u64, sub_primitive_offset: Option<usize>, visible: bool) {
    let mut primitive_found = self.m_batch.m_primitives.iter_mut()
      .filter(|primitive| primitive.m_uuid == entity_uuid)
      .collect::<Vec<&mut GlPrimitiveInfo>>();
    
    if !primitive_found.is_empty() {
      
      // If we want all sub_primitives within the primitive to be hidden as well.
      if sub_primitive_offset.is_none() {
        for sub_primitive in primitive_found {
          sub_primitive.m_visible = visible;
        }
        return;
      }
      
      // If main primitive is matched.
      if sub_primitive_offset.unwrap() == 0 {
        primitive_found.get_mut(0).unwrap().m_visible = visible;
        return;
      }
      
      // If a sub primitive is matched instead.
      if let Some(sub_entity) = primitive_found.get_mut(sub_primitive_offset.unwrap()) {
        sub_entity.m_visible = visible;
      }
    }
  }
  
  fn get_max_msaa_count(&self) -> Result<u8, EnumRendererError> {
    // let framebuffer_color_sample_count: u8 = self.m_framebuffer.max_color_sample_count;
    // let framebuffer_depth_sample_count: u8 = self.m_framebuffer.max_depth_sample_count;
    //
    // return framebuffer_color_sample_count.min(framebuffer_depth_sample_count);
    let window = Engine::get_active_window();
    return Ok(window.m_samples.unwrap_or(1) as u8);
  }
  
  fn to_string(&self) -> String {
    unsafe {
      let api_vendor: &str = std::ffi::CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8)
        .to_str().unwrap_or("Cannot retrieve api vendor information!");
      let version: Vec<&str> = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
        .to_str().unwrap_or("Cannot retrieve api version information!")
        .split(" ")
        .collect();
      
      let api_version: &str = version.first().unwrap_or(&"Unknown api version!");
      let driver_version: &str = version.last().unwrap_or(&"Unknown driver version!");
      
      let device_name = std::ffi::CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8)
        .to_str().unwrap_or("Cannot retrieve renderer information!");
      let shading_info = std::ffi::CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8)
        .to_str().unwrap_or("Cannot retrieve shading language information!");
      
      let str: String = format!("Api =>\t\t\t OpenGL;\n\
      Api version =>\t\t {0};\n\
      Vendor =>\t\t {1};\n\
      Device name =>\t\t {2};\n\
      Driver version =>\t {3};\n\
      Shading language =>\t {4};",
        api_version, api_vendor, device_name, driver_version, shading_info);
      return str;
    }
  }
  
  fn toggle(&mut self, option: EnumRendererOption) -> Result<(), EnumRendererError> {
    match option {
      EnumRendererOption::ApiCallChecking(debug_type) => {
        match debug_type {
          EnumCallCheckingType::None => unsafe {
            gl::Disable(gl::DEBUG_OUTPUT);
            gl::Disable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
          }
          EnumCallCheckingType::Sync => unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
          },
          EnumCallCheckingType::Async => unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            // Disable sync messages.
            gl::Disable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            if gl::DebugMessageCallback::is_loaded() {
              gl::DebugMessageCallback(self.m_debug_callback, std::ptr::null());
              gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, std::ptr::null(), gl::TRUE);
            }
          }
          EnumCallCheckingType::SyncAndAsync => unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(self.m_debug_callback, std::ptr::null());
            gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, std::ptr::null(), gl::TRUE);
          }
        }
        log!(EnumLogColor::Blue, "INFO", "[GlContext] -->\t Debug mode {0}",
          (debug_type != EnumCallCheckingType::None).then(|| return "enabled").unwrap_or("disabled"));
      }
      EnumRendererOption::DepthTest(enabled) => {
        if enabled {
          check_gl_call!("GlContext", gl::Enable(gl::DEPTH_TEST));
        } else {
          check_gl_call!("GlContext", gl::Disable(gl::DEPTH_TEST));
        }
        log!(EnumLogColor::Blue, "INFO", "[GlContext] -->\t Depth test {0}",
          enabled.then(|| return "enabled").unwrap_or("disabled"));
      }
      EnumRendererOption::MSAA(sample_count) => {
        #[allow(unused)]
          let mut max_sample_count: u8 = 1;
        if sample_count.is_some() {
          max_sample_count = self.get_max_msaa_count()?;
          if max_sample_count < 2 {
            log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot enable MSAA!");
            return Err(renderer::EnumRendererError::from(EnumOpenGLError::MSAAError));
          } else if sample_count.unwrap() > max_sample_count {
            log!(EnumLogColor::Yellow, "WARN", "[GlContext] -->\t Cannot enable MSAA with X{0}! \
              Defaulting to {1}...", sample_count.unwrap(), max_sample_count);
          }
          check_gl_call!("GlContext", gl::Enable(gl::MULTISAMPLE));
        } else {
          check_gl_call!("GlContext", gl::Disable(gl::MULTISAMPLE));
        }
        log!(EnumLogColor::Blue, "INFO", "[GlContext] -->\t MSAA {0}",
          sample_count.is_some().then(|| return format!("enabled (X{0})", max_sample_count))
          .unwrap_or("disabled".to_string()));
      }
      EnumRendererOption::Blending(enabled, opt_factors) => {
        if enabled {
          check_gl_call!("GlContext", gl::Enable(gl::BLEND));
        } else {
          check_gl_call!("GlContext", gl::Disable(gl::BLEND));
        }
        
        if opt_factors.is_some() {
          check_gl_call!("GlContext", gl::BlendFunc(opt_factors.unwrap().0 as GLenum, opt_factors.unwrap().1 as GLenum));
        }
        
        log!(EnumLogColor::Blue, "INFO", "[GlContext] -->\t Blending {0}", enabled
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
      EnumRendererOption::Wireframe(enabled) => {
        self.m_wireframe_enabled = enabled;
        self.toggle_wireframe_ubo(enabled)?;
        log!(EnumLogColor::Blue, "INFO", "[GlContext] -->\t Wireframe mode {0}", enabled
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
      EnumRendererOption::SRGB(enabled) => {
        if enabled {
          check_gl_call!("GlContext", gl::Enable(gl::FRAMEBUFFER_SRGB));
        } else {
          check_gl_call!("GlContext", gl::Disable(gl::FRAMEBUFFER_SRGB));
        }
        log!(EnumLogColor::Blue, "INFO", "[GlContext] -->\t SRGB framebuffer {0}", enabled
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
      EnumRendererOption::CullFacing(face) => {
        if face.is_some() {
          check_gl_call!("GlContext", gl::Enable(gl::CULL_FACE));
          check_gl_call!("GlContext", gl::CullFace(face.unwrap() as GLenum));
        } else {
          check_gl_call!("GlContext", gl::Disable(gl::CULL_FACE));
        }
        log!(EnumLogColor::Blue, "INFO", "[GlContext] -->\t Cull facing {0}", face.is_some()
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
    }
    return Ok(());
  }
  
  fn flush(&mut self) -> Result<(), EnumRendererError> {
    self.on_render()?;
    
    self.m_batch.m_primitives.clear();
    self.m_batch.m_vao_buffers.clear();
    self.m_batch.m_vbo_buffers.clear();
    self.m_batch.m_ubo_buffers.clear();
    return Ok(());
  }
  
  fn enqueue(&mut self, r_asset: &REntity, shader_associated: &mut Shader) -> Result<(), EnumRendererError> {
    if r_asset.is_empty() {
      log!(EnumLogColor::Yellow, "WARN", "[GlContext] --> Entity [{0}] has no \
      vertices! Not sending it...", r_asset)
    }
    
    // Figure out if the entity type has already been enqueued. If so, only append to it in the vbo instead of creating another vao.
    let primitive_matched_with_shader_found = self.m_batch.m_primitives.iter()
      // Get the last one to properly offset the new primitive from the previous ones.
      .rposition(|primitive| primitive.m_linked_shader.m_id == shader_associated.get_id());
    
    // Check if this is the first primitive of this type. If so, alloc new buffers for it.
    if primitive_matched_with_shader_found.is_none() {
      self.alloc_buffers(r_asset, shader_associated)?;
    }
    
    let mut vao_index = self.m_batch.m_vao_buffers.len() - 1;
    let mut vbo_index = self.m_batch.m_vbo_buffers.len() - 1;
    let mut ibo_index = self.m_batch.m_ibo_buffers.len() - 1;
    
    if let Some(primitive) = self.m_batch.m_primitives.get(primitive_matched_with_shader_found.unwrap_or(0)) {
      vao_index = primitive.m_vao_index;
      vbo_index = primitive.m_vbo_index;
      ibo_index = primitive.m_ibo_index;
    }
    
    let total_vertex_count = r_asset.get_total_vertex_count();
    let total_index_count = r_asset.get_total_index_count();
    let size_per_vertex = r_asset.get_size();
    
    let mut new_primitive: GlPrimitiveInfo = GlPrimitiveInfo {
      m_uuid: r_asset.get_uuid(),
      m_linked_shader: GlShaderInfo {
        m_version: shader_associated.get_version(),
        m_id: shader_associated.get_id(),
      },
      m_vao_index: vao_index,
      m_vbo_index: vbo_index,
      m_ibo_index: ibo_index,
      m_vbo_length: size_per_vertex * total_vertex_count,
      m_ibo_length: size_of::<u32>() * total_index_count,
      m_vbo_count: total_vertex_count,
      m_ibo_count: total_index_count,
      m_size_per_vertex: size_per_vertex,
      m_visible: false,
    };
    
    self.push_buffers(&new_primitive, r_asset)?;
    // self.join_primitives(&mut new_primitive);
    self.m_batch.m_primitives.push(new_primitive);
    
    // If we already have a perspective camera ubo bound, skip.
    if !self.m_batch.m_ubo_buffers.iter().any(|ubo| ubo.get_name() == Some("ubo_camera")) {
      let mut camera_ubo = GlUbo::reserve(Some("ubo_camera"), EnumUboTypeSize::ViewProjection, 0)?;
      
      // If glsl version is lower than 420, then we cannot bind blocks in shaders and have to encode them here instead.
      if shader_associated.get_version() < 420 && shader_associated.get_lang() == EnumShaderLanguage::Glsl {
        camera_ubo.bind_block(shader_associated.get_id(), 0)?;
      }
      self.m_batch.m_ubo_buffers.push(camera_ubo);
    }
    
    // let mut result = 0;
    // check_gl_call!("GLContext", gl::GetVertexAttribiv(0, gl::VERTEX_ATTRIB_ARRAY_BUFFER_BINDING, &mut result));
    //
    // log!(EnumLogColor::Blue, "DEBUG", "[GlContext] -->\t Current Vbo bound to Vao at index 0 : {0}", result);
    #[cfg(feature = "debug")]
    {
      log!(EnumLogColor::Yellow, "INFO", "[GlContext] -->\t Enqueued {0} bytes in vbo {1} and {2} bytes in ibo {3}\
    \n{6:115}Total vbo size: {4}\n{6:115}Total ibo size: {5}", total_vertex_count,
        self.m_batch.m_vbo_buffers.get(vbo_index).unwrap().m_buffer_id, total_index_count,
        self.m_batch.m_ibo_buffers.get(ibo_index).unwrap().m_buffer_id,
        self.m_batch.m_vbo_buffers.get(vbo_index).unwrap().m_length,
        self.m_batch.m_ibo_buffers.get(ibo_index).unwrap().m_length, "");
    }
    return Ok(());
  }
  
  fn dequeue(&mut self, uuid: u64) -> Result<(), EnumRendererError> {
    if self.m_state != EnumRendererState::Submitted {
      return Ok(());
    }
    for index in 0..self.m_batch.m_primitives.len() {
      if self.m_batch.m_primitives[index].m_uuid == uuid {
        self.m_batch.m_vao_buffers[self.m_batch.m_primitives[index].m_vao_index].bind()?;
        
        if !self.m_batch.m_vbo_buffers.is_empty() && !self.m_batch.m_vbo_buffers[self.m_batch.m_primitives[index].m_vbo_index].is_empty() {
          // Free up space without reallocating buffer to save time and allow quick re enqueuing of the same entity.
          self.m_batch.m_vbo_buffers[self.m_batch.m_primitives[index].m_vbo_index]
            .strip(self.m_batch.m_primitives[index].m_vbo_length, self.m_batch.m_primitives[index].m_size_per_vertex *
              self.m_batch.m_primitives[index].m_vbo_count)?;
        }
        return Ok(());
      }
    }
    return Err(renderer::EnumRendererError::from(EnumOpenGLError::EntityUUIDNotFound));
  }
  
  fn update_ubo_camera(&mut self, view: Mat4, projection: Mat4) -> Result<(), EnumRendererError> {
    let ubo_camera_index_found = self.m_batch.m_ubo_buffers.iter_mut()
      .position(|ubo| ubo.get_name() == Some("ubo_camera"));
    
    if ubo_camera_index_found.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot update camera ubo, 'ubo_camera' not found in batch!");
      return Err(EnumRendererError::UboNotFound);
    }
    
    self.m_batch.m_ubo_buffers[ubo_camera_index_found.unwrap()].push(EnumUboType::ViewProjection(view, projection))?;
    return Ok(());
  }
  
  fn update_ubo_model(&mut self, model_transform: Mat4, instance_offset: usize) -> Result<(), EnumRendererError> {
    let ubo_model_index_found = self.m_batch.m_ubo_buffers.iter_mut()
      .position(|ubo| ubo.get_name() == Some("ubo_model"));
    
    if ubo_model_index_found.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot update transform ubo, ubo not found in batch!");
      return Err(EnumRendererError::UboNotFound);
    }
    
    self.m_batch.m_ubo_buffers[ubo_model_index_found.unwrap()].push(EnumUboType::Transform(model_transform,
      instance_offset))?;
    return Ok(());
  }
  
  fn free(&mut self) -> Result<(), EnumRendererError> {
    if self.m_state == EnumRendererState::NotCreated {
      log!(EnumLogColor::Yellow, "WARN", "[GlContext] -->\t Cannot free resources : OpenGL renderer \
      has not been created!");
      return Err(renderer::EnumRendererError::from(EnumOpenGLError::InvalidContext));
    }
    
    if self.m_state == EnumRendererState::Deleted {
      log!(EnumLogColor::Yellow, "WARN", "[GlContext] -->\t Cannot free resources : OpenGL renderer \
      has already been deleted!");
      return Err(renderer::EnumRendererError::from(EnumOpenGLError::InvalidContext));
    }
    
    log!(EnumLogColor::Purple, "INFO", "[GlContext] -->\t Freeing buffers...");
    // Free ubos.
    for ubo in self.m_batch.m_ubo_buffers.iter_mut() {
      ubo.free()?;
    };
    
    // Free vaos.
    for vao in self.m_batch.m_vao_buffers.iter_mut() {
      vao.free()?;
    };
    
    // Free vbos.
    for vbo in self.m_batch.m_vbo_buffers.iter_mut() {
      vbo.free()?;
    };
    log!(EnumLogColor::Green, "INFO", "[GlContext] -->\t Freed buffers successfully");
    
    self.m_state = EnumRendererState::Deleted;
    return Ok(());
  }
}

impl GlContext {
  fn load_extensions() -> Result<Vec<String>, EnumOpenGLError> {
    let mut ext_count = 0;
    unsafe { gl::GetIntegerv(gl::NUM_EXTENSIONS, &mut ext_count) };
    let mut gl_extensions_available: Vec<String> = Vec::with_capacity(ext_count as usize);
    
    for index in 0..ext_count {
      let gl_ext = unsafe { gl::GetStringi(gl::EXTENSIONS, index as GLuint) };
      match unsafe { std::ffi::CStr::from_ptr(gl_ext.cast()).to_str() } {
        Ok(gl_ext_name) => {
          gl_extensions_available.push(String::from(gl_ext_name));
        }
        Err(_err) => {
          log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot convert OpenGL extension name \
          pointer to Rust str! Error => {_err:?}");
          return Err(EnumOpenGLError::CStringError);
        }
      }
    }
    return Ok(gl_extensions_available);
  }
  
  fn toggle_wireframe_ubo(&mut self, value: bool) -> Result<(), EnumRendererError> {
    // Find ubo.
    let result = self.m_batch.m_ubo_buffers.iter_mut().find(|ubo| ubo.get_name() == Some("ubo_wireframe"));
    let c_void = value
      .then(|| &value as *const _ as *const std::ffi::c_void)
      .unwrap_or(std::ptr::null());
    
    if result.is_none() {
      // Setup wireframe ubo.
      let mut wireframe_ubo = GlUbo::reserve(Some("ubo_wireframe"), EnumUboTypeSize::Bool, 3)?;
      for matched in self.m_batch.m_primitives.iter_mut()
        .filter(|matched| matched.m_linked_shader.m_version < 420) {
        wireframe_ubo.bind_block(matched.m_linked_shader.m_id, 3)?
      }
      
      wireframe_ubo.push(EnumUboType::Custom(c_void))?;
      return Ok(());
    }
    
    result.unwrap().push(EnumUboType::Custom(c_void))?;
    return Ok(());
  }
  
  fn alloc_buffers(&mut self, sendable_entity: &REntity, shader: &mut Shader) -> Result<(), EnumOpenGLError> {
    let mut new_vao = GlVao::new()?;
    let mut new_vbo = GlVbo::new();
    let mut new_ibo = GlIbo::new();
    
    if sendable_entity.get_total_index_count() > 0 {
      new_ibo.reserve(size_of::<u32>() * sendable_entity.get_total_index_count())?;
    }
    new_vbo.reserve(sendable_entity.get_size() * sendable_entity.get_total_vertex_count())?;
    Self::set_attributes(sendable_entity, &mut new_vao)?;
    
    let mut model_ubo = GlUbo::reserve(Some("ubo_model"), EnumUboTypeSize::Transform(255), 1)?;
    // If glsl version is lower than 420, then we cannot bind blocks in shaders and have to encode them here instead.
    if shader.get_version() < 420 && shader.get_lang() == EnumShaderLanguage::Glsl {
      model_ubo.bind_block(shader.get_id(), 1)?;
    }
    
    self.m_batch.m_vao_buffers.push(new_vao);
    self.m_batch.m_vbo_buffers.push(new_vbo);
    self.m_batch.m_ibo_buffers.push(new_ibo);
    self.m_batch.m_ubo_buffers.push(model_ubo);
    return Ok(());
  }
  
  fn push_buffers(&mut self, new_primitive: &GlPrimitiveInfo, sendable_entity: &REntity) -> Result<(), EnumOpenGLError> {
    // Figure out if the entity type has already been enqueued. If so, only append to it in the vbo instead of creating another vao.
    let primitive_matched_with_shader_found = self.m_batch.m_primitives.iter()
      // Get the last one to properly offset the new primitive from the previous ones.
      .rposition(|primitive| primitive.m_linked_shader.m_id == new_primitive.m_linked_shader.m_id);
    
    let vbo = self.m_batch.m_vbo_buffers.get_mut(new_primitive.m_vbo_index).unwrap();
    let ibo = self.m_batch.m_ibo_buffers.get_mut(new_primitive.m_ibo_index).unwrap();
    
    if primitive_matched_with_shader_found.is_some() {
      vbo.expand(sendable_entity.get_size() * sendable_entity.get_total_vertex_count())?;
      if sendable_entity.get_total_index_count() > 0 {
        ibo.expand(size_of::<u32>() * sendable_entity.get_total_index_count())?;
      }
    }
    
    self.push_data(&new_primitive, primitive_matched_with_shader_found.is_some(), sendable_entity)?;
    
    // If we had to reallocate our vbo to append more data to it, thus migrating over to a new buffer
    // and as a result, leaving our old vbo id that linked to the vao attrib array binding behind.
    // It is important to 'rebind' the vao's attrib buffer binding by re-enabling vertex attributes.
    // God OpenGL is so obscure sometimes...
    if self.m_batch.m_vbo_buffers.get_mut(new_primitive.m_vbo_index).unwrap().has_migrated() {
      Self::set_attributes(sendable_entity, self.m_batch.m_vao_buffers.get_mut(new_primitive.m_vao_index).unwrap())?;
    }
    
    return Ok(());
  }
  
  fn push_data(&mut self, new_primitive: &GlPrimitiveInfo, primitive_match_found: bool, sendable_entity: &REntity) -> Result<(), EnumOpenGLError> {
    let model_transform = sendable_entity.get_matrix();
    let mut base_index: usize = 0;
    
    // If we have pushed other primitives previously.
    if !primitive_match_found {
      for primitive in self.m_batch.m_primitives.iter() {
        base_index += primitive.m_vbo_count;
      }
    }
    
    let vbo: &mut GlVbo = self.m_batch.m_vbo_buffers.get_mut(new_primitive.m_vbo_index).unwrap();
    let ibo: &mut GlIbo = self.m_batch.m_ibo_buffers.get_mut(new_primitive.m_ibo_index).unwrap();
    let ubo: &mut GlUbo = self.m_batch.m_ubo_buffers.iter_mut().find(|ubo| ubo.get_name() == Some("ubo_model"))
      .unwrap();
    
    for (position, sub_mesh) in sendable_entity.m_sub_meshes.iter().enumerate() {
      log!("INFO", "[GlContext] -->\t Enqueuing {0}: '{1}'...", (sendable_entity.m_sub_meshes.len() == 1)
        .then(|| "primitive".to_string())
        .unwrap_or(format!("sub primitive {0}", position)), sub_mesh.get_name());
      log!("INFO", "[GlContext] -->\t Info:\n{0:115}{1}", "", sub_mesh);
      
      let indices_offset = sub_mesh.get_indices().iter()
        .map(|index| *index + base_index as u32)
        .collect::<Vec<u32>>();
      
      vbo.push(sub_mesh.get_vertices())?;
      if indices_offset.len() > 0 {
        ibo.push(&indices_offset)?;
      }
      
      ubo.push(EnumUboType::Transform(model_transform, sub_mesh.get_entity_id() as usize))?;
    }
    return Ok(());
  }
  
  fn join_primitives(&mut self, new_primitive: &mut GlPrimitiveInfo) {
    let mut ibo_count: usize = new_primitive.m_ibo_count;
    
    // Find any primitive that shares the same shader, thus the same material type.
    // Keep adding its ibo count to the base primitive to increase the draw count (glDrawElementBaseVertex).
    let matched_primitives = self.m_batch.m_primitives.iter()
      .filter(|p| new_primitive.m_linked_shader.m_id == p.m_linked_shader.m_id)
      .enumerate()
      .map(|(position, p)| {
        ibo_count += p.m_ibo_count;
        return position;
      })
      .collect::<Vec<usize>>();
    
    new_primitive.m_ibo_count = ibo_count;
    
    for index in matched_primitives.into_iter() {
      // Remove all primitives that come after the base one, since we don't want to iterate through them.
      // We already have all the info we need appended the in the buffers, this is to minimize on draw calls during for loop.
      self.m_batch.m_primitives.remove(index);
    }
  }
  
  fn set_attributes(entity: &REntity, vao: &mut GlVao) -> Result<(), EnumOpenGLError> {
    // Establish vao attributes.
    let mut attributes: Vec<GlVertexAttribute> = Vec::with_capacity(5);
    
    match entity.m_type {
      EnumPrimitiveType::Mesh(is_flat_shaded) => {
        // IDs.
        attributes.push(GlVertexAttribute::new(EnumAttributeType::UnsignedInt(1), false,
          0, 0)?);
        
        // Positions.
        attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec3, false,
          EnumVertexMemberOffset::AtPos as usize, 0)?);
        
        // Normals.
        if is_flat_shaded {
          attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec3, false,
            EnumVertexMemberOffset::AtNormal as usize, 1)?);
        } else {
          attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec3, false,
            EnumVertexMemberOffset::AtNormal as usize, 0)?);
        }
        
        // Colors.
        attributes.push(GlVertexAttribute::new(EnumAttributeType::UnsignedInt(1), false,
          EnumVertexMemberOffset::AtColor as usize, 0)?);
        
        // Texture coordinates.
        attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec2, false,
          EnumVertexMemberOffset::AtTexCoords as usize, 0)?);
      }
      _ => todo!()
    };
    
    // Enable all added attributes.
    return vao.enable_attributes(entity.get_size(), attributes);
  }
}

extern "system" fn gl_error_callback(error_code: GLenum, e_type: GLenum, _id: GLuint,
                                     severity: GLenum, _length: GLsizei, error_message: *const GLchar,
                                     _user_param: *mut std::ffi::c_void) {
  let mut final_error_msg: String = "".to_string();
  if error_code != gl::NO_ERROR {
    final_error_msg += format!("\nCode =>\t\t 0x{0:X};", error_code).as_str();
    match e_type {
      gl::DEBUG_TYPE_ERROR => {
        final_error_msg += "\nType =>\t\t Error;"
      }
      gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => {
        final_error_msg += "\nType =>\t\t Deprecated behavior;"
      }
      gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => {
        final_error_msg += "\nType =>\t\t Undefined behavior;"
      }
      gl::DEBUG_TYPE_PORTABILITY => {
        final_error_msg += "\nType =>\t\t Portability;"
      }
      gl::DEBUG_TYPE_PERFORMANCE => return,
      gl::DEBUG_TYPE_MARKER => {
        final_error_msg += "\nType =>\t\t Marker;"
      }
      gl::DEBUG_TYPE_PUSH_GROUP => {
        final_error_msg += "\nType =>\t\t Push group;"
      }
      gl::DEBUG_TYPE_POP_GROUP => {
        final_error_msg += "\nType =>\t\t Pop group;"
      }
      // Ignore other types.
      gl::DEBUG_TYPE_OTHER => { return; }
      _ => {
        final_error_msg += "\nType =>\t\t Error;"
      }
    }
    
    match severity {
      gl::DEBUG_SEVERITY_HIGH => {
        final_error_msg += "\nSeverity =>\t High (Fatal);"
      }
      gl::DEBUG_SEVERITY_MEDIUM => {
        final_error_msg += "\nSeverity =>\t Medium;"
      }
      gl::DEBUG_SEVERITY_LOW => {
        final_error_msg += "\nSeverity =>\t Low;"
      }
      gl::DEBUG_SEVERITY_NOTIFICATION => {
        final_error_msg += "\nSeverity =>\t Info;"
      }
      _ => {
        final_error_msg += "\nSeverity =>\t Unknown;"
      }
    }
    
    let test = unsafe { std::ffi::CStr::from_ptr(error_message.cast_mut()) };
    let str = test.to_str()
      .expect("[GlContext] -->\t Failed to convert C string to Rust String in gl_error_callback()");
    
    final_error_msg += format!("\nMessage =>\t {0}\n", str).as_str();
    
    match severity {
      gl::DEBUG_SEVERITY_HIGH => { log!(EnumLogColor::Red, "ERROR", "[Driver] -->\t OpenGL Driver Notification :{0}", final_error_msg); }
      gl::DEBUG_SEVERITY_MEDIUM => { log!(EnumLogColor::Yellow, "WARN", "[Driver] -->\t OpenGL Driver Notification :{0}", final_error_msg); }
      gl::DEBUG_SEVERITY_LOW => { log!(EnumLogColor::Yellow, "WARN", "[Driver] -->\t OpenGL Driver Notification :{0}", final_error_msg); }
      gl::DEBUG_SEVERITY_NOTIFICATION => { log!("INFO", "[Driver] -->\t OpenGL Driver Notification :{0}", final_error_msg); }
      _ => {
        log!(EnumLogColor::Red, "ERROR", "[Driver] -->\t OpenGL Driver Notification :{0}", final_error_msg);
      }
    }
    if severity == gl::DEBUG_SEVERITY_HIGH {
      log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Fatal OpenGL driver error encountered! Exiting...");
    }
  }
}