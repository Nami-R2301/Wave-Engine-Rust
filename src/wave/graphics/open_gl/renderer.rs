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
use std::collections::HashSet;
extern crate gl46;

use gl46::GlFns;
use crate::{check_gl_call, log};
use crate::wave::assets::renderable_assets::REntity;
use crate::wave::camera::{Camera};
use crate::wave::graphics::open_gl::buffer::{EnumAttributeType, EnumUboType, EnumUboTypeSize, GLchar,
  GLenum, GLsizei, GlUbo, GLuint, GlVao, GlVbo, GlVertexAttribute, GLvoid};
use crate::wave::graphics::{open_gl, renderer};
use crate::wave::graphics::renderer::{EnumFeature, TraitContext};
use crate::wave::graphics::shader::Shader;
use crate::wave::window::Window;

/*
///////////////////////////////////   OpenGL renderer   ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
 */

pub(crate) static mut S_GL_4_6: Option<GlFns> = None;

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
pub enum EnumError {
  InvalidContext,
  InvalidOperation(GLenum),
  MSAAError,
  InvalidBufferOperation(open_gl::buffer::EnumError),
  InvalidShaderOperation(open_gl::shader::EnumError)
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

#[macro_export]
macro_rules! check_gl_call {
    () => {};
    ($name:literal, $gl_function:expr) => {
      unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
      unsafe {
        $gl_function;
        let error = gl::GetError();
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t Error when executing gl call! \
          Code => 0x{1:x}", $name, error);
          return Err(crate::wave::graphics::open_gl::renderer::EnumError::InvalidOperation(error).into());
        }
      }
    };
    ($name:literal, let mut $var:ident: $var_type:ty = $gl_function:expr) => {
      unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
      let mut $var:$var_type = unsafe { $gl_function };
      unsafe {
        let error = gl::GetError();
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t Error when executing gl call! \
               Code => 0x{1:x}", $name, error);
          return Err(crate::wave::graphics::open_gl::renderer::EnumError::InvalidOperation(error).into());
        }
      }
    };
    ($name:literal, let $var:ident: $var_type:ty = $gl_function:expr) => {
      unsafe { while gl::GetError() != gl::NO_ERROR {} };  // Clear previous errors.
      let $var:$var_type = unsafe { $gl_function };
      unsafe {
        let error = gl::GetError();
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t Error when executing gl call! \
             Code => 0x{1:x}", $name, error);
          return Err(crate::wave::graphics::open_gl::renderer::EnumError::InvalidOperation(error).into());
        }
      }
    };
    ($name:literal, $var:ident = $gl_function:expr) => {
      unsafe { while gl::GetError() != gl::NO_ERROR {} };
      unsafe { $var = $gl_function; }
      unsafe {
        let error = gl::GetError();
        if error != gl::NO_ERROR {
          log!(EnumLogColor::Red, "ERROR", "[{0}] -->\t Error when executing gl call! \
           Code => 0x{1:x}", $name, error);
          return Err(crate::wave::graphics::open_gl::renderer::EnumError::InvalidOperation(error).into());
        }
      }
    };
}

struct GlBatchPrimitives {
  m_shaders: Vec<u32>,
  m_vao_buffers: Vec<GlVao>,
  m_vbo_buffers: Vec<GlVbo>,
  m_ubo_buffers: Vec<GlUbo>
}

impl GlBatchPrimitives {
  pub fn new() -> Self {
    return GlBatchPrimitives {
      m_shaders: Vec::new(),
      m_vao_buffers: Vec::new(),
      m_vbo_buffers: Vec::new(),
      m_ubo_buffers: Vec::new()
    };
  }
}


pub struct GlContext {
  m_batch: GlBatchPrimitives,
  m_debug_callback: gl::types::GLDEBUGPROC,
}

