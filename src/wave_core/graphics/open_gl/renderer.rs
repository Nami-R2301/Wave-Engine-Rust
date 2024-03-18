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

extern crate gl46;

use std::any::Any;
use std::collections::{HashMap, HashSet};

use gl46::GlFns;

use crate::{log};
use crate::wave_core::assets::renderable_assets::{EnumPrimitiveType, EnumVertexMemberOffset, REntity};
use crate::wave_core::camera::Camera;
use crate::wave_core::events::EnumEvent;
use crate::wave_core::graphics::{open_gl, renderer};
use crate::wave_core::graphics::open_gl::buffer::{EnumAttributeType, EnumUboType, EnumUboTypeSize, GLchar, GLenum, GLsizei, GlUbo, GLuint, GlVao, GlVbo, GlVertexAttribute, GLvoid};
use crate::wave_core::graphics::renderer::{EnumCallCheckingType, EnumRendererOption, EnumState, TraitContext};
use crate::wave_core::graphics::shader::{EnumShaderLanguageType, Shader};
use crate::wave_core::math::{Mat4};
use crate::wave_core::window::Window;
use crate::wave_core::{Engine, S_ENGINE};

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
        if engine.m_renderer.m_options.contains(&crate::wave_core::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::wave_core::graphics::renderer::EnumCallCheckingType::None)) ||
          engine.m_renderer.m_options.contains(&crate::wave_core::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::wave_core::graphics::renderer::EnumCallCheckingType::Async))
         {
          unsafe { $gl_function };
        } else if engine.m_renderer.m_options.contains(&crate::wave_core::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::wave_core::graphics::renderer::EnumCallCheckingType::Sync)) ||
        engine.m_renderer.m_options.contains(&crate::wave_core::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::wave_core::graphics::renderer::EnumCallCheckingType::SyncAndAsync)) {
          unsafe { $gl_function };
          let error = unsafe { gl::GetError() };
          if error != gl::NO_ERROR {
            log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t OpenGL call error :\nCall =>\t\t {1:?}\n\
            Code =>\t\t 0x{2:x}\nWhere =>\t {3}", $name, stringify!($gl_function), error, trace!());
            return Err(crate::wave_core::graphics::open_gl::renderer::EnumError::InvalidOperation(error).into());
          }
        } else {
          unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
          unsafe { $gl_function };
          let error = unsafe { gl::GetError() };
          if error != gl::NO_ERROR {
            log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t OpenGL call error :\nCall =>\t\t {1:?}\n\
            Code =>\t\t 0x{2:x}\nWhere =>\t {3}", $name, stringify!($gl_function), error, trace!());
            return Err(crate::wave_core::graphics::open_gl::renderer::EnumError::InvalidOperation(error).into());
          }
        }
    };
    ($name:literal, let $var:ident: $var_type:ty = $gl_function:expr) => {
      let $var:$var_type;
      let engine = unsafe { &mut *S_ENGINE.expect("Cannot log, no active Engine!") };
        if engine.m_renderer.m_options.contains(&crate::wave_core::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::wave_core::graphics::renderer::EnumCallCheckingType::None)) ||
          engine.m_renderer.m_options.contains(&crate::wave_core::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::wave_core::graphics::renderer::EnumCallCheckingType::Async))
         {
          $var = unsafe { $gl_function };
        } else if engine.m_renderer.m_options.contains(&crate::wave_core::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::wave_core::graphics::renderer::EnumCallCheckingType::Sync)) ||
        engine.m_renderer.m_options.contains(&crate::wave_core::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::wave_core::graphics::renderer::EnumCallCheckingType::SyncAndAsync)) {
          $var = unsafe { $gl_function };
          let error = unsafe { gl::GetError() };
          if error != gl::NO_ERROR {
            log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t OpenGL call error :\nCall =>\t\t {1:?}\n\
            Code =>\t\t 0x{2:x}\nWhere =>\t {3}", $name, stringify!($gl_function), error, trace!());
            return Err(crate::wave_core::graphics::open_gl::renderer::EnumError::InvalidOperation(error).into());
          }
        } else {
        unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
        $var = unsafe { $gl_function };
        let error = unsafe { gl::GetError() };
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t OpenGL call error :\nCall =>\t\t {1:?}\n\
            Code =>\t\t 0x{2:x}\nWhere =>\t {3}", $name, stringify!($gl_function), error, trace!());
          return Err(crate::wave_core::graphics::open_gl::renderer::EnumError::InvalidOperation(error).into());
        }
      }
    };
    ($name:literal, $var:ident = $gl_function:expr) => {
      $var = Default::default();
     let engine = unsafe { &mut *S_ENGINE.expect("Cannot log, no active Engine!") };
        if engine.m_renderer.m_options.contains(&crate::wave_core::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::wave_core::graphics::renderer::EnumCallCheckingType::None)) ||
          engine.m_renderer.m_options.contains(&crate::wave_core::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::wave_core::graphics::renderer::EnumCallCheckingType::Async))
         {
          $var = unsafe { $gl_function };
        } else if engine.m_renderer.m_options.contains(&crate::wave_core::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::wave_core::graphics::renderer::EnumCallCheckingType::Sync)) ||
        engine.m_renderer.m_options.contains(&crate::wave_core::graphics::renderer::EnumRendererOption::ApiCallChecking(crate::wave_core::graphics::renderer::EnumCallCheckingType::SyncAndAsync)) {
          $var = unsafe { $gl_function };
          let error = unsafe { gl::GetError() };
          if error != gl::NO_ERROR {
            log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t OpenGL call error :\nCall =>\t\t {1:?}\n\
            Code =>\t\t 0x{2:x}\nWhere =>\t {3}", $name, stringify!($gl_function), error, trace!());
            return Err(crate::wave_core::graphics::open_gl::renderer::EnumError::InvalidOperation(error).into());
          }
        } else {
        unsafe { while gl::GetError() != gl::NO_ERROR {} };
        $var = unsafe { $gl_function };
        let error = unsafe { gl::GetError() };
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t OpenGL call error :\nCall =>\t\t {1:?}\n\
            Code =>\t\t 0x{2:x}\nWhere =>\t {3}", $name, stringify!($gl_function), error, trace!());
          return Err(crate::wave_core::graphics::open_gl::renderer::EnumError::InvalidOperation(error).into());
        }
      }
    };
}

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
pub enum EnumError {
  CStringError,
  ApiFunctionLoadingError,
  UnsupportedApiFunction,
  InvalidContext,
  InvalidOperation(GLenum),
  MSAAError,
  EntityUUIDNotFound,
  InvalidEntityType,
  InvalidBufferOperation(open_gl::buffer::EnumError),
  InvalidShaderOperation(open_gl::shader::EnumError),
}

