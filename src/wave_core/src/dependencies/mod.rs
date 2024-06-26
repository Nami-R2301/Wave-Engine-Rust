/*
 MIT License

 Copyright (c) 2024 Nami Reghbati

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

/// Visible externals.

// Rendering and shader compilation.
#[cfg(feature = "vulkan")]
pub extern crate ash;
#[cfg(feature = "vulkan")]
pub extern crate shaderc;

pub extern crate gl;
pub extern crate gl46;

// Windowing and UI.
pub extern crate glfw;
pub extern crate imgui;

// Utilities and macros.
pub extern crate chrono;
pub extern crate bitflags;
pub extern crate rand;

/// Private macros.
pub(crate) extern crate num;
pub(crate) extern crate imgui_opengl_renderer;
pub(crate) extern crate assimp;