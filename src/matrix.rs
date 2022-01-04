use crate::fuzzy_eq::*;
use crate::tuple::*;

use std::convert::From;
use std::ops::{Index, IndexMut, Mul};

type Matrix2fArrayRow = [f64; 2];
type Matrix3fArrayRow = [f64; 3];
type Matrix4fArrayRow = [f64; 4];
type Matrix2fArray = [Matrix2fArrayRow; 2];
type Matrix3fArray = [Matrix3fArrayRow; 3];
type Matrix4fArray = [Matrix4fArrayRow; 4];

// @TODO: refactor to utilize one Matrix struct is the future.
// Are const templete parameters on option?

#[derive(Debug, Clone, Copy)]
pub struct Matrix2f {
    data: Matrix2fArray,
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix3f {
    data: Matrix3fArray,
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix4f {
    data: Matrix4fArray,
}

impl Matrix4f {
    pub fn new() -> Matrix4f {
        Matrix4f::from([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ])
    }

    pub fn identity() -> Matrix4f {
        Matrix4f::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Matrix3f {
    fn new() -> Matrix3f {
        Matrix3f::from([
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
        ])
    }
}

impl Matrix2f {
    fn new() -> Matrix2f {
        Matrix2f::from([
            [0.0, 0.0],
            [0.0, 0.0],
        ])
    }
}

impl From<Matrix2fArray> for Matrix2f {
    fn from(data: Matrix2fArray) -> Self {
        Matrix2f { data }
    }
}

impl From<Matrix3fArray> for Matrix3f {
    fn from(data: Matrix3fArray) -> Self {
        Matrix3f { data }
    }
}

impl From<Matrix4fArray> for Matrix4f {
    fn from(data: Matrix4fArray) -> Self {
        Matrix4f { data }
    }
}

impl Index<usize> for Matrix2f {
    type Output = Matrix2fArrayRow;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Index<usize> for Matrix3f {
    type Output = Matrix3fArrayRow;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Index<usize> for Matrix4f {
    type Output = Matrix4fArrayRow;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix4f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl IndexMut<usize> for Matrix3f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl IndexMut<usize> for Matrix2f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl FuzzyEq<Matrix2f> for Matrix2f {
    fn fuzzy_eq(&self, other: &Matrix2f) -> bool {
        self[0][0].fuzzy_eq(&other[0][0]) 
            && self[0][1].fuzzy_eq(&other[0][1])
            && self[1][0].fuzzy_eq(&other[1][0])
            && self[1][1].fuzzy_eq(&other[1][1])
    }
}

impl FuzzyEq<Matrix3f> for Matrix3f {
    fn fuzzy_eq(&self, other: &Matrix3f) -> bool {
        self[0][0].fuzzy_eq(&other[0][0])
            && self[0][1].fuzzy_eq(&other[0][1])
            && self[0][2].fuzzy_eq(&other[0][2])
            && self[1][0].fuzzy_eq(&other[1][0])
            && self[1][1].fuzzy_eq(&other[1][1])
            && self[1][2].fuzzy_eq(&other[1][2])
            && self[2][0].fuzzy_eq(&other[2][0])
            && self[2][1].fuzzy_eq(&other[2][1])
            && self[2][2].fuzzy_eq(&other[2][2])
    }
}

impl FuzzyEq<Matrix4f> for Matrix4f {
    fn fuzzy_eq(&self, other: &Matrix4f) -> bool {
        self[0][0].fuzzy_eq(&other[0][0])
            && self[0][1].fuzzy_eq(&other[0][1])
            && self[0][2].fuzzy_eq(&other[0][2])
            && self[0][3].fuzzy_eq(&other[0][3])
            && self[1][0].fuzzy_eq(&other[1][0])
            && self[1][1].fuzzy_eq(&other[1][1])
            && self[1][2].fuzzy_eq(&other[1][2])
            && self[1][3].fuzzy_eq(&other[1][3])
            && self[2][0].fuzzy_eq(&other[2][0])
            && self[2][1].fuzzy_eq(&other[2][1])
            && self[2][2].fuzzy_eq(&other[2][2])
            && self[2][3].fuzzy_eq(&other[2][3])
            && self[3][0].fuzzy_eq(&other[3][0])
            && self[3][1].fuzzy_eq(&other[3][1])
            && self[3][2].fuzzy_eq(&other[3][2])
            && self[3][3].fuzzy_eq(&other[3][3])
    }
}

impl Mul<Matrix4f> for Matrix4f {
    type Output = Matrix4f;

    fn mul(self, other: Matrix4f) -> Self::Output {
        let mut matrix = Matrix4f::new();
        
        for row in 0..4 {
            for column in 0..4 {
                matrix[row][column] = self[row][0] * other[0][column]
                                    + self[row][1] * other[1][column]
                                    + self[row][2] * other[2][column]
                                    + self[row][3] * other[3][column];
            }
        }
        matrix
    }
}

impl Mul<Tuple> for Matrix4f {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        Tuple::new(
            self[0][0] * other.x + self[0][1] * other.y + self[0][2] * other.z + self[0][3] * other.w,
            self[1][0] * other.x + self[1][1] * other.y + self[1][2] * other.z + self[1][3] * other.w,
            self[2][0] * other.x + self[2][1] * other.y + self[2][2] * other.z + self[2][3] * other.w,
            self[3][0] * other.x + self[3][1] * other.y + self[3][2] * other.z + self[3][3] * other.w,
            
        )
    }
}

impl Mul<Matrix3f> for Matrix3f {
    type Output = Matrix3f;

    fn mul(self, other: Matrix3f) -> Self::Output {
        let mut matrix = Matrix3f::new();

        for row in 0..3 {
            for column in 0..3 {
                matrix[row][column] = self[row][0] * other[0][column]
                                    + self[row][1] * other[1][column]
                                    + self[row][2] * other[2][column];
            }
        }
        matrix
    }
}

impl Mul<Matrix2f> for Matrix2f {
    type Output = Matrix2f;

    fn mul(self, other: Matrix2f) -> Self::Output {
        let mut matrix = Matrix2f::new();

        for row in 0..2 {
            for column in 0..2 {
                matrix[row][column] = self[row][0] * other[0][column]
                                    + self[row][1] * other[1][column]
            }
        }
        matrix
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn constructing_and_inspecting_4x4_matrix() {
        let matrix = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(matrix[0][0], 1.0);
        assert_eq!(matrix[0][1], 2.0);
        assert_eq!(matrix[0][2], 3.0);
        assert_eq!(matrix[0][3], 4.0);
        assert_eq!(matrix[1][0], 5.5);
        assert_eq!(matrix[1][1], 6.5);
        assert_eq!(matrix[1][2], 7.5);
        assert_eq!(matrix[1][3], 8.5);
        assert_eq!(matrix[2][0], 9.0);
        assert_eq!(matrix[2][1], 10.0);
        assert_eq!(matrix[2][2], 11.0);
        assert_eq!(matrix[2][3], 12.0);
        assert_eq!(matrix[3][0], 13.5);
        assert_eq!(matrix[3][1], 14.5);
        assert_eq!(matrix[3][2], 15.5);
        assert_eq!(matrix[3][3], 16.5);
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let matrix = Matrix2f::from([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(matrix[0][0], -3.0);
        assert_eq!(matrix[0][1], 5.0);
        assert_eq!(matrix[1][0], 1.0);
        assert_eq!(matrix[1][1], -2.0);
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let matrix = Matrix3f::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_eq!(matrix[0][0], -3.0);
        assert_eq!(matrix[0][1], 5.0);
        assert_eq!(matrix[0][2], 0.0);
        assert_eq!(matrix[1][0], 1.0);
        assert_eq!(matrix[1][1], -2.0);
        assert_eq!(matrix[1][2], -7.0);
        assert_eq!(matrix[2][0], 0.0);
        assert_eq!(matrix[2][1], 1.0);
        assert_eq!(matrix[2][2], 1.0);
    }

    #[test]
    fn matrix_equality_with_identical_2x2_matrices() {
        let matrix1 = Matrix2f::from([[0.123456789, 1.0], [2.0, 3.0]]);
        let matrix2 = Matrix2f::from([[0.123456788, 1.0], [2.0, 3.0]]);

        assert_fuzzy_eq!(matrix1, matrix2);
        // assert!(matrix1.fuzzy_eq(&matrix2));
    }

    #[test]
    fn matrix_equality_with_almost_identical_2x2_matrices() {
        let matrix1 = Matrix2f::from([[0.123456789, 1.0], [2.0, 3.0]]);
        let matrix2 = Matrix2f::from([[0.123456780, 1.0], [2.0, 3.0]]);

        assert_fuzzy_eq!(matrix1, matrix2);
        // assert!(matrix1.fuzzy_eq(&matrix2));

    }

    #[test]
    fn matrix_equality_with_identical_3x3_matrices() {
        let matrix1 = Matrix3f::from([[0.123456789, 1.0, 2.0], [2.0, 3.0, 4.0], [5.0, 6.0, 7.77777777777777777]]);
        let matrix2 = Matrix3f::from([[0.123456780, 1.0, 2.0], [2.0, 3.0, 4.0], [5.0, 6.0, 7.77777777777777777]]);

        assert_fuzzy_eq!(matrix1, matrix2);
    }

    #[test]
    fn matrix_equality_with_almost_identical_3x3_matrices() {
        let matrix1 = Matrix3f::from([[0.123456789, 1.0, 2.0], [2.0, 3.0, 4.0], [5.0, 6.0, 7.77777777777777777]]);
        let matrix2 = Matrix3f::from([[0.123456789, 1.0, 2.0], [2.0, 3.0, 4.0], [5.0, 6.0, 7.77777777777777]]);

        assert_fuzzy_eq!(matrix1, matrix2);
    }

    #[test]
    fn matrix_equality_with_identical_4x4_matrices() {
        let matrix1 = Matrix4f::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let matrix2 = Matrix4f::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_fuzzy_eq!(matrix1, matrix2);
    }

    #[test]
    fn matrix_equality_with_almost_identical_4x4_matrices() {
        let matrix1 = Matrix4f::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0000000000001],
        ]);

        let matrix2 = Matrix4f::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_fuzzy_eq!(matrix1, matrix2);
    }

    #[test]
    fn matrix_inequality_with_non_identical_4x4_matrices() {
        let matrix1 = Matrix4f::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let matrix2 = Matrix4f::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 2.0],
        ]);

        assert_fuzzy_ne!(matrix1, matrix2);
    }

    #[test]
    fn multiplying_two_4x4_matrices() {
        let matrix1 = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2 = Matrix4f::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]); 

        let expected_result = Matrix4f::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        let actual_result = matrix1 * matrix2;

        assert_fuzzy_eq!(actual_result, expected_result);

    }

    #[test]
    fn multiplying_4x4_matrix_by_identity_matrix() {
        let matrix1 = Matrix4f::from([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        let matrix2 = Matrix4f::identity();

        let expected_result = matrix1;
        let actual_result = matrix1 * matrix2;

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn a_4x4_matrix_multiplied_by_a_point() {
        let matrix = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let point = Tuple::point(1.0, 2.0, 3.0);

        let expected_result = Tuple::point(18.0, 24.0, 33.0);
        let actual_result = matrix * point;

        assert_fuzzy_eq!(actual_result, expected_result);
    }
}
