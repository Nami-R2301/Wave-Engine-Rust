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

/*
///////////////////////////////////   OpenGL    ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
 */

extern crate gl;

use std::mem::size_of;
pub(crate) use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLint, GLintptr, GLsizei, GLsizeiptr, GLuint,
  GLvoid};

use crate::{check_gl_call};
use crate::utils::macros::logger::*;
use crate::assets::renderable_assets::{Vertex};
use crate::graphics::open_gl;
use crate::math::Mat4;
#[cfg(feature = "debug")]
use crate::Engine;
use crate::S_ENGINE;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
enum EnumState {
  NotCreated,
  Created,
  Bound,
  Unbound,
  Deleted,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum EnumError {
  InvalidApi,
  InvalidBufferSize,
  InvalidBufferOffset,
  InvalidVertexAttribute,
  InvalidAttributeDivisor,
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
  pub(crate) fn new(gl_type: EnumAttributeType, should_normalize: bool, vbo_offset: usize, attribute_divisor: u8) -> Result<Self, open_gl::renderer::EnumError> {
    let mut max_attrib_div: i32 = 0;
    check_gl_call!("GlVertexAttr", gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attrib_div));
    
    if attribute_divisor > max_attrib_div as u8 {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot assign attribute divisor of {0} to \
      vertex attribute, since it exceeds the maximum vertex attributes available!", attribute_divisor);
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidAttributeDivisor));
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
pub(crate) struct GlVao {
  m_state: EnumState,
  m_renderer_id: u32,
}

impl GlVao {
  pub(crate) fn new() -> Result<Self, open_gl::renderer::EnumError> {
    let mut new_vao: GLuint = 0;
    check_gl_call!("GlVao", gl::CreateVertexArrays(1, &mut new_vao));
    return Ok(GlVao {
      m_state: EnumState::Created,
      m_renderer_id: new_vao,
    });
  }
  
  pub(crate) fn bind(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    if self.m_state == EnumState::Created || self.m_state == EnumState::Unbound {
      check_gl_call!("GlVao", gl::BindVertexArray(self.m_renderer_id));
    }
    self.m_state = EnumState::Bound;
    return Ok(());
  }
  
  pub(crate) fn enable_attributes(&mut self, attributes: Vec<GlVertexAttribute>) -> Result<(), open_gl::renderer::EnumError> {
    if attributes.is_empty() {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidVertexAttribute));
    }
    
    let mut max_attrib_div: i32 = 0;
    check_gl_call!("Buffer (Attribute divisor)", gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attrib_div));
    
    self.bind()?;
    let stride = size_of::<Vertex>();
    
    for (index, attribute) in attributes.iter().enumerate() {
      if index > max_attrib_div as usize {
        log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Vertex attribute index exceeds maximum \
        vertex attributes supported!");
        return Err(open_gl::renderer::EnumError::from(EnumError::InvalidVertexAttribute));
      }
      
      if attribute.m_gl_type == gl::UNSIGNED_INT || attribute.m_gl_type == gl::INT {
        check_gl_call!("OpenGL", gl::VertexAttribIPointer(index as u32, attribute.m_count,
          attribute.m_gl_type, stride as GLsizei, attribute.m_buffer_offset as *const GLvoid));
      } else {
        check_gl_call!("OpenGL", gl::VertexAttribPointer(index as u32, attribute.m_count,
          attribute.m_gl_type, attribute.m_normalized, stride as GLsizei, attribute.m_buffer_offset as *const GLvoid));
      }
      check_gl_call!("OpenGL", gl::EnableVertexAttribArray(index as u32));
    }
    
    return Ok(());
  }
  
  pub(crate) fn unbind(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    if self.m_state == EnumState::Bound {
      check_gl_call!("GlVao", gl::BindVertexArray(0));
    }
    self.m_state = EnumState::Unbound;
    return Ok(());
  }
  
  pub(crate) fn free(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    self.unbind()?;
    
    if self.m_state == EnumState::Deleted || self.m_state == EnumState::NotCreated {
      log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Cannot delete GlVao : Already deleted \
      or not created in the first place!");
      return Ok(());
    }
    
    log!(EnumLogColor::Purple, "INFO", "[GlBuffer] -->\t Freeing GlVao...");
    check_gl_call!("GlVao", gl::DeleteVertexArrays(1, &self.m_renderer_id));
    log!(EnumLogColor::Green, "INFO", "[GlBuffer] -->\t Freed GlVao successfully");
    
    self.m_state = EnumState::Deleted;
    return Ok(());
  }
}

