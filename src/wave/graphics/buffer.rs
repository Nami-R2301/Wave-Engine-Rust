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
///////////////////////////////////   Vulkan    ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
 */



/*
///////////////////////////////////   OpenGL    ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
 */

#[cfg(feature = "OpenGL")]
extern crate gl;

#[cfg(feature = "OpenGL")]
use crate::{log, check_gl_call};

#[cfg(feature = "OpenGL")]
use crate::wave::graphics::renderer::{EnumApi, EnumState, EnumErrors, Renderer};

#[cfg(feature = "OpenGL")]
pub use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};

#[cfg(feature = "OpenGL")]
use gl::types::GLintptr;

#[cfg(feature = "OpenGL")]
pub enum EnumAttributeType {
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

#[cfg(feature = "OpenGL")]
pub struct GlVertexAttribute {
  pub m_gl_type: GLenum,
  pub m_count: i32,
  pub m_buffer_offset: usize,
  pub m_normalized: u8,
  pub m_attribute_divisor: u8
}

#[cfg(feature = "OpenGL")]
impl GlVertexAttribute {
  pub fn new(gl_type: EnumAttributeType, should_normalize: bool, buffer_offset: usize, attribute_divisor: u8) -> Result<Self, EnumErrors> {
    
    let mut max_attrib_div: i32 = 0;
    check_gl_call!("Buffer (Attribute divisor)", gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attrib_div));
    
    if attribute_divisor > max_attrib_div as u8 {
      log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Cannot assign attribute divisor of {0} to \
      vertex attribute, since it exceeds the maximum vertex attributes available!", attribute_divisor);
      return Err(EnumErrors::InvalidAttributeDivisor);
    }
    
    return Ok(match gl_type {
      EnumAttributeType::UnsignedShort(count) => {
        GlVertexAttribute {
          m_gl_type: gl::UNSIGNED_SHORT,
          m_count: count,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor
        }
      }
      EnumAttributeType::Short(count) => {
        GlVertexAttribute {
          m_gl_type: gl::SHORT,
          m_count: count,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor
        }
      }
      EnumAttributeType::UnsignedInt(count) => {
        GlVertexAttribute {
          m_gl_type: gl::UNSIGNED_INT,
          m_count: count,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor
        }
      }
      EnumAttributeType::Int(count) => {
        GlVertexAttribute {
          m_gl_type: gl::INT,
          m_count: count,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor
        }
      }
      EnumAttributeType::Float(count) => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: count,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor
        }
      }
      EnumAttributeType::Double(count) => {
        GlVertexAttribute {
          m_gl_type: gl::DOUBLE,
          m_count: count,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor
        }
      }
      EnumAttributeType::Vec2 => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: 2,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor
        }
      }
      EnumAttributeType::Vec3 => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: 3,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor
        }
      }
      EnumAttributeType::Vec4 => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: 4,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor
        }
      }
      EnumAttributeType::Mat4 => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: 4 * 4,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor
        }
      }
    });
  }
}

#[cfg(feature = "OpenGL")]
pub struct GlVao {
  m_renderer_id: u32,
}

#[cfg(feature = "OpenGL")]
impl GlVao {
  pub fn new() -> Result<Self, EnumErrors> {
    let mut new_vao: GLuint = 0;
    check_gl_call!("Vao", gl::CreateVertexArrays(1, &mut new_vao));
    return Ok(GlVao {
      m_renderer_id: new_vao,
    });
  }
  
  pub fn delete(&mut self) -> Result<(), EnumErrors> {
    self.unbind()?;
    check_gl_call!("Vao", gl::DeleteVertexArrays(1, &self.m_renderer_id));
    return Ok(());
  }
  
  pub fn bind(&mut self) -> Result<(), EnumErrors> {
    check_gl_call!("Vao", gl::BindVertexArray(self.m_renderer_id));
    return Ok(());
  }
  
