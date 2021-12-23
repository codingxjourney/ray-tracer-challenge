use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;

use super::util::*;

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        f64_fuzzy_eq(self.red, other.red) && f64_fuzzy_eq(self.green, other.green) && f64_fuzzy_eq(self.blue, other.blue)
    }
}

#[derive(Debug)]
struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue}
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Color::new( self.red + other.red, self.green + other.green, self.blue + other.blue )
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Self::Output {
        Color::new(self.red - other.red, self.green - other.green, self.blue - other.blue)
    }
} 

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Self::Output {
        Color::new(self.red * other, self.green * other, self.blue * other)
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Self::Output {
        Color::new(self.red * other.red, self.green * other.green, self.blue * other.blue)
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, other: f64) -> Self::Output {
        Color::new(self.red / other, self.green / other, self.blue / other)
    }
}

impl Div<Color> for Color {
    type Output = Color;

    fn div(self, other: Color) -> Self::Output {
        Color::new(self.red / other.red, self.green / other.green, self.blue / other.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_red_green_blue_tuple() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.7, 0.4, 0.75);
        let c2 = Color::new(0.7, 0.3, 0.25);

        let expected_result = Color::new(1.4, 0.7, 1.0);
        let actual_result = c1 + c2;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn substracting_colors() {
        let c1 = Color::new(0.7, 0.4, 0.75);
        let c2 = Color::new(0.7, 0.3, 0.25);

        let expected_result = Color::new(0.0, 0.1, 0.5);
        let actual_result = c1 - c2;

        assert_eq!(actual_result, expected_result);
    }
        
    #[test]
    fn multiplying_a_color_by_scalar() {
        let a = Color::new(1.0, -2.0, 3.0);
        let multiplier_scalar: f64 = 3.5;

        let expected_result = Color::new(3.5, -7.0, 10.5);
        let actual_result = a * multiplier_scalar;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiplying_two_color() {
        let c1 = Color::new(1.0, 0.4, 0.7);
        let c2 = Color::new(0.8, 1.0, 0.1);

        let expected_result = Color::new(0.8, 0.4, 0.07);
        let actual_result = c1 * c2;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn dividing_a_color_by_scalar() {
        let a = Color::new(1.0, -2.0, 3.0);
        let divisor_scalar = 2.0;

        let expected_result = Color::new(0.5, -1.0, 1.5);
        let actual_result = a / divisor_scalar;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn dividing_two_colors() {
        let c1 = Color::new(0.1, 1.0, 0.7);
        let c2 = Color::new(0.4, 2.0, 0.07);

        let expected_result = Color::new(0.25, 0.5, 10.0);
        let actual_result = c1 / c2;

        assert_eq!(actual_result, expected_result);
    }

}
    