impl From<open_gl::buffer::EnumError> for EnumError {
  fn from(value: open_gl::buffer::EnumError) -> Self {
    return EnumError::InvalidBufferOperation(value);
  }
}

impl From<open_gl::shader::EnumError> for EnumError {
  fn from(value: open_gl::shader::EnumError) -> Self {
    return EnumError::InvalidShaderOperation(value);
  }
}

struct GlPrimitiveInfo {
  m_uuid: u64,
  m_linked_shader: *mut Shader,
  m_vao_index: usize,
  m_vbo_index: usize,
  m_vbo_offset: usize,
  m_vbo_size: usize, // Size per vertex for the primitive in vbo.
  m_vbo_count: usize, // Vertex count for the primitive in vbo.
}

struct GlBatchPrimitives {
  m_primitives: Vec<GlPrimitiveInfo>,
  m_vao_buffers: Vec<GlVao>,
  m_vbo_buffers: Vec<GlVbo>,
  m_ubo_buffers: Vec<GlUbo>,
}

impl GlBatchPrimitives {
  pub fn new() -> Self {
    return GlBatchPrimitives {
      m_primitives: Vec::new(),
      m_vao_buffers: Vec::new(),
      m_vbo_buffers: Vec::new(),
      m_ubo_buffers: Vec::new(),
    };
  }
}


pub struct GlContext {
  pub(crate) m_ext: HashMap<String, ()>,
  pub(crate) m_state: EnumState,
  m_batch: GlBatchPrimitives,
  m_debug_callback: gl::types::GLDEBUGPROC,
}

impl TraitContext for GlContext {
  fn default() -> Self {
    return Self {
      m_state: EnumState::NotCreated,
      m_ext: HashMap::new(),
      m_batch: GlBatchPrimitives::new(),
      m_debug_callback: Some(gl_error_callback),
    };
  }
  
