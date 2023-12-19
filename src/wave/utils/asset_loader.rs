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

use std::io::BufRead;

use crate::log;
use crate::wave::assets::renderable_assets::REntity;
use crate::wave::graphics::color::Color;

/*
///////////////////////////////////   Asset Loader  ///////////////////////////////////
///////////////////////////////////                 ///////////////////////////////////
///////////////////////////////////                 ///////////////////////////////////
 */

#[derive(Debug, Clone, PartialEq)]
pub enum EnumErrors {
  InvalidPath,
  InvalidFileExtension,
  InvalidRead,
  InvalidShapeData,
}

#[derive(Debug, Clone)]
pub struct ResLoader {}

impl ResLoader {
  /// Generate a [Vertex] asset, given a file path to the desired asset file to load data from.
  /// The supported file types are **obj** and **gltf**. Additionally, the base resource path will be
  /// automatically supplied when looking for the file (i.e. "test.obj" => "res/assets/test.obj").
  /// Thus, only supply file paths below the **assets/** file tree.
  ///
  /// # Arguments
  ///
  /// * `file_name`: A file path **including** the extension of its format.
  ///
  /// # Returns:
  ///   - Result<Shape, EnumErrors> : Will return a valid shape if successful, otherwise an [EnumErrors]
  ///     on any error encountered. These include, but are not limited to :
  ///     + [EnumErrors::InvalidPath] : If the file path provided is not a valid for for assets or
  ///     if the working directory leads to the incorrect *res* path.
  ///     + [EnumErrors::InvalidFileExtension] : If the file extension is not supported.
  ///     + [EnumErrors::InvalidRead] : If the file could not be read due to invalid formatting or
  ///     missing read permissions.
  ///
  /// # Examples
  ///
  /// ```text
  /// use wave::utils::asset_loader::{EnumErrors, ResLoader}
  ///
  /// let cube = ResLoader::new("objs/cube");
  /// let sphere = ResLoader::new("sphere.gltt");
  /// let diamond = ResLoader::new("res/assets/objs/diamond.obj");
  /// let pyramid = ResLoader::new("objs/pyramid.obj");
  ///
  /// assert_eq!(cube, EnumErrors::InvalidPath);
  /// assert_eq!(sphere, EnumErrors::InvalidFileExtension);
  /// assert_eq!(diamond, EnumErrors::InvalidPath);
  /// assert!(pyramid.is_ok());
  /// ```
  pub fn new(file_name: &str) -> Result<REntity, EnumErrors> {
    let asset_path = &("res/assets/".to_string() + file_name);
    let path = std::path::Path::new(asset_path).extension();
    
    return match path {
      None => {
        log!(EnumLogColor::Red, "ERROR", "[ResLoader] -->\t Could not find path {0}! Make sure it \
          exists and you have the appropriate permissions to read it.", asset_path);
        Err(EnumErrors::InvalidPath)
      }
      Some(_) => {
        let path_str = path.unwrap().to_str().unwrap();
        match path_str {
          "obj" => { ResLoader::load_obj(asset_path) }
          "gltf" => { ResLoader::load_gltf(asset_path) }
          &_ => {
            log!(EnumLogColor::Red, "Error", "[ResLoader] -->\t Asset file format {0} not supported!",
              asset_path);
            Err(EnumErrors::InvalidFileExtension)
          }
        }
      }
    };
  }
  
