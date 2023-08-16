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

use crate::wave::assets::renderable_assets::GlREntity;

use crate::log;

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
  pub fn new(file_name: &str) -> Result<GlREntity, EnumErrors> {
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
  
  /// Internal function to load an *obj*'s file contents into a [GlREntity] container to eventually pass
  /// on to the GPU. Due to the tightly packed oriented dataset of [GlREntity], [GlREntity] can be made of
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
  fn load_obj(file_name: &str) -> Result<GlREntity, EnumErrors> {
    let file = std::fs::File::open(file_name);
    return match file {
      Ok(_) => {
        let buffer = std::io::BufReader::new(file.unwrap()).lines();
        let mut vertices: Vec<f32> = Vec::new();
        let mut normals: Vec<f32> = Vec::new();
        let mut texture_coords: Vec<f32> = Vec::new();
        let mut indices: Vec<Vec<u32>> = Vec::new();
        
        for line in buffer {
          if line.is_err() {
            log!(EnumLogColor::Yellow, "WARN", "[ResLoader] -->\t Cannot read line {0}! \
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
          
          // If data is somehow corrupted.
          if (line_split.len() < 2 || line_split.len() > 4) &&
            (&line.as_ref().unwrap()[0..1] == "v" || &line.as_ref().unwrap()[0..1] == "f") {
            log!(EnumLogColor::Red, "ERROR", "[ResLoader] -->\t Error reading obj file at line {0}, \
              the data does not correspond to a 2D or 3D shape! Fix error and try again...",
              line.unwrap() );
            return Err(EnumErrors::InvalidShapeData);
          }
          
          match &line.as_ref().unwrap()[0..2] {
            "v " => { vertices.append(&mut ResLoader::read_obj_vertex(line_split)); }
            "f " => { indices.append(&mut ResLoader::read_obj_indices(line_split)); }
            "vn" => { normals.append(&mut ResLoader::read_obj_normals(line_split)); }
            "vt" => { texture_coords.append(&mut ResLoader::read_obj_texture_coords(line_split)); }
            _ => {}
          }
        }
        Ok(ResLoader::reorganize_data(vertices, indices, normals, texture_coords))
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[ResLoader] -->\t Cannot open file! Error => {0}", err);
        Err(EnumErrors::InvalidPath)
      }
    };
  }
  
  /// Internal function to read an *obj* line containing a vertex position (i.e. "v 1.00, 0.00, 1.00").
  /// Note that due to the tightly packed architecture of the resulting [GlREntity], [GlREntity] can be made of
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
           f32, on line {1}! Assigning default value of {2} instead...", position,
            unsafe {*line_split.as_ptr() }, f32::MIN);
          return f32::MIN;
        }
        return result.unwrap();
      }).collect::<Vec<f32>>();
  }
  
  /// Internal function to read an *obj* line containing up to three vertex, normal, and texture indices
  /// (i.e. "f v/vt/vn v/vt/vn v/vt/vn") and parse it into a dynamic array of `u32` 'triplets'
  /// `Vec<[v: u32, vt: u32, vn: u32]>`
  ///
  /// \
  /// Note that due to the tightly packed architecture of the resulting [GlREntity], this should work for
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
  ///   - `Vec<Vec<u32>, Global>` : If valid, a dynamically sized array containing parsed coordinates
  /// into unsigned shorts. If there's an error when parsing data, the coordinate in question will default to
  /// the minimum value possible for `u32`.
  ///
  /// # Examples
  ///
  /// "f 5/1/1 2/1/1 6/7/3" => \[\[4,0,0], \[1,0,0], \[5,6,2]]
  fn read_obj_indices(line_split: Vec<&str>) -> Vec<Vec<u32>> {
    return line_split.into_iter()
      .map(|index| {
        let triplet: Vec<&str> = index.split('/').collect();
        triplet.into_iter().map(|index_in_triplet| {
          let result = index_in_triplet.parse::<u32>();
          if result.is_err() {
            log!(EnumLogColor::Yellow, "WARN", "[ResLoader] -->\t Cannot parse index {0} to \
                u16! Assigning default value of {1} instead...", index, u32::MIN);
            return u32::MIN;
          }
          return result.unwrap() - 1;
        })
          .collect()
      })
      .collect::<Vec<Vec<u32>>>();
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
           f32! Assigning default value of {1} instead...", position, f32::MIN);
          return f32::MIN;
        }
        return result.unwrap();
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
            to f32! Assigning default value of {1} instead...", position, f32::MIN);
          return f32::MIN;
        }
        return result.unwrap();
      })
      .collect::<Vec<f32>>();
  }
  
  ///
  /// Reorganize vertices, normals, and texture coords to fit indices and to ultimately convert
  /// the resulting object into a struct of arrays, instead of arrays of struct, also known as
  /// a tightly packed dataset.
  ///
  /// # Arguments
  ///
  /// * `vertices`: A [Vec<\[f32\]>] containing all vertex positions (x,y,z) of a primitive.
  ///   **Consumed on use**.
  /// * `indices`: A [Vec<Vec<\[u32\]>>] of index 'triplets', each containing obj-style indices -> (v/vt/vn).
  ///   **Consumed on use**.
  /// * `normals`: A [Vec<\[f32\]>] containing all normal positions (x,y,z) of a primitive.
  ///   **Consumed on use**.
  /// * `texture_coords`: A [Vec<\[f32\]>] containing all texture coord positions (x,y) of a primitive.
  ///   **Consumed on use**.
  ///
  /// # Returns:
  /// - [GlREntity] : A new renderable entity containing all attributes relevant to the GPU.
  ///
  /// # Examples
  ///
  /// ```text
  ///
  /// ```
  fn reorganize_data(vertices: Vec<f32>, indices: Vec<Vec<u32>>, normals: Vec<f32>,
                     texture_coords: Vec<f32>) -> GlREntity {
    let mut object: GlREntity = GlREntity::new();
    
    for index in indices {
      object.m_vertices.push(vertices[(index[0] * 3) as usize]);
      object.m_vertices.push(vertices[((index[0] * 3) + 1) as usize]);
      object.m_vertices.push(vertices[((index[0] * 3) + 2) as usize]);
      
      object.m_normals.push(normals[(index[2] * 3) as usize]);
      object.m_normals.push(normals[((index[2] * 3) + 1) as usize]);
      object.m_normals.push(normals[((index[2] * 3) + 2) as usize]);
      
      // A color (RGBA) for each position (Vec3).
      object.m_colors.append(&mut Vec::from([0.0, 1.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 1.0]));
      
      object.m_texture_coords.push(texture_coords[(index[1] * 2) as usize]);
      object.m_texture_coords.push(texture_coords[((index[1] * 2) + 1) as usize]);
    }
    object.register();  // Assign a random entity ID common for all vertices.
    return object;
  }
  
  fn load_gltf(_file_name: &str) -> Result<GlREntity, EnumErrors> {
    todo!()
  }
}