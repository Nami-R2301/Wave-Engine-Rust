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

use std::ffi::c_void;
use std::ptr::null;

use gl::types::{GLchar, GLenum, GLsizei, GLuint};
use crate::wave::graphics::renderer::EnumApi::OpenGL;
use crate::wave::window::GlWindow;

#[derive(Debug)]
pub enum EnumApi {
  OpenGL(String),
  Vulkan,
  DirectX,
}

#[derive(Debug, Copy, Clone)]
pub enum EnumFeature {
  Debug(bool),
  DepthTest(bool),
  CullFacing(bool, GLenum),
  Wireframe(bool),
  MSAA(bool, u8),
  SRGB(bool),
  Blending(bool, GLenum, GLenum),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumErrors {
  Ok,
  Init,
  NotImplemented,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumState {
  Ok,
  Error,
  CriticalError,
}

static mut S_STATE: EnumState = EnumState::Ok;
static S_ERROR_CALLBACK: extern "system" fn(error_code: GLenum, e_type: GLenum, id: GLuint,
                            severity: GLenum, length: GLsizei, error_message: *const GLchar,
                            user_param: *mut c_void) = gl_error_callback;

pub struct GlRenderer {}

extern "system" fn gl_error_callback(_error_code: GLenum, _e_type: GLenum, _id: GLuint,
                     _severity: GLenum, _length: GLsizei, _error_message: *const GLchar,
                     _user_param: *mut c_void) {}

unsafe fn init_api() -> EnumErrors {
  gl::load_with(|f_name| GlWindow::get_current_window().get_proc_address_raw(f_name));
  
  gl::Viewport(0, 0, 1920, 1080);
  gl::ClearColor(0.15, 0.15, 0.15, 1.0);
  return EnumErrors::Ok;
}

impl GlRenderer {
  pub fn new() -> Result<(), EnumErrors> {
    let result = unsafe { init_api() };
    if result != EnumErrors::Ok {
      return Err(result);
    }
    return Ok(());
  }
  
  pub fn shutdown() -> Result<(), EnumErrors> {
    if unsafe { S_STATE == EnumState::Error } {
      return Err(EnumErrors::NotImplemented);
    }
    return Ok(())
  }
  
  pub unsafe fn get_renderer_info() -> String {
    let renderer_info = std::ffi::CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8)
      .to_str().unwrap_or("Cannot retrieve information!");
    let str: String = format!("Renderer Hardware => {0}", renderer_info);
    return str;
  }
  
  pub unsafe fn get_api_info() -> EnumApi {
    let api_vendor = std::ffi::CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8)
      .to_str().unwrap_or("Cannot retrieve information!");
    let api_version= std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
      .to_str().unwrap_or("Cannot retrieve information!");
    let str: String = format!("{0}, {1}", api_vendor, api_version);
    return OpenGL(str);
  }
  
  pub unsafe fn get_shading_info() -> String {
    let shading_info = std::ffi::CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8)
      .to_str().unwrap_or("Cannot retrieve information!");
    let str: String = format!("Shading Language => {0}", shading_info);
    return str;
  }
  
  pub fn get_state() -> EnumState {
    return unsafe { S_STATE };
  }
  
  pub fn get_callback() -> extern "system" fn(error_code: GLenum, e_type: GLenum, id: GLuint,
                              severity: GLenum, length: GLsizei, error_message: *const GLchar,
                              user_param: *mut c_void) {
    return S_ERROR_CALLBACK;
  }
  
  pub fn toggle_feature(feature: EnumFeature) {
    match feature {
      EnumFeature::Debug(flag) => unsafe {
        if flag {
          gl::Enable(gl::DEBUG_OUTPUT);
          gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
          gl::DebugMessageCallback(Option::from(S_ERROR_CALLBACK), null())
        } else {
          gl::Disable(gl::DEBUG_OUTPUT);
          gl::Disable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        }
      }
      EnumFeature::DepthTest(flag) => unsafe {
        if flag {
          gl::Enable(gl::DEPTH_TEST);
        } else {
          gl::Disable(gl::DEPTH_TEST);
        }
      }
      EnumFeature::MSAA(flag, _) => unsafe {
        if flag {
          gl::Enable(gl::MULTISAMPLE);
        } else {
          gl::Disable(gl::MULTISAMPLE);
        }
      }
      EnumFeature::Blending(flag, s_factor, d_factor) => unsafe {
        if flag {
          gl::Enable(gl::BLEND);
          gl::BlendFunc(s_factor, d_factor);
        } else {
          gl::Disable(gl::BLEND);
        }
      }
      EnumFeature::Wireframe(flag) => unsafe {
        if flag {
          gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        } else {
          gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
      }
      EnumFeature::SRGB(flag) => unsafe {
        if flag {
          gl::Enable(gl::FRAMEBUFFER_SRGB);
        } else {
          gl::Disable(gl::FRAMEBUFFER_SRGB);
        }
      }
      EnumFeature::CullFacing(flag, face) => unsafe {
        if flag {
          gl::Enable(gl::CULL_FACE);
          gl::CullFace(face);
        } else {
          gl::Disable(gl::CULL_FACE);
        }
      }
    }
  }
}