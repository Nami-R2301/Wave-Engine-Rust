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

use crate::utils::macros::logger::*;
#[cfg(feature = "debug")]
use crate::Engine;

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

impl Display for EnumAssetError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[ResLoader] -->\t Error encountered while loading resource : {:?}", self)
  }
}

impl std::error::Error for EnumAssetError {

}

#[derive(Debug)]
pub struct AssetLoader {}

impl AssetLoader {
  /// Generate a [Object3D] asset, given a file path to the desired asset file to load data from.
  /// The supported file types are **obj** and **gltf**. Additionally, the base resource path will be
  /// automatically supplied when looking for the file (i.e. "test.obj" => "res/assets/test.obj").
  /// Thus, only supply file paths below the **assets/** file tree.
  ///
  /// # Arguments
  ///
  /// * `file_name`: A file path **including** the extension of its format.
  ///
  /// # Returns:
  ///   - Result<Shape, EnumError> : Will return a valid shape if successful, otherwise an [EnumAssetError]
  ///     on any error encountered. These include, but are not limited to :
  ///     + [EnumAssetError::InvalidPath] : If the file path provided is not a valid for for assets or
  ///     if the working directory leads to the incorrect *res* path.
  ///     + [EnumAssetError::InvalidFileExtension] : If the file extension is not supported.
  ///     + [EnumAssetError::InvalidRead] : If the file could not be read due to invalid formatting or
  ///     missing read permissions.
  ///
  /// # Examples
  ///
  /// ```text
  /// use wave_core::utils::asset_loader::{EnumError, ResLoader}
  ///
  /// let cube = ResLoader::new("objs/cube");
  /// let sphere = ResLoader::new("sphere.gltt");
  /// let diamond = ResLoader::new("res/assets/objs/diamond.obj");
  /// let pyramid = ResLoader::new("objs/pyramid.obj");
  ///
  /// assert_eq!(cube, EnumError::InvalidPath);
  /// assert_eq!(sphere, EnumError::InvalidFileExtension);
  /// assert_eq!(diamond, EnumError::InvalidPath);
  /// assert!(pyramid.is_ok());
  /// ```
  pub fn new(file_name: &str) -> Result<assimp::scene::Scene, EnumAssetError> {
    let asset_path = &("res/assets/".to_string() + file_name);
    let path = std::path::Path::new(asset_path).extension();
    
    return match path {
      None => {
        log!(EnumLogColor::Red, "ERROR", "[ResLoader] -->\t Could not find path {0}! Make sure it \
          exists and you have the appropriate permissions to read it.", asset_path);
        Err(EnumAssetError::InvalidPath)
      }
      Some(_) => {
        let mut importer = assimp::import::Importer::new();
        importer.triangulate(true);
        importer.join_identical_vertices(true);
        importer.sort_by_primitive_type(|sort_type| {
          sort_type.enable = true;
          sort_type.remove = vec![PrimitiveType::Line, PrimitiveType::Point];
        });
        importer.find_invalid_data(|invalid_data| invalid_data.enable = true);
        importer.fix_infacing_normals(true);
        // Does the index buffer job for us.
        importer.find_degenerates(|find_degen| find_degen.enable = true);
        importer.remove_redudant_materials(|rm_red_mat| rm_red_mat.enable = true);
        importer.generate_normals(|gen_normals| {
          gen_normals.enable = true;
          gen_normals.smooth = true;
        });
        importer.improve_cache_locality(|impv_cache| impv_cache.enable = true);
        importer.optimize_meshes(true);
        importer.split_large_meshes(|split_large| split_large.enable = true);
        importer.measure_time(true);
        
        // #[cfg(feature = "debug")]
        // {
        //   assimp::log::LogStream::set_verbose_logging(true);
        //   let mut logger = assimp::log::LogStream::stdout();
        //   logger.attach();
        // }
        
        let scene = importer.read_file(asset_path);
        
        if scene.is_err() || scene.as_ref().unwrap().is_incomplete() {
          log!(EnumLogColor::Red, "Error", "[ResLoader] -->\t Asset file {0} incomplete or corrupted!", asset_path);
          return Err(EnumAssetError::InvalidShapeData);
        }
        
        return Ok(scene.unwrap());
      }
    };
  }
}