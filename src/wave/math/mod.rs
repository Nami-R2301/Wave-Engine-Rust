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

/*
///////////////////////////////////   VEC2  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

use super::super::create_vec;

create_vec!(Vec2<T> { x, y, });

impl<T> std::ops::Index<usize> for Vec2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        return match index {
            0 => &self.x,
            1 => &self.y,
            _ => &self.x,
        };
    }
}

impl<T> std::ops::IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        return match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.x,
        };
    }
}

/*
///////////////////////////////////   VEC3  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

create_vec!(Vec3<T> {x, y, z, });

///////////////////// INDEXING ////////////////////////

impl<T> std::ops::Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        return match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        };
    }
}

impl<T> std::ops::IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        return match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => &mut self.x,
        };
    }
}

/*
///////////////////////////////////   VEC4  ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
///////////////////////////////////         ///////////////////////////////////
 */

create_vec!(Vec4<T> { x, y, z, w, });

///////////////////// INDEXING ////////////////////////

impl<T> std::ops::Index<usize> for Vec4<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        return match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => &self.x,
        };
    }
}

impl<T> std::ops::IndexMut<usize> for Vec4<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        return match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => &mut self.x,
        };
    }
}

/*
///////////////////////////////////   4X4 MATRICES      ///////////////////////////////////
///////////////////////////////////  (ROW MAJOR ORDER)  ///////////////////////////////////
///////////////////////////////////                     ///////////////////////////////////
 */

#[derive(Debug, Clone)]
pub struct Mat4 {
    pub value_ptr: Vec4<Vec4<f32>>,
}

impl Mat4 {
    pub fn new(initialize_identity: bool) -> Mat4 {
        if initialize_identity {
            return Mat4 {
                value_ptr: Vec4 {
                    x: Vec4 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    y: Vec4 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    z: Vec4 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    w: Vec4 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 1.0,
                    },
                },
            };
        }
        return Mat4 {
            value_ptr: Vec4 {
                x: Vec4::new(),
                y: Vec4::new(),
                z: Vec4::new(),
                w: Vec4::new(),
            },
        };
    }

    pub fn new_shared(initialize_identity: bool) -> Box<Mat4> {
        return Box::new(Mat4::new(initialize_identity));
    }

    pub fn delete(&mut self) {
        self.value_ptr.x.delete();
        self.value_ptr.y.delete();
        self.value_ptr.z.delete();
        self.value_ptr.w.delete();
    }

    pub fn from(vec4s: [[f32; 4]; 4]) -> Self {
        return Mat4 {
            value_ptr: Vec4 {
                x: Vec4::from(&vec4s[0]),
                y: Vec4::from(&vec4s[1]),
                z: Vec4::from(&vec4s[2]),
                w: Vec4::from(&vec4s[3]),
            },
        };
    }

    pub fn transpose(matrix: &Mat4) -> Mat4 {
        let mut result: Mat4 = matrix.clone();

        for i in 0..4usize {
            for j in 0..4usize {
                let mut row_major_value: f32 = matrix[i][j];
                std::mem::swap(&mut result[j][i], &mut row_major_value);
            }
        }
        return result;
    }
}

///////////////////// DISPLAY ////////////////////////

impl std::fmt::Display for Mat4 {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            format,
            "[Mat4] -->  {0:.3}, {1:.3}, {2:.3}, {3:.3}\n\
                                    {4:.3}, {5:.3}, {6:.3}, {7:.3}\n\
                                    {8:.3}, {9:.3}, {10:.3}, {11:.3}\n\
                                    {12:.3}, {13:.3}, {14:.3}, {15:.3}\n",
            &self.value_ptr[0][0],
            &self.value_ptr[0][1],
            &self.value_ptr[0][2],
            &self.value_ptr[0][3],
            &self.value_ptr[1][0],
            &self.value_ptr[1][1],
            &self.value_ptr[1][2],
            &self.value_ptr[1][3],
            &self.value_ptr[2][0],
            &self.value_ptr[2][1],
            &self.value_ptr[2][2],
            &self.value_ptr[2][3],
            &self.value_ptr[3][0],
            &self.value_ptr[3][1],
            &self.value_ptr[3][2],
            &self.value_ptr[3][3]
        )
    }
}

///////////////////// INDEXING ////////////////////////

impl std::ops::Index<usize> for Mat4 {
    type Output = Vec4<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.value_ptr[index];
    }
}

impl std::ops::IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.value_ptr[index];
    }
}

///////////////////// EQUALITY ////////////////////////

impl PartialEq for Mat4 {
    fn eq(&self, other: &Self) -> bool {
        if &self.value_ptr == &other.value_ptr {
            return true;
        }
        for row in 0..4usize {
            for col in 0..4usize {
                if self[row][col] != other[row][col] {
                    return false;
                }
            }
        }
        return true;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

///////////////////// ARITHMETIC ////////////////////////

impl std::ops::Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, other_matrix: Self) -> Mat4 {
        let mut default_matrix: Mat4 = Mat4::new(false);

        for col in 0..4usize {
            default_matrix.value_ptr.x[col] += (self.value_ptr.x.x * other_matrix.value_ptr.x[col])
                + (self.value_ptr.x.y * other_matrix.value_ptr.y[col])
                + (self.value_ptr.x.z * other_matrix.value_ptr.z[col])
                + (self.value_ptr.x.w * other_matrix.value_ptr.w[col]);

            default_matrix.value_ptr.y[col] += (self.value_ptr.y.x * other_matrix.value_ptr.x[col])
                + (self.value_ptr.y.y * other_matrix.value_ptr.y[col])
                + (self.value_ptr.y.z * other_matrix.value_ptr.z[col])
                + (self.value_ptr.y.w * other_matrix.value_ptr.w[col]);

            default_matrix.value_ptr.z[col] += (self.value_ptr.z.x * other_matrix.value_ptr.x[col])
                + (self.value_ptr.z.y * other_matrix.value_ptr.y[col])
                + (self.value_ptr.z.z * other_matrix.value_ptr.z[col])
                + (self.value_ptr.z.w * other_matrix.value_ptr.w[col]);

            default_matrix.value_ptr.w[col] += (self.value_ptr.w.x * other_matrix.value_ptr.x[col])
                + (self.value_ptr.w.y * other_matrix.value_ptr.y[col])
                + (self.value_ptr.w.z * other_matrix.value_ptr.z[col])
                + (self.value_ptr.w.w * other_matrix.value_ptr.w[col]);
        }
        return default_matrix;
    }
}