  fn on_new(window: &mut Window) -> Result<Self, renderer::EnumError> {
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
          return Err(renderer::EnumError::from(EnumError::ApiFunctionLoadingError));
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
      m_state: EnumState::Created,
      m_batch: GlBatchPrimitives::new(),
      m_debug_callback: Some(gl_error_callback),
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
        .to_str().unwrap_or("Cannot retrieve api version information!")
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
  
  fn on_event(&mut self, event: &EnumEvent) -> Result<bool, renderer::EnumError> {
    return match event {
      EnumEvent::FramebufferEvent(width, height) => {
        check_gl_call!("GlContext", gl::Viewport(0, 0, *width as GLsizei, *height as GLsizei));
        Ok(true)
      }
      _ => Ok(false)
    };
  }
  
  fn on_render(&mut self) -> Result<(), renderer::EnumError> {
    check_gl_call!("Renderer", gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));
    for primitive in self.m_batch.m_primitives.iter() {
      check_gl_call!("Renderer", gl::UseProgram((*primitive.m_linked_shader).get_id()));
      self.m_batch.m_vao_buffers[primitive.m_vao_index].bind()?;
      // self.m_batch.m_ibo_buffers[primitive.m_ibo_index].bind()?;
      check_gl_call!("Renderer", gl::DrawArrays(gl::TRIANGLES, primitive.m_vbo_offset as i32, primitive.m_vbo_count as GLsizei));
    }
    return Ok(());
  }
  
  fn submit(&mut self, window: &mut Window, features: &HashSet<EnumRendererOption>) -> Result<(), renderer::EnumError> {
    // Enable or disable features AFTER context creation since we need a context to load our openGL
    // functions.
    for feature in features {
      self.toggle(*feature)?;
    }
    
    let window_framebuffer_size = window.get_framebuffer_size();
    check_gl_call!("Renderer", gl::Viewport(0, 0, window_framebuffer_size.0 as i32, window_framebuffer_size.1 as i32));
    check_gl_call!("Renderer", gl::ClearColor(0.05, 0.05, 0.05, 1.0));
    
    check_gl_call!("Renderer", gl::FrontFace(gl::CW));
    return Ok(());
  }
  
