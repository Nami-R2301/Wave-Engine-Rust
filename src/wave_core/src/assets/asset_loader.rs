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

use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use assimp;
use assimp::import::structs::PrimitiveType;

#[cfg(feature = "debug")]
use crate::Engine;
use crate::utils::macros::logger::*;

/*
///////////////////////////////////   Asset Loader  ///////////////////////////////////
///////////////////////////////////                 ///////////////////////////////////
///////////////////////////////////                 ///////////////////////////////////
 */

#[derive(Debug, Clone, PartialEq)]
pub enum EnumAssetError {
  InvalidPath,
  InvalidFileExtension,
  InvalidRead,
  InvalidShapeData,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum EnumAssetPrimitiveMode {
  Plain,
  Indexed
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum EnumAssetHint {
  PrimitiveDataIs(EnumAssetPrimitiveMode),
  OptimizeGraphs(bool),
  SplitLargeMeshes(bool, usize),
  GenerateNormals(bool),
  Triangulate(bool),
  ReduceMeshes(bool),
  OnlyTriangles(bool)
}

impl Display for EnumAssetError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[AssetLoader] -->\t Error encountered while loading resource : {:?}", self)
  }
}

impl std::error::Error for EnumAssetError {}

#[derive(Debug)]
pub struct AssetLoader {
  m_hints: HashSet<EnumAssetHint>
}

impl AssetLoader {
  pub fn default() -> Self {
    let mut hints = HashSet::with_capacity(9);
    hints.insert(EnumAssetHint::PrimitiveDataIs(EnumAssetPrimitiveMode::Indexed));
    hints.insert(EnumAssetHint::GenerateNormals(true));
    hints.insert(EnumAssetHint::Triangulate(true));
    hints.insert(EnumAssetHint::SplitLargeMeshes(true, 500_000));
    hints.insert(EnumAssetHint::ReduceMeshes(true));
    hints.insert(EnumAssetHint::OnlyTriangles(true));
    
    return Self {
      m_hints: hints,
    }
  }
  
  pub fn new() -> Self {
    return Self {
      m_hints: HashSet::with_capacity(5)
    }
  }
  
  pub fn hint(&mut self, hint: EnumAssetHint) {
    self.m_hints.insert(hint);
  }
  
  pub fn apply(&self, file_name: &str) -> Result<assimp::scene::Scene, EnumAssetError> {
    let asset_path = &("res/assets/".to_string() + file_name);
    let path = std::path::Path::new(asset_path);
    
    if !path.exists() {
      log!(EnumLogColor::Red, "ERROR", "[AssetLoader] -->\t Could not find path {0}! Make sure it \
          exists and you have the appropriate permissions to read it.", asset_path);
      return Err(EnumAssetError::InvalidPath);
    }
    
    let mut importer = assimp::import::Importer::new();
    
    for hint in self.m_hints.iter() {
      match hint {
        EnumAssetHint::PrimitiveDataIs(primitive_type) => {
          match primitive_type {
            EnumAssetPrimitiveMode::Plain => {
              // Does the index buffer job for us.
              importer.find_degenerates(|find_degen| find_degen.enable = true);
              importer.join_identical_vertices(false);
            }
            EnumAssetPrimitiveMode::Indexed => {
              // Toggle vertex indexing.
              importer.join_identical_vertices(true);
            }
          }
        }
        EnumAssetHint::OptimizeGraphs(bool) => {
          importer.optimize_graph(|opt_graph| {
            opt_graph.enable = *bool;
          })
        }
        EnumAssetHint::SplitLargeMeshes(bool, vertex_limit) => {
          importer.split_large_meshes(|split_large| {
            split_large.enable = *bool;
            if *bool {
              split_large.vertex_limit = *vertex_limit as i32;
            }
          });
        }
        EnumAssetHint::ReduceMeshes(bool) => {
          importer.optimize_meshes(*bool);
        }
        EnumAssetHint::GenerateNormals(bool) => {
          importer.generate_normals(|gen_normals| {
            gen_normals.enable = *bool;
            gen_normals.smooth = *bool;
          });
        }
        EnumAssetHint::Triangulate(bool) => importer.triangulate(*bool),
        EnumAssetHint::OnlyTriangles(bool) => {
          importer.sort_by_primitive_type(|sort_type| {
            sort_type.enable = *bool;
            if *bool {
              sort_type.remove = vec![PrimitiveType::Line, PrimitiveType::Point];
            }
          });
        }
      }
    }
    
    
    importer.find_invalid_data(|invalid_data| invalid_data.enable = true);
    importer.fix_infacing_normals(true);
    importer.remove_redudant_materials(|rm_red_mat| rm_red_mat.enable = true);
    importer.improve_cache_locality(|impv_cache| impv_cache.enable = true);
    importer.measure_time(true);
    
    // #[cfg(feature = "debug")]
    // {
    //   assimp::log::LogStream::set_verbose_logging(true);
    //   let mut logger = assimp::log::LogStream::stdout();
    //   logger.attach();
    // }
    
    let scene = importer.read_file(asset_path);
    
    if scene.is_err() || scene.as_ref().unwrap().is_incomplete() {
      log!(EnumLogColor::Red, "Error", "[AssetLoader] -->\t Asset file {0} incomplete or corrupted!", asset_path);
      return Err(EnumAssetError::InvalidShapeData);
    }
    
    return Ok(scene.unwrap());
  }
}