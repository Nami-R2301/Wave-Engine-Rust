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

extern crate gl;

pub use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};
use gl::types::GLintptr;

use crate::{check_gl_call, log};
use crate::wave::graphics::renderer::{EnumErrors};

pub struct GlVao {
  m_renderer_id: u32,
}

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
  
  pub fn unbind(&self) -> Result<(), EnumErrors> {
    check_gl_call!("Vao", gl::BindVertexArray(0));
    return Ok(());
  }
}

pub struct GlVbo {
  pub m_renderer_id: u32,
  pub m_size: usize,
  pub m_count: usize,
}

impl GlVbo {
  pub fn new(buffer_size: usize, element_count: usize) -> Result<Self, EnumErrors> {
    let mut new_vbo: GLuint = 0;
    check_gl_call!("Vbo", gl::CreateBuffers(1, &mut new_vbo));
    check_gl_call!("Vbo", gl::BindBuffer(gl::ARRAY_BUFFER, new_vbo));
    check_gl_call!("Vbo", gl::BufferData(gl::ARRAY_BUFFER, buffer_size as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    return Ok(GlVbo {
      m_renderer_id: new_vbo,
      m_size: buffer_size,
      m_count: element_count,
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
    let mut previous_bound_program: GLint = 0;
    check_gl_call!("Vbo", gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut previous_bound_program));
    
    self.bind()?;
    check_gl_call!("Vbo", gl::BufferSubData(gl::ARRAY_BUFFER, byte_offset as GLsizeiptr,
      alloc_size as GLsizeiptr, data));
    
    check_gl_call!("Vbo", gl::UseProgram(previous_bound_program as GLuint));
    return Ok(());
  }
  
  pub fn append(&mut self, data: *const GLvoid, alloc_size: usize, vertex_count: usize) -> Result<(), EnumErrors> {
    let old_size: usize = self.m_size;
    self.expand(alloc_size)?;
    self.m_count += vertex_count;
    
    // Set new data in new buffer.
    self.set_data(data, alloc_size, old_size)?;
    return Ok(());
  }
  
  pub fn strip(&mut self, dealloc_size: usize, vertex_count: usize) -> Result<(), EnumErrors> {
    self.shrink(dealloc_size)?;
    self.m_count -= vertex_count;
    return Ok(());
  }
  
  pub fn expand(&mut self, alloc_size: usize) -> Result<(), EnumErrors> {
    if alloc_size <= self.m_size {
      return Err(EnumErrors::WrongSize);
    }
    
    self.bind()?;
    // Create new vbo to fit all contents.
    let mut new_buffer: GLuint = 0;
    check_gl_call!("Vbo", gl::CreateBuffers(1, &mut new_buffer));
    check_gl_call!("Vbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, new_buffer));
    check_gl_call!("Vbo", gl::BufferData(gl::COPY_WRITE_BUFFER, alloc_size as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    // Copy existing buffer contents up to the byte offset to new buffer.
    check_gl_call!("Vbo", gl::CopyBufferSubData(gl::COPY_READ_BUFFER, gl::COPY_WRITE_BUFFER,
      0, 0, self.m_size as GLintptr));
    
    // Swap buffers.
    self.unbind()?;
    check_gl_call!("Vbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    self.m_renderer_id = new_buffer;
    self.m_size += alloc_size;
    
    return Ok(());
  }
  
  pub fn shrink(&mut self, dealloc_size: usize) -> Result<(), EnumErrors> {
    if dealloc_size >= self.m_size {
      return Err(EnumErrors::WrongSize);
    }
    
    self.bind()?;
    // Create new vbo to fit all contents.
    let mut new_buffer: GLuint = 0;
    check_gl_call!("Vbo", gl::CreateBuffers(1, &mut new_buffer));
    check_gl_call!("Vbo", gl::BindBuffer(gl::COPY_WRITE_BUFFER, new_buffer));
    check_gl_call!("Vbo", gl::BufferData(gl::COPY_WRITE_BUFFER, dealloc_size as GLsizeiptr,
      std::ptr::null(), gl::DYNAMIC_DRAW));
    
    // Copy existing buffer contents up to the byte offset to new buffer.
    check_gl_call!("Vbo", gl::CopyBufferSubData(gl::COPY_READ_BUFFER, gl::COPY_WRITE_BUFFER,
      0, 0, self.m_size as GLintptr));
    
    // Swap buffers.
    self.unbind()?;
    check_gl_call!("Vbo", gl::DeleteBuffers(1, &self.m_renderer_id));
    self.m_renderer_id = new_buffer;
    self.m_size -= dealloc_size;
    
    return Ok(());
  }
}