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

use wave_engine::wave::assets::renderable_assets::GlREntity;
use wave_engine::wave::utils::asset_loader::ResLoader;

#[test]
fn test_obj_loader() {
  let cube = ResLoader::new("cube.obj");
  
  match cube {
    Ok(gl_vertices) => {
      let gl_renderable_entity: GlREntity = GlREntity::from(gl_vertices);
      assert_ne!(gl_renderable_entity.m_entity_id[0], u32::MAX);
      assert_eq!(gl_renderable_entity.m_vertices.len(), 36 * 3);  // Count * Vec3 (x,y,z).
    }
    Err(_) => {
      assert!(false);
    }
  }
}