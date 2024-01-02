/*
///////////////////////////////////   OpenGL    ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
///////////////////////////////////             ///////////////////////////////////
 */

extern crate gl;

pub use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLint, GLintptr, GLsizei, GLsizeiptr, GLuint,
  GLvoid};

use crate::{check_gl_call, log};
use crate::wave::graphics::open_gl::renderer::EnumOpenGLErrors;
use crate::wave::graphics::renderer::{EnumError, EnumState, Renderer};
use crate::wave::EnumApi;

#[allow(unused)]
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

pub struct GlVertexAttribute {
  pub m_gl_type: GLenum,
  pub m_count: i32,
  pub m_buffer_offset: usize,
  pub m_normalized: u8,
  pub m_attribute_divisor: u8,
}

impl GlVertexAttribute {
  pub fn new(gl_type: EnumAttributeType, should_normalize: bool, buffer_offset: usize, attribute_divisor: u8) -> Result<Self, EnumError> {
    let mut max_attrib_div: i32 = 0;
    check_gl_call!("Buffer (Attribute divisor)", gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attrib_div));
    
    if attribute_divisor > max_attrib_div as u8 {
      log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Cannot assign attribute divisor of {0} to \
      vertex attribute, since it exceeds the maximum vertex attributes available!", attribute_divisor);
      return Err(EnumError::InvalidAttributeDivisor);
    }
    
    return Ok(match gl_type {
      EnumAttributeType::UnsignedShort(count) => {
        GlVertexAttribute {
          m_gl_type: gl::UNSIGNED_SHORT,
          m_count: count,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Short(count) => {
        GlVertexAttribute {
          m_gl_type: gl::SHORT,
          m_count: count,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::UnsignedInt(count) => {
        GlVertexAttribute {
          m_gl_type: gl::UNSIGNED_INT,
          m_count: count,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Int(count) => {
        GlVertexAttribute {
          m_gl_type: gl::INT,
          m_count: count,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Float(count) => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: count,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Double(count) => {
        GlVertexAttribute {
          m_gl_type: gl::DOUBLE,
          m_count: count,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Vec2 => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: 2,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Vec3 => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: 3,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Vec4 => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: 4,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
      EnumAttributeType::Mat4 => {
        GlVertexAttribute {
          m_gl_type: gl::FLOAT,
          m_count: 4 * 4,
          m_buffer_offset: buffer_offset,
          m_normalized: should_normalize as u8,
          m_attribute_divisor: attribute_divisor,
        }
      }
    });
  }
}

pub struct GlVao {
  m_renderer_id: u32,
}

impl GlVao {
  pub fn new() -> Result<Self, EnumError> {
    let mut new_vao: GLuint = 0;
    check_gl_call!("Vao", gl::CreateVertexArrays(1, &mut new_vao));
    return Ok(GlVao {
      m_renderer_id: new_vao,
    });
  }
  
  pub fn delete(&mut self) -> Result<(), EnumError> {
    self.unbind()?;
    check_gl_call!("Vao", gl::DeleteVertexArrays(1, &self.m_renderer_id));
    return Ok(());
  }
  
  pub fn bind(&mut self) -> Result<(), EnumError> {
    check_gl_call!("Vao", gl::BindVertexArray(self.m_renderer_id));
    return Ok(());
  }
  
  pub fn unbind(&mut self) -> Result<(), EnumError> {
    check_gl_call!("Vao", gl::BindVertexArray(0));
    return Ok(());
  }
  
  pub fn enable_attributes(&mut self, attributes: Vec<GlVertexAttribute>) -> Result<(), EnumError> {
    if attributes.is_empty() {
      return Err(EnumError::InvalidVertexAttribute);
    }
    
    let mut max_attrib_div: i32 = 0;
    check_gl_call!("Buffer (Attribute divisor)", gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attrib_div));
    
    self.bind()?;
    for (index, attribute) in attributes.iter().enumerate() {
      if index > max_attrib_div as usize {
        log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Vertex attribute index exceeds maximum \
        vertex attributes supported!");
        return Err(EnumError::InvalidVertexAttribute);
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

impl Drop for GlVao {
  fn drop(&mut self) {
    let renderer = Renderer::get().as_ref()
      .expect("[Buffer] -->\t Cannot drop Vao, renderer is null! Exiting...");
    if renderer.get_type() == EnumApi::OpenGL && renderer.get_state() != EnumState::Shutdown {
      match self.delete() {
        Ok(_) => {}
        Err(err) => {
          log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Error while dropping VAO : \
          OpenGL returned with Error => {:?}", err)
        }
      }
    }
  }
}

pub struct GlVbo {
  pub m_renderer_id: u32,
  pub m_capacity: usize,
  pub m_size: usize,
  pub m_count: usize,
}

impl GlVbo {
  pub fn new(alloc_size: usize, vertex_count: usize) -> Result<Self, EnumError> {
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
  
  pub fn delete(&mut self) -> Result<(), EnumError> {
    self.unbind()?;
    check_gl_call!("Vbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    return Ok(());
  }
  
  pub fn bind(&mut self) -> Result<(), EnumError> {
    check_gl_call!("Vbo", gl::BindBuffer(gl::ARRAY_BUFFER, self.m_renderer_id));
    return Ok(());
  }
  
  pub fn unbind(&self) -> Result<(), EnumError> {
    check_gl_call!("Vbo", gl::BindBuffer(gl::ARRAY_BUFFER, 0));
    return Ok(());
  }
  
  pub fn set_data(&mut self, data: *const GLvoid, alloc_size: usize, byte_offset: usize) -> Result<(), EnumError> {
    self.bind()?;
    check_gl_call!("Vbo", gl::BufferSubData(gl::ARRAY_BUFFER, byte_offset as GLsizeiptr,
      alloc_size as GLsizeiptr, data));
    
    return Ok(());
  }
  
  #[allow(unused)]
  pub fn append(&mut self, data: *const GLvoid, vertex_size: usize, vertex_count: usize) -> Result<(), EnumError> {
    if vertex_size == 0 || vertex_count == 0 {
      return Err(EnumError::InvalidBufferSize);
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
  
  #[allow(unused)]
  pub fn strip(&mut self, buffer_offset: usize, vertex_size: usize, vertex_count: usize) -> Result<(), EnumError> {
    if vertex_size * vertex_count == 0 || vertex_size * vertex_count > self.m_size {
      return Err(EnumError::InvalidBufferSize);
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
  
  #[allow(unused)]
  pub fn expand(&mut self, alloc_size: usize) -> Result<(), EnumError> {
    if alloc_size == 0 {
      return Err(EnumError::InvalidBufferSize);
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
  
  #[allow(unused)]
  pub fn shrink(&mut self, dealloc_size: usize) -> Result<(), EnumError> {
    if dealloc_size == 0 {
      return Err(EnumError::InvalidBufferSize);
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

impl Drop for GlVbo {
  fn drop(&mut self) {
    let renderer = Renderer::get().as_ref()
      .expect("[Buffer] -->\t Cannot drop Vbo, renderer is null! Exiting...");
    if renderer.get_type() == EnumApi::OpenGL && renderer.get_state() != EnumState::Shutdown {
      match self.delete() {
        Ok(_) => {}
        Err(err) => {
          log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Error while dropping VBO : \
          OpenGL returned with Error => {:?}", err)
        }
      }
    }
  }
}