  fn get_max_msaa_count(&self) -> Result<u8, renderer::EnumError> {
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
  
  fn toggle(&mut self, option: EnumRendererOption) -> Result<(), renderer::EnumError> {
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
          check_gl_call!("Renderer", gl::Enable(gl::DEPTH_TEST));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::DEPTH_TEST));
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
            return Err(renderer::EnumError::from(EnumError::MSAAError));
          } else if sample_count.unwrap() > max_sample_count {
            log!(EnumLogColor::Yellow, "WARN", "[GlContext] -->\t Cannot enable MSAA with X{0}! \
              Defaulting to {1}...", sample_count.unwrap(), max_sample_count);
          }
          check_gl_call!("Renderer", gl::Enable(gl::MULTISAMPLE));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::MULTISAMPLE));
        }
        log!(EnumLogColor::Blue, "INFO", "[GlContext] -->\t MSAA {0}",
          sample_count.is_some().then(|| return format!("enabled (X{0})", max_sample_count))
          .unwrap_or("disabled".to_string()));
      }
      EnumRendererOption::Blending(enabled, s_factor, d_factor) => {
        if enabled {
          check_gl_call!("Renderer", gl::Enable(gl::BLEND));
          check_gl_call!("Renderer", gl::BlendFunc(s_factor as GLenum, d_factor as GLenum));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::BLEND));
        }
        log!(EnumLogColor::Blue, "INFO", "[GlContext] -->\t Blending {0}", enabled
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
      EnumRendererOption::Wireframe(enabled) => {
        if enabled {
          check_gl_call!("Renderer", gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE));
        } else {
          check_gl_call!("Renderer", gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL));
        }
        log!(EnumLogColor::Blue, "INFO", "[GlContext] -->\t Wireframe mode {0}", enabled
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
      EnumRendererOption::SRGB(enabled) => {
        if enabled {
          check_gl_call!("Renderer", gl::Enable(gl::FRAMEBUFFER_SRGB));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::FRAMEBUFFER_SRGB));
        }
        log!(EnumLogColor::Blue, "INFO", "[GlContext] -->\t SRGB framebuffer {0}", enabled
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
      EnumRendererOption::CullFacing(face) => {
        if face.is_some() {
          check_gl_call!("Renderer", gl::Enable(gl::CULL_FACE));
          check_gl_call!("Renderer", gl::CullFace(face.unwrap() as GLenum));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::CULL_FACE));
        }
        log!(EnumLogColor::Blue, "INFO", "[GlContext] -->\t Cull facing {0}", face.is_some()
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
    }
    return Ok(());
  }
  
  fn setup_camera(&mut self, camera: &Camera) -> Result<(), renderer::EnumError> {
    // Setup view-projection ubo.
    let mut ubo = GlUbo::new(EnumUboTypeSize::ViewProjection, Some("ubo_camera"), 0)?;
    
    // Apply to all shaders.
    for primitive in self.m_batch.m_primitives.iter() {
      unsafe {
        // If glsl version is lower than 420, then we cannot bind blocks in shaders and have to encode them here instead.
        if (*primitive.m_linked_shader).get_version() < 420 && (*primitive.m_linked_shader).get_lang() == EnumShaderLanguageType::Glsl {
          ubo.bind_block((*primitive.m_linked_shader).get_id(), 0)?;
        }
      }
    }
    
    ubo.set_data(EnumUboType::ViewProjection(camera.get_view_matrix(), camera.get_projection_matrix()))?;
    self.m_batch.m_ubo_buffers.push(ubo);
    return Ok(());
  }
  
  fn flush(&mut self) -> Result<(), renderer::EnumError> {
    self.on_render()?;
    
    self.m_batch.m_primitives.clear();
    self.m_batch.m_vao_buffers.clear();
    self.m_batch.m_vbo_buffers.clear();
    self.m_batch.m_ubo_buffers.clear();
    return Ok(());
  }
  
  fn enqueue(&mut self, sendable_entity: &REntity, shader_associated: &mut Shader) -> Result<(), renderer::EnumError> {
    if sendable_entity.is_empty() {
      log!(EnumLogColor::Yellow, "WARN", "[GlContext] --> Entity [{0}] has no \
      vertices! Not sending it...", sendable_entity)
    }
    
    let mut vao_index: usize = 0;
    let mut vbo_index: usize = 0;
    let mut vbo_offset: usize = 0;
    
    if !self.m_batch.m_vao_buffers.is_empty() {
      vao_index = self.m_batch.m_vao_buffers.len() - 1;
    }
    
    // Figure out if the entity type has already been enqueued. If so, only append to the vbo instead of creating another vao.
    self.m_batch.m_primitives.iter()
      .find(|primitive| primitive.m_linked_shader == shader_associated)
      .map(|matched| {
        vbo_index = matched.m_vbo_index;
        vbo_offset = self.m_batch.m_vbo_buffers[matched.m_vbo_index].m_size;
      });
    
    if self.m_batch.m_vao_buffers.is_empty() {
      
      // Allocate main dynamic vbo to hold all the data provided.
      let mut vbo = GlVbo::new(sendable_entity.size(), sendable_entity.total_vertex_count())?;
      let mut vao = GlVao::new()?;
      
      log!("INFO", "[GlContext] -->\t Submitting primitive {0}...", sendable_entity.m_data.get_name());
      let vertices = sendable_entity.m_data.get_vertices();
      
      let new_primitive: GlPrimitiveInfo = GlPrimitiveInfo {
        m_uuid: sendable_entity.get_uuid(),
        m_linked_shader: shader_associated,
        m_vao_index: vao_index,
        m_vbo_index: vbo_index,
        m_vbo_offset: vbo_offset,
        m_vbo_size: sendable_entity.size(),
        m_vbo_count: vertices.len(),
      };
      
      vbo.set_data(vertices.as_ptr() as *const GLvoid, sendable_entity.size() * vertices.len(), vbo_offset)?;
      // vbo_offset += vertices.len() * sendable_entity.size();
      self.m_batch.m_primitives.push(new_primitive);
      
      //   for sub_primitive in sendable_entity.get_submeshes() {
      //     let vertices = sub_primitive.get_vertices();
      //
      //     let new_primitive: GlPrimitiveInfo = GlPrimitiveInfo {
      //       m_uuid: sendable_entity.get_uuid(),
      //       m_linked_shader: shader_associated,
      //       m_vao_index: vao_index,
      //       m_vbo_index: vbo_index,
      //       m_vbo_offset: vbo_offset,
      //       m_vbo_size: sendable_entity.size(),
      //       m_vbo_count: vertices.len()
      //     };
      //
      //     vbo.set_data(vertices.as_ptr() as *const GLvoid, sendable_entity.size() * vertices.len(), vbo_offset)?;
      //     vbo_offset += vertices.len() * sendable_entity.size();
      //     self.m_batch.m_primitives.push(new_primitive);
      //   }
      
      self.set_attributes(sendable_entity, &mut vao)?;
      self.m_batch.m_vao_buffers.push(vao);
      self.m_batch.m_vbo_buffers.push(vbo);
      
      // Only set ubo for view and projection.
      let mut ubo_model = GlUbo::new(EnumUboTypeSize::Transform, Some("ubo_model"), 1)?;
      
      // If glsl version is lower than 420, then we cannot bind blocks in shaders and have to encode them here instead.
      if shader_associated.get_version() < 420 && shader_associated.get_lang() == EnumShaderLanguageType::Glsl {
        ubo_model.bind_block(shader_associated.get_id(), 1)?;
      }
      ubo_model.set_data(EnumUboType::Transform(sendable_entity.get_matrix()))?;
      self.m_batch.m_ubo_buffers.push(ubo_model);
    }
    
    return Ok(());
  }
  
  fn dequeue(&mut self, uuid: u64) -> Result<(), renderer::EnumError> {
    for index in 0..self.m_batch.m_primitives.len() {
      if self.m_batch.m_primitives[index].m_uuid == uuid {
        self.m_batch.m_vao_buffers[self.m_batch.m_primitives[index].m_vao_index].bind()?;
        
        if !self.m_batch.m_vbo_buffers[self.m_batch.m_primitives[index].m_vbo_index].is_empty() {
          // Free up space without reallocating buffer to save time and allow quick re enqueuing of the same entity.
          self.m_batch.m_vbo_buffers[self.m_batch.m_primitives[index].m_vbo_index]
            .strip(self.m_batch.m_primitives[index].m_vbo_offset, self.m_batch.m_primitives[index].m_vbo_size,
              self.m_batch.m_primitives[index].m_vbo_count)?;
        }
        return Ok(());
      }
    }
    return Err(renderer::EnumError::from(EnumError::EntityUUIDNotFound));
  }
  
  fn update(&mut self, shader_associated: &mut Shader, transform: Mat4) -> Result<(), renderer::EnumError> {
    let shader_found = self.m_batch.m_primitives.iter_mut()
      .find(|primitive| shader_associated.get_id() == unsafe { (*primitive.m_linked_shader).get_id() });
    
    if shader_found.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot update shader ({0}), shader not found in batch!",
        shader_associated.get_id());
      return Err(renderer::EnumError::ShaderNotFound);
    }
    
    let ubo_model_found = self.m_batch.m_ubo_buffers.iter_mut()
      .find(|ubo| ubo.get_name() == Some("ubo_model"));
    
    // If we didn't manually bind to a block name (for glsl versions < 420), otherwise binding is optional.
    if ubo_model_found.is_none() && shader_associated.get_version() < 420 {
      log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot update transform ubo, ubo not found in batch!");
      return Err(renderer::EnumError::UboNotFound);
    }
    
    self.m_batch.m_ubo_buffers[0].bind()?;
    self.m_batch.m_ubo_buffers[0].set_data(EnumUboType::Transform(transform))?;
    return Ok(());
  }
  
  fn free(&mut self) -> Result<(), renderer::EnumError> {
    if self.m_state == EnumState::NotCreated {
      log!(EnumLogColor::Yellow, "WARN", "[GlContext] -->\t Cannot free resources : OpenGL renderer \
      has not been created!");
      return Err(renderer::EnumError::from(EnumError::InvalidContext));
    }
    
    if self.m_state == EnumState::Deleted {
      log!(EnumLogColor::Yellow, "WARN", "[GlContext] -->\t Cannot free resources : OpenGL renderer \
      has already been deleted!");
      return Err(renderer::EnumError::from(EnumError::InvalidContext));
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
    
    self.m_state = EnumState::Deleted;
    return Ok(());
  }
}

impl GlContext {
  fn load_extensions() -> Result<Vec<String>, EnumError> {
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
          return Err(EnumError::CStringError);
        }
      }
    }
    return Ok(gl_extensions_available);
  }
  
  fn set_attributes(&mut self, entity: &REntity, vao: &mut GlVao) -> Result<(), EnumError> {
    return match entity.m_type {
      EnumPrimitiveType::Mesh(is_flat_shaded) => {
        // Establish vao attributes.
        let mut attributes: Vec<GlVertexAttribute> = Vec::with_capacity(5);
        
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
        
        // Enable all added attributes.
        vao.enable_attributes(attributes)
      }
      _ => todo!()
    };
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
      gl::DEBUG_TYPE_PERFORMANCE => {
        final_error_msg += "\nType =>\t\t Performance;"
      }
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