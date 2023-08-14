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
use crate::wave::assets::renderable_assets::Object;

/*
///////////////////////////////////   Asset Loader  ///////////////////////////////////
///////////////////////////////////                 ///////////////////////////////////
///////////////////////////////////                 ///////////////////////////////////
 */

#[derive(Debug, Clone, PartialEq)]
pub enum EnumErrors {
  InvalidPath,
  InvalidFileFormat,
  InvalidRead,
}

#[derive(Debug, Clone)]
pub struct ResLoader {}

impl ResLoader {
  pub fn new(file_path: &str) -> Result<Object, EnumErrors> {
    let path = std::path::Path::new(file_path).extension();
    
    return match path {
      None => {
        log!(EnumLogColor::Red, "ERROR", "[ResLoader] -->\t Could not find path {0}! Make sure it \
          exists and you have the appropriate permissions to read it.", file_path);
        Err(EnumErrors::InvalidPath)
      }
      Some(_) => {
        let path_str = path.unwrap().to_str().unwrap();
        match path_str {
          "obj" => { ResLoader::load_obj(file_path) }
          "gltf" => { ResLoader::load_gltf(file_path) }
          &_ => {
            log!(EnumLogColor::Red, "Error", "[ResLoader] -->\t Asset file format {0} not supported!",
              file_path);
            Err(EnumErrors::InvalidFileFormat)
          }
        }
      }
    };
  }
  
  fn load_obj(file_name: &str) -> Result<Object, EnumErrors> {
    let file = std::fs::File::open(file_name);
    return match file {
      Ok(_) => {
        let buffer = std::io::BufReader::new(file.unwrap()).lines();
        let mut vertices: Vec<f32> = Vec::new();
        let mut normals: Vec<f32> = Vec::new();
        let mut texture_coords: Vec<f32> = Vec::new();
        let mut indices: Vec<Vec<u16>> = Vec::new();
        
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
          
          match &line.as_ref().unwrap()[0..2] {
            "v " => { vertices.append(&mut ResLoader::read_vertex(line_split)); }
            "f " => { indices.append(&mut ResLoader::read_indices(line_split)); }
            "vn" => { normals.append(&mut ResLoader::read_normals(line_split)); }
            "vt" => { texture_coords.append(&mut ResLoader::read_texture_coords(line_split)); }
            _ => {}
          }
        }
        Ok(ResLoader::reorganize_data(&vertices, indices, &normals, &texture_coords))
      }
      Err(err) => {
        log!(EnumLogColor::Red, "ERROR", "[ResLoader] -->\t Cannot open file! Error => {0}", err);
        Err(EnumErrors::InvalidFileFormat)
      }
    };
  }
  
  
  fn read_vertex(line_split: Vec<&str>) -> Vec<f32> {
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
  
  fn read_indices(line_split: Vec<&str>) -> Vec<Vec<u16>> {
    return line_split.into_iter()
      .map(|index| {
        let triplet: Vec<&str> = index.split('/').collect();
        triplet.into_iter().map(|index_in_triplet| {
          let result = index_in_triplet.parse::<u16>();
          if result.is_err() {
            log!(EnumLogColor::Yellow, "WARN", "[ResLoader] -->\t Cannot parse index {0} to \
                u16! Assigning default value of {1} instead...", index, u16::MIN);
            return u16::MIN;
          }
          return result.unwrap() - 1;
        })
          .collect()
      })
      .collect::<Vec<Vec<u16>>>();
  }
  
  fn read_normals(line_split: Vec<&str>) -> Vec<f32> {
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
  
  fn read_texture_coords(line_split: Vec<&str>) -> Vec<f32> {
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
  
  fn reorganize_data(vertices: &Vec<f32>, indices: Vec<Vec<u16>>, normals: &Vec<f32>,
                     texture_coords: &Vec<f32>) -> Object {
    let mut gl_object: Object = Object::new();
    
    for triplet in indices {
      gl_object.m_ids.push(1);
      
      gl_object.m_positions.push(vertices[(triplet[0] * 3) as usize]);
      gl_object.m_positions.push(vertices[((triplet[0] * 3) + 1) as usize]);
      gl_object.m_positions.push(vertices[((triplet[0] * 3) + 2) as usize]);
      
      gl_object.m_normals.push(normals[(triplet[2] * 3) as usize]);
      gl_object.m_normals.push(normals[((triplet[2] * 3) + 1) as usize]);
      gl_object.m_normals.push(normals[((triplet[2] * 3) + 2) as usize]);
      
      // A color (RGBA) for each position (Vec3).
      gl_object.m_colors.append(&mut Vec::from([0.0, 1.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 1.0]));
      
      gl_object.m_texture_coords.push(texture_coords[(triplet[1] * 2) as usize]);
      gl_object.m_texture_coords.push(texture_coords[((triplet[1] * 2) + 1) as usize]);
    }
    return gl_object;
  }
  
  fn load_gltf(_file_name: &str) -> Result<Object, EnumErrors> {
    todo!()
  }
}