use std::ops;

use super::fuzzy_eq::*;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

// Tuple type related functions
impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
    #[allow(dead_code)]
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }
    #[allow(dead_code)]
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
    #[allow(dead_code)]
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

// TODO: may be implement own FazzyPartialEq trait in the future combined with 
//  assert_fazzy_eq! macro, would be a lot nicer 
impl PartialEq<Tuple> for Tuple {
    fn eq(&self, other: &Self) -> bool {
        f64_fuzzy_eq(self.x, other.x) && f64_fuzzy_eq(self.y, other.y) && f64_fuzzy_eq(self.z, other.z) && f64_fuzzy_eq(self.w, other.w)
    }
}

impl ops::Add<Self> for Tuple {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        Tuple::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }
}

impl ops::Sub<Self> for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Tuple::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }
}

impl ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Tuple::new(self.x * other, self.y * other, self.z * other, self.w * other)
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Tuple::new(self.x / other, self.y / other, self.z / other, self.w / other)
    }
}

// Tuple maths operations
impl Tuple {
    #[allow(dead_code)]
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    } 

    #[allow(dead_code)]
    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    #[allow(dead_code)]
    pub fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }


    #[allow(dead_code)]
    pub fn cross(&self, other: &Tuple) -> Tuple {
        if !self.is_vector() || !other.is_vector() {
            panic!("Cross product can be calculated for two vectors");
        }
        Tuple::vector(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_does_fill_properties() {
        let point = Tuple::point(4.3, -4.2, 3.1);

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
    }
    
    #[test]
    fn point_has_w_value_of_one() {
        let point = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn point_says_it_is_a_point() {
        let point = Tuple::point(4.3, -4.2, 3.1);
        assert!(point.is_point());
    }
    
    #[test]
    fn tuple_are_added_up() {
        let tuple_one = Tuple::new(3.0, -2.0, 3.0, 1.0);
        let tuple_two = Tuple::new(-2.0, 3.0, 3.0, 0.0);
        let expected_tuple = Tuple::new(1.0, 1.0, 6.0, 1.0);

        assert_eq!(tuple_one + tuple_two, expected_tuple);
    }

    #[test]
    fn tuple_are_substracted() {
        let point_one = Tuple::point(3.0, 2.0, 1.0);
        let point_two = Tuple::point(5.0, 6.0, 7.0);

        let expected_result = Tuple::vector(-2.0, -4.0, -6.0);
        let actual_result = point_one - point_two;

        assert!(actual_result.is_vector());
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn substracting_a_vector_from_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        let expected_result = Tuple::point(-2.0, -4.0, -6.0);
        let actual_result = p - v;

        assert!(actual_result.is_point());
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn substracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        let expected_result = Tuple::vector(-2.0, -4.0, -6.0);
        let actual_result = v1 - v2;

        assert!(actual_result.is_vector());
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn substracting_a_vector_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v1 = Tuple::vector(5.0, 6.0, 7.0);

        let expected_result = Tuple::vector(-5.0, -6.0, -7.0);
        let actual_result = zero - v1;

        assert!(actual_result.is_vector());
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn negating_a_tuple() {
        let v = Tuple::new(1.0, -2.0, 3.0, -4.0);
        
        let expected_result = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        let actual_result = -v;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiplying_a_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let multiplier_scalar: f64 = 3.5;

        let expected_result = Tuple::new(3.5, -7.0, 10.5, -14.0);
        let actual_result = a * multiplier_scalar;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiplying_a_tuple_by_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let fraction = 0.5;

        let expected_result = Tuple::new(0.5, -1.0, 1.5, -2.0);
        let actual_result = a * fraction;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn dividing_a_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let divisor_scalar = 2.0;

        let expected_result = Tuple::new(0.5, -1.0, 1.5, -2.0);
        let actual_result = a / divisor_scalar;

        assert_eq!(actual_result, expected_result);
    }
    
    #[test]
    fn compute_magnitude_of_vector_1_0_0() {
        let v = Tuple::vector(1.0, 0.0, 0.0);

        let expected_result = 1.0;
        let actual_result = v.magnitude();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn compute_magnitude_of_vector_0_1_0() {
        let v = Tuple::vector(0.0, 1.0, 0.0);

        let expected_result = 1.0;
        let actual_result = v.magnitude();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn compute_magnitude_of_vector_0_0_1() {
        let v = Tuple::vector(0.0, 0.0, 1.0);

        let expected_result = 1.0;
        let actual_result = v.magnitude();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn compute_magnitude_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let expected_result = v.magnitude();
        let actual_result = (14.0 as f64).sqrt();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn compute_magnitude_of_negative_vector_1_2_3() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);

        let expected_result = v.magnitude();
        let actual_result = (14.0 as f64).sqrt();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn normalizing_of_vector_4_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);

        let expected_result = Tuple::vector(1.0, 0.0, 0.0);
        let actual_result = v.normalize();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn normalizing_of_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let expected_result = Tuple::vector(0.26726, 0.53452, 0.80178);
        let actual_result = v.normalize();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn magnitude_of_normalized_vector_is_1() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let expected_result = 1.0;
        let actual_result = v.normalize().magnitude();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let x = Tuple::vector(1.0, 2.0, 3.0);
        let y = Tuple::vector(2.0, 3.0, 4.0);

        let expected_result = 20.0;
        let actual_result = x.dot(&y);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn cross_product_of_two_vectors_1() {
        let x = Tuple::vector(1.0, 2.0, 3.0);
        let y = Tuple::vector(2.0, 3.0, 4.0);

        let expected_result = Tuple::vector(-1.0, 2.0, -1.0);
        let actual_result = x.cross(&y);

        assert_eq!(actual_result, expected_result);
    }    

    #[test]
    fn cross_product_of_two_vectors_2() {
        let x = Tuple::vector(1.0, 2.0, 3.0);
        let y = Tuple::vector(2.0, 3.0, 4.0);

        let expected_result = Tuple::vector(1.0, -2.0, 1.0);
        let actual_result = y.cross(&x);

        assert_eq!(actual_result, expected_result);
    }
}