  /// Internal function to load an *obj*'s file contents into a [REntity] container to eventually pass
  /// on to the GPU. Due to the tightly packed oriented dataset of [REntity], [REntity] can be made of
  /// 2D and/or 3D vertices.
  ///
  /// # Arguments
  ///
  /// * `file_name`: A file name representing the asset to be loaded in. Note, that all asset files
  /// should be located in *res/assets/* and that this loader will only work if the working directory
  /// is set to be the root of this project.
  ///
  /// # Returns:
  ///   - `Result<GlREntity, EnumErrors>` : Will return a valid shape if successful, otherwise an [EnumErrors]
  ///     on any error encountered. These include, but are not limited to :
  ///     + [EnumErrors::InvalidPath] : If the file path provided is not a valid for for assets or
  ///     if the working directory leads to the incorrect *res* path.
  ///     + [EnumErrors::InvalidFileExtension] : If the file does not correspond to a valid file type
  ///     (**obj**, **gltf**).
  ///     + [EnumErrors::InvalidRead] : If the file could not be read due to invalid formatting or
  ///     missing read permissions.
  ///     + [EnumErrors::InvalidShapeData] : If the file could not be loaded properly due to data
  ///     corruption or invalid shape data.
  ///
  fn load_obj(file_name: &str) -> Result<REntity, EnumErrors> {
    let file = std::fs::File::open(file_name);
    return match file {
      Ok(_) => {
        let buffer = std::io::BufReader::new(file.unwrap()).lines();
        let mut vertices: Vec<f32> = Vec::new();
        let mut normals: Vec<f32> = Vec::new();
        let mut texture_coords: Vec<f32> = Vec::new();
        let mut indices: Vec<usize> = Vec::new();
        
        for line in buffer {
          if line.as_ref().unwrap().is_empty() {
            continue;
          }
          if line.is_err() {
            log!(EnumLogColor::Yellow, "WARN", "[ResLoader] -->\t Cannot read line '{0}'! \
              Skipping it...", line.unwrap());
            continue;
          }
          
          // Move cursor to the beginning of the dataset => ['v 1.0, 0.0, 1.0...']
          //                                                    ^
          //                                                    |
          let line_split: Vec<&str> = line.as_ref().unwrap()
            [line.as_ref().unwrap().find(' ').unwrap_or(0) + 1..]
            .split(' ')
            .collect::<Vec<&str>>();
          
          match &line.as_ref().unwrap()[0..2] {
            "v " => { vertices.append(&mut ResLoader::read_obj_vertex(line_split)); }
            "f " => { indices.append(&mut ResLoader::read_obj_indices(line_split)); }
            "vn" => { normals.append(&mut ResLoader::read_obj_normals(line_split)); }
            "vt" => { texture_coords.append(&mut ResLoader::read_obj_texture_coords(line_split)); }
            _ => {}
          }
        }
        Ok(ResLoader::reorganize_data(&vertices, &indices, &normals, &texture_coords))
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[ResLoader] -->\t Cannot open file! Error => {0}", err);
        Err(EnumErrors::InvalidPath)
      }
    };
  }
  
  /// Internal function to read an *obj* line containing a vertex position (i.e. "v 1.00, 0.00, 1.00").
  /// Note that due to the tightly packed architecture of the resulting [REntity], [REntity] can be made of
  /// 2D and/or 3D vertices.
  ///
  /// # Arguments
  ///
  /// * `line_split`: A dynamically sized [Vec<\[&str\]>] containing each entry on line, delimited by a
  ///   space **AFTER** the first symbol at the start of the line.
  ///   (i.e. \["1.00","0.00","-1.00"]) that will be **consumed upon use**.
  ///
  /// # Returns:
  ///   - `Vec<f32, Global>` : If valid, a dynamically sized array containing parsed coordinates into
  ///   floats. If there's an error when parsing data, the coordinate in question will default to
  ///   the minimum value possible for `f32`.
  fn read_obj_vertex(line_split: Vec<&str>) -> Vec<f32> {
    return line_split.clone().into_iter()
      .map(|position| {
        let result = position.parse();
        if result.is_err() {
          log!(EnumLogColor::Yellow, "WARN", "[ResLoader] -->\t Cannot parse vertex position {0} to \
           f32! Assigning default value instead...", position);
        }
        return result.unwrap_or(f32::MIN);
      }).collect::<Vec<f32>>();
  }
  
  /// Internal function to read an *obj* line containing up to three vertex, normal, and texture indices
  /// (i.e. "f v/vt/vn v/vt/vn v/vt/vn") for each index.
  ///
  /// \
  /// Note that due to the tightly packed architecture of the resulting [REntity], this should work for
  /// 2D vertices as well. Conversely it also will work with more than 3D, which is a **bug**, not a
  /// feature, therefore this function takes into account that the line split provided is valid at
  /// all times.
  ///
  /// # Arguments
  ///
  /// * `line_split`: A dynamically sized [Vec<\[&str\]>] containing each entry on line, delimited by a
  ///   space **AFTER** the first symbol at the start of the line.
  ///   (i.e. \["1.00","0.00","-1.00"]) that will be **consumed upon use**.
  ///
  /// # Returns:
  ///   - `Vec<usize, Global>` : If valid, a dynamically sized array containing parsed coordinates
  /// into unsigned shorts. If there's an error when parsing data, the coordinate in question will default to
  /// the minimum value possible for `u32`.
  ///
  /// # Examples
  ///
  /// "f 5/1/1 2/1/1 6/7/3" => \[\[4,0,0], \[1,0,0], \[5,6,2]]
  fn read_obj_indices(line_split: Vec<&str>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    
    for triplet in line_split.into_iter() {
      let indices: Vec<&str> = triplet.split('/').collect();
      for index in indices {
        let index_result = index.parse::<usize>();
        if index_result.is_err() {
          log!(EnumLogColor::Yellow, "WARN", "[ResLoader] -->\t Cannot parse index {0} to \
              usize! Assigning default value instead...", index);
        }
        result.push(index_result.unwrap_or(usize::MAX) - 1);
      }
    }
    return result;
  }
  
