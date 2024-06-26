/*
 MIT License

 Copyright (c) 2023 Nami Reghbati

 Permission is hereby granted, free of charge, to1 any person obtaining a copy
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

/*
///////////////////////////////////   OpenGL    ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
 */

extern crate gl;

use std::mem::size_of_val;
pub(crate) use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLint, GLintptr, GLsizei, GLsizeiptr, GLuint,
  GLvoid};

use crate::check_gl_call;
#[cfg(feature = "debug")]
use crate::Engine;
use crate::graphics::open_gl::renderer::EnumOpenGLError;
use crate::math::Mat4;
use crate::S_ENGINE;
use crate::utils::macros::logger::*;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
enum EnumBufferState {
  NotCreated,
  Created,
  Bound,
  Unbound,
  Deleted,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum EnumGlBufferError {
  InvalidApi,
  InvalidBufferSize,
  InvalidCapacitySize,
  InvalidBufferOffset,
  InvalidReadBuffer,
  InvalidWriteBuffer,
  InvalidVertexAttribute,
  InvalidAttributeDivisor,
  InvalidUboTransformInstanceCount,
  InvalidVbo,
  InvalidVao,
  InvalidUbo,
  InvalidBlockBinding,
}

#[allow(unused)]
pub(crate) enum EnumAttributeType {
  UnsignedShort(i32),
  Short(i32),
  UnsignedInt(i32),
  Int(i32),
  Float(i32),
  Double(i32),
  Vec2,
  Vec3,
  Vec4,
  Mat4,
}

#[allow(unused)]
pub(crate) struct GlVertexAttribute {
  pub(crate) m_gl_type: GLenum,
  pub(crate) m_count: i32,
  pub(crate) m_buffer_size: usize,
  pub(crate) m_buffer_offset: usize,
  pub(crate) m_normalized: u8,
  pub(crate) m_attribute_divisor: u8,
}

impl GlVertexAttribute {
  pub(crate) fn new(gl_type: EnumAttributeType, should_normalize: bool, vbo_offset: usize, attribute_divisor: u8) -> Result<Self, EnumOpenGLError> {
    let mut max_attrib_div: i32 = 0;
    check_gl_call!("GlVertexAttr", gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attrib_div));
    
