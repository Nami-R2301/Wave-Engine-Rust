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
use gl::types::{GLint, GLintptr, GLvoid};

use crate::{Engine, S_ENGINE};
use crate::assets::r_assets::{EnumMaterialShading, EnumPrimitiveShading, EnumVertexMemberOffset, REntity, TraitPrimitive, Vertex};
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
#[derive(Debug, Copy, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
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
#[derive(Debug, Copy, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
pub enum EnumGlElementType {
  UnsignedByte = gl::UNSIGNED_BYTE,
  UnsignedShort = gl::UNSIGNED_SHORT,
  UnsignedInt = gl::UNSIGNED_INT,
}

#[allow(unused)]
#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
pub enum EnumGlDrawCommandFunction {
  DrawArray(EnumGlPrimitiveMode, GLint, GLsizei),
  DrawElements(EnumGlPrimitiveMode, GLsizei, EnumGlElementType, *const GLvoid),
  DrawElementsBaseVertex(EnumGlPrimitiveMode, GLsizei, EnumGlElementType, *const GLvoid, GLint),
  MultiDrawArrays(EnumGlPrimitiveMode, *const GLint, *const GLsizei, GLsizei),
  MultiDrawElements(EnumGlPrimitiveMode, *const GLsizei, EnumGlElementType, *const *const GLvoid, GLsizei),
  MultiDrawElementsBaseVertex(EnumGlPrimitiveMode, *const GLsizei, EnumGlElementType, *const *const GLvoid, GLsizei, *mut GLint),
  MultiDrawArraysIndirect(EnumGlPrimitiveMode, *const GLvoid, GLsizei, GLsizei),
  MultiDrawElementsIndirect(EnumGlPrimitiveMode, EnumGlElementType, *const GLvoid, GLsizei, GLsizei),
}

impl EnumGlDrawCommandFunction {
  pub(crate) fn draw(&self) -> Result<(), EnumRendererError> {
    return match self {
      EnumGlDrawCommandFunction::DrawArray(mode, base_vertex, vertex_count) => {
        check_gl_call!("GlContext", gl::DrawArrays(*mode as GLenum, *base_vertex, *vertex_count));
        Ok(())
      }
      EnumGlDrawCommandFunction::DrawElements(mode, index_count, e_type,
        ibo_offset) => {
        check_gl_call!("GlContext", gl::DrawElements(*mode as GLenum, *index_count, *e_type as GLenum, *ibo_offset));
        Ok(())
      }
      EnumGlDrawCommandFunction::DrawElementsBaseVertex(mode, index_count, e_type,
        ibo_offset, base_vertex) => {
        check_gl_call!("GlContext", gl::DrawElementsBaseVertex(*mode as GLenum, *index_count, *e_type as GLenum, *ibo_offset,
          *base_vertex));
        Ok(())
      }
      EnumGlDrawCommandFunction::MultiDrawArrays(mode, vertex_count_array,
        vertex_offset_array, primitive_count) => {
        check_gl_call!("GlContext", gl::MultiDrawArrays(*mode as GLenum, *vertex_offset_array, *vertex_count_array, *primitive_count));
        Ok(())
      }
      EnumGlDrawCommandFunction::MultiDrawElements(mode, index_count_array,
        e_type, ibo_offset_array, primitive_count) => {
        check_gl_call!("GlContext", gl::MultiDrawElements(*mode as GLenum, *index_count_array, *e_type as GLenum,
          *ibo_offset_array, *primitive_count));
        Ok(())
      }
      EnumGlDrawCommandFunction::MultiDrawElementsBaseVertex(mode, index_count_array,
        e_type, ibo_offset_array, primitive_count, base_vertex_array) => {
        check_gl_call!("GlContext", gl::MultiDrawElementsBaseVertex(*mode as GLenum, *index_count_array, *e_type as GLenum,
          *ibo_offset_array, *primitive_count, *base_vertex_array));
        Ok(())
      }
      EnumGlDrawCommandFunction::MultiDrawArraysIndirect(mode, indirect, draw_count,
        stride) => {
        check_gl_call!("GlContext", gl::MultiDrawArraysIndirect(*mode as GLenum, *indirect, *draw_count, *stride));
        Ok(())
      }
      EnumGlDrawCommandFunction::MultiDrawElementsIndirect(mode, index_type, indirect,
        draw_count, stride) => {
        check_gl_call!("GlContext", gl::MultiDrawElementsIndirect(*mode as GLenum, *index_type as GLenum, *indirect, *draw_count, *stride));
        Ok(())
      }
    };
  }
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
        write!(f, "glDrawElementsBaseVertex({0:?}, {1}, {2:?}, {3:?}, {4})", mode, index_count, index_type,
          (*index_offset).wrapping_add(0), vertex_offset)
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
        write!(f, "glMultiDrawArraysIndirect({0:?}, {1:?}, {2}, {3})", mode, draw_command_array_pointer, draw_count, stride_between_commands)
      }
      EnumGlDrawCommandFunction::MultiDrawElementsIndirect(mode, index_type,
        draw_command_array_pointer, draw_count, stride_between_commands) => {
        write!(f, "glMultiDrawElementsIndirect({0:?}, {1:?}, {2:?}, {3}, {4})", mode, index_type, draw_command_array_pointer, draw_count,
          stride_between_commands)
      }
    };
  }
}

