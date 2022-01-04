const EPSILON: f64 = 0.00001;

pub trait FuzzyEq<T> {
    fn fuzzy_eq(&self, other: &T) -> bool;

    fn fuzzy_ne(&self, other: &T) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl FuzzyEq<f64> for f64 {
    fn fuzzy_eq(&self, other: &f64) -> bool {
        (*self - *other).abs() < EPSILON 
    }
}

// we don't need this function any more
// pub fn f64_fuzzy_eq(left: f64, right: f64) -> bool {
//     (left - right).abs() < EPSILON
// }


// Not really sure what I am doing here, as I don't have a great understanding
// of macros yet! But I will fix and understand in future.
// @TODO Check if we can ensure more explicitly the two operands the "FuzzyEq" trait
#[macro_export]
macro_rules! assert_fuzzy_eq {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val.fuzzy_ne(right_val) {
                    panic!("asserting, fuzzy equality, {:?} is not fuzzy equal to {:?}", left_val, right_val);
                }
            }
        }
    });
}

#[macro_export]
macro_rules! assert_fuzzy_ne {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val.fuzzy_eq(right_val) {
                    panic!("asserting, fuzzy in-equality, {:?} is fuzzy equal to {:?}", left_val, right_val);
                }
            }
        }
    });
}



    // ($left:expr, $right:expr, $($arg:tt)+) => ({
    //     match (&$left, &$right) {
    //         (left_val, right_val) => {
    //             if !(*left_val == *right_val) {
    //                 let kind = $crate::panicking::AssertKind::Eq;
    //                 // The reborrows below are intentional. Without them, the stack slot for the
    //                 // borrow is initialized even before the values are compared, leading to a
    //                 // noticeable slow down.
    //                 $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::Some($crate::format_args!($($arg)+)));
    //             }
    //         }
    //     }
    // });