impl TraitContext for GlContext {
  fn on_new(window: &mut Window) -> Result<Self, renderer::EnumError> {
    // Init context.
    window.init_opengl_surface();
    gl::load_with(|f_name| window.get_api_ref().get_proc_address_raw(f_name));
    unsafe {
      S_GL_4_6 = Some(GlFns::load_from(&|f_name| {
        let string = std::ffi::CStr::from_ptr(f_name as *const std::ffi::c_char);
        window.get_api_ref().get_proc_address_raw(string.to_str().unwrap())
      }).unwrap());
    }
    return Ok(GlContext {
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
  
  fn get_max_shader_version_available(&self) -> f32 {
    let shading_info: Vec<&str> = unsafe {
      std::ffi::CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8)
        .to_str().unwrap_or("Cannot retrieve api version information!")
        .split(" ")
        .collect()
    };
    let shading_language_version_str = shading_info.first().unwrap();
    return shading_language_version_str.parse::<f32>().unwrap_or(0.0);
  }
  
  fn on_events(&mut self, window_event: glfw::WindowEvent) -> Result<bool, renderer::EnumError> {
    return match window_event {
      glfw::WindowEvent::FramebufferSize(width, height) => {
        check_gl_call!("Renderer", gl::Viewport(0, 0, width, height));
        Ok(true)
      }
      _ => { Ok(false) }
    }
  }
  
  fn on_render(&mut self) -> Result<(), renderer::EnumError> {
    check_gl_call!("Renderer", gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));
    for index in 0usize..self.m_batch.m_shaders.len() {
      check_gl_call!("Renderer", gl::UseProgram(self.m_batch.m_shaders[index]));
      self.m_batch.m_vao_buffers[index].bind()?;
      check_gl_call!("Renderer", gl::DrawArrays(gl::TRIANGLES, 0, self.m_batch.m_vbo_buffers[index].m_count as GLsizei));
    }
    return Ok(());
  }
  
  fn on_delete(&mut self) -> Result<(), renderer::EnumError> {
    return Ok(());
  }
  
  fn submit(&mut self, features: &HashSet<EnumFeature>) -> Result<(), renderer::EnumError> {
    // Enable or disable features AFTER context creation since we need a context to load our openGL
    // functions.
    for feature in features {
      self.toggle(*feature)?;
    }
    
    let window_opt = Window::get();
    if window_opt.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot set OpenGl viewport dimensions : \
      No active window context!");
      return Err(renderer::EnumError::from(EnumError::InvalidContext));
    }
    
    let window = window_opt.unwrap();
    let window_framebuffer_size = unsafe {
      (*window).get_framebuffer_size()
    };
    check_gl_call!("Renderer", gl::Viewport(0, 0, window_framebuffer_size.0 as i32, window_framebuffer_size.1 as i32));
    check_gl_call!("Renderer", gl::ClearColor(0.15, 0.15, 0.15, 1.0));
    
    check_gl_call!("Renderer", gl::FrontFace(gl::CW));
    return Ok(());
  }
  
  fn get_max_msaa_count(&self) -> Result<u8, renderer::EnumError> {
    // let framebuffer_color_sample_count: u8 = self.m_framebuffer.max_color_sample_count;
    // let framebuffer_depth_sample_count: u8 = self.m_framebuffer.max_depth_sample_count;
    //
    // return framebuffer_color_sample_count.min(framebuffer_depth_sample_count);
    let window = Window::get();
    if window.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot retrieve MSAA max count supported \
      by the window context : No active window context available!");
      return Err(renderer::EnumError::from(EnumError::InvalidContext));
    }
    return Ok(unsafe { (*window.unwrap()).m_samples });
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
      