#[allow(unused)]
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct GlDrawArraysIndirectCommand {
  m_count: u32,
  m_instance_count: u32,
  m_first_vertex: u32,
  m_first_instance: u32,
}

#[allow(unused)]
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct GlDrawElementsIndirectCommand {
  m_count: u32,
  m_instance_count: u32,
  m_first_index: u32,
  m_first_vertex: i32,
  m_first_instance: u32,
}

#[derive(Clone)]
struct GlPrimitiveInfo {
  m_uuid: u64,
  m_ibo_offset: GLintptr,
  m_vbo_count: GLsizei,
  m_ibo_count: GLsizei,
  m_base_vertex: i32,
  m_base_index: i32,
  m_entity_offset: usize,
  m_visible: bool,  // Make primitive appear or disappear upon request from the user
}

struct GlDrawCommandInfo {
  m_linked_shader: u32,
  m_vao_index: usize,
  m_vbo_index: usize,
  m_ibo_index: usize,
  m_primitives: Vec<GlPrimitiveInfo>,
}

struct GlRendererCommands {
  m_draw_commands: Vec<GlDrawCommandInfo>,
  m_draw_command_index_count_array: Vec<GLsizei>,
  m_draw_command_index_offset_array: Vec<GLintptr>,
  m_draw_command_vertex_count_array: Vec<GLsizei>,
  m_draw_command_vertex_offset_array: Vec<i32>,
  m_draw_command_base_indices: Vec<i32>,
}

impl GlRendererCommands {
  pub fn new() -> Self {
    return GlRendererCommands {
      m_draw_commands: Vec::new(),
      m_draw_command_index_count_array: Vec::new(),
      m_draw_command_index_offset_array: Vec::new(),
      m_draw_command_vertex_count_array: Vec::new(),
      m_draw_command_vertex_offset_array: Vec::new(),
      m_draw_command_base_indices: Vec::new()
    };
  }
}

pub struct GlContext {
  pub(crate) m_ext: HashMap<String, ()>,
  pub(crate) m_state: EnumRendererState,
  pub(crate) m_version: u32,
  m_commands: GlRendererCommands,
  m_vao_buffers: Vec<GlVao>,
  m_vbo_buffers: Vec<GlVbo>,
  m_indirect_buffers: Vec<GlVbo>,
  m_ibo_buffers: Vec<GlIbo>,
  m_ubo_buffers: Vec<GlUbo>,
  m_debug_callback: gl::types::GLDEBUGPROC,
  m_batch_mode: EnumRendererOptimizationMode,
}

