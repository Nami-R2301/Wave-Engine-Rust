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

use std::fmt::{Display, Formatter};

use assimp;
use assimp::import::structs::PrimitiveType;

#[cfg(feature = "debug")]
use crate::Engine;
use crate::TraitHint;
use crate::utils::macros::logger::*;

/*
///////////////////////////////////   Asset Loader  ///////////////////////////////////
///////////////////////////////////                 ///////////////////////////////////
///////////////////////////////////                 ///////////////////////////////////
 */

impl From<std::io::Error> for EnumAssetError {
  fn from(value: std::io::Error) -> Self {
    log!(EnumLogColor::Red, "ERROR", "[AssetLoader] -->\t Error while reading directory or file, Error => {0}", value);
    return EnumAssetError::InvalidRead;
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumAssetError {
  InvalidPath,
  InvalidFileExtension,
  InvalidRead,
  InvalidShapeData,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum EnumAssetPrimitiveMode {
  Plain,
  Indexed,
}

impl Default for EnumAssetPrimitiveMode {
  fn default() -> Self {
    return EnumAssetPrimitiveMode::Indexed;
  }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum EnumAssetHint {
  VertexDataIs(EnumAssetPrimitiveMode),
  SplitLargeMeshes(Option<usize>),
  GenerateNormals(bool),
  GenerateUvs(bool),
  Triangulate(bool),
  ReduceMeshes(bool),
  OnlyTriangles(bool),
}

impl EnumAssetHint {
  pub fn is(&self, other: &Self) -> bool {
    return match (self, other) {
      (EnumAssetHint::VertexDataIs(_), EnumAssetHint::VertexDataIs(_)) => true,
      (EnumAssetHint::SplitLargeMeshes(_), EnumAssetHint::SplitLargeMeshes(_)) => true,
      (EnumAssetHint::GenerateNormals(_), EnumAssetHint::GenerateNormals(_)) => true,
      (EnumAssetHint::GenerateUvs(_), EnumAssetHint::GenerateUvs(_)) => true,
      (EnumAssetHint::Triangulate(_), EnumAssetHint::Triangulate(_)) => true,
      (EnumAssetHint::ReduceMeshes(_), EnumAssetHint::ReduceMeshes(_)) => true,
      (EnumAssetHint::OnlyTriangles(_), EnumAssetHint::OnlyTriangles(_)) => true,
      _ => false
    };
  }
  
  pub fn get_value(&self) -> &dyn std::any::Any {
    return match self {
      EnumAssetHint::VertexDataIs(flag) => flag,
      EnumAssetHint::SplitLargeMeshes(vertex_limit) => vertex_limit,
      EnumAssetHint::GenerateNormals(flag) => flag,
      EnumAssetHint::GenerateUvs(flag) => flag,
      EnumAssetHint::Triangulate(flag) => flag,
      EnumAssetHint::ReduceMeshes(flag) => flag,
      EnumAssetHint::OnlyTriangles(flag) => flag
    };
  }
}

impl Display for EnumAssetError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[AssetLoader] -->\t Error encountered while loading resource : {:?}", self)
  }
}

impl std::error::Error for EnumAssetError {}

pub struct AssetInfo<'a> {
  pub(crate) m_is_indexed: bool,
  pub(crate) m_data: assimp::scene::Scene<'a>,
}

#[derive(Debug)]
pub struct AssetLoader {
  m_hints: Vec<EnumAssetHint>,
}

impl TraitHint<EnumAssetHint> for AssetLoader {
  fn set_hint(&mut self, hint: EnumAssetHint) {
    if let Some(position) = self.m_hints.iter().position(|h| h.is(&hint)) {
      self.m_hints.remove(position);
    }
    
    self.m_hints.push(hint);
  }
  
  fn reset_hints(&mut self) {
    self.m_hints.clear();
  }
}

impl AssetLoader {
  pub fn new() -> Self {
    return Self {
      m_hints: Vec::with_capacity(6)
    };
  }
  
  pub fn load_from_folder(&self, folder_path_str: &str) -> Result<Vec<AssetInfo>, EnumAssetError> {
    let folder_path = std::path::Path::new(folder_path_str);
    let mut assets = Vec::with_capacity(5);
    
    if !folder_path.exists() || !folder_path.is_dir() {
      log!(EnumLogColor::Red, "ERROR", "[AssetLoader] -->\t Could not find path {0:?}! Make sure it \
          exists and you have the appropriate permissions to read it.", folder_path);
      return Err(EnumAssetError::InvalidPath);
    }
    
    let folder_read_result = folder_path.read_dir()?;
    
    for entry_result in folder_read_result {
      if let Ok(entry) = entry_result {
        log!(EnumLogColor::Purple, "ERROR", "[AssetLoader] -->\t Loading asset {0:?} from folder {1:?}...",
          entry.file_name(), folder_path);
        
        let asset_file_name = entry.file_name();
        if let Ok(asset) = self.load(asset_file_name.to_str().unwrap()) {
          assets.push(asset);
        }
      }
    }
    return Ok(assets);
  }
  