    if attribute_divisor > max_attrib_div as u8 {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot assign attribute divisor of {0} to \
      vertex attribute, since it exceeds the maximum vertex attributes available!", attribute_divisor);
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidAttributeDivisor));
    }
    
    return Ok(match gl_type {
      EnumAttributeType::UnsignedShort(count) => {
        GlVertexAttribute {
          m_gl_type: gl::UNSIGNED_SHORT,
          m_count: count,
          m_buffer_size: 4,
          m_buffer_offset: vbo_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Short(count) => {
        GlVertexAttribute {
          m_gl_type: gl::SHORT,
          m_count: count,
          m_buffer_size: 4,
          m_buffer_offset: 0,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::UnsignedInt(count) => {
        GlVertexAttribute {
          m_gl_type: gl::UNSIGNED_INT,
          m_count: count,
          m_buffer_size: 4,
          m_buffer_offset: vbo_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Int(count) => {
        GlVertexAttribute {
          m_gl_type: gl::INT,
          m_count: count,
          m_buffer_size: 4,
          m_buffer_offset: vbo_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Float(count) => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: count,
          m_buffer_size: 4,
          m_buffer_offset: vbo_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Double(count) => {
        GlVertexAttribute {
          m_gl_type: gl::DOUBLE,
          m_count: count,
          m_buffer_size: 8,
          m_buffer_offset: vbo_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Vec2 => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: 2,
          m_buffer_size: 4 * 2,
          m_buffer_offset: vbo_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Vec3 => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: 3,
          m_buffer_size: 4 * 3,
          m_buffer_offset: vbo_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Vec4 => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: 4,
          m_buffer_size: 4 * 4,
          m_buffer_offset: vbo_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Mat4 => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: 4 * 4,
          m_buffer_size: 4 * 4 * 4,
          m_buffer_offset: vbo_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
    });
  }
}

#[allow(unused)]
#[derive(Clone)]
pub(crate) struct GlVao {
  m_state: EnumBufferState,
  m_buffer_id: u32,
}

impl Default for GlVao {
  fn default() -> Self {
    return GlVao {
      m_state: EnumBufferState::Created,
      m_buffer_id: 0,
    };
  }
}

impl GlVao {
  pub(crate) fn new() -> Result<Self, EnumOpenGLError> {
    let mut new_vao: GLuint = 0;
    check_gl_call!("GlVao", gl::CreateVertexArrays(1, &mut new_vao));
    return Ok(GlVao {
      m_state: EnumBufferState::Created,
      m_buffer_id: new_vao,
    });
  }
  
  pub(crate) fn bind(&mut self) -> Result<(), EnumOpenGLError> {
    if self.m_state == EnumBufferState::Created || self.m_state == EnumBufferState::Unbound {
      check_gl_call!("GlVao", gl::BindVertexArray(self.m_buffer_id));
    }
    self.m_state = EnumBufferState::Bound;
    return Ok(());
  }
  
  pub(crate) fn enable_attributes(&mut self, stride: usize, attributes: Vec<GlVertexAttribute>) -> Result<(), EnumOpenGLError> {
    if attributes.is_empty() {
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidVertexAttribute));
    }
    
    let mut max_attrib_div: i32 = 0;
    check_gl_call!("Buffer (Attribute divisor)", gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attrib_div));
    
    self.bind()?;
    
    for (index, attribute) in attributes.iter().enumerate() {
      if index > max_attrib_div as usize {
        log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Vertex attribute index exceeds maximum \
        vertex attributes supported!");
        return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidVertexAttribute));
      }
      
      if attribute.m_gl_type == gl::UNSIGNED_INT || attribute.m_gl_type == gl::INT {
        check_gl_call!("OpenGL", gl::VertexAttribIPointer(index as u32, attribute.m_count,
          attribute.m_gl_type, stride as GLsizei, attribute.m_buffer_offset as *const GLvoid));
      } else {
        if attribute.m_count == 4 * 4 {
          // We are enabling a matrix location, thus 4 Vec4 due to glsl limitation in vec4 alignment.
          check_gl_call!("OpenGL", gl::VertexAttribPointer(index as u32, 4,
          attribute.m_gl_type, attribute.m_normalized, stride as GLsizei, attribute.m_buffer_offset as *const GLvoid));
          check_gl_call!("OpenGL", gl::EnableVertexAttribArray(index as u32));
          
          check_gl_call!("OpenGL", gl::VertexAttribPointer((index + 1) as u32, 4,
          attribute.m_gl_type, attribute.m_normalized, stride as GLsizei,
            (attribute.m_buffer_offset + attribute.m_count as usize) as *const GLvoid));
          check_gl_call!("OpenGL", gl::EnableVertexAttribArray((index + 1) as GLuint));
          
          check_gl_call!("OpenGL", gl::VertexAttribPointer((index + 2) as u32, 4,
          attribute.m_gl_type, attribute.m_normalized, stride as GLsizei,
            (attribute.m_buffer_offset + (attribute.m_count * 2) as usize) as *const GLvoid));
          check_gl_call!("OpenGL", gl::EnableVertexAttribArray((index + 2) as GLuint));
          
          check_gl_call!("OpenGL", gl::VertexAttribPointer((index + 3) as u32, 4,
          attribute.m_gl_type, attribute.m_normalized, stride as GLsizei,
            (attribute.m_buffer_offset + (attribute.m_count * 3) as usize) as *const GLvoid));
          check_gl_call!("OpenGL", gl::EnableVertexAttribArray((index + 3) as GLuint));
          continue;
        }
        check_gl_call!("OpenGL", gl::VertexAttribPointer(index as u32, attribute.m_count,
          attribute.m_gl_type, attribute.m_normalized, stride as GLsizei, attribute.m_buffer_offset as *const GLvoid));
      }
      check_gl_call!("OpenGL", gl::EnableVertexAttribArray(index as u32));
    }
    
    return Ok(());
  }
  
  pub(crate) fn unbind(&mut self) -> Result<(), EnumOpenGLError> {
    if self.m_state == EnumBufferState::Bound {
      check_gl_call!("GlVao", gl::BindVertexArray(0));
    }
    self.m_state = EnumBufferState::Unbound;
    return Ok(());
  }
  
  pub(crate) fn free(&mut self) -> Result<(), EnumOpenGLError> {
    self.unbind()?;
    
    if self.m_state == EnumBufferState::Deleted || self.m_state == EnumBufferState::NotCreated {
      log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Cannot delete GlVao : Already deleted \
      or not created in the first place!");
      return Ok(());
    }
    
    if gl::DeleteVertexArrays::is_loaded() {
      log!(EnumLogColor::Purple, "INFO", "[GlBuffer] -->\t Freeing GlVao...");
      check_gl_call!("GlVao", gl::DeleteVertexArrays(1, &self.m_buffer_id));
      log!(EnumLogColor::Green, "INFO", "[GlBuffer] -->\t Freed GlVao successfully");
    }
    
    self.m_state = EnumBufferState::Deleted;
    return Ok(());
  }
}

const C_VBO_SIZE_LIMIT: usize = 10_000_000;  // bytes.

#[allow(unused)]
pub(crate) struct GlVbo {
  pub(crate) m_buffer_id: u32,
  pub(crate) m_capacity: usize,
  pub(crate) m_length: usize,
  pub(crate) m_count: usize,
  pub(crate) m_type: GLenum,
  m_state: EnumBufferState,
  m_old_buffer_id: u32,
}

impl Default for GlVbo {
  fn default() -> Self {
    return Self {
      m_buffer_id: 0,
      m_state: EnumBufferState::NotCreated,
      m_capacity: 0,
      m_length: 0,
      m_count: 0,
      m_type: gl::ARRAY_BUFFER,
      m_old_buffer_id: 0,
    };
  }
}

impl GlVbo {
  pub(crate) fn new(vbo_type: GLenum, capacity: usize) -> Result<Self, EnumOpenGLError> {
    let mut new_vbo: GLuint = 0;
    
    if capacity == 0 || capacity >= C_VBO_SIZE_LIMIT {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot reserve size of {0} bytes for vbo, size is either 0 \
      or size exceeds the custom limit enforced (10 Megabytes) per Vertex buffer!", capacity);
      return Err(EnumOpenGLError::InvalidBufferOperation(EnumGlBufferError::InvalidBufferSize));
    }
    
    check_gl_call!("GlVbo", gl::CreateBuffers(1, &mut new_vbo));
    check_gl_call!("GlVbo", gl::BindBuffer(vbo_type, new_vbo));
    check_gl_call!("GlVbo", gl::BufferData(vbo_type, capacity as GLsizeiptr, std::ptr::null(), gl::DYNAMIC_DRAW));
    
    return Ok(Self {
      m_buffer_id: new_vbo,
      m_capacity: capacity,
      m_length: 0,
      m_count: 0,
      m_type: vbo_type,
      m_state: EnumBufferState::Created,
      m_old_buffer_id: 0,
    });
  }
  
  #[allow(unused)]
  pub(crate) fn resize(&mut self, to_size: usize) -> Result<(), EnumOpenGLError> {
    if to_size == 0 {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot resize vbo, size is either 0 \
      or size exceeds the custom limit enforced (5 Megabytes) per Vertex buffer!");
      return Err(EnumOpenGLError::InvalidBufferOperation(EnumGlBufferError::InvalidBufferSize));
    }
    
    if to_size == self.m_capacity {
      log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Attempted to resize with the current capacity size, \
      ignoring it...");
      return Ok(());
    }
    
    return (to_size < self.m_capacity).then(|| self.shrink(to_size)).unwrap_or(self.expand(to_size));
  }
  
  pub(crate) fn has_migrated(&self) -> bool {
    return self.m_old_buffer_id != self.m_buffer_id;
  }
  
  pub(crate) fn bind(&mut self) -> Result<(), EnumOpenGLError> {
    if self.m_state != EnumBufferState::Deleted || self.m_state != EnumBufferState::NotCreated {
      check_gl_call!("GlVbo", gl::BindBuffer(self.m_type, self.m_buffer_id));
    }
    self.m_state = EnumBufferState::Bound;
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn clear(&mut self) -> Result<(), EnumOpenGLError> {
    self.bind()?;
    
    check_gl_call!("GlVbo", gl::BufferSubData(self.m_type, 0, self.m_length as GLsizeiptr, std::ptr::null()));
    self.m_length = 0;
    
    return Ok(());
  }
  
  pub(crate) fn push<T>(&mut self, data: &Vec<T>) -> Result<(), EnumOpenGLError> {
    if data.len() == 0 {
      log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Cannot append data in vbo {0}, data is empty!", self.m_buffer_id);
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidBufferSize));
    }
    
    let vec_size = size_of_val(&data[0]) * data.len();
    let old_size: usize = self.m_length;
    
    if self.m_length + vec_size > self.m_capacity {
      log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Cannot append additional data in current vbo {0}, Vbo full, \
      expanding it...", self.m_buffer_id);
      self.expand(vec_size)?;
    }
    self.m_length += vec_size;
    self.m_count += data.len();
    
    // Set new data in new buffer.
    self.bind()?;
    check_gl_call!("GlVbo", gl::BufferSubData(self.m_type, old_size as GLsizeiptr, vec_size as GLsizeiptr,
      data.as_ptr() as *const GLvoid));
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn strip(&mut self, buffer_offset: usize, size: usize, count: usize) -> Result<(), EnumOpenGLError> {
    if size * count == 0 || size * count > self.m_length {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot strip data from vbo, size is either 0 or exceeds buffer length!");
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidBufferSize));
    }
    self.bind()?;
    if size * count == self.m_length {
      check_gl_call!("GlVbo", gl::MapBufferRange(self.m_type, buffer_offset as GLintptr, size as GLsizeiptr,
        gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_BUFFER_BIT));
    } else {
      check_gl_call!("GlVbo", gl::MapBufferRange(self.m_type, buffer_offset as GLintptr, size as GLsizeiptr,
        gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_RANGE_BIT));
    }
    check_gl_call!("GlVbo", gl::UnmapBuffer(self.m_type));
    
    self.m_length -= size;
    self.m_count -= count;
    return Ok(());
  }
  
  pub(crate) fn expand(&mut self, alloc_size: usize) -> Result<(), EnumOpenGLError> {
    if alloc_size == 0 || alloc_size + self.m_capacity > C_VBO_SIZE_LIMIT {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot resize vbo, size is either 0 \
      or size exceeds the custom limit enforced (10 Megabytes) per Vertex buffer!");
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidCapacitySize));
    }
    
    log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Expanding Vbo {0} from {1} bytes to {2} bytes...",
      self.m_buffer_id, self.m_capacity, alloc_size + self.m_capacity);
    
    check_gl_call!("GlVbo", gl::BindBuffer(gl::COPY_READ_BUFFER, self.m_buffer_id));
    
    // Create new GlVbo to fit all contents.
    let mut new_buffer: GLuint = 0;
    check_gl_call!("GlVbo", gl::CreateBuffers(1, &mut new_buffer));
    check_gl_call!("GlVbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, new_buffer));
    check_gl_call!("GlVbo", gl::BufferData(gl::COPY_WRITE_BUFFER, (alloc_size + self.m_capacity) as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    // Check if either buffers are mapped.
    let mut src_result: i32 = 0;
    let mut dest_result: i32 = 0;
    check_gl_call!("GlVbo", gl::GetBufferParameteriv(gl::COPY_READ_BUFFER, gl::BUFFER_MAPPED, &mut src_result));
    check_gl_call!("GlVbo", gl::GetBufferParameteriv(gl::COPY_WRITE_BUFFER, gl::BUFFER_MAPPED, &mut dest_result));
    
    if src_result == gl::TRUE as i32 {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot expand vbo, the source buffer to read data from for copying is mapped \
      or you forgot to bind GL_COPY_READ_BUFFER to current vbo before expanding!");
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidReadBuffer));
    }
    
    if dest_result == gl::TRUE as i32 {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot expand vbo, the destination buffer to write data to for copying is mapped \
      or you forgot to bind GL_COPY_WRITE_BUFFER to current vbo before expanding!");
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidWriteBuffer));
    }
    
    // Copy existing buffer contents up to the byte offset to new buffer.
    check_gl_call!("GlVbo", gl::CopyBufferSubData(gl::COPY_READ_BUFFER, gl::COPY_WRITE_BUFFER, 0, 0, self.m_length as GLintptr));
    
    // Swap buffers.
    self.unbind()?;
    self.m_old_buffer_id = self.m_buffer_id;
    check_gl_call!("GlVbo", gl::DeleteBuffers(1, &self.m_buffer_id));
    self.m_buffer_id = new_buffer;
    self.m_capacity += alloc_size;
    
    // Cleanup.
    check_gl_call!("GlVbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, 0));
    check_gl_call!("GlVbo", gl::BindBuffer(gl::COPY_READ_BUFFER, 0));
    
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn shrink(&mut self, dealloc_size: usize) -> Result<(), EnumOpenGLError> {
    if dealloc_size == 0 {
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidBufferSize));
    }
    
    check_gl_call!("GlVbo", gl::BindBuffer(gl::COPY_READ_BUFFER, self.m_buffer_id));
    
    // Create new GlVbo to fit all contents.
    let mut new_buffer: GLuint = 0;
    check_gl_call!("GlVbo", gl::CreateBuffers(1, &mut new_buffer));
    check_gl_call!("GlVbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, new_buffer));
    check_gl_call!("GlVbo", gl::BufferData(gl::COPY_WRITE_BUFFER, (self.m_capacity - dealloc_size) as GLsizeiptr,
      std::ptr::null(), gl::STATIC_DRAW));
    
    // Check if either buffers are mapped.
    let mut src_result: i32 = 0;
    let mut dest_result: i32 = 0;
    check_gl_call!("GlVbo", gl::GetBufferParameteriv(gl::COPY_READ_BUFFER, gl::BUFFER_MAPPED, &mut src_result));
    check_gl_call!("GlVbo", gl::GetBufferParameteriv(gl::COPY_WRITE_BUFFER, gl::BUFFER_MAPPED, &mut dest_result));
    
    if src_result == gl::TRUE as i32 {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot shrink vbo, the source buffer to read data from for copying is mapped \
      or you forgot to bind GL_COPY_READ_BUFFER to current vbo before expanding!");
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidReadBuffer));
    }
    
    if dest_result == gl::TRUE as i32 {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot shrink vbo, the destination buffer to write data to for copying is mapped \
      or you forgot to bind GL_COPY_WRITE_BUFFER to current vbo before expanding!");
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidWriteBuffer));
    }
    
    // Copy existing buffer contents up to the byte offset to new buffer.
    check_gl_call!("GlVbo", gl::CopyBufferSubData(gl::COPY_READ_BUFFER, gl::COPY_WRITE_BUFFER, 0, 0, self.m_length as GLintptr));
    
    // Swap buffers.
    self.unbind()?;
    self.m_old_buffer_id = self.m_buffer_id;
    check_gl_call!("GlVbo", gl::DeleteBuffers(1, &self.m_buffer_id));
    self.m_buffer_id = new_buffer;
    self.m_capacity -= dealloc_size;
    
    // Cleanup.
    check_gl_call!("GlVbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, 0));
    check_gl_call!("GlVbo", gl::BindBuffer(gl::COPY_READ_BUFFER, 0));
    
    return Ok(());
  }
  
  pub(crate) fn unbind(&mut self) -> Result<(), EnumOpenGLError> {
    if self.m_state != EnumBufferState::Deleted || self.m_state != EnumBufferState::NotCreated {
      check_gl_call!("GlVbo", gl::BindBuffer(self.m_type, 0));
    }
    self.m_state = EnumBufferState::Unbound;
    return Ok(());
  }
  
  #[allow(unused)]
  pub fn is_empty(&self) -> bool {
    return self.m_length == 0;
  }
  
  pub(crate) fn free(&mut self) -> Result<(), EnumOpenGLError> {
    self.unbind()?;
    
    if self.m_state == EnumBufferState::Deleted || self.m_state == EnumBufferState::NotCreated {
      log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Cannot delete GlVbo : Already deleted \
      or not created in the first place!");
      return Ok(());
    }
    
    if gl::DeleteBuffers::is_loaded() {
      log!(EnumLogColor::Purple, "INFO", "[GlBuffer] -->\t Freeing GlVbo {0}...", self.m_buffer_id);
      check_gl_call!("GlVbo", gl::DeleteBuffers(1, &self.m_buffer_id));
      log!(EnumLogColor::Green, "INFO", "[GlBuffer] -->\t Freed GlVbo successfully");
    }
    
    self.m_state = EnumBufferState::Deleted;
    return Ok(());
  }
}

