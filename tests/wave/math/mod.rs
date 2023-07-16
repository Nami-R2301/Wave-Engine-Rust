use app::wave::math::*;

/*
///////////////////////////////////   VEC2  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

#[test]
fn test_vec2_add() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: -1, y: -2 };
  
  assert_eq!(&vec2_left + &vec2_right, Vec2 { x: 0, y: 0 });
  
  let vec2_left: Vec2<f32> = Vec2 { x: 1.0, y: 2.000001 };  // f32's max precision arithmetic => 6.
  let vec2_right: Vec2<f32> = Vec2 { x: -1.0, y: -2.0 };
  
  assert_ne!(&vec2_left + &vec2_right, Vec2 { x: 0.0, y: 0.0 });
  
  let vec2_left: Vec2<f32> = Vec2 { x: 1.0, y: 2.0000001 };  // This should round down due to precision > 6.
  let vec2_right: Vec2<f32> = Vec2 { x: -1.0, y: -2.0 };
  let result = &vec2_left + &vec2_right;
  
  assert_eq!(result, Vec2 { x: 0.0, y: 0.0 });
}

#[test]
fn test_vec2_sub() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: -1, y: -2 };
  
  assert_ne!(&vec2_left - &vec2_right, Vec2 { x: 0, y: 0 });
}

#[test]
fn test_vec2_mul() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: -1, y: -2 };
  
  assert_eq!(&vec2_left + &vec2_right, Vec2 { x: 0, y: 0 });
}

#[test]
fn test_vec2_div() {
  let vec2_left: Vec2<i32> = Vec2 { x: 0, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: 1, y: -2 };
  
  assert_eq!(&vec2_left / &vec2_right, Vec2 { x: 0, y: -1 });
}

#[test]
fn test_vec2_debug() {
  let vec2: Vec2<f32> = Vec2 { x: 1.111111, y: 2.222222 };
  
  let string: String = format!("{0}", vec2);
  
  assert_eq!(string.as_str(), "[Vec2] --> x: 1.111, y: 2.222");
}

#[test]
fn test_vec2_eq() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: -1, y: -2 };
  
  assert_eq!(&vec2_left - &Vec2 { x: 2, y: 4 }, vec2_right);
}

#[test]
fn test_vec2_display() {}

/*
///////////////////////////////////   VEC3  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

#[test]
fn test_vec3_add() {
  let vec3_left: Vec3<i32> = Vec3 { x: 1, y: 2, z: 5 };
  let vec3_right: Vec3<i32> = Vec3 { x: -1, y: -2, z: -5 };
  
  assert_eq!(&vec3_left + &vec3_right, Vec3 { x: 0, y: 0, z: 0 });
}

#[test]
fn test_vec3_sub() {
  let mut vec3_left: Vec3<i32> = Vec3 { x: 1, y: 2, z: 3 };
  let vec3_right: Vec3<i32> = Vec3 { x: -1, y: -2, z: -3 };
  
  vec3_left -= vec3_right;
  assert_eq!(vec3_left, Vec3 { x: 2, y: 4, z: 6 });
  
  vec3_left -= vec3_right;
  assert_eq!(vec3_left, Vec3 { x: 3, y: 6, z: 9 });
}

#[test]
fn test_vec3_mul() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: 0, y: -2 };
  
  assert_eq!(&vec2_left * &vec2_right, Vec2 { x: 0, y: -4 });
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn test_vec3_div() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: 0, y: 0 };
  
  assert_eq!(&vec2_left / &vec2_right, Vec2 { x: 0, y: 0 });
}

#[test]
fn test_vec3_eq() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: -1, y: -2 };
  
  assert_eq!(&vec2_left + &vec2_right, Vec2 { x: 0, y: 0 });
}

/*
///////////////////////////////////   VEC4  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

#[test]
fn test_vec4_add() {
  let vec4_left: Vec4<i32> = Vec4 { x: 1, y: 2, z: 5, w: -1 };
  let vec4_right: Vec4<i32> = Vec4 { x: -1, y: -2, z: -5, w: 1 };
  
  assert_eq!(&vec4_left + &vec4_right, Vec4::new());
}

#[test]
fn test_vec4_sub() {
  let mut vec4_left: Vec4<i32> = Vec4 { x: 1, y: 2, z: 3, w: 4 };
  let vec4_right: Vec4<i32> = Vec4 { x: -1, y: -2, z: -3, w: -4 };
  
  vec4_left -= vec4_right;
  assert_eq!(&vec4_left, &Vec4 { x: 2, y: 4, z: 6, w: 8 });
  
  vec4_left -= vec4_right;
  assert_eq!(&vec4_left, &Vec4 { x: 3, y: 6, z: 9, w: 12 });
}

#[test]
fn test_vec4_mul() {
  let vec4_left: Vec4<i32> = Vec4 { x: 1, y: 2, z: 3, w: -4 };
  let vec4_right: Vec4<i32> = Vec4 { x: 0, y: -2, z: 3, w: -4 };
  
  assert_eq!(&vec4_left * &vec4_right, Vec4 { x: 0, y: -4, z: 9, w: 16 });
}

#[test]
fn test_vec4_div() {
  let vec4_left: Vec4<i32> = Vec4 { x: 1, y: 2, z: 3, w: 4 };
  let vec4_right: Vec4<i32> = Vec4 { x: -1, y: -2, z: -3, w: -4 };
  
  assert_eq!(&vec4_left / &vec4_right, Vec4 { x: -1, y: -1, z: -1, w: -1 });
}

#[test]
fn test_vec4_eq() {
  let vec4_left: Vec4<i32> = Vec4 { x: 1, y: 2, z: 3, w: 4 };
  let vec4_right: Vec4<i32> = Vec4 { x: -1, y: -2, z: -3, w: -4 };
  
  assert_eq!(&(&vec4_left + &Vec4::new()) * &Vec4 { x: -1, y: -1, z: -1, w: -1 },
    vec4_right);
}

#[test]
fn test_matrix_index() {
  let matrix: GlMatrix = GlMatrix {
    value_ptr: Vec4 {
      x: Vec4 { x: 1.0, y: 0.0, z: 0.0, w: 5.0 },
      y: Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 10.0 },
      z: Vec4 { x: 0.0, y: 0.0, z: 1.0, w: 2.0 },
      w: Vec4 { x: 4.0, y: 20.0, z: 11.0, w: 1.0 },
    }
  };
  
  assert_eq!(matrix[0][0], 1.0);
  assert_eq!(matrix[0][3], 5.0);
  assert_eq!(matrix[3][0], 4.0);
  assert_eq!(matrix[3][1], 20.0);
  assert_eq!(matrix[1][2], 0.0);
}

#[test]
fn test_matrix_print() {
  let mut matrix: Box<GlMatrix> = GlMatrix::new(true);
  
  let _string: String = format!("{0}", matrix);
  assert_eq!(_string.as_str(), "[GlMatrix] -->  1.000, 0.000, 0.000, 0.000\n\
                                               0.000, 1.000, 0.000, 0.000\n\
                                               0.000, 0.000, 1.000, 0.000\n\
                                               0.000, 0.000, 0.000, 1.000\n");
  matrix.delete();
  let _string: String = format!("{0}", matrix);
  assert_eq!(_string.as_str(), "[GlMatrix] -->  0.000, 0.000, 0.000, 0.000\n\
                                               0.000, 0.000, 0.000, 0.000\n\
                                               0.000, 0.000, 0.000, 0.000\n\
                                               0.000, 0.000, 0.000, 0.000\n");
  
}