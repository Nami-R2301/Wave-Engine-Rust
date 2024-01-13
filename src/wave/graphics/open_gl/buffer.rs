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

pub use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLint, GLintptr, GLsizei, GLsizeiptr, GLuint,
  GLvoid};

use crate::{check_gl_call, log};
use crate::wave::graphics::open_gl;
use crate::wave::graphics::renderer::{EnumApi, EnumState, Renderer};
use crate::wave::math::Mat4;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum EnumError {
  InvalidBufferSize,
  InvalidBufferOffset,
  InvalidVertexAttribute,
  InvalidAttributeDivisor,
  InvalidVbo,
  InvalidVao,
  InvalidUbo,
}

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

#[allow(unused)]
pub struct GlVertexAttribute {
  pub m_gl_type: GLenum,
  pub m_count: i32,
  pub m_buffer_size: usize,
  pub m_buffer_offset: usize,
  pub m_normalized: u8,
  pub m_attribute_divisor: u8,
}

impl GlVertexAttribute {
  pub fn new(gl_type: EnumAttributeType, should_normalize: bool, vbo_offset: usize, attribute_divisor: u8) -> Result<Self, EnumError> {
    let mut max_attrib_div: i32 = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attrib_div) };
    
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
pub struct GlVao {
  m_renderer_id: u32,
}

impl GlVao {
  pub fn new() -> Result<Self, open_gl::renderer::EnumError> {
    let mut new_vao: GLuint = 0;
    check_gl_call!("Vao", gl::CreateVertexArrays(1, &mut new_vao));
    return Ok(GlVao {
      m_renderer_id: new_vao,
    });
  }
  
  pub fn on_delete(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    self.unbind()?;
    check_gl_call!("Vao", gl::DeleteVertexArrays(1, &self.m_renderer_id));
    return Ok(());
  }
  
  pub fn bind(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    check_gl_call!("Vao", gl::BindVertexArray(self.m_renderer_id));
    return Ok(());
  }
  
  pub fn unbind(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    check_gl_call!("Vao", gl::BindVertexArray(0));
    return Ok(());
  }
  
  pub fn enable_attributes(&mut self, mut attributes: Vec<GlVertexAttribute>) -> Result<(), open_gl::renderer::EnumError> {
    if attributes.is_empty() {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidVertexAttribute));
    }
    
    let mut max_attrib_div: i32 = 0;
    check_gl_call!("Buffer (Attribute divisor)", gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attrib_div));
    
    self.bind()?;
    let mut stride = 0;
    let mut offset = 0;
    for attribute in attributes.iter_mut() {
      attribute.m_buffer_offset += offset;
      offset += attribute.m_buffer_size;
      stride += attribute.m_buffer_size;
    }
    
    for (index, attribute) in attributes.iter().enumerate() {
      if index > max_attrib_div as usize {
        log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Vertex attribute index exceeds maximum \
        vertex attributes supported!");
        return Err(open_gl::renderer::EnumError::from(EnumError::InvalidVertexAttribute));
      }
      
      if attribute.m_gl_type == gl::UNSIGNED_INT || attribute.m_gl_type == gl::INT {
        check_gl_call!("Renderer", gl::VertexAttribIPointer(index as u32, attribute.m_count,
          attribute.m_gl_type, stride as GLsizei, attribute.m_buffer_offset as *const GLvoid));
      } else {
        check_gl_call!("Renderer", gl::VertexAttribPointer(index as u32, attribute.m_count,
          attribute.m_gl_type, attribute.m_normalized, stride as GLsizei, attribute.m_buffer_offset as *const GLvoid));
      }
      check_gl_call!("Renderer", gl::EnableVertexAttribArray(index as u32));
    }
    
    return Ok(());
  }
}

impl Drop for GlVao {
  fn drop(&mut self) {
    let renderer = Renderer::get();
    if renderer.is_some() {
      unsafe {
        if (*renderer.unwrap()).m_type == EnumApi::OpenGL && (*renderer.unwrap()).m_state != EnumState::Shutdown {
          match self.on_delete() {
            Ok(_) => {}
            #[allow(unused)]
            Err(err) => {
              log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Error while dropping VAO : \
                OpenGL returned with Error => {:?}", err)
            }
          }
        }
      }
    }
  }
}