#[allow(unused)]
pub(crate) struct GlVbo {
  pub(crate) m_renderer_id: u32,
  pub(crate) m_capacity: usize,
  pub(crate) m_size: usize,
  pub(crate) m_count: usize,
  m_state: EnumState,
}

impl GlVbo {
  pub(crate) fn new(size_per_vertex: usize, vertex_count: usize) -> Result<Self, open_gl::renderer::EnumError> {
    let mut new_vbo: GLuint = 0;
    check_gl_call!("GlVbo", gl::CreateBuffers(1, &mut new_vbo));
    check_gl_call!("GlVbo", gl::BindBuffer(gl::ARRAY_BUFFER, new_vbo));
    check_gl_call!("GlVbo", gl::BufferData(gl::ARRAY_BUFFER, (size_per_vertex * vertex_count) as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    return Ok(GlVbo {
      m_renderer_id: new_vbo,
      m_capacity: size_per_vertex * vertex_count,
      m_size: size_per_vertex * vertex_count,
      m_count: vertex_count,
      m_state: EnumState::Created,
    });
  }
  
  pub(crate) fn bind(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    if self.m_state == EnumState::Created || self.m_state == EnumState::Unbound {
      check_gl_call!("GlVbo", gl::BindBuffer(gl::ARRAY_BUFFER, self.m_renderer_id));
    }
    self.m_state = EnumState::Bound;
    return Ok(());
  }
  
  pub(crate) fn set_data(&mut self, data: *const GLvoid, alloc_size: usize, byte_offset: usize) -> Result<(), open_gl::renderer::EnumError> {
    self.bind()?;
    check_gl_call!("GlVbo", gl::BufferSubData(gl::ARRAY_BUFFER, byte_offset as GLsizeiptr,
      alloc_size as GLsizeiptr, data));
    
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn append(&mut self, data: *const GLvoid, vertex_size: usize, vertex_count: usize) -> Result<(), open_gl::renderer::EnumError> {
    if vertex_size == 0 || vertex_count == 0 {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidBufferSize));
    }
    let old_size: usize = self.m_size;
    
    if (vertex_size * vertex_count) + self.m_size >= self.m_capacity {
      self.expand(vertex_size * vertex_count)?;
    }
    self.m_size += vertex_size * vertex_count;
    self.m_count += vertex_count;
    
    // Set new data in new buffer.
    self.set_data(data, vertex_size * vertex_count, old_size)?;
    return Ok(());
  }
  
  pub(crate) fn strip(&mut self, buffer_offset: usize, vertex_size: usize, vertex_count: usize) -> Result<(), open_gl::renderer::EnumError> {
    if vertex_size * vertex_count == 0 || vertex_size * vertex_count > self.m_size {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidBufferSize));
    }
    self.bind()?;
    if vertex_size * vertex_count == self.m_size {
      check_gl_call!("GlVbo", gl::MapBufferRange(gl::ARRAY_BUFFER,
        buffer_offset as GLintptr, (vertex_size * vertex_count) as GLsizeiptr,
        gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_BUFFER_BIT));
    } else {
      check_gl_call!("GlVbo", gl::MapBufferRange(gl::ARRAY_BUFFER,
        buffer_offset as GLintptr, (vertex_size * vertex_count) as GLsizeiptr,
        gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_RANGE_BIT));
    }
    check_gl_call!("GlVbo", gl::UnmapBuffer(gl::ARRAY_BUFFER));
    
