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
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::mem::size_of;

use gl46::GlFns;
use gl::types::{GLint, GLvoid};

use crate::{Engine, S_ENGINE};
use crate::assets::r_assets::{EnumMaterial, EnumPrimitive, EnumVertexMemberOffset, REntity, TraitPrimitive};
use crate::events::EnumEvent;
use crate::graphics::{open_gl, renderer};
use crate::graphics::open_gl::buffer::{EnumAttributeType, EnumUboType, EnumUboTypeSize, GLchar, GLenum, GlIbo, GLsizei, GlUbo, GLuint, GlVao, GlVbo, GlVertexAttribute};
use crate::graphics::renderer::{EnumRendererBlendingFactor, EnumRendererCallCheckingMode, EnumRendererCull, EnumRendererError, EnumRendererHint, EnumRendererOptimizationMode, EnumRendererRenderPrimitiveAs, EnumRendererState, TraitContext};
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
        if engine.m_renderer.m_hints.contains(&crate::graphics::renderer::EnumRendererHint::ApiCallChecking(crate::graphics::renderer::EnumRendererCallCheckingMode::None)) ||
          engine.m_renderer.m_hints.contains(&crate::graphics::renderer::EnumRendererHint::ApiCallChecking(crate::graphics::renderer::EnumRendererCallCheckingMode::Async))
         {
          unsafe { $gl_function };
        } else if engine.m_renderer.m_hints.contains(&crate::graphics::renderer::EnumRendererHint::ApiCallChecking(crate::graphics::renderer::EnumRendererCallCheckingMode::Sync)) ||
        engine.m_renderer.m_hints.contains(&crate::graphics::renderer::EnumRendererHint::ApiCallChecking(crate::graphics::renderer::EnumRendererCallCheckingMode::SyncAndAsync)) {
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
        if engine.m_renderer.m_hints.contains(&crate::graphics::renderer::EnumRendererHint::ApiCallChecking(crate::graphics::renderer::EnumRendererCallCheckingMode::None)) ||
          engine.m_renderer.m_hints.contains(&crate::graphics::renderer::EnumRendererHint::ApiCallChecking(crate::graphics::renderer::EnumRendererCallCheckingMode::Async))
         {
          $var = unsafe { $gl_function };
        } else if engine.m_renderer.m_hints.contains(&crate::graphics::renderer::EnumRendererHint::ApiCallChecking(crate::graphics::renderer::EnumRendererCallCheckingMode::Sync)) ||
        engine.m_renderer.m_hints.contains(&crate::graphics::renderer::EnumRendererHint::ApiCallChecking(crate::graphics::renderer::EnumRendererCallCheckingMode::SyncAndAsync)) {
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
        if engine.m_renderer.m_hints.contains(&crate::graphics::renderer::EnumRendererHint::ApiCallChecking(crate::graphics::renderer::EnumRendererCallCheckingMode::None)) ||
          engine.m_renderer.m_hints.contains(&crate::graphics::renderer::EnumRendererHint::ApiCallChecking(crate::graphics::renderer::EnumRendererCallCheckingMode::Async))
         {
          $var = unsafe { $gl_function };
        } else if engine.m_renderer.m_hints.contains(&crate::graphics::renderer::EnumRendererHint::ApiCallChecking(crate::graphics::renderer::EnumRendererCallCheckingMode::Sync)) ||
        engine.m_renderer.m_hints.contains(&crate::graphics::renderer::EnumRendererHint::ApiCallChecking(crate::graphics::renderer::EnumRendererCallCheckingMode::SyncAndAsync)) {
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

impl From<EnumRendererBlendingFactor> for GLenum {
  fn from(value: EnumRendererBlendingFactor) -> Self {
    return match value {
      EnumRendererBlendingFactor::Zero => gl::ZERO,
      EnumRendererBlendingFactor::One => gl::ONE,
      EnumRendererBlendingFactor::SrcColor => gl::SRC_COLOR,
      EnumRendererBlendingFactor::OneMinusSrcColor => gl::ONE_MINUS_SRC_COLOR,
      EnumRendererBlendingFactor::DstColor => gl::DST_COLOR,
      EnumRendererBlendingFactor::OneMinusDstColor => gl::ONE_MINUS_DST_COLOR,
      EnumRendererBlendingFactor::SrcAlpha => gl::SRC_ALPHA,
      EnumRendererBlendingFactor::OneMinusSrcAlpha => gl::ONE_MINUS_SRC_ALPHA,
      EnumRendererBlendingFactor::DstAlpha => gl::DST_ALPHA,
      EnumRendererBlendingFactor::OneMinusDstAlpha => gl::ONE_MINUS_DST_ALPHA,
      EnumRendererBlendingFactor::ConstantColor => gl::CONSTANT_COLOR,
      EnumRendererBlendingFactor::OneMinusConstantColor => gl::ONE_MINUS_CONSTANT_COLOR,
      EnumRendererBlendingFactor::ConstantAlpha => gl::CONSTANT_ALPHA,
      EnumRendererBlendingFactor::OneMinusConstantAlpha => gl::ONE_MINUS_CONSTANT_ALPHA
    };
  }
}

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
pub enum EnumOpenGLError {
  CStringError,
  ApiFunctionLoadingError,
  UnsupportedApiFunction,
  InvalidSubPrimitiveIndex,
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

#[repr(u32)]
#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
pub enum EnumGlPrimitiveMode {
  Point = gl::POINTS,
  Line = gl::LINES,
  LineStrip = gl::LINE_STRIP,
  LineLoop = gl::LINE_LOOP,
  LineAdjacency = gl::LINES_ADJACENCY,
  LineStripAdjacency = gl::LINE_STRIP_ADJACENCY,
  Triangle = gl::TRIANGLES,
  TriangleStrip = gl::TRIANGLE_STRIP,
  TriangleFan = gl::TRIANGLE_FAN,
  TriangleAdjacency = gl::TRIANGLES_ADJACENCY,
  TriangleStripAdjacency = gl::TRIANGLE_STRIP_ADJACENCY,
}

#[repr(u32)]
#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
pub enum EnumGlElementType {
  UnsignedByte = gl::UNSIGNED_BYTE,
  UnsignedShort = gl::UNSIGNED_SHORT,
  UnsignedInt = gl::UNSIGNED_INT,
}

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
pub enum EnumGlDrawCommandFunction {
  DrawArray(EnumGlPrimitiveMode, GLint, GLsizei),
  DrawElements(EnumGlPrimitiveMode, GLsizei, EnumGlElementType, *const GLvoid),
  DrawElementsBaseVertex(EnumGlPrimitiveMode, GLsizei, EnumGlElementType, usize, GLint),
  MultiDrawArrays(EnumGlPrimitiveMode, *const GLint, *const GLsizei, GLsizei),
  MultiDrawElements(EnumGlPrimitiveMode, *const GLsizei, EnumGlElementType, *const *const GLvoid, GLsizei),
  MultiDrawElementsBaseVertex(EnumGlPrimitiveMode, *const GLsizei, EnumGlElementType, *const *const GLvoid, GLsizei, *mut GLint),
  MultiDrawArraysIndirect(EnumGlPrimitiveMode, *const GLvoid, GLsizei, GLsizei),
  MultiDrawElementsIndirect(EnumGlPrimitiveMode, EnumGlElementType, *const GLvoid, GLsizei, GLsizei),
}

impl Display for EnumGlDrawCommandFunction {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return match self {
      EnumGlDrawCommandFunction::DrawArray(mode, vertex_offset, vertex_count) => {
        write!(f, "glDrawArrays({0:?}, {1}, {2})", mode, vertex_offset, vertex_count)
      }
      EnumGlDrawCommandFunction::DrawElements(mode, index_count, index_type, index_array) => {
        write!(f, "glDrawElements({0:?}, {1}, {2:?}, {3:?})", mode, index_count, index_type, index_array)
      }
      EnumGlDrawCommandFunction::DrawElementsBaseVertex(mode, index_count, index_type,
        index_offset, vertex_offset) => {
        write!(f, "glDrawElementsBaseVertex({0:?}, {1}, {2:?}, {3}, {4})", mode, index_count, index_type, index_offset, vertex_offset)
      }
      EnumGlDrawCommandFunction::MultiDrawArrays(mode, vertex_array_pointer, vertex_count_array_pointer,
        vertex_array_count) => {
        write!(f, "glMultiDrawArrays({0:?}, {1:?}, {2:?}, {3})", mode, vertex_array_pointer, vertex_count_array_pointer, vertex_array_count)
      }
      EnumGlDrawCommandFunction::MultiDrawElements(mode, index_count_array_pointer, index_type,
        index_array_pointer, index_array_count) => {
        write!(f, "glMultiDrawElements({0:?}, {1:?}, {2:?}, {3:?}, {4})", mode, index_count_array_pointer, index_type, index_array_pointer,
          index_array_count)
      }
      EnumGlDrawCommandFunction::MultiDrawElementsBaseVertex(mode, index_count_array_pointer,
        index_type, index_array_pointer, index_array_count,
        vertex_offsets_array_pointer) => {
        write!(f, "glMultiDrawElements({0:?}, {1:?}, {2:?}, {3:?}, {4}, {5:?})", mode, index_count_array_pointer, index_type,
          index_array_pointer, index_array_count, vertex_offsets_array_pointer)
      }
      EnumGlDrawCommandFunction::MultiDrawArraysIndirect(mode, draw_command_array_pointer,
        draw_count, stride_between_commands) => {
        write!(f, "glMultiDrawArrays({0:?}, {1:?}, {2}, {3})", mode, draw_command_array_pointer, draw_count, stride_between_commands)
      }
      EnumGlDrawCommandFunction::MultiDrawElementsIndirect(mode, index_type,
        draw_command_array_pointer, draw_count, stride_between_commands) => {
        write!(f, "glMultiDrawArrays({0:?}, {1:?}, {2:?}, {3}, {4})", mode, index_type, draw_command_array_pointer, draw_count,
          stride_between_commands)
      }
    };
  }
}

#[derive(Clone)]
struct GlShaderInfo {
  m_id: u32,
}

// Modern glMultiDrawElementsIndirect's 'indirect' draw command struct.
#[repr(C, packed)]
struct GlDrawCommandInfo {
  m_index_count: usize,
  m_instance_count: usize,
  m_first_index: usize,
  m_base_vertex: usize,
  m_base_instance: usize,
}

#[derive(Clone)]
struct GlPrimitiveInfo {
  m_uuid: u64,
  m_linked_shader: GlShaderInfo,
  m_vao_index: usize,
  m_vbo_index: usize,
  m_ibo_index: usize,
  m_vbo_count: usize,
  m_ibo_count: usize,
  m_base_vertex: usize,
  m_base_index: usize,
  m_entity_offset: usize,
  m_primitive_count: usize,
  m_sub_primitive_index: usize,
  m_visible: bool,  // Make primitive appear or disappear upon request from the user
}

struct GlRendererCommands {
  m_primitives: Vec<GlPrimitiveInfo>,
  m_draw_commands: Vec<EnumGlDrawCommandFunction>,
  m_vao_buffers: Vec<GlVao>,
  m_vbo_buffers: Vec<GlVbo>,
  m_ibo_buffers: Vec<GlIbo>,
  m_ubo_buffers: Vec<GlUbo>,
}

impl GlRendererCommands {
  pub fn new() -> Self {
    return GlRendererCommands {
      m_primitives: Vec::new(),
      m_draw_commands: Vec::with_capacity(2),
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
  m_commands: GlRendererCommands,
  m_debug_callback: gl::types::GLDEBUGPROC,
  m_options: Vec<EnumRendererHint>,
  m_batch_mode: EnumRendererOptimizationMode,
}

impl TraitContext for GlContext {
  fn default() -> Self {
    return Self {
      m_state: EnumRendererState::NotCreated,
      m_ext: HashMap::new(),
      m_commands: GlRendererCommands::new(),
      m_debug_callback: Some(gl_error_callback),
      m_options: Vec::new(),
      m_batch_mode: EnumRendererOptimizationMode::default(),
    };
  }
  
  fn on_new(window: &mut Window, options: Vec<EnumRendererHint>) -> Result<Self, EnumRendererError> {
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
      m_commands: GlRendererCommands::new(),
      m_debug_callback: Some(gl_error_callback),
      m_options: Vec::from(options),
      m_batch_mode: EnumRendererOptimizationMode::default(),
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
      let mut previous_ibo: usize = 0;
      let mut ibo_offset_counter: usize = 0;
      
      
      for primitive in self.m_commands.m_primitives.iter() {
        if primitive.m_linked_shader.m_id != previous_shader_id as u32 {
          check_gl_call!("GlContext", gl::UseProgram(primitive.m_linked_shader.m_id));
          
          self.m_commands.m_vao_buffers[primitive.m_vao_index].bind()?;
          
          previous_shader_id = primitive.m_linked_shader.m_id as i32;
          previous_ibo = primitive.m_ibo_index;
          ibo_offset_counter = 0;
        }
        
        if primitive.m_ibo_count > 0 && previous_ibo != primitive.m_ibo_index {
          self.m_commands.m_ibo_buffers[primitive.m_ibo_index].bind()?;
          ibo_offset_counter = 0;
        }
        
        if primitive.m_visible {
          match self.m_batch_mode {
            EnumRendererOptimizationMode::NoOptimizations | EnumRendererOptimizationMode::MinimizeVertexBuffers => {
              if primitive.m_ibo_count == 0 {
                check_gl_call!("GlContext", gl::DrawArrays(gl::TRIANGLES, primitive.m_base_vertex as i32, primitive.m_vbo_count as GLsizei));
                continue;
              }
              check_gl_call!("GlContext", gl::DrawElementsBaseVertex(gl::TRIANGLES, primitive.m_ibo_count as GLsizei,
              gl::UNSIGNED_INT, ibo_offset_counter as *const _, primitive.m_base_index as i32));
            }
            EnumRendererOptimizationMode::MinimizeIndexBuffers => {
              if primitive.m_ibo_count == 0 {
                check_gl_call!("GlContext", gl::DrawArrays(gl::TRIANGLES, primitive.m_base_vertex as i32, primitive.m_vbo_count as GLsizei));
                continue;
              }
              
              check_gl_call!("GlContext", gl::DrawElementsBaseVertex(gl::TRIANGLES, primitive.m_ibo_count as GLsizei,
              gl::UNSIGNED_INT, ibo_offset_counter as *const _, 0));
            }
            EnumRendererOptimizationMode::MinimizeDrawCalls | EnumRendererOptimizationMode::All => {
              if primitive.m_ibo_count == 0 {
                check_gl_call!("GlContext", gl::DrawArrays(gl::TRIANGLES, 0,
                  self.m_commands.m_vbo_buffers[primitive.m_vbo_index].m_count as GLsizei));
                continue;
              }
              check_gl_call!("GlContext", gl::DrawElements(gl::TRIANGLES,
                self.m_commands.m_ibo_buffers[primitive.m_ibo_index].m_count as GLsizei,
              gl::UNSIGNED_INT, std::ptr::null() as *const _));
            }
          }
        }
        ibo_offset_counter += primitive.m_ibo_count * size_of::<u32>();
      }
    }
    return Ok(());
  }
  
  fn apply(&mut self, window: &mut Window) -> Result<(), EnumRendererError> {
    // Enable or disable features AFTER context creation since we need a context to load our openGL
    // functions.
    self.toggle_options()?;
    
    let window_framebuffer_size = window.get_framebuffer_size();
    check_gl_call!("GlContext", gl::Viewport(0, 0, window_framebuffer_size.0 as i32, window_framebuffer_size.1 as i32));
    check_gl_call!("GlContext", gl::ClearColor(0.025, 0.025, 0.025, 1.0));
    
    self.m_state = EnumRendererState::Submitted;
    return Ok(());
  }
  
  fn toggle_visibility_of(&mut self, entity_uuid: u64, instance_offset: Option<usize>, visible: bool) -> Result<(), EnumRendererError> {
    let primitives_similar_found = self.m_commands.m_primitives.iter_mut()
      .filter(|primitive| primitive.m_uuid == entity_uuid)
      .collect::<Vec<&mut GlPrimitiveInfo>>();
    
    if !primitives_similar_found.is_empty() {
      // If we want all sub_primitives within the primitive to be hidden as well.
      if instance_offset.is_none() {
        for sub_primitive in primitives_similar_found.into_iter() {
          sub_primitive.m_visible = visible;
        }
        return Ok(());
      }
      
      if instance_offset.is_some() {
        for (position, main_primitive) in primitives_similar_found.into_iter().enumerate() {
          if position == instance_offset.unwrap() {
            main_primitive.m_visible = visible;
          }
        }
        return Ok(());
      }
    }
    log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot toggle visibility of entity {0}, entity not found!", entity_uuid);
    return Err(EnumRendererError::EntityNotFound);
  }
  
  fn toggle_primitive_mode(&mut self, mode: EnumRendererRenderPrimitiveAs, entity_uuid: u64, instance_offset: Option<usize>, instance_count: usize) -> Result<(), EnumRendererError> {
    match mode {
      EnumRendererRenderPrimitiveAs::Filled => {
        check_gl_call!("GlContext", gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL));
        self.toggle_solid_wireframe(false, entity_uuid, instance_offset, instance_count)?
      }
      EnumRendererRenderPrimitiveAs::Points => {
        check_gl_call!("GlContext", gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT));
        self.toggle_solid_wireframe(false, entity_uuid, instance_offset, instance_count)?
      }
      EnumRendererRenderPrimitiveAs::Wireframe => {
        check_gl_call!("GlContext", gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE));
        self.toggle_solid_wireframe(false, entity_uuid, instance_offset, instance_count)?
      }
      EnumRendererRenderPrimitiveAs::SolidWireframe => {
        check_gl_call!("GlContext", gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL));
        self.toggle_solid_wireframe(true, entity_uuid, instance_offset, instance_count)?
      }
    }
    return Ok(());
  }
  
  fn get_max_msaa_count(&self) -> Result<u8, EnumRendererError> {
    // let framebuffer_color_sample_count: u8 = self.m_framebuffer.max_color_sample_count;
    // let framebuffer_depth_sample_count: u8 = self.m_framebuffer.max_depth_sample_count;
    //
    // return framebuffer_color_sample_count.min(framebuffer_depth_sample_count);
    let window = Engine::get_active_window();
    return Ok(window.m_samples as u8);
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
  
  fn toggle_options(&mut self) -> Result<(), EnumRendererError> {
    let options_cloned = self.m_options.clone();
    for option in options_cloned.into_iter() {
      match option {
        EnumRendererHint::ApiCallChecking(debug_type) => {
          match debug_type {
            EnumRendererCallCheckingMode::None => unsafe {
              gl::Disable(gl::DEBUG_OUTPUT);
              gl::Disable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            }
            EnumRendererCallCheckingMode::Sync => unsafe {
              gl::Enable(gl::DEBUG_OUTPUT);
              gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            },
            EnumRendererCallCheckingMode::Async => unsafe {
              gl::Enable(gl::DEBUG_OUTPUT);
              // Disable sync messages.
              gl::Disable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
              if gl::DebugMessageCallback::is_loaded() {
                gl::DebugMessageCallback(self.m_debug_callback, std::ptr::null());
                gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, std::ptr::null(), gl::TRUE);
              }
            }
            EnumRendererCallCheckingMode::SyncAndAsync => unsafe {
              gl::Enable(gl::DEBUG_OUTPUT);
              gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
              gl::DebugMessageCallback(self.m_debug_callback, std::ptr::null());
              gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, std::ptr::null(), gl::TRUE);
            }
          }
          log!("INFO", "[GlContext] -->\t Debug mode {0}",
          (debug_type != EnumRendererCallCheckingMode::None).then(|| return "enabled").unwrap_or("disabled"));
        }
        EnumRendererHint::DepthTest(enabled) => {
          if enabled {
            check_gl_call!("GlContext", gl::Enable(gl::DEPTH_TEST));
          } else {
            check_gl_call!("GlContext", gl::Disable(gl::DEPTH_TEST));
          }
          log!("INFO", "[GlContext] -->\t Depth test {0}",
          enabled.then(|| return "enabled").unwrap_or("disabled"));
        }
        EnumRendererHint::MSAA(sample_count) => {
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
          log!("INFO", "[GlContext] -->\t MSAA {0}",
          sample_count.is_some().then(|| return format!("enabled (X{0})", max_sample_count))
          .unwrap_or("disabled".to_string()));
        }
        EnumRendererHint::Blending(opt_factors) => {
          if opt_factors.is_some() {
            check_gl_call!("GlContext", gl::Enable(gl::BLEND));
          } else {
            check_gl_call!("GlContext", gl::Disable(gl::BLEND));
          }
          
          if opt_factors.is_some() {
            check_gl_call!("GlContext", gl::BlendFunc(GLenum::from(opt_factors.unwrap().0), GLenum::from(opt_factors.unwrap().1)));
          }
          
          log!("INFO", "[GlContext] -->\t Blending {0}", opt_factors.is_some()
          .then(|| return format!("enabled: Blend function -> ({0}, {1})", opt_factors.unwrap().0, opt_factors.unwrap().1))
          .unwrap_or("disabled".to_string()));
        }
        EnumRendererHint::SRGB(enabled) => {
          if enabled {
            check_gl_call!("GlContext", gl::Enable(gl::FRAMEBUFFER_SRGB));
          } else {
            check_gl_call!("GlContext", gl::Disable(gl::FRAMEBUFFER_SRGB));
          }
          log!("INFO", "[GlContext] -->\t SRGB framebuffer {0}", enabled
          .then(|| return "enabled")
          .unwrap_or("disabled"));
        }
        EnumRendererHint::CullFacing(face) => {
          if face.is_some() {
            check_gl_call!("GlContext", gl::Enable(gl::CULL_FACE));
            match face.unwrap() {
              EnumRendererCull::Front => {
                check_gl_call!("GlContext", gl::CullFace(gl::FRONT));
              }
              EnumRendererCull::Back => {
                check_gl_call!("GlContext", gl::CullFace(gl::BACK));
              }
              EnumRendererCull::FrontAndBack => {
                check_gl_call!("GlContext", gl::CullFace(gl::FRONT_AND_BACK));
              }
            }
          } else {
            check_gl_call!("GlContext", gl::Disable(gl::CULL_FACE));
          }
          log!("INFO", "[GlContext] -->\t Cull facing {0}", face.is_some()
          .then(|| return format!("enabled: {0}", face.unwrap()))
          .unwrap_or("disabled".to_string()));
        }
        EnumRendererHint::Optimization(mode) => {
          self.m_batch_mode = mode;
        }
        EnumRendererHint::SplitLargeVertexBuffers(_vertex_limit) => {}
        EnumRendererHint::SplitLargeIndexBuffers(_index_limit) => {}
      }
    }
    return Ok(());
  }
  
  fn flush(&mut self) -> Result<(), EnumRendererError> {
    self.on_render()?;
    
    self.m_commands.m_draw_commands.clear();
    self.m_commands.m_primitives.clear();
    self.m_commands.m_vao_buffers.clear();
    self.m_commands.m_vbo_buffers.clear();
    self.m_commands.m_ubo_buffers.clear();
    return Ok(());
  }
  
  fn enqueue(&mut self, r_asset: &REntity, shader_associated: &mut Shader) -> Result<(), EnumRendererError> {
    if r_asset.is_empty() {
      log!(EnumLogColor::Yellow, "WARN", "[GlContext] --> Entity [{0}] has no \
      vertices! Not sending it...", r_asset)
    }
    
    // Figure out if the entity type has already been enqueued. If so, only append to it in the vbo instead of creating another vao.
    let primitive_matched_with_shader_found = self.m_commands.m_primitives.iter()
      // Get the last one to properly offset the new primitive from the previous ones.
      .find(|primitive| primitive.m_linked_shader.m_id == shader_associated.get_id());
    
    // Check if this is the first primitive of this type. If so, alloc new buffers for it.
    if primitive_matched_with_shader_found.is_none() {
      self.alloc_buffers(r_asset, shader_associated)?;
    }
    
    let mut ibo_index = 0;
    if !self.m_commands.m_ibo_buffers.is_empty() {
      ibo_index = self.m_commands.m_ibo_buffers.len() - 1;
    }
    
    let mut vao_index = self.m_commands.m_vao_buffers.len() - 1;
    let mut vbo_index = self.m_commands.m_vbo_buffers.len() - 1;
    let mut base_vertex = 0;
    let mut base_index = 0;
    let mut last_primitive_offset = 0;
    
    // Figure out if the entity type has already been enqueued. If so, only append to it in the vbo instead of creating another vao.
    let previous_primitives_found = self.m_commands.m_primitives.iter()
      // Get the last one to properly offset the new primitive from the previous ones.
      .filter(|primitive| primitive.m_linked_shader.m_id == shader_associated.get_id() &&
        primitive.m_uuid != r_asset.get_uuid())
      .collect::<Vec<&GlPrimitiveInfo>>();
    
    if !previous_primitives_found.is_empty() {
      if let Some(primitive) = previous_primitives_found.first() {
        vao_index = primitive.m_vao_index;
        vbo_index = primitive.m_vbo_index;
        ibo_index = primitive.m_ibo_index;
        base_vertex = primitive.m_base_vertex;
        base_index = primitive.m_base_index;
        last_primitive_offset += primitive.m_entity_offset;
        
        for sub_primitive in previous_primitives_found.into_iter() {
          base_vertex += sub_primitive.m_vbo_count;
          base_index += sub_primitive.m_vbo_count;
          last_primitive_offset += sub_primitive.m_primitive_count;
        }
      }
    }
    
    let mut total_vertex_count: usize = 0;
    let mut total_index_count: usize = 0;
    
    if self.m_batch_mode == EnumRendererOptimizationMode::MinimizeVertexBuffers ||
      self.m_batch_mode == EnumRendererOptimizationMode::MinimizeDrawCalls || self.m_batch_mode == EnumRendererOptimizationMode::All {
      let main_primitive = GlPrimitiveInfo {
        m_uuid: r_asset.get_uuid(),
        m_linked_shader: GlShaderInfo {
          m_id: shader_associated.get_id(),
        },
        m_vao_index: vao_index,
        m_vbo_index: vbo_index,
        m_ibo_index: ibo_index,
        m_vbo_count: r_asset.get_total_vertex_count(),
        m_ibo_count: r_asset.get_total_index_count(),
        m_base_vertex: base_vertex,
        m_base_index: base_index,
        m_entity_offset: last_primitive_offset,
        m_primitive_count: r_asset.m_sub_meshes.len(),
        m_sub_primitive_index: 0,
        m_visible: false,
      };
      
      self.push_primitive(main_primitive);
    }
    
    for (position, sub_mesh) in r_asset.m_sub_meshes.iter().enumerate() {
      total_vertex_count += sub_mesh.get_vertices_ref().len();
      total_index_count += sub_mesh.get_indices().len();
      
      let new_sub_primitive = GlPrimitiveInfo {
        m_uuid: r_asset.get_uuid(),
        m_linked_shader: GlShaderInfo {
          m_id: shader_associated.get_id(),
        },
        m_vao_index: vao_index,
        m_vbo_index: vbo_index,
        m_ibo_index: ibo_index,
        m_vbo_count: sub_mesh.get_vertices_ref().len(),
        m_ibo_count: sub_mesh.get_indices().len(),
        m_base_vertex: base_vertex,
        m_base_index: base_index,
        m_entity_offset: last_primitive_offset,
        m_primitive_count: 1,
        m_sub_primitive_index: position,
        m_visible: false,
      };
      
      self.push_buffers(&new_sub_primitive, r_asset)?;
      
      if self.m_batch_mode == EnumRendererOptimizationMode::MinimizeIndexBuffers ||
        self.m_batch_mode == EnumRendererOptimizationMode::NoOptimizations {
        self.push_primitive(new_sub_primitive);
      }
      
      base_vertex += sub_mesh.get_vertices_ref().len();
    }
    
    // If we already have a perspective camera ubo bound, skip.
    if !self.m_commands.m_ubo_buffers.iter().any(|ubo| ubo.get_name() == Some("ubo_camera")) {
      let mut camera_ubo = GlUbo::new(Some("ubo_camera"), EnumUboTypeSize::ViewProjection, 0)?;
      
      // If glsl version is lower than 420, then we cannot bind blocks in shaders and have to encode them here instead.
      if shader_associated.get_version() < 420 {
        camera_ubo.bind_block(shader_associated.get_id(), 0)?;
      }
      self.m_commands.m_ubo_buffers.push(camera_ubo);
    }
    
    // let mut result = 0;
    // check_gl_call!("GLContext", gl::GetVertexAttribiv(0, gl::VERTEX_ATTRIB_ARRAY_BUFFER_BINDING, &mut result));
    //
    // log!(EnumLogColor::Blue, "DEBUG", "[GlContext] -->\t Current Vbo bound to Vao at index 0 : {0}", result);
    #[cfg(feature = "debug")]
    {
      let ibo_id: String;
      let ibo_len: String;
      
      if self.m_commands.m_ibo_buffers.is_empty() {
        ibo_id = String::from("N/A");
        ibo_len = String::from("N/A");
      } else {
        ibo_id = format!("{0}", self.m_commands.m_ibo_buffers.get(ibo_index).unwrap().m_buffer_id);
        ibo_len = format!("{0}", self.m_commands.m_ibo_buffers.get(ibo_index).unwrap().m_length);
      }
      
      log!(EnumLogColor::Yellow, "INFO", "[GlContext] -->\t Enqueued {0} vertices in vbo {1} and {2} indices in ibo {3}\
    \n{6:115}Total vbo size: {4}\n{6:115}Total ibo size: {5}", total_vertex_count,
        self.m_commands.m_vbo_buffers.get(vbo_index).unwrap().m_buffer_id, total_index_count, ibo_id,
        self.m_commands.m_vbo_buffers.get(vbo_index).unwrap().m_length, ibo_len, "");
    }
    return Ok(());
  }
  
  fn dequeue(&mut self, _uuid: u64) -> Result<(), EnumRendererError> {
    return Ok(());
  }
  
  fn update_ubo_camera(&mut self, view: Mat4, projection: Mat4) -> Result<(), EnumRendererError> {
    let ubo_camera_index_found = self.m_commands.m_ubo_buffers.iter_mut()
      .position(|ubo| ubo.get_name() == Some("ubo_camera"));
    
    if ubo_camera_index_found.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot update camera ubo, 'ubo_camera' not found in batch!");
      return Err(EnumRendererError::UboNotFound);
    }
    
    self.m_commands.m_ubo_buffers[ubo_camera_index_found.unwrap()].push(EnumUboType::ViewProjection(view, projection))?;
    return Ok(());
  }
  
  fn update_ubo_model(&mut self, model_transform: Mat4, entity_uuid: u64, instance_offset: Option<usize>, instance_count: usize) -> Result<(), EnumRendererError> {
    let ubo_model_index_found = self.m_commands.m_ubo_buffers.iter_mut()
      .find(|ubo| ubo.get_name() == Some("ubo_model"));
    
    if ubo_model_index_found.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot update transform ubo, ubo not found in batch!");
      return Err(EnumRendererError::UboNotFound);
    }
    
    let ubo = ubo_model_index_found.unwrap();
    if self.m_batch_mode == EnumRendererOptimizationMode::MinimizeDrawCalls || self.m_batch_mode == EnumRendererOptimizationMode::All {
      for instance_index in 0..instance_count {
        ubo.push(EnumUboType::Transform(model_transform, entity_uuid as usize + instance_index))?;
      }
      return Ok(());
    }
    
    if let Some(primitive) = self.m_commands.m_primitives.iter().find(|p| p.m_uuid == entity_uuid) {
      if instance_count == 0 {
        ubo.push(EnumUboType::Transform(model_transform, primitive.m_entity_offset + instance_offset.unwrap_or(0)))?;
        return Ok(());
      }
      for instance_index in 0..instance_count {
        ubo.push(EnumUboType::Transform(model_transform, primitive.m_entity_offset + instance_offset.unwrap_or(0) + instance_index))?;
      }
    }
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
    for ubo in self.m_commands.m_ubo_buffers.iter_mut() {
      ubo.free()?;
    };
    
    // Free vaos.
    for vao in self.m_commands.m_vao_buffers.iter_mut() {
      vao.free()?;
    };
    
    // Free vbos.
    for vbo in self.m_commands.m_vbo_buffers.iter_mut() {
      vbo.free()?;
    };
    
    // Free ibos.
    for ibo in self.m_commands.m_ibo_buffers.iter_mut() {
      ibo.free()?;
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
  
  fn toggle_solid_wireframe(&mut self, value: bool, entity_uuid: u64, instance_offset: Option<usize>, instance_count: usize) -> Result<(), EnumRendererError> {
    // Find ubo.
    let wireframe_ubo_found = self.m_commands.m_ubo_buffers.iter_mut()
      .find(|ubo| ubo.get_name() == Some("ubo_wireframe"));
    
    if wireframe_ubo_found.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot toggle wireframe mode for {0}, wireframe ubo not found!", entity_uuid);
      return Err(EnumRendererError::UboNotFound);
    }
    
    let ubo = wireframe_ubo_found.unwrap();
    
    if self.m_batch_mode == EnumRendererOptimizationMode::All || self.m_batch_mode == EnumRendererOptimizationMode::MinimizeDrawCalls {
      for instance_index in 0..instance_count {
        ubo.push(EnumUboType::Wireframe(value, entity_uuid as usize + instance_index))?;
      }
      return Ok(());
    }
    
    if let Some(primitive) = self.m_commands.m_primitives.iter().find(|p| p.m_uuid == entity_uuid) {
      if instance_count == 0 {
        ubo.push(EnumUboType::Wireframe(value, primitive.m_entity_offset + instance_offset.unwrap_or(0)))?;
        return Ok(());
      }
      for instance_index in 0..instance_count {
        let primitive_index = primitive.m_entity_offset + instance_offset.unwrap_or(0) + instance_index;
        ubo.push(EnumUboType::Wireframe(value, primitive_index))?;
      }
    }
    return Ok(());
  }
  
  fn alloc_buffers(&mut self, sendable_entity: &REntity, shader: &mut Shader) -> Result<(), EnumOpenGLError> {
    let mut new_vao = GlVao::new()?;
    let new_vbo = GlVbo::new(sendable_entity.get_size() * sendable_entity.get_total_vertex_count())?;
    
    if sendable_entity.get_total_index_count() > 0 {
      let new_ibo = GlIbo::new(size_of::<u32>() * sendable_entity.get_total_index_count())?;
      self.m_commands.m_ibo_buffers.push(new_ibo);
    }
    
    Self::set_attributes(sendable_entity, &mut new_vao)?;
    
    let mut model_ubo = GlUbo::new(Some("ubo_model"), EnumUboTypeSize::Transform(255), 1)?;
    let mut wireframe_ubo = GlUbo::new(Some("ubo_wireframe"), EnumUboTypeSize::Wireframe(255), 9)?;
    // If glsl version is lower than 420, then we cannot bind blocks in shaders and have to encode them here instead.
    if shader.get_version() < 420 && shader.get_lang() == EnumShaderLanguage::Glsl {
      model_ubo.bind_block(shader.get_id(), 1)?;
      wireframe_ubo.bind_block(shader.get_id(), 9)?;
    }
    
    self.m_commands.m_vao_buffers.push(new_vao);
    self.m_commands.m_vbo_buffers.push(new_vbo);
    self.m_commands.m_ubo_buffers.push(model_ubo);
    self.m_commands.m_ubo_buffers.push(wireframe_ubo);
    return Ok(());
  }
  
  fn push_buffers(&mut self, new_primitive: &GlPrimitiveInfo, r_asset: &REntity) -> Result<(), EnumOpenGLError> {
    if r_asset.m_sub_meshes.get(new_primitive.m_sub_primitive_index).is_none() {
      log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot push buffers for sub primitive index {0}, index out of bounds!",
        new_primitive.m_sub_primitive_index);
      
      return Err(EnumOpenGLError::InvalidSubPrimitiveIndex);
    }
    self.push_data(new_primitive, r_asset.m_sub_meshes.get(new_primitive.m_sub_primitive_index).unwrap())?;
    
    // If we had to reallocate our vbo to append more data to it, thus migrating over to a new buffer
    // and as a result, leaving our old vbo id that linked to the vao attrib array binding behind.
    // It is important to 'rebind' the vao's attrib buffer binding by re-enabling vertex attributes.
    // God OpenGL is so obscure sometimes...
    if self.m_commands.m_vbo_buffers.get_mut(new_primitive.m_vbo_index).unwrap().has_migrated() {
      Self::set_attributes(r_asset, self.m_commands.m_vao_buffers.get_mut(new_primitive.m_vao_index).unwrap())?;
    }
    
    let model_transform = r_asset.get_matrix();
    
    // Push wireframe flag.
    let ubo_wireframe = self.m_commands.m_ubo_buffers.iter_mut()
      .find(|ubo| ubo.get_name() == Some("ubo_wireframe"))
      .unwrap();
    ubo_wireframe.push(EnumUboType::Wireframe(r_asset.m_primitive_mode == EnumRendererRenderPrimitiveAs::Wireframe ||
      r_asset.m_primitive_mode == EnumRendererRenderPrimitiveAs::SolidWireframe,
      new_primitive.m_entity_offset + new_primitive.m_sub_primitive_index))?;
    
    // Push model transform.
    let ubo_model: &mut GlUbo = self.m_commands.m_ubo_buffers.iter_mut().find(|ubo| ubo.get_name() == Some("ubo_model"))
      .unwrap();
    ubo_model.push(EnumUboType::Transform(model_transform, new_primitive.m_entity_offset + new_primitive.m_sub_primitive_index))?;
    
    return Ok(());
  }
  
  fn push_data(&mut self, primitive_info: &GlPrimitiveInfo, primitive: &Box<dyn TraitPrimitive>) -> Result<(), EnumOpenGLError> {
    let vbo: &mut GlVbo = self.m_commands.m_vbo_buffers.get_mut(primitive_info.m_vbo_index).unwrap();
    
    log!("INFO", "[GlContext] -->\t Enqueuing {0}: '{1}'...", (primitive_info.m_sub_primitive_index == 1)
        .then( | | "primitive".to_string())
        .unwrap_or(format ! ("sub primitive {0}", primitive_info.m_sub_primitive_index)), primitive.get_name());
    log!("INFO", "[GlContext] -->\t Info:\n{0:115}{1}", "", primitive);
    
    let indices = primitive.get_indices();
    if self.m_batch_mode != EnumRendererOptimizationMode::NoOptimizations &&
      self.m_batch_mode != EnumRendererOptimizationMode::MinimizeVertexBuffers {
      if indices.len() > 0 {
        let ibo: &mut GlIbo = self.m_commands.m_ibo_buffers.get_mut(primitive_info.m_ibo_index).unwrap();
        
        let indices_offset = indices.iter()
          .map(|index| *index + primitive_info.m_base_index as u32)
          .collect::<Vec<u32>>();
        
        ibo.push(&indices_offset)?;
      }
    } else if indices.len() > 0 {
      let ibo: &mut GlIbo = self.m_commands.m_ibo_buffers.get_mut(primitive_info.m_ibo_index).unwrap();
      
      ibo.push(&indices)?;
    }
    vbo.push(primitive.get_vertices_ref())?;
    return Ok(());
  }
  
  fn push_primitive(&mut self, primitive_info: GlPrimitiveInfo) {
    // Check if we have the feature toggle on.
    if self.m_batch_mode == EnumRendererOptimizationMode::MinimizeDrawCalls || self.m_batch_mode == EnumRendererOptimizationMode::All {
      // Find first primitive that shares the same shader, thus the same material type.
      // Keep adding its ibo count to the base primitive to increase the draw count (glDrawElementBaseVertex).
      let matched_primitives = self.m_commands.m_primitives.iter_mut()
        .filter(|p| primitive_info.m_linked_shader.m_id == p.m_linked_shader.m_id &&
          p.m_uuid != primitive_info.m_uuid)
        .collect::<Vec<&mut GlPrimitiveInfo>>();
      
      if matched_primitives.is_empty() {
        self.m_commands.m_primitives.push(primitive_info);
        return;
      }
      
      for _ in 0..matched_primitives.len() {
        if let Some(matched_primitive) = self.m_commands.m_primitives.iter()
          .position(|p| p.m_linked_shader.m_id == primitive_info.m_linked_shader.m_id &&
            p.m_uuid != primitive_info.m_uuid) {
          self.m_commands.m_primitives.remove(matched_primitive);
        }
      }
    }
    
    if self.m_batch_mode == EnumRendererOptimizationMode::MinimizeVertexBuffers && primitive_info.m_sub_primitive_index != 0 {
      return;
    }
    self.m_commands.m_primitives.push(primitive_info);
  }
  
  fn set_attributes(entity: &REntity, vao: &mut GlVao) -> Result<(), EnumOpenGLError> {
    // Establish vao attributes.
    let mut attributes: Vec<GlVertexAttribute> = Vec::with_capacity(5);
    
    match entity.m_type {
      EnumPrimitive::Mesh(material) => {
        // IDs.
        attributes.push(GlVertexAttribute::new(EnumAttributeType::UnsignedInt(1), false,
          EnumVertexMemberOffset::EntityIDOffset as usize, 0)?);
        
        // Texture info.
        attributes.push(GlVertexAttribute::new(EnumAttributeType::Int(1), false,
          EnumVertexMemberOffset::TextureInfoOffset as usize, 0)?);
        
        // Positions.
        attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec3, false,
          EnumVertexMemberOffset::PositionOffset as usize, 0)?);
        
        // Normals.
        if material == EnumMaterial::Flat {
          attributes.push(GlVertexAttribute::new(EnumAttributeType::UnsignedInt(1), false,
            EnumVertexMemberOffset::NormalOffset as usize, 1)?);
        } else {
          attributes.push(GlVertexAttribute::new(EnumAttributeType::UnsignedInt(1), false,
            EnumVertexMemberOffset::NormalOffset as usize, 0)?);
        }
        
        // Colors.
        attributes.push(GlVertexAttribute::new(EnumAttributeType::UnsignedInt(1), false,
          EnumVertexMemberOffset::ColorOffset as usize, 0)?);
        
        // Texture coordinates.
        attributes.push(GlVertexAttribute::new(EnumAttributeType::Vec2, false,
          EnumVertexMemberOffset::TexCoordsOffset as usize, 0)?);
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