  pub fn load(&self, file_path: &str) -> Result<AssetInfo, EnumAssetError> {
    let path = std::path::Path::new(file_path);
    
    if !path.exists() {
      log!(EnumLogColor::Red, "ERROR", "[AssetLoader] -->\t Could not find path {0}! Make sure it \
          exists and you have the appropriate permissions to read it.", file_path);
      return Err(EnumAssetError::InvalidPath);
    }
    
    let mut importer = assimp::import::Importer::new();
    
    // Default hints.
    let mut vertex_data_type = EnumAssetHint::VertexDataIs(Default::default());
    let mut split_large_meshes = EnumAssetHint::SplitLargeMeshes(None);
    let mut generate_normals = EnumAssetHint::GenerateNormals(false);
    let mut generate_uvs = EnumAssetHint::GenerateUvs(false);
    let mut triangulate = EnumAssetHint::Triangulate(true);
    let mut reduce_meshes = EnumAssetHint::ReduceMeshes(false);
    let mut only_triangles = EnumAssetHint::OnlyTriangles(true);
    
    for hint in self.m_hints.iter() {
      match hint {
        EnumAssetHint::VertexDataIs(primitive_type) => vertex_data_type = EnumAssetHint::VertexDataIs(*primitive_type),
        EnumAssetHint::SplitLargeMeshes(limit) => split_large_meshes = EnumAssetHint::SplitLargeMeshes(*limit),
        EnumAssetHint::GenerateNormals(flag) => generate_normals = EnumAssetHint::GenerateNormals(*flag),
        EnumAssetHint::GenerateUvs(flag) => generate_uvs = EnumAssetHint::GenerateUvs(*flag),
        EnumAssetHint::Triangulate(flag) => triangulate = EnumAssetHint::Triangulate(*flag),
        EnumAssetHint::ReduceMeshes(flag) => reduce_meshes = EnumAssetHint::ReduceMeshes(*flag),
        EnumAssetHint::OnlyTriangles(flag) => only_triangles = EnumAssetHint::OnlyTriangles(*flag),
      }
    }
    
    self.set_options(&mut importer,
      vec![vertex_data_type.clone(), split_large_meshes, generate_normals, generate_uvs, triangulate, reduce_meshes, only_triangles]);
    
    importer.gen_uv_coords(true);
    importer.find_invalid_data(|invalid_data| invalid_data.enable = true);
    importer.fix_infacing_normals(true);
    // importer.remove_redudant_materials(|rm_red_mat| rm_red_mat.enable = true);
    importer.improve_cache_locality(|impv_cache| impv_cache.enable = true);
    importer.measure_time(true);
    
    // #[cfg(feature = "debug")]
    // {
    //   importer.validate_data_structure(true);
    //   assimp::log::LogStream::set_verbose_logging(true);
    //   let mut logger = assimp::log::LogStream::stdout();
    //   logger.attach();
    // }
    
      let scene = importer.read_file(file_path);
      
      if scene.is_err() || scene.as_ref().unwrap().is_incomplete() {
        log!(EnumLogColor::Red, "Error", "[AssetLoader] -->\t Asset file {0} incomplete or corrupted!", file_path);
        return Err(EnumAssetError::InvalidShapeData);
      }
    
    return Ok(AssetInfo {
      m_is_indexed: vertex_data_type.get_value()
        .downcast_ref::<EnumAssetPrimitiveMode>()
        .is_some_and(|mode| *mode == EnumAssetPrimitiveMode::Indexed),
      m_data: scene.unwrap(),
    });
  }
  
  fn set_options(&self, importer: &mut assimp::Importer, hints: Vec<EnumAssetHint>) {
    for hint in hints.into_iter() {
      match hint {
        EnumAssetHint::VertexDataIs(primitive_type) => {
          match primitive_type {
            EnumAssetPrimitiveMode::Plain => {
              importer.find_degenerates(|find_degen| find_degen.enable = true);
              // Don't join indices and let vertices repeat themselves if be, since we want to render without indexing.
              importer.join_identical_vertices(false);
            }
            EnumAssetPrimitiveMode::Indexed => {
              // Make each index point to a single vertex if possible (turn on indexing).
              importer.join_identical_vertices(true);
            }
          };
        }
        EnumAssetHint::SplitLargeMeshes(vertex_limit) => {
          importer.split_large_meshes(|split_large| {
            split_large.enable = vertex_limit.is_some();
            if split_large.enable {
              split_large.vertex_limit = vertex_limit.unwrap() as i32;
            }
          });
        }
        EnumAssetHint::GenerateNormals(bool) => {
          importer.generate_normals(|gen_normals| {
            gen_normals.enable = bool;
            gen_normals.smooth = bool;
          });
        }
        EnumAssetHint::GenerateUvs(bool) => importer.gen_uv_coords(bool),
        EnumAssetHint::Triangulate(bool) => importer.triangulate(bool),
        EnumAssetHint::ReduceMeshes(bool) => {
          importer.optimize_meshes(bool);
          importer.optimize_graph(|opt_graph| {
            opt_graph.enable = bool;
          });
        }
        EnumAssetHint::OnlyTriangles(bool) => {
          importer.sort_by_primitive_type(|sort_type| {
            sort_type.enable = bool;
            if bool {
              sort_type.remove = vec![PrimitiveType::Line, PrimitiveType::Point];
            }
          });
        }
      }
    }
  }
}