    self.m_size -= vertex_size * vertex_count;
    self.m_count -= vertex_count;
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn expand(&mut self, alloc_size: usize) -> Result<(), open_gl::renderer::EnumError> {
    if alloc_size == 0 {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidBufferSize));
    }
    
    self.bind()?;
    // Create new GlVbo to fit all contents.
    let mut new_buffer: GLuint = 0;
    check_gl_call!("GlVbo", gl::CreateBuffers(1, &mut new_buffer));
    check_gl_call!("GlVbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, new_buffer));
    check_gl_call!("GlVbo", gl::BufferData(gl::COPY_WRITE_BUFFER, (alloc_size + self.m_capacity) as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    // Copy existing buffer contents up to the byte offset to new buffer.
    check_gl_call!("GlVbo", gl::CopyBufferSubData(gl::COPY_READ_BUFFER, gl::COPY_WRITE_BUFFER,
      0, 0, self.m_size as GLintptr));
    
    // Swap buffers.
    self.unbind()?;
    check_gl_call!("GlVbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    self.m_renderer_id = new_buffer;
    self.m_capacity += alloc_size;
    
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn shrink(&mut self, dealloc_size: usize) -> Result<(), open_gl::renderer::EnumError> {
    if dealloc_size == 0 {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidBufferSize));
    }
    
    self.bind()?;
    // Create new GlVbo to fit all contents.
    let mut new_buffer: GLuint = 0;
    check_gl_call!("GlVbo", gl::CreateBuffers(1, &mut new_buffer));
    check_gl_call!("GlVbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, new_buffer));
    check_gl_call!("GlVbo", gl::BufferData(gl::COPY_WRITE_BUFFER, (self.m_capacity - dealloc_size) as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    // Copy existing buffer contents up to the byte offset to new buffer.
    check_gl_call!("GlVbo", gl::CopyBufferSubData(gl::COPY_READ_BUFFER, gl::COPY_WRITE_BUFFER,
      0, 0, self.m_size as GLintptr));
    
    // Swap buffers.
    self.unbind()?;
    check_gl_call!("GlVbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    self.m_renderer_id = new_buffer;
    self.m_capacity -= dealloc_size;
    
    return Ok(());
  }
  
  pub(crate) fn unbind(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    if self.m_state == EnumState::Bound {
      check_gl_call!("GlVbo", gl::BindBuffer(gl::ARRAY_BUFFER, 0));
    }
    self.m_state = EnumState::Unbound;
    return Ok(());
  }
  
  pub fn is_empty(&self) -> bool {
    return self.m_size == 0 || self.m_count == 0;
  }
  
  pub(crate) fn free(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    self.unbind()?;
    
    if self.m_state == EnumState::Deleted || self.m_state == EnumState::NotCreated {
      log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Cannot delete GlVao : Already deleted \
      or not created in the first place!");
      return Ok(());
    }
    
    log!(EnumLogColor::Purple, "INFO", "[GlBuffer] -->\t Freeing GlVbo...");
    check_gl_call!("GlVbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    log!(EnumLogColor::Green, "INFO", "[GlBuffer] -->\t Freed GlVbo successfully");
    
    self.m_state = EnumState::Deleted;
    return Ok(());
  }
}

#[allow(unused)]
pub(crate) struct GlIbo {
  pub(crate) m_renderer_id: u32,
  pub(crate) m_capacity: usize,
  pub(crate) m_size: usize,
  pub(crate) m_count: usize,
  m_state: EnumState,
}

#[allow(unused)]
impl GlIbo {
  pub(crate) fn new(size_per_index: usize, index_count: usize) -> Result<Self, open_gl::renderer::EnumError> {
    let mut new_ibo: GLuint = 0;
    check_gl_call!("GlIbo", gl::CreateBuffers(1, &mut new_ibo));
    check_gl_call!("GlIbo", gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, new_ibo));
    check_gl_call!("GlIbo", gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (size_per_index * index_count) as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    return Ok(Self {
      m_renderer_id: new_ibo,
      m_capacity: size_per_index * index_count,
      m_size: size_per_index * index_count,
      m_count: index_count,
      m_state: EnumState::Created,
    });
  }
  
  pub(crate) fn bind(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    if self.m_state == EnumState::Created || self.m_state == EnumState::Unbound {
      check_gl_call!("GlIbo", gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.m_renderer_id));
    }
    self.m_state = EnumState::Bound;
    return Ok(());
  }
  
  pub(crate) fn set_data(&mut self, data: *const GLvoid, alloc_size: usize, byte_offset: usize) -> Result<(), open_gl::renderer::EnumError> {
    self.bind()?;
    check_gl_call!("GlIbo", gl::BufferSubData(gl::ELEMENT_ARRAY_BUFFER, byte_offset as GLsizeiptr,
      alloc_size as GLsizeiptr, data));
    
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn append(&mut self, data: *const GLvoid, vertex_size: usize, vertex_count: usize) -> Result<(), open_gl::renderer::EnumError> {
    if vertex_size == 0 || vertex_count == 0 {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidBufferSize));
    }
    let old_size: usize = self.m_size;
    
    if (vertex_size * vertex_count) + self.m_size >= self.m_capacity {
      self.expand(vertex_size * vertex_count)?;
    }
    self.m_size += vertex_size * vertex_count;
    self.m_count += vertex_count;
    
    // Set new data in new buffer.
    self.set_data(data, vertex_size * vertex_count, old_size)?;
    return Ok(());
  }
  
  pub(crate) fn strip(&mut self, buffer_offset: usize, vertex_size: usize, vertex_count: usize) -> Result<(), open_gl::renderer::EnumError> {
    if vertex_size * vertex_count == 0 || vertex_size * vertex_count > self.m_size {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidBufferSize));
    }
    self.bind()?;
    if vertex_size * vertex_count == self.m_size {
      check_gl_call!("GlIbo", gl::MapBufferRange(gl::ELEMENT_ARRAY_BUFFER,
        buffer_offset as GLintptr, (vertex_size * vertex_count) as GLsizeiptr,
        gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_BUFFER_BIT));
    } else {
      check_gl_call!("GlIbo", gl::MapBufferRange(gl::ELEMENT_ARRAY_BUFFER,
        buffer_offset as GLintptr, (vertex_size * vertex_count) as GLsizeiptr,
        gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_RANGE_BIT));
    }
    check_gl_call!("GlIbo", gl::UnmapBuffer(gl::ELEMENT_ARRAY_BUFFER));
    
    self.m_size -= vertex_size * vertex_count;
    self.m_count -= vertex_count;
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn expand(&mut self, alloc_size: usize) -> Result<(), open_gl::renderer::EnumError> {
    if alloc_size == 0 {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidBufferSize));
    }
    
    self.bind()?;
    // Create new GlVbo to fit all contents.
    let mut new_buffer: GLuint = 0;
    check_gl_call!("GlIbo", gl::CreateBuffers(1, &mut new_buffer));
    check_gl_call!("GlIbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, new_buffer));
    check_gl_call!("GlIbo", gl::BufferData(gl::COPY_WRITE_BUFFER, (alloc_size + self.m_capacity) as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    // Copy existing buffer contents up to the byte offset to new buffer.
    check_gl_call!("GlIbo", gl::CopyBufferSubData(gl::COPY_READ_BUFFER, gl::COPY_WRITE_BUFFER,
      0, 0, self.m_size as GLintptr));
    
    // Swap buffers.
    self.unbind()?;
    check_gl_call!("GlIbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    self.m_renderer_id = new_buffer;
    self.m_capacity += alloc_size;
    
    return Ok(());
  }
  
  #[allow(unused)]
  pub(crate) fn shrink(&mut self, dealloc_size: usize) -> Result<(), open_gl::renderer::EnumError> {
    if dealloc_size == 0 {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidBufferSize));
    }
    
    self.bind()?;
    // Create new GlVbo to fit all contents.
    let mut new_buffer: GLuint = 0;
    check_gl_call!("GlIbo", gl::CreateBuffers(1, &mut new_buffer));
    check_gl_call!("GlIbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, new_buffer));
    check_gl_call!("GlIbo", gl::BufferData(gl::COPY_WRITE_BUFFER, (self.m_capacity - dealloc_size) as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    // Copy existing buffer contents up to the byte offset to new buffer.
    check_gl_call!("GlIbo", gl::CopyBufferSubData(gl::COPY_READ_BUFFER, gl::COPY_WRITE_BUFFER,
      0, 0, self.m_size as GLintptr));
    
    // Swap buffers.
    self.unbind()?;
    check_gl_call!("GlIbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    self.m_renderer_id = new_buffer;
    self.m_capacity -= dealloc_size;
    
    return Ok(());
  }
  
  pub(crate) fn unbind(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    if self.m_state == EnumState::Bound {
      check_gl_call!("GlIbo", gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0));
    }
    self.m_state = EnumState::Unbound;
    return Ok(());
  }
  
  pub fn is_empty(&self) -> bool {
    return self.m_size == 0 || self.m_count == 0;
  }
  
  pub(crate) fn free(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    self.unbind()?;
    
    if self.m_state == EnumState::Deleted || self.m_state == EnumState::NotCreated {
      log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Cannot delete GlVao : Already deleted \
      or not created in the first place!");
      return Ok(());
    }
    
    log!(EnumLogColor::Purple, "INFO", "[GlBuffer] -->\t Freeing GlVbo...");
    check_gl_call!("GlIbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    log!(EnumLogColor::Green, "INFO", "[GlBuffer] -->\t Freed GlVbo successfully");
    
    self.m_state = EnumState::Deleted;
    return Ok(());
  }
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EnumUboType {
  Transform(Mat4),
  ViewProjection(Mat4, Mat4),
  MVP(Mat4, Mat4, Mat4),
}

#[allow(unused)]
#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum EnumUboTypeSize {
  Transform = Mat4::get_size(),
  ViewProjection = Mat4::get_size() * 2,
  MVP = Mat4::get_size() * 3,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GlUbo {
  m_buffer_id: u32,
  m_name: Option<&'static str>,
  m_size: EnumUboTypeSize,
  m_state: EnumState,
}

impl GlUbo {
  pub(crate) fn new(size: EnumUboTypeSize, block_name: Option<&'static str>, binding: u32) -> Result<Self, open_gl::renderer::EnumError> {
    let mut buffer_id = 0;
    
    check_gl_call!("GlUbo", gl::CreateBuffers(1, &mut buffer_id));
    check_gl_call!("GlUbo", gl::BindBuffer(gl::UNIFORM_BUFFER, buffer_id));
    check_gl_call!("GlUbo", gl::BufferData(gl::UNIFORM_BUFFER, size as GLsizeiptr, std::ptr::null(),
    gl::DYNAMIC_DRAW));
    check_gl_call!("GlUbo", gl::BindBufferBase(gl::UNIFORM_BUFFER, binding, buffer_id));
    
    return Ok(Self {
      m_buffer_id: buffer_id,
      m_name: block_name,
      m_size: size,
      m_state: EnumState::Created,
    });
  }
  
  #[allow(unused)]
  pub(crate) fn get_id(&self) -> u32 {
    return self.m_buffer_id;
  }
  
  pub(crate) fn get_name(&self) -> Option<&str> {
    return self.m_name;
  }
  
  pub(crate) fn bind(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    check_gl_call!("GlUbo", gl::BindBuffer(gl::UNIFORM_BUFFER, self.m_buffer_id));
    return Ok(());
  }
  
  pub(crate) fn bind_block(&mut self, shader_id: u32, binding: u32) -> Result<(), open_gl::renderer::EnumError> {
    if self.m_name.is_none() {
      log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot bind block for ubo, no block name associated with ubo {0}!", self.m_buffer_id);
      return Err(open_gl::renderer::EnumError::InvalidBufferOperation(EnumError::InvalidBlockBinding));
    }
    
    if self.m_state == EnumState::Created || self.m_state == EnumState::Unbound {
      let mut result: i32 = 0;
      check_gl_call!("GlRenderer", gl::GetIntegerv(gl::MAX_UNIFORM_BUFFER_BINDINGS, &mut result));
      
      if result < binding as i32 {
        log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot bind Ubo, binding {0} exceeds max supported block bindings!",
          binding);
        return Err(open_gl::renderer::EnumError::InvalidBufferOperation(EnumError::InvalidBlockBinding));
      }
      
      let mut num_blocks: i32 = 0;
      check_gl_call!("GlUbo", gl::GetProgramiv(shader_id, gl::ACTIVE_UNIFORM_BLOCKS, &mut num_blocks));
      
      if binding > num_blocks as u32 {
        log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot bind Ubo, Block index {0} exceeds block count {1} in shader {2}!",
          binding, num_blocks, shader_id);
        return Err(open_gl::renderer::EnumError::InvalidBufferOperation(EnumError::InvalidBlockBinding));
      }
      
      let c_string = std::ffi::CString::new(self.m_name.unwrap()).expect("Cannot transform block name to C str!");
      
      let u_block: u32;
      check_gl_call!("GlUbo", u_block = gl::GetUniformBlockIndex(shader_id, c_string.as_ptr()));
      if u_block == gl::INVALID_INDEX {
        log!(EnumLogColor::Red, "ERROR", "[GlBuffer] -->\t Cannot bind Ubo, 'block name' {0} not found in shader!", self.m_name.unwrap());
        return Err(open_gl::renderer::EnumError::InvalidBufferOperation(EnumError::InvalidBlockBinding));
      }
      check_gl_call!("GlUbo", gl::UniformBlockBinding(shader_id, u_block, binding));
    }
    
    self.m_state = EnumState::Bound;
    return Ok(());
  }
  
  pub(crate) fn set_data(&mut self, ubo_type: EnumUboType) -> Result<(), open_gl::renderer::EnumError> {
    match ubo_type {
      EnumUboType::Transform(transform) => {
        // Set transform matrix.
        check_gl_call!("GlUbo", gl::BufferSubData(gl::UNIFORM_BUFFER, 0 as GLintptr,
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
    }
    return Ok(());
  }
  
  pub(crate) fn unbind(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    if self.m_state == EnumState::Bound {
      check_gl_call!("GlUbo", gl::BindBuffer(gl::UNIFORM_BUFFER, 0));
    }
    self.m_state = EnumState::Unbound;
    return Ok(());
  }
  
  pub(crate) fn free(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    self.unbind()?;
    
    if self.m_state == EnumState::Deleted || self.m_state == EnumState::NotCreated {
      log!(EnumLogColor::Yellow, "WARN", "[GlBuffer] -->\t Cannot delete GlVao : Already deleted \
      or not created in the first place!");
      return Ok(());
    }
    
    log!(EnumLogColor::Purple, "INFO", "[GlBuffer] -->\t Freeing GlUbo...");
    check_gl_call!("GlUbo", gl::DeleteBuffers(1, &self.m_buffer_id));
    log!(EnumLogColor::Green, "INFO", "[GlBuffer] -->\t Freed GlUbo successfully");
    
    self.m_state = EnumState::Deleted;
    return Ok(());
  }
}