const C_IBO_SIZE_LIMIT: usize = 10_000_000;  // 10 Mbs.

pub(crate) struct GlIbo {
  pub(crate) m_buffer_id: u32,
  pub(crate) m_capacity: usize,
  pub(crate) m_length: usize,
  pub(crate) m_count: usize,
  m_state: EnumBufferState,
}

impl Default for GlIbo {
  fn default() -> Self {
    return Self {
      m_buffer_id: 0,
      m_capacity: 0,
      m_length: 0,
      m_count: 0,
      m_state: EnumBufferState::NotCreated,
    };
  }
}

impl GlIbo {
  pub(crate) fn new(capacity: usize) -> Result<Self, EnumOpenGLError> {
    let mut new_ibo: GLuint = 0;
    
    if capacity == 0 || capacity > C_IBO_SIZE_LIMIT {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot reserve size of {0} bytes for ibo, size is either 0 \
      or size exceeds the custom limit enforced (10 Megabytes) per index buffer!", capacity);
      return Err(EnumOpenGLError::InvalidBufferOperation(EnumGlBufferError::InvalidBufferSize));
    }
    
    check_gl_call!("GlIbo", gl::CreateBuffers(1, &mut new_ibo));
    check_gl_call!("GlIbo", gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, new_ibo));
    check_gl_call!("GlIbo", gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, capacity as GLsizeiptr, std::ptr::null(), gl::DYNAMIC_DRAW));
    
    return Ok(Self {
      m_buffer_id: new_ibo,
      m_capacity: capacity,
      m_length: 0,
      m_count: 0,
      m_state: EnumBufferState::Created,
    });
  }
  
  pub(crate) fn bind(&mut self) -> Result<(), EnumOpenGLError> {
    if self.m_state != EnumBufferState::Deleted || self.m_state != EnumBufferState::NotCreated {
      check_gl_call!("GlIbo", gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.m_buffer_id));
    }
    self.m_state = EnumBufferState::Bound;
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn clear(&mut self) -> Result<(), EnumOpenGLError> {
    self.bind()?;
    
    check_gl_call!("GlIbo", gl::BufferSubData(gl::ELEMENT_ARRAY_BUFFER, 0, self.m_length as GLsizeiptr, std::ptr::null()));
    self.m_length = 0;
    
    return Ok(());
  }
  
  pub(crate) fn push<T>(&mut self, data: &Vec<T>) -> Result<(), EnumOpenGLError> {
    if data.len() == 0 {
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidBufferSize));
    }
    
    let vec_size = size_of_val(&data[0]) * data.len();
    let old_size: usize = self.m_length;
    
    if self.m_length + vec_size > self.m_capacity {
      log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Cannot append additional data in current ibo {0}, Ibo full, \
      expanding it...", self.m_buffer_id);
      self.expand(vec_size)?;
    }
    self.m_length += vec_size;
    self.m_count += data.len();
    
    self.bind()?;
    // Set new data in new buffer.
    check_gl_call!("GlIbo", gl::BufferSubData(gl::ELEMENT_ARRAY_BUFFER, old_size as GLsizeiptr,
      vec_size as GLsizeiptr, data.as_ptr() as *const GLvoid));
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn strip(&mut self, buffer_offset: usize, size: usize) -> Result<(), EnumOpenGLError> {
    if size == 0 || size > self.m_length {
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidBufferSize));
    }
    self.bind()?;
    if size == self.m_length {
      check_gl_call!("GlIbo", gl::MapBufferRange(gl::ELEMENT_ARRAY_BUFFER,
        buffer_offset as GLintptr, size as GLsizeiptr, gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_BUFFER_BIT));
    } else {
      check_gl_call!("GlIbo", gl::MapBufferRange(gl::ELEMENT_ARRAY_BUFFER,
        buffer_offset as GLintptr, size as GLsizeiptr, gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_RANGE_BIT));
    }
    check_gl_call!("GlIbo", gl::UnmapBuffer(gl::ELEMENT_ARRAY_BUFFER));
    
    let size_per_index = self.m_length / self.m_count;
    self.m_length -= size;
    self.m_count -= size / size_per_index;
    return Ok(());
  }
  
  pub(crate) fn expand(&mut self, alloc_size: usize) -> Result<(), EnumOpenGLError> {
    if alloc_size == 0 || alloc_size + self.m_capacity > C_IBO_SIZE_LIMIT {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot resize ibo, size is either 0 \
      or size exceeds the custom limit enforced (10 Megabytes) per Index buffer!");
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidCapacitySize));
    }
    
    log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Expanding Ibo {0} from {1} bytes to {2} bytes...",
      self.m_buffer_id, self.m_capacity, alloc_size + self.m_capacity);
    
    check_gl_call!("GlIbo", gl::BindBuffer(gl::COPY_READ_BUFFER, self.m_buffer_id));
    
    // Create new GlVbo to fit all contents.
    let mut new_buffer: GLuint = 0;
    check_gl_call!("GlIbo", gl::CreateBuffers(1, &mut new_buffer));
    check_gl_call!("GlIbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, new_buffer));
    check_gl_call!("GlIbo", gl::BufferData(gl::COPY_WRITE_BUFFER, (alloc_size + self.m_capacity) as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    // Check if either buffers are mapped.
    let mut src_result: i32 = 0;
    let mut dest_result: i32 = 0;
    check_gl_call!("GlIbo", gl::GetBufferParameteriv(gl::COPY_READ_BUFFER, gl::BUFFER_MAPPED, &mut src_result));
    check_gl_call!("GlIbo", gl::GetBufferParameteriv(gl::COPY_WRITE_BUFFER, gl::BUFFER_MAPPED, &mut dest_result));
    
    if src_result == gl::TRUE as i32 {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot expand ibo, the source buffer to read data from for copying is mapped \
      or you forgot to bind GL_COPY_READ_BUFFER to current vbo before expanding!");
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidReadBuffer));
    }
    
    if dest_result == gl::TRUE as i32 {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot expand ibo, the destination buffer to write data to for copying is mapped \
      or you forgot to bind GL_COPY_WRITE_BUFFER to current vbo before expanding!");
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidWriteBuffer));
    }
    
    // Copy existing buffer contents up to the byte offset to new buffer.
    check_gl_call!("GlIbo", gl::CopyBufferSubData(gl::COPY_READ_BUFFER, gl::COPY_WRITE_BUFFER, 0, 0, self.m_length as GLintptr));
    
    // Swap buffers.
    self.unbind()?;
    check_gl_call!("GlIbo", gl::DeleteBuffers(1, &self.m_buffer_id));
    self.m_buffer_id = new_buffer;
    self.m_capacity += alloc_size;
    
    // Cleanup.
    check_gl_call!("GlIbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, 0));
    check_gl_call!("GlIbo", gl::BindBuffer(gl::COPY_READ_BUFFER, 0));
    
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn shrink(&mut self, dealloc_size: usize) -> Result<(), EnumOpenGLError> {
    if dealloc_size == 0 {
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidBufferSize));
    }
    
    check_gl_call!("GlIbo", gl::BindBuffer(gl::COPY_READ_BUFFER, self.m_buffer_id));
    
    // Create new GlVbo to fit all contents.
    let mut new_buffer: GLuint = 0;
    check_gl_call!("GlIbo", gl::CreateBuffers(1, &mut new_buffer));
    check_gl_call!("GlIbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, new_buffer));
    check_gl_call!("GlIbo", gl::BufferData(gl::COPY_WRITE_BUFFER, (self.m_capacity - dealloc_size) as GLsizeiptr,
      std::ptr::null(), gl::STATIC_DRAW));
    
    // Check if either buffers are mapped.
    let mut src_result: i32 = 0;
    let mut dest_result: i32 = 0;
    check_gl_call!("GlIbo", gl::GetBufferParameteriv(gl::COPY_READ_BUFFER, gl::BUFFER_MAPPED, &mut src_result));
    check_gl_call!("GlIbo", gl::GetBufferParameteriv(gl::COPY_WRITE_BUFFER, gl::BUFFER_MAPPED, &mut dest_result));
    
    if src_result == gl::TRUE as i32 {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot shrink ibo, the source buffer to read data from for copying is mapped \
      or you forgot to bind GL_COPY_READ_BUFFER to current vbo before expanding!");
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidReadBuffer));
    }
    
    if dest_result == gl::TRUE as i32 {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot shrink ibo, the destination buffer to write data to for copying is mapped \
      or you forgot to bind GL_COPY_WRITE_BUFFER to current vbo before expanding!");
      return Err(EnumOpenGLError::from(EnumGlBufferError::InvalidWriteBuffer));
    }
    
    // Copy existing buffer contents up to the byte offset to new buffer.
    check_gl_call!("GlIbo", gl::CopyBufferSubData(gl::COPY_READ_BUFFER, gl::COPY_WRITE_BUFFER, 0, 0, self.m_length as GLintptr));
    
    // Swap buffers.
    check_gl_call!("GlIbo", gl::DeleteBuffers(1, &self.m_buffer_id));
    self.m_buffer_id = new_buffer;
    self.m_capacity -= dealloc_size;
    
    // Cleanup.
    check_gl_call!("GlIbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, 0));
    check_gl_call!("GlIbo", gl::BindBuffer(gl::COPY_READ_BUFFER, 0));
    
    return Ok(());
  }
  
  pub(crate) fn unbind(&mut self) -> Result<(), EnumOpenGLError> {
    if self.m_state != EnumBufferState::Deleted || self.m_state != EnumBufferState::NotCreated {
      check_gl_call!("GlIbo", gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0));
    }
    self.m_state = EnumBufferState::Unbound;
    return Ok(());
  }
  
  #[allow(unused)]
  pub fn is_empty(&self) -> bool {
    return self.m_length == 0 || self.m_count == 0;
  }
  
  pub(crate) fn free(&mut self) -> Result<(), EnumOpenGLError> {
    self.unbind()?;
    
    if self.m_state == EnumBufferState::Deleted || self.m_state == EnumBufferState::NotCreated {
      log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Cannot delete GlIbo : Already deleted \
      or not created in the first place!");
      return Ok(());
    }
    
    if gl::DeleteBuffers::is_loaded() {
      log!(EnumLogColor::Purple, "INFO", "[GlBuffer] -->\t Freeing GlIbo {0}...", self.m_buffer_id);
      check_gl_call!("GlIbo", gl::DeleteBuffers(1, &self.m_buffer_id));
      log!(EnumLogColor::Green, "INFO", "[GlBuffer] -->\t Freed GlIbo successfully");
    }
    
    self.m_state = EnumBufferState::Deleted;
    return Ok(());
  }
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EnumUboType {
  Transform(Mat4, usize),
  ViewProjection(Mat4, Mat4),
  MVP(Mat4, Mat4, Mat4),
  Wireframe(bool, usize),
}

