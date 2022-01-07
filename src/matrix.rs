use crate::fuzzy_eq::*;
use crate::tuple::*;

use std::convert::From;
use std::ops::{Index, IndexMut, Mul};
use std::usize;
use num_traits::Float;

// type Matrix2f = Matrix<f64, 2>;
// type Matrix3f = Matrix<f64, 3>;

// // type Matrix2fArrayRow = [f64; 2];
// // type Matrix3fArrayRow = [f64; 3];
// type Matrix4fArrayRow = [f64; 4];
// // type Matrix2fArray = [Matrix2fArrayRow; 2];
// // type Matrix3fArray = [Matrix3fArrayRow; 3];
// type Matrix4fArray = [Matrix4fArrayRow; 4];

// @TODO: refactor to utilize one Matrix struct is the future.
// Are const templete parameters on option?

#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const D: usize> where T: Float {
    data: [[T; D]; D],
}

// #[derive(Debug, Clone, Copy)]
// pub struct Matrix3f {
//     data: Matrix3fArray,
// }

// #[derive(Debug, Clone, Copy)]
// pub struct Matrix4f {
//     data: Matrix4fArray,
// }

// 4x4 Matrix
impl<T> Matrix<T, 4> where T: Float {
    // pub fn new() -> Matrix4f {
    //     Matrix4f::from([
    //         [0.0, 0.0, 0.0, 0.0],
    //         [0.0, 0.0, 0.0, 0.0],
    //         [0.0, 0.0, 0.0, 0.0],
    //         [0.0, 0.0, 0.0, 0.0],
    //     ])
    // }

    // pub fn identity() -> Matrix4f {
    //     Matrix4f::from([
    //         [1.0, 0.0, 0.0, 0.0],
    //         [0.0, 1.0, 0.0, 0.0],
    //         [0.0, 0.0, 1.0, 0.0],
    //         [0.0, 0.0, 0.0, 1.0],
    //     ])
    // }

    

    pub fn submatrix(&self, row: usize, column: usize) -> Matrix<T, 3> {
        let mut matrix = Matrix::new();

        let mut source_row: usize = 0;
        let mut source_column: usize = 0;
        let mut target_row: usize = 0;
        let mut target_column: usize = 0;

        while target_row < 3 {
            if source_row == row {
                source_row += 1
            }

            while target_column < 3 {
                if source_column == column {
                    source_column += 1
                }
                matrix[target_row][target_column] = self[source_row][source_column];

                source_column += 1;
                target_column += 1;
            }

            target_row += 1;
            target_column = 0;

            source_row += 1;
            source_column = 0;
        }
        matrix
    }

    pub fn minor(&self, row: usize, column: usize) -> T {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> T {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            // Even value
            minor
        } else {
            -minor
        }
    }

    pub fn determinant(&self) -> T {
        let mut determinant: T = T::zero();
        
        for column in 0..4 {
            determinant = determinant + self.cofactor(0, column) * self[0][column];
        } 

        determinant
    }

    pub fn is_invertible(&self) -> bool where T: FuzzyEq<T> {
        self.determinant().fuzzy_ne(&T::zero())
    }

    pub fn inverse(&self) -> Matrix<T, 4> where T: FuzzyEq<T> {
        if !self.is_invertible() {
            panic!("Matrix is not invertible, but inverse was called!");
        }

        let mut matrix = Matrix::new();
        let determinant = self.determinant();

        for row in 0..4 {
            for column in 0..4 {
                let cofactor = self.cofactor(row, column);
                
                // transposed storage
                matrix[column][row] = cofactor / determinant;
            }
        }

        matrix
    }
}

// 3x3 Matrix
impl<T, const D: usize> Matrix<T, D> where T: Float {

    // Created "new matrix" according to all "NxN matrices" in Generic way
    fn new() -> Matrix<T, D> {
        Matrix::from([[T::zero(); D]; D])
    }
    
    // for 4x4 matrix
    pub fn diagonal(value: T) -> Matrix<T, D> {
        let mut matrix = Matrix::new();
        
        for i in 0..D {
            matrix[i][i] = value
        }

        matrix
    }