  pub fn unbind(&mut self) -> Result<(), EnumErrors> {
    check_gl_call!("Vao", gl::BindVertexArray(0));
    return Ok(());
  }
  
  pub fn enable_attributes(&mut self, attributes: Vec<GlVertexAttribute>) -> Result<(), EnumErrors> {
    if attributes.is_empty() {
      return Err(EnumErrors::InvalidVertexAttribute);
    }
    
    let mut max_attrib_div: i32 = 0;
    check_gl_call!("Buffer (Attribute divisor)", gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attrib_div));
    
    self.bind()?;
    for (index, attribute) in attributes.iter().enumerate() {
      if index > max_attrib_div as usize {
        log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Vertex attribute index exceeds maximum \
        vertex attributes supported!");
        return Err(EnumErrors::InvalidVertexAttribute);
      }
      
      if attribute.m_gl_type == gl::UNSIGNED_INT || attribute.m_gl_type == gl::INT {
        check_gl_call!("Renderer", gl::VertexAttribIPointer(index as u32, attribute.m_count,
          attribute.m_gl_type, 0, attribute.m_buffer_offset as *const GLvoid));
      } else {
        check_gl_call!("Renderer", gl::VertexAttribPointer(index as u32, attribute.m_count,
          attribute.m_gl_type, attribute.m_normalized, 0, attribute.m_buffer_offset as *const GLvoid));
      }
      check_gl_call!("Renderer", gl::EnableVertexAttribArray(index as u32));
    }
    
    return Ok(());
  }
}

#[cfg(feature = "OpenGL")]
impl Drop for GlVao {
  fn drop(&mut self) {
    unsafe {
      let renderer = Renderer::get().as_ref()
        .expect("[Buffer] -->\t Cannot drop Vao, renderer is null! Exiting...");
      if renderer.m_api.get_type() == EnumApi::OpenGL && renderer.get_state() != EnumState::Shutdown {
        gl::DeleteVertexArrays(1, &self.m_renderer_id);
      }
    }
  }
}

#[cfg(feature = "OpenGL")]
pub struct GlVbo {
  pub m_renderer_id: u32,
  pub m_capacity: usize,
  pub m_size: usize,
  pub m_count: usize,
}