  ///
  ///
  /// # Arguments
  ///
  /// * `line_split`:
  ///
  /// returns: Vec<f32, Global>
  ///
  /// # Examples
  ///
  /// ```
  ///
  /// ```
  fn read_obj_normals(line_split: Vec<&str>) -> Vec<f32> {
    return line_split.into_iter()
      .map(|position| {
        let result = position.parse();
        if result.is_err() {
          log!(EnumLogColor::Yellow, "WARN", "[ResLoader] -->\t Cannot parse normal position {0} to \
           f32! Assigning default value instead...", position);
        }
        return result.unwrap_or(f32::MIN);
      })
      .collect::<Vec<f32>>();
  }
  
  ///
  ///
  /// # Arguments
  ///
  /// * `line_split`:
  ///
  /// returns: Vec<f32, Global>
  ///
  /// # Examples
  ///
  /// ```
  ///
  /// ```
  fn read_obj_texture_coords(line_split: Vec<&str>) -> Vec<f32> {
    return line_split.into_iter()
      .map(|position| {
        let result = position.parse();
        if result.is_err() {
          log!(EnumLogColor::Yellow, "WARN", "[ResLoader] -->\t Cannot parse texture coordinate {0} \
            to f32! Assigning default value instead...", position);
        }
        return result.unwrap_or(f32::MIN);
      })
      .collect::<Vec<f32>>();
  }
  
  ///
  /// Reorganize vertices, normals, and texture coords to fit indices, converting
  /// the resulting object into a struct of arrays, instead of arrays of struct, also known as
  /// a tightly packed dataset.
  ///
  /// # Arguments
  ///
  /// * `vertices`: A [Vec<\[f32\]>] reference containing all vertex positions (x,y,z) of a primitive.
  /// * `indices`: A [Vec<\[u32\]>] reference of index 'triplets', each containing obj-style indices -> (v/vt/vn).
  /// * `normals`: A [Vec<\[f32\]>] reference containing all normal positions (x,y,z) of a primitive.
  /// * `texture_coords`: A [Vec<\[f32\]>] reference containing all texture coord positions (x,y) of a primitive.
  ///
  /// # Returns:
  /// - [REntity] : A new renderable entity containing all attributes relevant to the GPU.
  ///
  /// # Examples
  ///
  /// ```text
  ///
  /// ```
  fn reorganize_data(vertices: &Vec<f32>, indices: &Vec<usize>, normals: &Vec<f32>,
                     texture_coords: &Vec<f32>) -> REntity {
    let mut object: REntity = REntity::default();
    
    for index in (0..indices.len()).step_by(3) {
      object.m_entity_id.push(0);
      
      object.m_vertices.push(vertices[indices[index] * 3]);
      object.m_vertices.push(vertices[(indices[index] * 3) + 1]);
      
      // If our vertex is in 3D space.
      if vertices.len() % 3 == 0 {
        object.m_vertices.push(vertices[(indices[index] * 3) + 2]);
      }
      
      // If we have texture coordinates, add them.
      if indices[index + 1] != usize::MAX - 1 {
        if texture_coords[indices[index + 1] * 2] != f32::MIN {
          object.m_texture_coords.push(texture_coords[indices[index + 1] * 2]);
          object.m_texture_coords.push(texture_coords[(indices[index + 1] * 2) + 1]);
        }
      }
      
      // If we have normals, add them.
      if indices[index + 2] != usize::MAX - 1 {
        if normals[indices[index + 2] * 3] != f32::MIN {
          object.m_normals.push(normals[indices[index + 2] * 3]);
          object.m_normals.push(normals[(indices[index + 2] * 3) + 1]);
          
          // If our normal is in 3D space.
          if normals[(indices[index + 2] * 3) + 2] != f32::MIN {
            object.m_normals.push(normals[(indices[index + 2] * 3) + 2]);
          }
        }
      }
      
      // Assign a color (RGBA) for each vertex. Default color for vertices : Green.
      object.m_colors.push(Color::default());
    }
    object.register();  // Assign a random entity ID common for all vertices.
    return object;
  }
  
  fn load_gltf(_file_name: &str) -> Result<REntity, EnumErrors> {
    todo!()
  }
}