#[allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum EnumUboTypeSize {
  Transform(usize),
  ViewProjection,
  MVP,
  Bool,
  Int,
  Uint,
  Float,
  Double,
  Long,
  Wireframe(usize),
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GlUbo {
  m_buffer_id: u32,
  m_name: Option<&'static str>,
  m_capacity: usize,
  m_length: usize,
  m_count: usize,
  m_state: EnumBufferState,
}

impl GlUbo {
  pub(crate) fn new(block_name: Option<&'static str>, ubo_type: EnumUboTypeSize, binding: u32) -> Result<Self, EnumOpenGLError> {
    let mut buffer_id = 0;
    let alloc_size: usize;
    let data_count: usize;
    
    match ubo_type {
      EnumUboTypeSize::Transform(count) => {
        alloc_size = Mat4::get_size() * count;
        data_count = count;
      }
      EnumUboTypeSize::ViewProjection => {
        alloc_size = Mat4::get_size() * 2;
        data_count = 2;
      }
      EnumUboTypeSize::MVP => {
        alloc_size = Mat4::get_size() * 3;
        data_count = 3;
      }
      EnumUboTypeSize::Wireframe(count) => {
        alloc_size = 16 * count;
        data_count = count;
      }
      _ => {
        alloc_size = 16;
        data_count = 1;
      }
    }
    check_gl_call!("GlUbo", gl::CreateBuffers(1, &mut buffer_id));
    check_gl_call!("GlUbo", gl::BindBuffer(gl::UNIFORM_BUFFER, buffer_id));
    check_gl_call!("GlUbo", gl::BufferData(gl::UNIFORM_BUFFER, alloc_size as GLsizeiptr, std::ptr::null(), gl::DYNAMIC_DRAW));
    check_gl_call!("GlUbo", gl::BindBufferBase(gl::UNIFORM_BUFFER, binding, buffer_id));
    
    return Ok(Self {
      m_buffer_id: buffer_id,
      m_name: block_name,
      m_capacity: alloc_size,
      m_length: alloc_size,
      m_count: data_count,
      m_state: EnumBufferState::Created,
    });
  }
  