    // for 4x4 matrix
    pub fn identity() -> Matrix<T, D> {
        Matrix::diagonal(T::one())
    }

    // for 4x4 matrix
    pub fn traspose(&self) -> Matrix<T, D> {
        let mut matrix: Matrix<T, D> = Matrix::new();

        for row in 0..D {
            for column in 0..D {
                matrix[row][column] = self[column][row];
            }
        }
        matrix
    }

}

impl<T> Matrix<T, 3> where T: Float {
    // fn new() -> Matrix3f {
    //     Matrix3f::from([[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
    // }

    // @FIX_IT: Find a nicer way to do this.
    pub fn submatrix(&self, row: usize, column: usize) -> Matrix<T, 2> {
        let mut matrix: Matrix<T, 2> = Matrix::new();

        let mut source_row: usize = 0;
        let mut source_column: usize = 0;
        let mut target_row: usize = 0;
        let mut target_column: usize = 0;

        while target_row < 2 {
            if source_row == row {
                // skip row to be removed
                source_row += 1;
            }

            while target_column < 2 {
                if source_column == column {
                    //skip column to be removed
                    source_column += 1;
                }
                matrix[target_row][target_column] = self[source_row][source_column];

                source_column += 1;
                target_column += 1;
            }
            source_row += 1;
            source_column = 0;

            target_row += 1;
            target_column = 0;
        }

        matrix
    }

    pub fn minor(&self, row: usize, column: usize) -> T {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> T {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            // Even value
            minor
        } else {
            -minor
        }
    }

    pub fn determinant(&self) -> T {
        let mut determinant: T = T::zero();
        
        for column in 0..3 {
            determinant = determinant + self.cofactor(0, column) * self[0][column];
        } 

        determinant
    }
}