impl TraitContext for GlContext {
  fn new() -> Self {
    return Self {
      m_state: EnumRendererState::NotCreated,
      m_ext: HashMap::new(),
      m_commands: GlRendererCommands::new(),
      m_vao_buffers: Vec::new(),
      m_vbo_buffers: Vec::new(),
      m_indirect_buffers: Vec::new(),
      m_ibo_buffers: Vec::new(),
      m_ubo_buffers: Vec::new(),
      m_debug_callback: Some(gl_error_callback),
      m_batch_mode: EnumRendererOptimizationMode::default(),
      m_version: 460,
    };
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
      let mut previous_shader_id: i32 = -1;
      let mut previous_ibo: i32 = -1;
      
      for draw_command in self.m_commands.m_draw_commands.iter() {
        if draw_command.m_linked_shader != previous_shader_id as u32 {
          check_gl_call!("GlContext", gl::UseProgram(draw_command.m_linked_shader));
          
          self.m_vao_buffers[draw_command.m_vao_index].bind()?;
          previous_shader_id = draw_command.m_linked_shader as i32;
          
          if draw_command.m_ibo_index != previous_ibo as usize && !self.m_ibo_buffers.is_empty() {
            self.m_ibo_buffers[draw_command.m_ibo_index].bind()?;
            previous_ibo = draw_command.m_ibo_index as i32;
          }
        }
        
        let new_draw: EnumGlDrawCommandFunction;
        
        if self.m_ibo_buffers.is_empty() || self.m_ibo_buffers[draw_command.m_ibo_index].is_empty() {
          if self.m_version >= 430 && self.m_batch_mode == EnumRendererOptimizationMode::MinimizeDrawCalls {
            // Be careful to only load indirect structs from GPU instead of from client-side, since that requires compatibility profile.
            new_draw = EnumGlDrawCommandFunction::MultiDrawArraysIndirect(EnumGlPrimitiveMode::Triangle,
              std::ptr::null() as *const _,
              draw_command.m_primitives.len() as GLsizei,
              0);
            new_draw.draw()?;
            continue;
          }
          new_draw = EnumGlDrawCommandFunction::MultiDrawArrays(EnumGlPrimitiveMode::Triangle,
            self.m_commands.m_draw_command_vertex_count_array.as_ptr() as *const GLsizei,
            self.m_commands.m_draw_command_vertex_offset_array.as_ptr() as *const GLsizei,
            draw_command.m_primitives.len() as GLsizei);
          new_draw.draw()?;
          continue;
        }
        
        match self.m_batch_mode {
          EnumRendererOptimizationMode::MinimizeDrawCalls => {
            if self.m_version >= 430 {
              // Be careful to only load indirect structs from GPU instead of from client-side, since that requires compatibility profile.
              new_draw = EnumGlDrawCommandFunction::MultiDrawElementsIndirect(EnumGlPrimitiveMode::Triangle,
                EnumGlElementType::UnsignedInt,
                std::ptr::null() as *const _,
                draw_command.m_primitives.len() as GLsizei,
                0);
              new_draw.draw()?;
              continue;
            }
            
            new_draw = EnumGlDrawCommandFunction::DrawElements(EnumGlPrimitiveMode::Triangle,
              self.m_ibo_buffers[draw_command.m_ibo_index].m_count as i32,
              EnumGlElementType::UnsignedInt,
              std::ptr::null() as *const _);
          }
          EnumRendererOptimizationMode::NoOptimizations => {
            new_draw = EnumGlDrawCommandFunction::MultiDrawElementsBaseVertex(EnumGlPrimitiveMode::Triangle,
              self.m_commands.m_draw_command_index_count_array.as_ptr() as *const GLsizei,
              EnumGlElementType::UnsignedInt,
              self.m_commands.m_draw_command_index_offset_array.as_ptr() as *const *const GLvoid,
              draw_command.m_primitives.len() as GLsizei,
              self.m_commands.m_draw_command_base_indices.as_mut_ptr() as *mut GLint);
          }
        }
        
        new_draw.draw()?;
      }
    }
    return Ok(());
  }
  
  fn apply(&mut self, window: &mut Window, renderer_hints: &Vec<EnumRendererHint>) -> Result<(), EnumRendererError> {
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
    
    self.m_state = EnumRendererState::Created;
    self.m_batch_mode = EnumRendererOptimizationMode::default();
    self.m_commands = GlRendererCommands::new();
    self.m_debug_callback = Some(gl_error_callback);
    self.m_ext = hash_map;
    
    // Enable or disable features AFTER context creation since we need a context to load our openGL
    // functions.
    self.toggle_options(renderer_hints)?;
    
    let window_framebuffer_size = window.get_framebuffer_size();
    check_gl_call!("GlContext", gl::Viewport(0, 0, window_framebuffer_size.0 as i32, window_framebuffer_size.1 as i32));
    check_gl_call!("GlContext", gl::ClearColor(0.025, 0.025, 0.025, 1.0));
    
    self.m_state = EnumRendererState::Submitted;
    return Ok(());
  }
  
  fn toggle_visibility_of(&mut self, entity_uuid: u64, instance_offset: Option<usize>, instance_count: usize, visible: bool) -> Result<(), EnumRendererError> {
    let similar_commands = self.m_commands.m_draw_commands.iter_mut()
      .filter(|command|
        command.m_primitives.iter().rfind(|p| p.m_uuid == entity_uuid).is_some())
      .collect::<Vec<&mut GlDrawCommandInfo>>();
    
    if !similar_commands.is_empty() {
      // If we want all sub_primitives within the primitive to be hidden as well.
      for command in similar_commands.into_iter() {
        for primitive_index in instance_offset.unwrap_or(0)..instance_count {
          command.m_primitives[primitive_index].m_visible = visible;
        }
      }
      return Ok(());
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
  
  fn toggle_options(&mut self, renderer_hints: &Vec<EnumRendererHint>) -> Result<(), EnumRendererError> {
    for renderer_hint in renderer_hints.iter() {
      match renderer_hint {
        EnumRendererHint::ApiCallChecking(debug_type) => {
          match *debug_type {
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
          (*debug_type != EnumRendererCallCheckingMode::None).then(|| return "enabled").unwrap_or("disabled"));
        }
        EnumRendererHint::DepthTest(enabled) => {
          if *enabled {
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
          if *enabled {
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
          self.m_batch_mode = *mode;
        }
        EnumRendererHint::SplitLargeVertexBuffers(_vertex_limit) => {}
        EnumRendererHint::SplitLargeIndexBuffers(_index_limit) => {}
        EnumRendererHint::ForceApiVersion(version_requested) => {
          if *version_requested <= self.get_max_shader_version_available() as u32 {
            self.m_version = *version_requested;
            log!("INFO", "[GlContext] -->\t Forcing API version: {0}", version_requested);
          }
        }
      }
    }
    return Ok(());
  }
  
  fn flush(&mut self) -> Result<(), EnumRendererError> {
    self.on_render()?;
    
    self.m_commands.m_draw_commands.clear();
    self.m_vao_buffers.clear();
    self.m_vbo_buffers.clear();
    self.m_ubo_buffers.clear();
    return Ok(());
  }
  
  fn enqueue(&mut self, r_asset: &REntity, shader_associated: &mut Shader) -> Result<(), EnumRendererError> {
    if r_asset.is_empty() {
      log!(EnumLogColor::Yellow, "WARN", "[GlContext] --> Entity [{0}] has no \
      vertices! Not sending it...", r_asset)
    }
    
    // Figure out if the entity type has already been enqueued. If so, only append to it in the vbo instead of creating another vao.
    let primitive_matched_with_shader_found = self.m_commands.m_draw_commands.iter()
      // Get the last one to properly offset the new primitive from the previous ones.
      .find(|primitive| primitive.m_linked_shader == shader_associated.get_id());
    
    // Check if this is the first primitive of this type. If so, alloc new buffers for it.
    if primitive_matched_with_shader_found.is_none() {
      self.alloc_buffers(r_asset, shader_associated)?;
    }
    
    let mut ibo_index = 0;
    if !self.m_ibo_buffers.is_empty() {
      ibo_index = self.m_ibo_buffers.len() - 1;
    }
    
    let mut vao_index = self.m_vao_buffers.len() - 1;
    let mut vbo_index = self.m_vbo_buffers.len() - 1;
    let mut base_vertex: i32 = 0;
    let mut base_index: i32 = 0;
    let mut ibo_offset = 0;
    let mut last_primitive_offset = 0;
    
    // Figure out if the entity type has already been enqueued. If so, only append to it in the vbo instead of creating another vao.
    let previous_similar_entities = self.m_commands.m_draw_commands.iter()
      // Get the last one to properly offset the new primitive from the previous ones.
      .rfind(|command| command.m_linked_shader == shader_associated.get_id() &&
        command.m_primitives.iter().any(|p| p.m_uuid != r_asset.get_uuid()));
    
    if let Some(command) = previous_similar_entities {
      vao_index = command.m_vao_index;
      vbo_index = command.m_vbo_index;
      ibo_index = command.m_ibo_index;
      last_primitive_offset = command.m_primitives.len();
      
      if let Some(last_primitive) = command.m_primitives.last() {
        ibo_offset = last_primitive.m_ibo_offset + (last_primitive.m_ibo_count * size_of::<u32>() as i32) as GLintptr;
      }
      
      for primitive in command.m_primitives.iter() {
        base_vertex += primitive.m_vbo_count;
        base_index += primitive.m_vbo_count;
      }
    }
    
    let mut total_vertex_count: usize = 0;
    let mut total_index_count: usize = 0;
    
    let mut command = GlDrawCommandInfo {
      m_linked_shader: shader_associated.get_id(),
      m_vao_index: vao_index,
      m_vbo_index: vbo_index,
      m_ibo_index: ibo_index,
      m_primitives: Vec::with_capacity(r_asset.get_primitive_count()),
    };
    
    let transform = r_asset.get_matrix();
    for (position, sub_mesh) in r_asset.m_sub_meshes.iter().enumerate() {
      total_vertex_count += sub_mesh.get_vertices_ref().len();
      total_index_count += sub_mesh.get_indices().len();
      
      let new_primitive = GlPrimitiveInfo {
        m_uuid: r_asset.get_uuid(),
        m_ibo_offset: ibo_offset,
        m_vbo_count: sub_mesh.get_vertices_ref().len() as i32,
        m_ibo_count: sub_mesh.get_indices().len() as i32,
        m_base_vertex: base_vertex,
        m_base_index: base_index,
        m_entity_offset: last_primitive_offset + position,
        m_visible: false,
      };
      
      self.push_buffers(&new_primitive, vao_index, vbo_index, ibo_index, sub_mesh, transform)?;
      command.m_primitives.push(new_primitive);
      
      base_vertex += sub_mesh.get_vertices_ref().len() as i32;
      ibo_offset += (sub_mesh.get_indices().len() * size_of::<u32>()) as isize;
    }
    
    self.push_command(command)?;
    
    // If we already have a perspective camera ubo bound, skip.
    if !self.m_ubo_buffers.iter().any(|ubo| ubo.get_name() == Some("ubo_camera")) {
      let mut camera_ubo = GlUbo::new(Some("ubo_camera"), EnumUboTypeSize::ViewProjection, 0)?;
      
      // If glsl version is lower than 420, then we cannot bind blocks in shaders and have to encode them here instead.
      if shader_associated.get_version() < 420 {
        camera_ubo.bind_block(shader_associated.get_id(), 0)?;
      }
      self.m_ubo_buffers.push(camera_ubo);
    }
    
    // let mut result = 0;
    // check_gl_call!("GLContext", gl::GetVertexAttribiv(0, gl::VERTEX_ATTRIB_ARRAY_BUFFER_BINDING, &mut result));
    //
    // log!(EnumLogColor::Blue, "DEBUG", "[GlContext] -->\t Current Vbo bound to Vao at index 0 : {0}", result);
    #[cfg(feature = "debug")]
    {
      let ibo_id: String;
      let ibo_len: String;
      
      if self.m_ibo_buffers.is_empty() {
        ibo_id = String::from("N/A");
        ibo_len = String::from("N/A");
      } else {
        ibo_id = format!("{0}", self.m_ibo_buffers.get(ibo_index).unwrap().m_buffer_id);
        ibo_len = format!("{0}", self.m_ibo_buffers.get(ibo_index).unwrap().m_length);
      }
      
      log!(EnumLogColor::Yellow, "INFO", "[GlContext] -->\t Enqueued {0} vertices in vbo {1} and {2} indices in ibo {3}\
    \n{6:115}Total vbo size: {4}\n{6:115}Total ibo size: {5}", total_vertex_count,
        self.m_vbo_buffers.get(vbo_index).unwrap().m_buffer_id, total_index_count, ibo_id,
        self.m_vbo_buffers.get(vbo_index).unwrap().m_length, ibo_len, "");
    }
    
    return Ok(());
  }
  
  fn dequeue(&mut self, _uuid: u64) -> Result<(), EnumRendererError> {
    return Ok(());
  }
  
  fn update_ubo_camera(&mut self, view: Mat4, projection: Mat4) -> Result<(), EnumRendererError> {
    let ubo_camera_index_found = self.m_ubo_buffers.iter_mut()
      .position(|ubo| ubo.get_name() == Some("ubo_camera"));
    
    if ubo_camera_index_found.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot update camera ubo, 'ubo_camera' not found in batch!");
      return Err(EnumRendererError::UboNotFound);
    }
    
    self.m_ubo_buffers[ubo_camera_index_found.unwrap()].push(EnumUboType::ViewProjection(view, projection))?;
    return Ok(());
  }
  
  fn update_ubo_model(&mut self, model_transform: Mat4, entity_uuid: u64, instance_offset: Option<usize>, instance_count: usize) -> Result<(), EnumRendererError> {
    let ubo_model_index_found = self.m_ubo_buffers.iter_mut()
      .find(|ubo| ubo.get_name() == Some("ubo_model"));
    
    if ubo_model_index_found.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot update transform ubo, ubo not found in batch!");
      return Err(EnumRendererError::UboNotFound);
    }
    
    let ubo = ubo_model_index_found.unwrap();
    
    for instance_index in instance_offset.unwrap_or(0)..instance_count {
      ubo.push(EnumUboType::Transform(model_transform, entity_uuid as usize + instance_index))?;
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
    for ubo in self.m_ubo_buffers.iter_mut() {
      ubo.free()?;
    };
    
    // Free vaos.
    for vao in self.m_vao_buffers.iter_mut() {
      vao.free()?;
    };
    
    // Free vbos.
    for vbo in self.m_vbo_buffers.iter_mut() {
      vbo.free()?;
    };
    
    // Free indirect buffers if supported.
    for indirect_buffer in self.m_indirect_buffers.iter_mut() {
      indirect_buffer.free()?;
    }
    
    // Free ibos.
    for ibo in self.m_ibo_buffers.iter_mut() {
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
    let wireframe_ubo_found = self.m_ubo_buffers.iter_mut()
      .find(|ubo| ubo.get_name() == Some("ubo_wireframe"));
    
    if wireframe_ubo_found.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[GlContext] -->\t Cannot toggle wireframe mode for {0}, wireframe ubo not found!", entity_uuid);
      return Err(EnumRendererError::UboNotFound);
    }
    
    let ubo = wireframe_ubo_found.unwrap();
    
    for instance_index in instance_offset.unwrap_or(0)..instance_count {
      ubo.push(EnumUboType::Wireframe(value, entity_uuid as usize + instance_index))?;
    }
    return Ok(());
  }
  
  fn alloc_buffers(&mut self, sendable_entity: &REntity, shader: &mut Shader) -> Result<(), EnumOpenGLError> {
    let mut new_vao = GlVao::new()?;
    let new_vbo = GlVbo::new(gl::ARRAY_BUFFER, sendable_entity.get_size() * sendable_entity.get_total_vertex_count())?;
    
    if sendable_entity.get_total_index_count() > 0 {
      let new_ibo = GlIbo::new(size_of::<u32>() * sendable_entity.get_total_index_count())?;
      self.m_ibo_buffers.push(new_ibo);
    }
    
    Self::set_attributes(&sendable_entity.m_type, &mut new_vao)?;
    
    let mut model_ubo = GlUbo::new(Some("ubo_model"), EnumUboTypeSize::Transform(255), 1)?;
    let mut wireframe_ubo = GlUbo::new(Some("ubo_wireframe"), EnumUboTypeSize::Wireframe(255), 9)?;
    // If glsl version is lower than 420, then we cannot bind blocks in shaders and have to encode them here instead.
    if shader.get_version() < 420 && shader.get_lang() == EnumShaderLanguage::Glsl {
      model_ubo.bind_block(shader.get_id(), 1)?;
      wireframe_ubo.bind_block(shader.get_id(), 9)?;
    }
    
    self.m_vao_buffers.push(new_vao);
    self.m_vbo_buffers.push(new_vbo);
    self.m_ubo_buffers.push(model_ubo);
    self.m_ubo_buffers.push(wireframe_ubo);
    return Ok(());
  }
  
  fn push_buffers(&mut self, new_primitive: &GlPrimitiveInfo, vao_index: usize, vbo_index: usize, ibo_index: usize,
                  primitive: &Box<dyn TraitPrimitive>, transform_matrix: Mat4) -> Result<(), EnumOpenGLError> {
    self.push_data(new_primitive, vbo_index, ibo_index, primitive)?;
    
    // If we had to reallocate our vbo to append more data to it, thus migrating over to a new buffer
    // and as a result, leaving our old vbo id that linked to the vao attrib array binding behind.
    // It is important to 'rebind' the vao's attrib buffer binding by re-enabling vertex attributes.
    // God OpenGL is so obscure sometimes...
    if self.m_vbo_buffers.get_mut(vbo_index).unwrap().has_migrated() {
      Self::set_attributes(&primitive.get_type(), self.m_vao_buffers.get_mut(vao_index).unwrap())?;
    }
    
    // Push wireframe flag.
    let ubo_wireframe = self.m_ubo_buffers.iter_mut()
      .find(|ubo| ubo.get_name() == Some("ubo_wireframe"))
      .unwrap();
    ubo_wireframe.push(EnumUboType::Wireframe(false, new_primitive.m_entity_offset))?;
    
    // Push model transform.
    let ubo_model: &mut GlUbo = self.m_ubo_buffers.iter_mut().find(|ubo| ubo.get_name() == Some("ubo_model"))
      .unwrap();
    ubo_model.push(EnumUboType::Transform(transform_matrix, new_primitive.m_entity_offset))?;
    
    return Ok(());
  }
  
  fn push_data(&mut self, primitive_info: &GlPrimitiveInfo, vbo_index: usize, ibo_index: usize, primitive: &Box<dyn TraitPrimitive>) -> Result<(), EnumOpenGLError> {
    let vbo: &mut GlVbo = self.m_vbo_buffers.get_mut(vbo_index).unwrap();
    
    log!("INFO", "[GlContext] -->\t Enqueuing {0}: '{1}'...", primitive_info.m_entity_offset, primitive.get_name());
    log!("INFO", "[GlContext] -->\t Info:\n{0:115}{1}", "", primitive);
    
    let indices = primitive.get_indices();
    if self.m_batch_mode == EnumRendererOptimizationMode::MinimizeDrawCalls {
      if indices.len() > 0 {
        let ibo: &mut GlIbo = self.m_ibo_buffers.get_mut(ibo_index).unwrap();
        
        let indices_offset = indices.iter()
          .map(|index| *index + primitive_info.m_base_index as u32)
          .collect::<Vec<u32>>();
        
        ibo.push(&indices_offset)?;
      }
    } else if indices.len() > 0 {
      let ibo: &mut GlIbo = self.m_ibo_buffers.get_mut(ibo_index).unwrap();
      
      ibo.push(&indices)?;
    }
    vbo.push(primitive.get_vertices_ref())?;
    return Ok(());
  }
  
  fn push_command_data(&mut self, command: &GlDrawCommandInfo) -> Result<(), EnumRendererError> {
    let indirect_draw_buffer: Option<&mut GlVbo>;
    let mut contains_indices = false;
    let mut indirect_arrays_commands_vec = Vec::with_capacity(command.m_primitives.len());
    let mut indirect_elements_commands_vec = Vec::with_capacity(command.m_primitives.len());
    
    if self.m_indirect_buffers.is_empty() && self.m_version >= 430 &&
      self.m_batch_mode == EnumRendererOptimizationMode::MinimizeDrawCalls {
      if command.m_primitives.iter().any(|p| p.m_ibo_count > 0) {
        contains_indices = true;
        self.m_indirect_buffers.push(GlVbo::new(gl::DRAW_INDIRECT_BUFFER, size_of::<GlDrawElementsIndirectCommand>())?)
      } else {
        self.m_indirect_buffers.push(GlVbo::new(gl::DRAW_INDIRECT_BUFFER, size_of::<GlDrawArraysIndirectCommand>())?)
      }
    }
    
    if self.m_version >= 430 && self.m_batch_mode == EnumRendererOptimizationMode::MinimizeDrawCalls {
      indirect_draw_buffer = Some(self.m_indirect_buffers.last_mut().unwrap());
    } else {
      indirect_draw_buffer = None;
    }
    
    // Setup multi-draw components.
    for primitive_index in 0..command.m_primitives.len() {
      if command.m_primitives[primitive_index].m_ibo_count == 0 {
        if self.m_version >= 430 && self.m_batch_mode == EnumRendererOptimizationMode::MinimizeDrawCalls {
          let new_gl_indirect_command = GlDrawArraysIndirectCommand {
            m_count: command.m_primitives[primitive_index].m_vbo_count as u32,
            m_instance_count: 1,
            m_first_vertex: command.m_primitives[primitive_index].m_base_vertex as u32,
            m_first_instance: 0,
          };
          indirect_arrays_commands_vec.push(new_gl_indirect_command);
          continue;
        }
        
        self.m_commands.m_draw_command_vertex_count_array.push(command.m_primitives[primitive_index].m_vbo_count as GLsizei);
        self.m_commands.m_draw_command_vertex_offset_array.push(command.m_primitives[primitive_index].m_base_vertex);
        continue;
      }
      
      contains_indices = true;
      if self.m_version >= 430 && self.m_batch_mode == EnumRendererOptimizationMode::MinimizeDrawCalls {
        let new_gl_indirect_command = GlDrawElementsIndirectCommand {
            m_count: command.m_primitives[primitive_index].m_ibo_count as u32,
            m_instance_count: 1,
            m_first_index: (command.m_primitives[primitive_index].m_ibo_offset as usize / size_of::<u32>()) as u32,
            m_first_vertex: 0,
            m_first_instance: 0,
          };
        indirect_elements_commands_vec.push(new_gl_indirect_command);
        continue;
      }
      
      self.m_commands.m_draw_command_index_count_array.push(command.m_primitives[primitive_index].m_ibo_count);
      self.m_commands.m_draw_command_index_offset_array.push(command.m_primitives[primitive_index].m_ibo_offset as GLintptr);
      self.m_commands.m_draw_command_base_indices.push(command.m_primitives[primitive_index].m_base_index);
    }
    
    if contains_indices {
      if let Some(buffer) = indirect_draw_buffer {
        buffer.push(&indirect_elements_commands_vec)?;
      }
      return Ok(());
    }
    
    if let Some(buffer) = indirect_draw_buffer {
      buffer.push(&indirect_arrays_commands_vec)?;
    }
    return Ok(());
  }
  
  fn push_command(&mut self, command: GlDrawCommandInfo) -> Result<(), EnumRendererError> {
    if let Some(previous_command) = self.m_commands.m_draw_commands.iter_mut()
      .rfind(|c| c.m_linked_shader == command.m_linked_shader) {
      previous_command.m_primitives.append(&mut command.m_primitives.clone());
      self.push_command_data(&command)?;
      return Ok(());
    }
    
    self.push_command_data(&command)?;
    self.m_commands.m_draw_commands.push(command);
    return Ok(());
  }
  
  fn set_attributes(entity_shading_type: &EnumPrimitiveShading, vao: &mut GlVao) -> Result<(), EnumOpenGLError> {
    // Establish vao attributes.
    let mut attributes: Vec<GlVertexAttribute> = Vec::with_capacity(5);
    let size;
    
    match entity_shading_type {
      EnumPrimitiveShading::Mesh(material) => {
        size = size_of::<Vertex>();
        
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
        if *material == EnumMaterialShading::Flat {
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
    return vao.enable_attributes(size, attributes);
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