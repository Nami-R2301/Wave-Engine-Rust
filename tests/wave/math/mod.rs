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
  let result = vec2_left - vec2_right;
  
  assert_ne!(result, Vec2 { x: 0, y: 0 });
  assert_eq!(result, Vec2 { x: 0, y: 0 });
}

#[test]
fn test_vec2_mul() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: -1, y: -2 };
  
  assert_eq!(&vec2_left + &vec2_right, Vec2 { x: 0, y: 0 });
}

#[test]
fn test_vec2_div() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: 0, y: -2 };
  
  assert_eq!(&vec2_left / &vec2_right, Vec2 { x: 0, y: -1});
}

#[test]
fn test_vec2_debug() {
  let vec2: Vec2<f32> = Vec2 { x: 1.111, y: 2.222 };
  
  let string: String = format!("{:?}", vec2);
  
  assert_eq!(string.as_str(), "Vec2 { x: 1.111, y: 2.222 }");
}

#[test]
fn test_vec2_eq() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: -1, y: -2 };
  let result: Vec2<i32> = vec2_left - Vec2 { x: 2, y: 4 };
  
  assert_eq!(result, vec2_right);
  assert_ne!(result, vec2_right);
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
  
  assert_eq!(vec3_left + vec3_right, Vec3 { x: 0, y: 0, z: 0 });
}

#[test]
fn test_vec3_sub() {
  let mut vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: -1, y: -2 };
  
  assert_eq!(&vec2_left - &vec2_right, Vec2 { x: 2, y: 4 });
  assert_eq!(&vec2_left - &vec2_right, vec2_left);
  
  let vec2_left_copy: Vec2<i32> = vec2_left;
  vec2_left -= vec2_right;
  assert_eq!(&vec2_left, &vec2_left_copy);
}

#[test]
fn test_vec3_mul() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: -1, y: -2 };
  
  assert_eq!(&vec2_left * &vec2_right, Vec2 { x: 0, y: 0 });
}

#[test]
fn test_vec3_div() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: -1, y: -2 };
  
  assert_eq!(&vec2_left + &vec2_right, Vec2 { x: 0, y: 0 });
}

#[test]
fn test_vec3_eq() {
  let vec2_left: Vec2<i32> = Vec2 { x: 1, y: 2 };
  let vec2_right: Vec2<i32> = Vec2 { x: -1, y: -2 };
  
  assert_eq!(&vec2_left + &vec2_right, Vec2 { x: 0, y: 0 });
}