  #[allow(unused)]
  pub(crate) fn get_id(&self) -> u32 {
    return self.m_buffer_id;
  }
  
  #[allow(unused)]
  pub fn len(&self) -> usize {
    return self.m_length;
  }
  
  #[allow(unused)]
  pub fn count(&self) -> usize {
    return self.m_count;
  }
  
  pub(crate) fn get_name(&self) -> Option<&str> {
    return self.m_name;
  }
  
  pub(crate) fn bind(&mut self) -> Result<(), EnumOpenGLError> {
    check_gl_call!("GlUbo", gl::BindBuffer(gl::UNIFORM_BUFFER, self.m_buffer_id));
    self.m_state = EnumBufferState::Bound;
    return Ok(());
  }
  
  pub(crate) fn bind_block(&mut self, shader_id: u32, binding: u32) -> Result<(), EnumOpenGLError> {
    if self.m_name.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot bind block for ubo, no block name associated with ubo {0}!", self.m_buffer_id);
      return Err(EnumOpenGLError::InvalidBufferOperation(EnumGlBufferError::InvalidBlockBinding));
    }
    
    if self.m_state == EnumBufferState::Created || self.m_state == EnumBufferState::Unbound {
      let mut result: i32 = 0;
      check_gl_call!("GlRenderer", gl::GetIntegerv(gl::MAX_UNIFORM_BUFFER_BINDINGS, &mut result));
      
      if result < binding as i32 {
        log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot bind Ubo, binding {0} exceeds max supported block bindings!",
          binding);
        return Err(EnumOpenGLError::InvalidBufferOperation(EnumGlBufferError::InvalidBlockBinding));
      }
      
