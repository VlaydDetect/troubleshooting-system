use ndarray::prelude::*;

mod information_criterion;
mod error;
mod prelude;
mod math;

use information_criterion::*;
use crate::prelude::W;

fn main() {
    let matrix = array![
        [0, 1, 1, 1, 1],
        [0, 0, 0, 1, 1],
        [1, 1, 0, 1, 1],
        [1, 1, 0, 0, 1],
        [0, 0, 0, 0, 0],
    ];

    let matrix: Array2<bool> = W(matrix).into();

    // println!("{:?}", num_elems(&W(matrix).into()))
    // println!("{}", information_of(3, &matrix));
    println!("First Controlled Element: {}", find_trouble(&matrix).unwrap())
}