      let str: String = format!("Api => OpenGL,\n{0:<113}\
      Api version => {4},\n{0:<113}\
      Vendor => {2},\n{0:<113}\
      Device name => {1},\n{0:<113}\
      Driver version => {3},\n{0:<113}\
      Shading language => {5}",
        "", device_name, api_vendor, driver_version, api_version, shading_info);
      return str;
    }
  }
  
  fn toggle(&mut self, feature: EnumFeature) -> Result<(), renderer::EnumError> {
    match feature {
      EnumFeature::Debug(enabled) => {
        if enabled {
          check_gl_call!("Renderer", gl::Enable(gl::DEBUG_OUTPUT));
          check_gl_call!("Renderer", gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS));
          check_gl_call!("Renderer", gl::DebugMessageCallback(self.m_debug_callback, std::ptr::null()));
          check_gl_call!("Renderer", gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE,
            gl::DONT_CARE, 0, std::ptr::null(), gl::TRUE));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::DEBUG_OUTPUT));
          check_gl_call!("Renderer", gl::Disable(gl::DEBUG_OUTPUT_SYNCHRONOUS));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Debug mode {0}",
          enabled.then(|| return "enabled").unwrap_or("disabled"));
      }
      EnumFeature::DepthTest(enabled) => {
        if enabled {
          check_gl_call!("Renderer", gl::Enable(gl::DEPTH_TEST));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::DEPTH_TEST));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Depth test {0}",
          enabled.then(|| return "enabled").unwrap_or("disabled"));
      }
      EnumFeature::MSAA(sample_count) => {
        #[allow(unused)]
        let mut max_sample_count: u8 = 0;
        if sample_count.is_some() {
          max_sample_count = self.get_max_msaa_count()?;
          if max_sample_count < 2 {
            log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t Cannot enable MSAA!");
            return Err(renderer::EnumError::from(EnumError::MSAAError));
          } else if sample_count.unwrap() > max_sample_count {
            log!(EnumLogColor::Yellow, "WARN", "[Renderer] -->\t Cannot enable MSAA with X{0}! \
              Defaulting to {1}...", sample_count.unwrap(), max_sample_count);
          }
          check_gl_call!("Renderer", gl::Enable(gl::MULTISAMPLE));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::MULTISAMPLE));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t MSAA {0}",
          sample_count.is_some().then(|| return format!("enabled (X{0})", max_sample_count))
          .unwrap_or("disabled".to_string()));
      }
      EnumFeature::Blending(enabled, s_factor, d_factor) => {
        if enabled {
          check_gl_call!("Renderer", gl::Enable(gl::BLEND));
          check_gl_call!("Renderer", gl::BlendFunc(s_factor as GLenum, d_factor as GLenum));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::BLEND));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Blending {0}", enabled
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
      EnumFeature::Wireframe(enabled) => {
        if enabled {
          check_gl_call!("Renderer", gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE));
        } else {
          check_gl_call!("Renderer", gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Wireframe mode {0}", enabled
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
      EnumFeature::SRGB(enabled) => {
        if enabled {
          check_gl_call!("Renderer", gl::Enable(gl::FRAMEBUFFER_SRGB));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::FRAMEBUFFER_SRGB));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t SRGB framebuffer {0}", enabled
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
      EnumFeature::CullFacing(face) => {
        if face.is_some() {
          check_gl_call!("Renderer", gl::Enable(gl::CULL_FACE));
          check_gl_call!("Renderer", gl::CullFace(face.unwrap() as GLenum));
        } else {
          check_gl_call!("Renderer", gl::Disable(gl::CULL_FACE));
        }
        log!(EnumLogColor::Blue, "INFO", "[Renderer] -->\t Cull facing {0}", face.is_some()
          .then(|| return "enabled")
          .unwrap_or("disabled"));
      }
    }
    return Ok(());
  }
  
  fn setup_camera_ubo(&mut self, camera: &Camera) -> Result<(), renderer::EnumError> {
    // Setup common ubos across all shaders.
    self.m_batch.m_ubo_buffers.push(GlUbo::new(EnumUboTypeSize::ViewProjection, 0)?);
    for shader_id in self.m_batch.m_shaders.iter() {
      self.m_batch.m_ubo_buffers[0].bind("u_camera", *shader_id)?;
    }
    self.m_batch.m_ubo_buffers[0].set_data(EnumUboType::ViewProjection(camera.get_view_matrix(),
      camera.get_projection_matrix()))?;
    return Ok(());
  }
  
  fn flush(&mut self) {
    todo!()
  }
  
  fn enqueue(&mut self, sendable_entity: &REntity, shader_associated: &mut Shader) -> Result<(), renderer::EnumError> {
    if sendable_entity.is_empty() {
      log!(EnumLogColor::Yellow, "WARN", "[Renderer] --> Entity {0} sent has no \
      vertices! Not sending it...", sendable_entity)
    }
    
    let offset: usize = 0;
    
    // Allocate main dynamic vbo to hold all the data provided.
    let mut vbo = GlVbo::new(sendable_entity.size(), sendable_entity.count())?;
    let mut vao = GlVao::new()?;
    let mut ubo_model = GlUbo::new(EnumUboTypeSize::Transform, 1)?;
    
    vbo.set_data(sendable_entity.m_data.as_ptr() as *const GLvoid,
      sendable_entity.size() * sendable_entity.count(), offset)?;
    
    // Establish vao attributes.
    let mut attributes: Vec<GlVertexAttribute> = Vec::with_capacity(5);
    
    // IDs.
    attributes.push(GlVertexAttribute::new(EnumAttributeType::UnsignedInt(1),
      false, 0, 0).map_err(|error| {
        return EnumError::InvalidBufferOperation(error);
    })?);
    
    // Positions.
    attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec3,
      false, 0,0).map_err(|error| {
      return EnumError::InvalidBufferOperation(error);
    })?);
    
    // Normals.
    if sendable_entity.is_flat_shaded() {
      attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec3,
        false, 0,1).map_err(|error| {
        return EnumError::InvalidBufferOperation(error);
      })?);
    } else {
      attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec3,
        false, 0,0).map_err(|error| {
        return EnumError::InvalidBufferOperation(error);
      })?);
    }
    
    // Colors.
    attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec4,
      false, 0,0).map_err(|error| {
      return EnumError::InvalidBufferOperation(error);
    })?);
    
    // Texture coordinates.
    attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec2,
      false,  0,0).map_err(|error| {
      return EnumError::InvalidBufferOperation(error);
    })?);
    
    // Enable all added attributes.
    vao.enable_attributes(attributes)?;
    
    ubo_model.bind("u_model", shader_associated.get_id())?;
    ubo_model.set_data(EnumUboType::Transform(sendable_entity.get_matrix()))?;
    
    self.m_batch.m_shaders.push(shader_associated.get_id());
    self.m_batch.m_vao_buffers.push(vao);
    self.m_batch.m_vbo_buffers.push(vbo);
    self.m_batch.m_ubo_buffers.push(ubo_model);
    
    return Ok(());
  }
  
  fn dequeue(&mut self, _id: &u64) -> Result<(), renderer::EnumError> {
    todo!()
  }
}

