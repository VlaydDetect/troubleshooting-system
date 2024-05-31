use ndarray::prelude::*;
use std::collections::HashMap;
use crate::error::{Error, Result};

pub type Mat = Array2<bool>;

fn get_root_idx(mat: &Mat) -> usize {
    mat.ncols() / 2
}

// pub fn split_in_half(mat: &Mat) -> Result<HashMap<Vec<bool>, usize>> {}