#[cfg(feature = "OpenGL")]
impl GlVbo {
  pub fn new(alloc_size: usize, vertex_count: usize) -> Result<Self, EnumErrors> {
    let mut new_vbo: GLuint = 0;
    check_gl_call!("Vbo", gl::CreateBuffers(1, &mut new_vbo));
    check_gl_call!("Vbo", gl::BindBuffer(gl::ARRAY_BUFFER, new_vbo));
    check_gl_call!("Vbo", gl::BufferData(gl::ARRAY_BUFFER, (alloc_size) as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    return Ok(GlVbo {
      m_renderer_id: new_vbo,
      m_capacity: alloc_size,
      m_size: alloc_size,
      m_count: vertex_count,
    });
  }
  
  pub fn delete(&mut self) -> Result<(), EnumErrors> {
    self.unbind()?;
    check_gl_call!("Vbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    return Ok(());
  }
  
  pub fn bind(&mut self) -> Result<(), EnumErrors> {
    check_gl_call!("Vbo", gl::BindBuffer(gl::ARRAY_BUFFER, self.m_renderer_id));
    return Ok(());
  }
  
  pub fn unbind(&self) -> Result<(), EnumErrors> {
    check_gl_call!("Vbo", gl::BindBuffer(gl::ARRAY_BUFFER, 0));
    return Ok(());
  }
  
  pub fn set_data(&mut self, data: *const GLvoid, alloc_size: usize, byte_offset: usize) -> Result<(), EnumErrors> {
    self.bind()?;
    check_gl_call!("Vbo", gl::BufferSubData(gl::ARRAY_BUFFER, byte_offset as GLsizeiptr,
      alloc_size as GLsizeiptr, data));
    
    return Ok(());
  }
  
  pub fn append(&mut self, data: *const GLvoid, vertex_size: usize, vertex_count: usize) -> Result<(), EnumErrors> {
    if vertex_size == 0 || vertex_count == 0 {
      return Err(EnumErrors::InvalidBufferSize);
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
  
  pub fn strip(&mut self, buffer_offset: usize, vertex_size: usize, vertex_count: usize) -> Result<(), EnumErrors> {
    if vertex_size * vertex_count == 0 || vertex_size * vertex_count > self.m_size {
      return Err(EnumErrors::InvalidBufferSize);
    }
    self.bind()?;
    if vertex_size * vertex_count == self.m_size {
      check_gl_call!("Vbo", gl::MapBufferRange(gl::ARRAY_BUFFER,
        buffer_offset as GLintptr, (vertex_size * vertex_count) as GLsizeiptr,
        gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_BUFFER_BIT));
    } else {
      check_gl_call!("Vbo", gl::MapBufferRange(gl::ARRAY_BUFFER,
        buffer_offset as GLintptr, (vertex_size * vertex_count) as GLsizeiptr,
        gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_RANGE_BIT));
    }
    check_gl_call!("Vbo", gl::UnmapBuffer(gl::ARRAY_BUFFER));
    
    self.m_size -= vertex_size * vertex_count;
    self.m_count -= vertex_count;
    return Ok(());
  }
  
  pub fn expand(&mut self, alloc_size: usize) -> Result<(), EnumErrors> {
    if alloc_size == 0 {
      return Err(EnumErrors::InvalidBufferSize);
    }
    
    self.bind()?;
    // Create new vbo to fit all contents.
    let mut new_buffer: GLuint = 0;
    check_gl_call!("Vbo", gl::CreateBuffers(1, &mut new_buffer));
    check_gl_call!("Vbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, new_buffer));
    check_gl_call!("Vbo", gl::BufferData(gl::COPY_WRITE_BUFFER, (alloc_size + self.m_capacity) as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    // Copy existing buffer contents up to the byte offset to new buffer.
    check_gl_call!("Vbo", gl::CopyBufferSubData(gl::COPY_READ_BUFFER, gl::COPY_WRITE_BUFFER,
      0, 0, self.m_size as GLintptr));
    
    // Swap buffers.
    self.unbind()?;
    check_gl_call!("Vbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    self.m_renderer_id = new_buffer;
    self.m_capacity += alloc_size;
    
    return Ok(());
  }
  
  pub fn shrink(&mut self, dealloc_size: usize) -> Result<(), EnumErrors> {
    if dealloc_size == 0 {
      return Err(EnumErrors::InvalidBufferSize);
    }
    
    self.bind()?;
    // Create new vbo to fit all contents.
    let mut new_buffer: GLuint = 0;
    check_gl_call!("Vbo", gl::CreateBuffers(1, &mut new_buffer));
    check_gl_call!("Vbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, new_buffer));
    check_gl_call!("Vbo", gl::BufferData(gl::COPY_WRITE_BUFFER, (self.m_capacity - dealloc_size) as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    // Copy existing buffer contents up to the byte offset to new buffer.
    check_gl_call!("Vbo", gl::CopyBufferSubData(gl::COPY_READ_BUFFER, gl::COPY_WRITE_BUFFER,
      0, 0, self.m_size as GLintptr));
    
    // Swap buffers.
    self.unbind()?;
    check_gl_call!("Vbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    self.m_renderer_id = new_buffer;
    self.m_capacity -= dealloc_size;
    
    return Ok(());
  }
}

#[cfg(feature = "OpenGL")]
impl Drop for GlVbo {
  fn drop(&mut self) {
    unsafe {
      let renderer = Renderer::get().as_ref()
        .expect("[Buffer] -->\t Cannot drop Vbo, renderer is null! Exiting...");
      if renderer.m_api.get_type() == EnumApi::OpenGL && renderer.get_state() != EnumState::Shutdown {
        gl::DeleteBuffers(1, &self.m_renderer_id);
      }
    }
  }
}