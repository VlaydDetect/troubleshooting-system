/// Generic Wrapper tuple struct for newtype pattern, mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);

// Personal preference.
pub use std::format as f;
use ndarray::Array2;

fn i32_to_bool(i: i32) -> bool {
    if i == 0 { false } else { true }
}

impl From<W<Array2<i32>>> for Array2<bool> {
    fn from(value: W<Array2<i32>>) -> Array2<bool> {
        value.0.mapv(|elem| i32_to_bool(elem))
    }
}