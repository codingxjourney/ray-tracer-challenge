use crate::fuzzy_eq::*;

pub type Matrix2f = [[f64; 2]; 2];
pub type Matrix3f = [[f64; 3]; 3];
pub type Matrix4f = [[f64; 4]; 4];

impl FuzzyEq<Matrix2f> for Matrix2f {
    fn fuzzy_eq(&self, other: &Matrix2f) -> bool {
        self[0][0].fuzzy_eq(&other[0][0]) 
            && self[0][1].fuzzy_eq(&other[0][1])
            && self[1][0].fuzzy_eq(&other[1][0])
            && self[1][1].fuzzy_eq(&other[1][1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_and_inspecting_4x4_matrix() {
        let matrix: Matrix4f = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];

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
        let matrix: Matrix2f = [[-3.0, 5.0], [1.0, -2.0]];

        assert_eq!(matrix[0][0], -3.0);
        assert_eq!(matrix[0][1], 5.0);
        assert_eq!(matrix[1][0], 1.0);
        assert_eq!(matrix[1][1], -2.0);
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let matrix = [[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]];

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
    fn matrix_fuzzy_equality_with_identical_2x2_matrices() {
        let matrix1 = [[0.123456789, 1.0], [2.0, 3.0]];
        let matrix2 = [[0.123456788, 1.0], [2.0, 3.0]];

        // assert!(matrix1.fuzzy_eq(&matrix2));
        assert_fuzzy_eq!(matrix1, matrix2);
    }

    #[test]
    fn matrix_fuzzy_equality_with_almost_cidentical_2x2_matrices() {
        let matrix1 = [[0.123456789, 1.0], [2.0, 3.0]];
        let matrix2 = [[0.123456780, 1.0], [2.0, 3.0]];

        assert_fuzzy_eq!(matrix1, matrix2);
        // assert!(matrix1.fuzzy_eq(&matrix2));

    }
}