extern "system" fn gl_error_callback(error_code: GLenum, e_type: GLenum, _id: GLuint,
                                     severity: GLenum, _length: GLsizei, error_message: *const GLchar,
                                     _user_param: *mut std::ffi::c_void) {
  let mut final_error_msg: String = "".to_string();
  if error_code != gl::NO_ERROR {
    match error_code {
      0x8246 => return, // Filter out performance info regarding using video memory for GL_DYNAMIC_DRAW.
      _ => { final_error_msg += &format!("Code => 0x{0:x}; ", error_code) }
    }
    
    match e_type {
      gl::DEBUG_TYPE_ERROR => { final_error_msg += "Type => Error; "; }
      gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => { final_error_msg += "Type => Deprecated behavior; "; }
      gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => { final_error_msg += "Type => Undefined behavior; "; }
      gl::DEBUG_TYPE_PORTABILITY => { final_error_msg += "Type => Portability; "; }
      gl::DEBUG_TYPE_PERFORMANCE => { final_error_msg += "Type => Performance; "; }
      gl::DEBUG_TYPE_MARKER => { final_error_msg += "Type => Marker; "; }
      gl::DEBUG_TYPE_PUSH_GROUP => { final_error_msg += "Type => Push group; "; }
      gl::DEBUG_TYPE_POP_GROUP => { final_error_msg += "Type => Pop group; "; }
      gl::DEBUG_TYPE_OTHER => { final_error_msg += "Type => Other; "; }
      _ => { final_error_msg = "Type => Unknown; ".to_string(); }
    }
    
    match severity {
      gl::DEBUG_SEVERITY_HIGH => { final_error_msg += "Severity => Fatal (High);\n" }
      gl::DEBUG_SEVERITY_MEDIUM => { final_error_msg += "Severity => Fatal (Medium);\n" }
      gl::DEBUG_SEVERITY_LOW => { final_error_msg += "Severity => Warn (Low);\n" }
      gl::DEBUG_SEVERITY_NOTIFICATION => { final_error_msg += "Severity => Warn (Info);\n" }
      _ => { final_error_msg += "Severity => Fatal (Unknown);\n" }
    }
    
    let test = unsafe { std::ffi::CStr::from_ptr(error_message.cast_mut()) };
    let str = test.to_str()
      .expect("[Renderer] -->\t Failed to convert C string to Rust String in gl_error_callback()");
    
    final_error_msg += str;
    
    match severity {
      gl::DEBUG_SEVERITY_HIGH => { log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t {0}", final_error_msg); }
      gl::DEBUG_SEVERITY_MEDIUM => { log!(EnumLogColor::Yellow, "WARN", "[Renderer] -->\t {0}", final_error_msg); }
      gl::DEBUG_SEVERITY_LOW => { log!(EnumLogColor::Yellow, "WARN", "[Renderer] -->\t {0}", final_error_msg); }
      gl::DEBUG_SEVERITY_NOTIFICATION => { log!(EnumLogColor::Yellow, "WARN", "[Renderer] -->\t {0}", final_error_msg); }
      _ => {
        log!(EnumLogColor::Red, "ERROR", "[Renderer] -->\t {0}", final_error_msg);
      }
    }
  }
}