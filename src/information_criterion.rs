use std::collections::HashMap;
use ndarray::prelude::*;
use crate::prelude::*;
use crate::error::{Error, Result};
use crate::math::{f_eq, log2};

pub type Mat = Array2<bool>;

fn i32_to_bool(i: i32) -> bool {
    if i == 0 { false } else { true }
}

impl From<W<Array2<i32>>> for Array2<bool> {
    fn from(value: W<Array2<i32>>) -> Array2<bool> {
        value.0.mapv(|elem| i32_to_bool(elem))
    }
}

pub fn num_elems(mat: &Mat) -> usize {
    mat.shape()[1]
}

fn entropy(mat: &Mat) -> f64 {
    (num_elems(mat) as f64).log2()
}

fn row_num_units(mat: &Mat, row: usize) -> usize {
    mat.row(row).iter().filter(|&elem| *elem).count()
}

pub fn information_of(i: usize, mat: &Mat) -> f64 {
    let n = num_elems(mat) as f64;
    let m = row_num_units(mat, i) as f64;
    let hi = m / n * m.log2() + (n - m) / n * (n - m).log2();
    entropy(mat) - hi
}

fn conditional_units_by(i: usize, j: usize, mat: &Mat, by_units: bool) -> u32 {
    let row_i = mat.row(i);
    let row_j = mat.row(j);

    let mut count = 0u32;
    for k in 0..row_i.len() {
        if row_i[k] && (row_j[k] == by_units) {
            count += 1;
        }
    }

    count
}

fn conditional_information_of(i: usize, j: usize, mat: &Mat) -> f64 {
    let n = num_elems(mat) as f64;
    let m = row_num_units(mat, j) as f64;

    let m1 = conditional_units_by(i, j, mat, true) as f64;
    let m2 = conditional_units_by(i, j, mat, false) as f64;

    // let a = m1 / n * (m1 / m).log2();
    // let b = (m - m1) / n * ((m - m1) / m).log2();
    // let c = m2 / n * (m2 / (n - m)).log2();
    // let d = (n - m - m2) / n * ((n - m - m2) / (n - m)).log2();

    -(m1 / n * log2(m1 / m) + (m - m1) / n * log2((m - m1) / m) + m2 / n * log2(m2 / (n - m)) + (n - m - m2) / n * log2((n - m - m2) / (n - m)))
}

fn find_zero_row(mat: &Mat) -> Vec<usize> {
    let mut zero_rows = vec![];
    for i in 0..mat.nrows() {
        if mat.row(i).iter().find(|&elem| *elem).is_none() {
            zero_rows.push(i)
        }
    }

    zero_rows
}

fn remove_rows(matrix: &Mat, to_remove: &[usize]) -> Result<Mat> {
    let mut keep_row = vec![true; matrix.nrows()];
    to_remove.iter().for_each(|row| keep_row[*row] = false);

    let elements_iter = matrix
        .axis_iter(Axis(0))
        .zip(keep_row.iter())
        .filter(|(_, keep)| **keep)
        .flat_map(|(row, _)| row.to_vec());

    let new_n_rows = matrix.nrows() - to_remove.len();

    Array::from_iter(elements_iter)
        .into_shape((new_n_rows, matrix.ncols())).map_err(|ex| Error::CannotReshapeMatrix)
}

pub fn find_trouble(mat: &Mat) -> Result<usize> {
    // let mut mat = remove_rows(mat, find_zero_row(mat).as_slice())?;
    // let n = num_elems(&mat);
    //
    // let mut map = HashMap::<usize, f64>::new();
    //
    // for i in 0..mat.nrows() {
    //     let info = information_of(i, &mat);
    //     map.insert(i, info);
    // }
    //
    // let max_val = map.values().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    // let max_elem_ids = map.iter().filter(|&item| f_eq(item.1, &max_val)).map(|item| item.0).collect::<Vec<_>>();
    //
    // let first_controlled = {
    //     let ids_with_units = max_elem_ids.into_iter().map(|&i| (i, row_num_units(&mat, i)));
    //     let min_units = ids_with_units.clone().map(|elem| elem.1).min().unwrap_or(n); // TODO: What should we do, if all the items with max info have the same number of zeros in the row? Is such a situation possible?
    //     // TODO: Maybe if there are several such elements, choose the one with average index
    //     ids_with_units.filter(|&(_, num)| num == min_units).map(|(i, _)| i).collect::<Vec<_>>()[0].clone()
    // };
    //
    // map.clear();
    //
    // for i in 0..mat.nrows() {
    //     if i != first_controlled {
    //         let info = conditional_information_of(i, first_controlled, &mat);
    //         map.insert(i, info);
    //     }
    // }
    //
    // let mut mat = remove_rows(&mat, &[first_controlled]);
    //
    // println!("{:?}", mat);
    //
    //
    // Ok(first_controlled)

    let mut mat = remove_rows(mat, find_zero_row(mat).as_slice())?;
    let mut map = HashMap::<usize, f64>::new();

    for i in 0..mat.nrows() {
        let info = information_of(i, &mat);
        map.insert(i, info);
    }


    Ok()
}