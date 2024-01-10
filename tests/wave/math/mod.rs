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
use wave_engine::wave::math::*;

/*
///////////////////////////////////   VEC2  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

#[test]
fn test_vec2_add() {
  let vec2_left: Vec2<i32> = Vec2::new(&[1, 2]);
  let vec2_right: Vec2<i32> = Vec2::new(&[-1, -2]);
  
  assert_eq!(vec2_left + vec2_right, Vec2::default());
  
  let vec2_left: Vec2<f32> = Vec2::new(&[1.0, 2.000001]); // f32's max precision arithmetic => 6.
  let vec2_right: Vec2<f32> = Vec2::new(&[-1.0, -2.0]);
  
  assert_ne!(vec2_left + vec2_right, Vec2::default());
  
  let vec2_left: Vec2<f32> = Vec2::new(&[1.0, 2.0000001]); // This should round down due to precision > 6.
  let vec2_right: Vec2<f32> = Vec2::new(&[-1.0, -2.0]);
  let result: Vec2<f32> = vec2_left + vec2_right;
  
  assert_eq!(result, Vec2::default());
}

#[test]
fn test_vec2_sub() {
  let vec2_left: Vec2<i32> = Vec2::new(&[1, 2]);
  let vec2_right: Vec2<i32> = Vec2::new(&[-1, -2]);
  
  assert_ne!(vec2_left - vec2_right, Vec2::default());
}

#[test]
fn test_vec2_mul() {
  let vec2_left: Vec2<i32> = Vec2::new(&[1, 2]);
  let vec2_right: Vec2<i32> = Vec2::new(&[-1, -2]);
  
  assert_eq!(vec2_left + vec2_right, Vec2::default());
}

#[test]
fn test_vec2_div() {
  let vec2_left: Vec2<i32> = Vec2::new(&[0, 2]);
  let vec2_right: Vec2<i32> = Vec2::new(&[1, -2]);
  
  assert_eq!(vec2_left / vec2_right, Vec2::new(&[0, -1]));
}

#[test]
fn test_vec2_debug() {
  let vec2: Vec2<f32> = Vec2::new(&[1.111111, 2.222222]);
  
  let string: String = format!("{0}", vec2);
  
  assert_eq!(string.as_str(), "[Vec2] --> x: 1.111, y: 2.222, ");
}

#[test]
fn test_vec2_eq() {
  let vec2_left: Vec2<i32> = Vec2::new(&[1, 2]);
  let vec2_right: Vec2<i32> = Vec2::new(&[-1, -2]);
  
  assert_eq!(vec2_left - Vec2::new(&[2, 4]), vec2_right);
}

/*
///////////////////////////////////   VEC3  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

#[test]
fn test_vec3_add() {
  let vec3_left: Vec3<i32> = Vec3::new(&[1, 2, 5]);
  let vec3_right: Vec3<i32> = Vec3::new(&[-1, -2, -5]);
  
  assert_eq!(vec3_left + vec3_right, Vec3::default());
}

#[test]
fn test_vec3_sub() {
  let mut vec3_left: Vec3<i32> = Vec3::new(&[1, 2, 3]);
  let vec3_right: Vec3<i32> = Vec3::new(&[-1, -2, -3]);
  
  vec3_left -= Vec3::new(&[-1, -2, -3]);
  assert_eq!(vec3_left, Vec3::new(&[2, 4, 6]));
  
  vec3_left -= vec3_right;
  assert_eq!(vec3_left, Vec3::new(&[3, 6, 9]));
}

#[test]
fn test_vec3_mul() {
  let vec2_left: Vec2<i32> = Vec2::new(&[1, 2]);
  let vec2_right: Vec2<i32> = Vec2::new(&[0, -2]);
  
  assert_eq!(vec2_left * vec2_right, Vec2::new(&[0, -4]));
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn test_vec3_div() {
  let vec2_left: Vec2<i32> = Vec2::new(&[1, 2]);
  let vec2_right: Vec2<i32> = Vec2::default();
  
  assert_eq!(vec2_left / vec2_right, Vec2::default());
}

#[test]
fn test_vec3_eq() {
  let vec2_left: Vec2<i32> = Vec2::new(&[1, 2]);
  let vec2_right: Vec2<i32> = Vec2::new(&[-1, -2]);
  
  assert_eq!(vec2_left + vec2_right, Vec2::default());
}

/*
///////////////////////////////////   VEC4  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

#[test]
fn test_vec4_add() {
  let vec4_left: Vec4<i32> = Vec4::new(&[1, 2, 5, -1]);
  let vec4_right: Vec4<i32> = Vec4::new(&[-1, -2, -5, 1]);
  
  assert_eq!(vec4_left + vec4_right, Vec4::default());
}

#[test]
fn test_vec4_sub() {
  let mut vec4_left: Vec4<i32> = Vec4::new(&[1, 2, 3, 4]);
  let vec4_right: Vec4<i32> = Vec4::new(&[-1, -2, -3, -4]);
  
  vec4_left -= vec4_right.clone();
  assert_eq!(&vec4_left, &Vec4::new(&[2, 4, 6, 8]));
  
  vec4_left -= vec4_right;
  assert_eq!(vec4_left, Vec4::new(&[3, 6, 9, 12]));
}

#[test]
fn test_vec4_mul() {
  let vec4_left: Vec4<i32> = Vec4::new(&[1, 2, 3, -4]);
  let vec4_right: Vec4<i32> = Vec4::new(&[0, -2, 3, -4]);
  
  assert_eq!(vec4_left * vec4_right, Vec4::new(&[0, -4, 9, 16]));
}

#[test]
fn test_vec4_div() {
  let vec4_left: Vec4<i32> = Vec4::new(&[1, 2, 3, 4]);
  let vec4_right: Vec4<i32> = Vec4::new(&[-1, -2, -3, -4]);
  
  assert_eq!(vec4_left / vec4_right, Vec4::new(&[-1, -1, -1, -1]));
}

#[test]
fn test_vec4_eq() {
  let vec4_left: Vec4<i32> = Vec4::new(&[1, 2, 3, 4]);
  let vec4_right: Vec4<i32> = Vec4::new(&[-1, -2, -3, -4]);
  
  assert_eq!((vec4_left + Vec4::default()) * Vec4::new(&[-1, -1, -1, -1]), vec4_right);
}


/*
///////////////////////////////////   MATRIX  ///////////////////////////////////
///////////////////////////////////           ///////////////////////////////////
///////////////////////////////////           ///////////////////////////////////
 */