      let mut num_blocks: i32 = 0;
      check_gl_call!("GlUbo", gl::GetProgramiv(shader_id, gl::ACTIVE_UNIFORM_BLOCKS, &mut num_blocks));
      
      if binding > num_blocks as u32 {
        log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot bind Ubo, Block index {0} exceeds block count {1} in shader {2}!",
          binding, num_blocks, shader_id);
        return Err(EnumOpenGLError::InvalidBufferOperation(EnumGlBufferError::InvalidBlockBinding));
      }
      
      let c_string = std::ffi::CString::new(self.m_name.unwrap()).expect("Cannot transform block name to C str!");
      
      let u_block: u32;
      check_gl_call!("GlUbo", u_block = gl::GetUniformBlockIndex(shader_id, c_string.as_ptr()));
      if u_block == gl::INVALID_INDEX {
        log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot bind Ubo, 'block name' {0} not found in shader!", self.m_name.unwrap());
        return Err(EnumOpenGLError::InvalidBufferOperation(EnumGlBufferError::InvalidBlockBinding));
      }
      check_gl_call!("GlUbo", gl::UniformBlockBinding(shader_id, u_block, binding));
    }
    
    self.m_state = EnumBufferState::Bound;
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn clear(&mut self) -> Result<(), EnumOpenGLError> {
    self.bind()?;
    
    check_gl_call!("GlUbo", gl::BufferSubData(gl::UNIFORM_BUFFER, 0, self.m_length as GLsizeiptr, std::ptr::null()));
    self.m_length = 0;
    
    return Ok(());
  }
  
  pub(crate) fn push(&mut self, ubo_type: EnumUboType) -> Result<(), EnumOpenGLError> {
    self.bind()?;
    match ubo_type {
      EnumUboType::Transform(transform, instance_index) => {
        if instance_index > self.m_count {
          log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot push transform data for instance {0}, instance index \
           exceeds buffer capacity!", instance_index);
          return Err(EnumOpenGLError::InvalidBufferOperation(EnumGlBufferError::InvalidBufferOffset));
        }
        // Set transform matrix.
        let instance_offset = Mat4::get_size() * instance_index;
        check_gl_call!("GlUbo", gl::BufferSubData(gl::UNIFORM_BUFFER, instance_offset as GLintptr,
          Mat4::get_size() as GLsizeiptr, transform.transpose().as_array().as_ptr() as *const std::ffi::c_void));
      }
      EnumUboType::ViewProjection(view, projection) => {
        // Set view matrix.
        check_gl_call!("GlUbo", gl::BufferSubData(gl::UNIFORM_BUFFER, 0 as GLintptr,
          Mat4::get_size() as GLsizeiptr, view.transpose().as_array().as_ptr() as *const std::ffi::c_void));
        
        // Set projection matrix.
        check_gl_call!("GlUbo", gl::BufferSubData(gl::UNIFORM_BUFFER, Mat4::get_size() as GLintptr,
          Mat4::get_size() as GLsizeiptr, projection.transpose().as_array().as_ptr() as *const std::ffi::c_void));
      }
      EnumUboType::MVP(model, view, projection) => {
        // Set Model matrix.
        check_gl_call!("GlUbo", gl::BufferSubData(gl::UNIFORM_BUFFER, 0 as GLintptr,
          Mat4::get_size() as GLsizeiptr, model.transpose().as_array().as_ptr() as *const std::ffi::c_void));
        
        // Set view matrix.
        check_gl_call!("GlUbo", gl::BufferSubData(gl::UNIFORM_BUFFER, Mat4::get_size() as GLintptr,
          Mat4::get_size() as GLsizeiptr, view.transpose().as_array().as_ptr() as *const std::ffi::c_void));
        
        // Set projection matrix.
        check_gl_call!("GlUbo", gl::BufferSubData(gl::UNIFORM_BUFFER, (Mat4::get_size() * 2) as GLintptr,
          Mat4::get_size() as GLsizeiptr, projection.transpose().as_array().as_ptr() as *const std::ffi::c_void));
      }
      EnumUboType::Wireframe(enabled, instance_index) => {
        if instance_index > self.m_count {
          log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot push wireframe data for instance {0}, instance index \
           exceeds buffer capacity!", instance_index);
          return Err(EnumOpenGLError::InvalidBufferOperation(EnumGlBufferError::InvalidBufferOffset));
        }
        let instance_offset = 16 * instance_index;  // Add offset of vec4 for std140 alignment reasons.
        
        // This step is necessary since it seems that glsl will always get 'true' from a boolean pointer even if its value is false.
        let convert_to_number = enabled.then(|| 1).unwrap_or(0);
        let c_void = &convert_to_number as *const _ as *const std::ffi::c_void;
        
        check_gl_call!("GlUbo", gl::BufferSubData(gl::UNIFORM_BUFFER, instance_offset as GLintptr, 4 as GLsizeiptr, c_void));
      }
    }
    return Ok(());
  }
  
  pub(crate) fn unbind(&mut self) -> Result<(), EnumOpenGLError> {
    if self.m_state == EnumBufferState::Bound {
      check_gl_call!("GlUbo", gl::BindBuffer(gl::UNIFORM_BUFFER, 0));
    }
    self.m_state = EnumBufferState::Unbound;
    return Ok(());
  }
  
  pub(crate) fn free(&mut self) -> Result<(), EnumOpenGLError> {
    self.unbind()?;
    
    if self.m_state == EnumBufferState::Deleted || self.m_state == EnumBufferState::NotCreated {
      log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Cannot delete GlUbo : Already deleted \
      or not created in the first place!");
      return Ok(());
    }
    
    if gl::DeleteBuffers::is_loaded() {
      log!(EnumLogColor::Purple, "INFO", "[GlBuffer] -->\t Freeing GlUbo {0}...", self.m_buffer_id);
      check_gl_call!("GlUbo", gl::DeleteBuffers(1, &self.m_buffer_id));
      log!(EnumLogColor::Green, "INFO", "[GlBuffer] -->\t Freed GlUbo successfully");
    }
    
    self.m_state = EnumBufferState::Deleted;
    return Ok(());
  }
}