// 2x2 Matrix
impl<T> Matrix<T, 2> where T: Float {
    pub fn determinant(&self) -> T {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

// Created "from matrix" according to all "NxN matrices" in Generic way
impl<T, const D: usize> From<[[T; D]; D]> for Matrix<T, D> where T: Float {
    fn from(data: [[T; D]; D]) -> Self {
        Matrix { data }
    }
}

// impl From<Matrix3fArray> for Matrix3f {
//     fn from(data: Matrix3fArray) -> Self {
//         Matrix3f { data }
//     }
// }

// impl From<MatrixArray> for Matrix {
//     fn from(data: MatrixArray) -> Self {
//         Matrix { data }
//     }
// }

// Created "Index function for all NxN matrix" in Generic way
impl<T, const D: usize> Index<usize> for Matrix<T, D> where T: Float {
    type Output = [T; D];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// impl Index<usize> for Matrix3f {
//     type Output = Matrix3fArrayRow;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.data[index]
//     }
// }

// impl Index<usize> for Matrix4f {
//     type Output = Matrix4fArrayRow;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.data[index]
//     }
// }

// impl IndexMut<usize> for Matrix4f {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.data[index]
//     }
// }

// impl IndexMut<usize> for Matrix3f {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.data[index]
//     }
// }

// Created "IndexMut function for all NxN matrix" in Generic way
impl<T, const D: usize> IndexMut<usize> for Matrix<T, D> where T: Float {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const D: usize> FuzzyEq<Self> for Matrix<T, D> where T: Float, T: FuzzyEq<T> {

    fn fuzzy_eq(&self, other: &Self) -> bool {
        for row in 0..D {
            for column in 0..D {
                if self[row][column].fuzzy_eq(&other[row][column]) {
                    return true;
                }
            }
        }
        return false;
    }

    // fn fuzzy_eq(&self, other: &Self) -> bool {
    //     self[0][0].fuzzy_eq(&other[0][0])
    //         && self[0][1].fuzzy_eq(&other[0][1])
    //         && self[1][0].fuzzy_eq(&other[1][0])
    //         && self[1][1].fuzzy_eq(&other[1][1])
    // }
}

// impl FuzzyEq<Matrix3f> for Matrix3f {
//     fn fuzzy_eq(&self, other: &Matrix3f) -> bool {
//         self[0][0].fuzzy_eq(&other[0][0])
//             && self[0][1].fuzzy_eq(&other[0][1])
//             && self[0][2].fuzzy_eq(&other[0][2])
//             && self[1][0].fuzzy_eq(&other[1][0])
//             && self[1][1].fuzzy_eq(&other[1][1])
//             && self[1][2].fuzzy_eq(&other[1][2])
//             && self[2][0].fuzzy_eq(&other[2][0])
//             && self[2][1].fuzzy_eq(&other[2][1])
//             && self[2][2].fuzzy_eq(&other[2][2])
//     }
// }

// impl FuzzyEq<Matrix4f> for Matrix4f {
//     fn fuzzy_eq(&self, other: &Matrix4f) -> bool {
//         self[0][0].fuzzy_eq(&other[0][0])
//             && self[0][1].fuzzy_eq(&other[0][1])
//             && self[0][2].fuzzy_eq(&other[0][2])
//             && self[0][3].fuzzy_eq(&other[0][3])
//             && self[1][0].fuzzy_eq(&other[1][0])
//             && self[1][1].fuzzy_eq(&other[1][1])
//             && self[1][2].fuzzy_eq(&other[1][2])
//             && self[1][3].fuzzy_eq(&other[1][3])
//             && self[2][0].fuzzy_eq(&other[2][0])
//             && self[2][1].fuzzy_eq(&other[2][1])
//             && self[2][2].fuzzy_eq(&other[2][2])
//             && self[2][3].fuzzy_eq(&other[2][3])
//             && self[3][0].fuzzy_eq(&other[3][0])
//             && self[3][1].fuzzy_eq(&other[3][1])
//             && self[3][2].fuzzy_eq(&other[3][2])
//             && self[3][3].fuzzy_eq(&other[3][3])
//     }
// }

impl<T, const D: usize> Mul<Matrix<T, D>> for Matrix<T, D> 
    where T: Float{
        type Output = Matrix<T, D>;

        fn mul(self, other: Matrix<T, D>) -> Self::Output {
            let mut matrix = Matrix::new();

            for row in 0..D {
                for column in 0..D {
                    for i in 0..D {
                        matrix[row][column] = matrix[row][column] + self[row][i] * other[i][column];
                    }
                }
            }

            matrix

        // for row in 0..4 {
        //     for column in 0..4 {
        //         matrix[row][column] = self[row][0] * other[0][column]
        //             + self[row][1] * other[1][column]
        //             + self[row][2] * other[2][column]
        //             + self[row][3] * other[3][column];
        //     }
        // }
        // matrix
        }
}

// @TODO: Generalize once Tuple has been refactored to use Float trait
impl<T> Mul<Tuple<T>> for Matrix<T, 4> where T: Float {
    type Output = Tuple<T>;

    fn mul(self, other: Tuple<T>) -> Self::Output {
        Tuple::new(
            self[0][0] * other.x
                + self[0][1] * other.y
                + self[0][2] * other.z
                + self[0][3] * other.w,
            self[1][0] * other.x
                + self[1][1] * other.y
                + self[1][2] * other.z
                + self[1][3] * other.w,
            self[2][0] * other.x
                + self[2][1] * other.y
                + self[2][2] * other.z
                + self[2][3] * other.w,
            self[3][0] * other.x
                + self[3][1] * other.y
                + self[3][2] * other.z
                + self[3][3] * other.w,
        )
    }
}

// I have not used this multiplication impl for 3x3 Matrix
// impl Mul<Matrix3f> for Matrix3f {
//     type Output = Matrix3f;

//     fn mul(self, other: Matrix3f) -> Self::Output {
//         let mut matrix = Matrix3f::new();

//         for row in 0..3 {
//             for column in 0..3 {
//                 matrix[row][column] = self[row][0] * other[0][column]
//                     + self[row][1] * other[1][column]
//                     + self[row][2] * other[2][column];
//             }
//         }
//         matrix
//     }
// }

// I have not used this multiplication impl for 2x2 Matrix
// impl Mul<Matrix2f> for Matrix2f {
//     type Output = Matrix2f;

//     fn mul(self, other: Matrix2f) -> Self::Output {
//         let mut matrix = Matrix::new();

//         for row in 0..2 {
//             for column in 0..2 {
//                 matrix[row][column] =
//                     self[row][0] * other[0][column] + self[row][1] * other[1][column]
//             }
//         }
//         matrix
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn constructing_and_inspecting_4x4_matrix() {
        let matrix = Matrix::from([
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
        let matrix = Matrix::from([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(matrix[0][0], -3.0);
        assert_eq!(matrix[0][1], 5.0);
        assert_eq!(matrix[1][0], 1.0);
        assert_eq!(matrix[1][1], -2.0);
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let matrix = Matrix::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

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
        let matrix1 = Matrix::from([[0.123456789, 1.0], [2.0, 3.0]]);
        let matrix2 = Matrix::from([[0.123456788, 1.0], [2.0, 3.0]]);

        assert_fuzzy_eq!(matrix1, matrix2);
        // assert!(matrix1.fuzzy_eq(&matrix2));
    }

    #[test]
    fn matrix_equality_with_almost_identical_2x2_matrices() {
        let matrix1 = Matrix::from([[0.123456789, 1.0], [2.0, 3.0]]);
        let matrix2 = Matrix::from([[0.123456780, 1.0], [2.0, 3.0]]);

        assert_fuzzy_eq!(matrix1, matrix2);
        // assert!(matrix1.fuzzy_eq(&matrix2));
    }

    #[test]
    fn matrix_equality_with_identical_3x3_matrices() {
        let matrix1 = Matrix::from([
            [0.123456789, 1.0, 2.0],
            [2.0, 3.0, 4.0],
            [5.0, 6.0, 7.77777777777777777],
        ]);
        let matrix2 = Matrix::from([
            [0.123456780, 1.0, 2.0],
            [2.0, 3.0, 4.0],
            [5.0, 6.0, 7.77777777777777777],
        ]);

        assert_fuzzy_eq!(matrix1, matrix2);
    }

    #[test]
    fn matrix_equality_with_almost_identical_3x3_matrices() {
        let matrix1 = Matrix::from([
            [0.123456789, 1.0, 2.0],
            [2.0, 3.0, 4.0],
            [5.0, 6.0, 7.77777777777777777],
        ]);
        let matrix2 = Matrix::from([
            [0.123456789, 1.0, 2.0],
            [2.0, 3.0, 4.0],
            [5.0, 6.0, 7.77777777777777],
        ]);

        assert_fuzzy_eq!(matrix1, matrix2);
    }

    #[test]
    fn matrix_equality_with_identical_4x4_matrices() {
        let matrix1 = Matrix::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let matrix2 = Matrix::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_fuzzy_eq!(matrix1, matrix2);
    }

    #[test]
    fn matrix_equality_with_almost_identical_4x4_matrices() {
        let matrix1 = Matrix::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0000000000001],
        ]);

        let matrix2 = Matrix::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_fuzzy_eq!(matrix1, matrix2);
    }

    #[test]
    fn matrix_inequality_with_non_identical_4x4_matrices() {
        let matrix1 = Matrix::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let matrix2 = Matrix::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 2.0],
        ]);

        assert_fuzzy_ne!(matrix1, matrix2);
    }

    #[test]
    fn multiplying_two_4x4_matrices() {
        let matrix1 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2 = Matrix::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected_result = Matrix::from([
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
        let matrix1 = Matrix::from([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        let matrix2 = Matrix::identity();

        let expected_result = matrix1;
        let actual_result = matrix1 * matrix2;

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn a_4x4_matrix_multiplied_by_a_point() {
        let matrix = Matrix::from([
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

    #[test]
    fn transposing_4x4_matrix() {
        let matrix = Matrix::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        let expected_result = Matrix::from([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        let actual_result = matrix.traspose();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn calculate_the_determinant_of_2x2_matrix() {
        let matrix = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);

        let expected_result = 17.0;

        let actual_result = matrix.determinant();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let matrix = Matrix::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, 3.0]]);

        let expected_result = Matrix::from([[-3.0, 2.0], [0.0, 6.0]]);

        let actual_result = matrix.submatrix(0, 2);

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let matrix = Matrix::from([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);

        let expected_result =
            Matrix::from([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

        let actual_result = matrix.submatrix(2, 1);

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn calculate_the_minor_of_3x3_matrix() {
        let matrix = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let sub = matrix.submatrix(1, 0);
        let determinant = sub.determinant();
        let minor = matrix.minor(1, 0);

        assert_fuzzy_eq!(25.0, determinant);
        assert_fuzzy_eq!(25.0, minor);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let matrix = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let minor1 = matrix.minor(0, 0);
        let minor2 = matrix.minor(1, 0);

        let cofactor1 = matrix.cofactor(0, 0);
        let cofactor2 = matrix.cofactor(1, 0);

        assert_fuzzy_eq!(-12.0, minor1);
        assert_fuzzy_eq!(-12.0, cofactor1);
        assert_fuzzy_eq!(25.0, minor2);
        assert_fuzzy_eq!(-25.0, cofactor2);
    }

    #[test]
    fn calculate_the_determinant_of_3x3_matrix() {
        let matrix = Matrix::from([
            [1.0, 2.0, 6.0],
            [-5.0, 8.0, -4.0],
            [2.0, 6.0, 4.0],
        ]);

        let cofactor00 = matrix.cofactor(0, 0);
        let cofactor01 = matrix.cofactor(0, 1);
        let cofactor02 = matrix.cofactor(0, 2);

        let determinant = matrix.determinant();
        
        assert_fuzzy_eq!(56.0, cofactor00);
        assert_fuzzy_eq!(12.0, cofactor01);
        assert_fuzzy_eq!(-46.0, cofactor02);

        assert_fuzzy_eq!(-196.0, determinant);
    }
    #[test]
    fn calculate_the_determinant_of_4x4_matrix() {
        let matrix = Matrix::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0]
        ]);

        let cofactor00 = matrix.cofactor(0, 0);
        let cofactor01 = matrix.cofactor(0, 1);
        let cofactor02 = matrix.cofactor(0, 2);
        let cofactor03 = matrix.cofactor(0, 3);

        let determinant = matrix.determinant();

        assert_fuzzy_eq!(690.0, cofactor00);
        assert_fuzzy_eq!(447.0, cofactor01);
        assert_fuzzy_eq!(210.0, cofactor02);
        assert_fuzzy_eq!(51.0, cofactor03);

        assert_fuzzy_eq!(-4071.0, determinant);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let matrix = Matrix::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        let determinant = matrix.determinant();

        assert_fuzzy_eq!(-2120.0, determinant);
        assert!(matrix.is_invertible());
    }

    #[test]
    fn testing_an_noninvertible_matrix_for_invertibility() {
        let matrix = Matrix::from([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        let determinant = matrix.determinant();

        assert_fuzzy_eq!(0.0, determinant);
        assert!(!matrix.is_invertible());
    }

    #[test]
    fn calculating_the_inverse_of_a_4x4_matrix() {
        let matrix = Matrix::from([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let determinant = matrix.determinant();
        let cofactor23 = matrix.cofactor(2, 3);
        let cofactor32 = matrix.cofactor(3, 2);

        let expected_result = Matrix::from([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        let actual_result = matrix.inverse();

        assert_fuzzy_eq!(532.0, determinant);
        assert_fuzzy_eq!(-160.0, cofactor23);
        assert_fuzzy_eq!(-160.0 / 532.0, actual_result[3][2]);
        assert_fuzzy_eq!(105.0, cofactor32);
        assert_fuzzy_eq!(105.0 / 532.0, actual_result[2][3]);
        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let m1 = Matrix::from([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let m2 = Matrix::from([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let m3 = m1 * m2;

        let actual_result = m3 * m2.inverse();

        assert_fuzzy_eq!(actual_result, m1);
    }

}