#[allow(unused)]
pub struct GlVbo {
  pub m_renderer_id: u32,
  pub m_capacity: usize,
  pub m_size: usize,
  pub m_count: usize,
}

impl GlVbo {
  pub fn new(size_per_vertex: usize, vertex_count: usize) -> Result<Self, open_gl::renderer::EnumError> {
    let mut new_vbo: GLuint = 0;
    check_gl_call!("Vbo", gl::CreateBuffers(1, &mut new_vbo));
    check_gl_call!("Vbo", gl::BindBuffer(gl::ARRAY_BUFFER, new_vbo));
    check_gl_call!("Vbo", gl::BufferData(gl::ARRAY_BUFFER, (size_per_vertex * vertex_count) as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    return Ok(GlVbo {
      m_renderer_id: new_vbo,
      m_capacity: size_per_vertex * vertex_count,
      m_size: size_per_vertex * vertex_count,
      m_count: vertex_count,
    });
  }
  
  pub fn on_delete(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    self.unbind()?;
    check_gl_call!("Vbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    return Ok(());
  }
  
  pub fn bind(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    check_gl_call!("Vbo", gl::BindBuffer(gl::ARRAY_BUFFER, self.m_renderer_id));
    return Ok(());
  }
  
  pub fn unbind(&self) -> Result<(), open_gl::renderer::EnumError> {
    check_gl_call!("Vbo", gl::BindBuffer(gl::ARRAY_BUFFER, 0));
    return Ok(());
  }
  
  pub fn set_data(&mut self, data: *const GLvoid, alloc_size: usize, byte_offset: usize) -> Result<(), open_gl::renderer::EnumError> {
    self.bind()?;
    check_gl_call!("Vbo", gl::BufferSubData(gl::ARRAY_BUFFER, byte_offset as GLsizeiptr,
      alloc_size as GLsizeiptr, data));
    
    return Ok(());
  }
  
  #[allow(unused)]
  pub fn append(&mut self, data: *const GLvoid, vertex_size: usize, vertex_count: usize) -> Result<(), open_gl::renderer::EnumError> {
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
  
  #[allow(unused)]
  pub fn strip(&mut self, buffer_offset: usize, vertex_size: usize, vertex_count: usize) -> Result<(), open_gl::renderer::EnumError> {
    if vertex_size * vertex_count == 0 || vertex_size * vertex_count > self.m_size {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidBufferSize));
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
  pub fn expand(&mut self, alloc_size: usize) -> Result<(), open_gl::renderer::EnumError> {
    if alloc_size == 0 {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidBufferSize));
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
  pub fn shrink(&mut self, dealloc_size: usize) -> Result<(), open_gl::renderer::EnumError> {
    if dealloc_size == 0 {
      return Err(open_gl::renderer::EnumError::from(EnumError::InvalidBufferSize));
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
    let renderer = Renderer::get();
    if renderer.is_some() {
      unsafe {
        if (*renderer.unwrap()).m_type == EnumApi::OpenGL && (*renderer.unwrap()).m_state != EnumState::Shutdown {
          match self.on_delete() {
            Ok(_) => {}
            #[allow(unused)]
            Err(err) => {
              log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Error while dropping VBO : \
                OpenGL returned with Error => {:?}", err)
            }
          }
        }
      }
    }
  }
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumUboType {
  Transform(Mat4),
  ViewProjection(Mat4, Mat4),
  MVP(Mat4, Mat4, Mat4)
}

#[allow(unused)]
#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EnumUboTypeSize {
  Transform = Mat4::get_size(),
  ViewProjection = Mat4::get_size() * 2,
  MVP = Mat4::get_size() * 3,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub struct GlUbo {
  m_buffer_id: u32,
  m_size: EnumUboTypeSize,
}

impl GlUbo {
  pub fn new(size: EnumUboTypeSize, binding: u32) -> Result<Self, open_gl::renderer::EnumError> {
    let mut buffer_id = 0;
    
    check_gl_call!("Ubo", gl::CreateBuffers(1, &mut buffer_id));
    check_gl_call!("Ubo", gl::BindBuffer(gl::UNIFORM_BUFFER, buffer_id));
    check_gl_call!("Ubo", gl::BufferData(gl::UNIFORM_BUFFER, size as GLsizeiptr, std::ptr::null(),
    gl::DYNAMIC_DRAW));
    check_gl_call!("Ubo", gl::BindBufferBase(gl::UNIFORM_BUFFER, binding, buffer_id));
    
    return Ok(Self {
      m_buffer_id: buffer_id,
      m_size: size,
    });
  }
  
  pub fn bind(&mut self, block_name: &'static str, shader_id: u32) -> Result<(), open_gl::renderer::EnumError> {
    let renderer = Renderer::get()
      .expect("[Ubo] -->\t Cannot retrieve renderer : Renderer is None!");
    let glsl_version = unsafe { (*renderer).get_max_shader_version_available() };
    if glsl_version < 4.2 {
      // If glsl < #version 420, uniform binding can't be done in shaders.
      let u_block: u32 = unsafe {
        gl::GetUniformBlockIndex(shader_id, block_name.as_ptr() as *const GLchar)
      };
      check_gl_call!("Ubo", gl::UniformBlockBinding(shader_id, u_block, 1));
    }
    return Ok(());
  }
  
  pub fn unbind(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    check_gl_call!("Ubo", gl::BindBuffer(gl::UNIFORM_BUFFER, 0));
    return Ok(());
  }
  
  pub fn set_data(&mut self, ubo_type: EnumUboType) -> Result<(), open_gl::renderer::EnumError> {
    match ubo_type {
      EnumUboType::Transform(transform) => {
        // Set transform matrix.
        check_gl_call!("Ubo", gl::BufferSubData(gl::UNIFORM_BUFFER, 0 as GLintptr,
          Mat4::get_size() as GLsizeiptr, transform.transpose().as_array().as_ptr() as *const std::ffi::c_void));
      }
      EnumUboType::ViewProjection(view, projection) => {
        // Set view matrix.
        check_gl_call!("Ubo", gl::BufferSubData(gl::UNIFORM_BUFFER, 0 as GLintptr,
          Mat4::get_size() as GLsizeiptr, view.transpose().as_array().as_ptr() as *const std::ffi::c_void));
        
        // Set projection matrix.
        check_gl_call!("Ubo", gl::BufferSubData(gl::UNIFORM_BUFFER, Mat4::get_size() as GLintptr,
          Mat4::get_size() as GLsizeiptr, projection.transpose().as_array().as_ptr() as *const std::ffi::c_void));
      }
      EnumUboType::MVP(model, view, projection) => {
        // Set Model matrix.
        check_gl_call!("Ubo", gl::BufferSubData(gl::UNIFORM_BUFFER, 0 as GLintptr,
          Mat4::get_size() as GLsizeiptr, model.transpose().as_array().as_ptr() as *const std::ffi::c_void));
        
        // Set view matrix.
        check_gl_call!("Ubo", gl::BufferSubData(gl::UNIFORM_BUFFER, Mat4::get_size() as GLintptr,
          Mat4::get_size() as GLsizeiptr, view.transpose().as_array().as_ptr() as *const std::ffi::c_void));
        
        // Set projection matrix.
        check_gl_call!("Ubo", gl::BufferSubData(gl::UNIFORM_BUFFER, (Mat4::get_size() * 2) as GLintptr,
          Mat4::get_size() as GLsizeiptr, projection.transpose().as_array().as_ptr() as *const std::ffi::c_void));
      }
    }
    return Ok(());
  }
  
  fn on_delete(&mut self) -> Result<(), open_gl::renderer::EnumError> {
    self.unbind()?;
    check_gl_call!("Ubo", gl::BindBuffer(gl::UNIFORM, 0));
    return Ok(());
  }
}

impl Drop for GlUbo {
  fn drop(&mut self) {
    let renderer = Renderer::get();
    if renderer.is_some() {
      unsafe {
        if (*renderer.unwrap()).m_type == EnumApi::OpenGL && (*renderer.unwrap()).m_state != EnumState::Shutdown {
          match self.on_delete() {
            Ok(_) => {}
            #[allow(unused)]
            Err(err) => {
              log!(EnumLogColor::Red, "ERROR", "[Buffer] -->\t Error while dropping UBO : \
              OpenGL returned with Error => {:?}", err)
            }
          }
        }
      }
    }
  }
}