#[test]
fn test_matrix_index() {
  let mut matrix: Mat4 = Mat4 {
    m_value_ptr: Vec4 {
      x: Vec4::new(&[1.0, 0.0, 0.0, 5.0]),
      y: Vec4::new(&[0.0, 1.0, 0.0, 10.0]),
      z: Vec4::new(&[0.0, 0.0, 1.0, 2.0]),
      w: Vec4::new(&[4.0, 20.0, 11.0, 1.0]),
    },
  };
  
  assert_eq!(matrix[0][0], 1.0);
  assert_eq!(matrix[0][3], 5.0);
  assert_eq!(matrix[3][0], 4.0);
  assert_eq!(matrix[3][1], 20.0);
  assert_eq!(matrix[1][2], 0.0);
  
  matrix.delete();
  assert_eq!(matrix, Mat4::new(0.0));
}

#[test]
fn test_matrix_print() {
  let mut matrix: Mat4 = Mat4::new(1.0);
  assert_eq!(matrix.to_string(), "[Mat4] -->  1.000, 0.000, 0.000, 0.000\n\
                                               0.000, 1.000, 0.000, 0.000\n\
                                               0.000, 0.000, 1.000, 0.000\n\
                                               0.000, 0.000, 0.000, 1.000\n"
  );
  matrix.delete();
  assert_eq!(matrix.to_string(), "[Mat4] -->  0.000, 0.000, 0.000, 0.000\n\
                                               0.000, 0.000, 0.000, 0.000\n\
                                               0.000, 0.000, 0.000, 0.000\n\
                                               0.000, 0.000, 0.000, 0.000\n"
  );
}

#[test]
fn test_matrix_transpose() {
  let mut matrix: Mat4 = Mat4::new(1.0);
  matrix[0][3] = 10.0;
  matrix[1][3] = 5.0;
  matrix[2][3] = 2.5;
  
  let transposed_matrix: Mat4 = matrix.transpose();
  
  assert_eq!(transposed_matrix.to_string(), "[Mat4] -->  1.000, 0.000, 0.000, 0.000\n\
                                               0.000, 1.000, 0.000, 0.000\n\
                                               0.000, 0.000, 1.000, 0.000\n\
                                               10.000, 5.000, 2.500, 1.000\n");
  assert_eq!(matrix.to_string(), "[Mat4] -->  1.000, 0.000, 0.000, 10.000\n\
                                               0.000, 1.000, 0.000, 5.000\n\
                                               0.000, 0.000, 1.000, 2.500\n\
                                               0.000, 0.000, 0.000, 1.000\n");
}

#[test]
fn test_matrix_mul() {
  let mut matrix: Mat4 = Mat4::new(1.0);
  matrix[0][3] = 10.0;
  matrix[1][3] = 5.0;
  matrix[2][3] = 2.5;
  
  let transposed_matrix: Mat4 = matrix.transpose();
  assert_eq!(transposed_matrix.to_string(), "[Mat4] -->  1.000, 0.000, 0.000, 0.000\n\
                                                             0.000, 1.000, 0.000, 0.000\n\
                                                             0.000, 0.000, 1.000, 0.000\n\
                                                             10.000, 5.000, 2.500, 1.000\n"
  );
  assert_eq!(matrix.to_string(), "[Mat4] -->  1.000, 0.000, 0.000, 10.000\n\
                                                  0.000, 1.000, 0.000, 5.000\n\
                                                  0.000, 0.000, 1.000, 2.500\n\
                                                  0.000, 0.000, 0.000, 1.000\n"
  );
  
  assert_eq!((matrix * transposed_matrix).to_string(), "[Mat4] -->  101.000, 50.000, 25.000, 10.000\n\
                     50.000, 26.000, 12.500, 5.000\n\
                     25.000, 12.500, 7.250, 2.500\n\
                     10.000, 5.000, 2.500, 1.000\n"
  );
}
