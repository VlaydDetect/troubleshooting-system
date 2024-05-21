use std::cmp::Ordering;
use std::collections::HashMap;
use ndarray::prelude::*;
use crate::error::{Error, Result};
use crate::math::{f_eq, log2};

pub type Mat = Array2<bool>;

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

pub fn remove_rows(matrix: &Mat, to_remove: &[usize]) -> Result<Mat> {
    let mut keep_row = vec![true; matrix.nrows()];
    to_remove.iter().for_each(|row| keep_row[*row] = false);

    let elements_iter = matrix
        .axis_iter(Axis(0))
        .zip(keep_row.iter())
        .filter(|(_row, keep)| **keep)
        .flat_map(|(row, _keep)| row.to_vec());

    let new_n_rows = matrix.nrows() - to_remove.len();
    Array::from_iter(elements_iter)
        .into_shape((new_n_rows, matrix.ncols())).map_err(|ex| Error::CannotReshapeMatrix)
}

pub fn get_min_required_elems(mat: &Mat) -> Result<Vec<usize>> {
    let mut mat = remove_rows(mat, find_zero_row(mat).as_slice())?;
    println!("without zero rows: {:?}", mat);
    let mut result = vec![];
    let n = num_elems(&mat);

    let mut l = 0_usize;
    loop {
        let mut map = HashMap::<usize, f64>::new();

        if l == 0 {
            for i in 0..mat.nrows() {
                let info = information_of(i, &mat);
                map.insert(i, info);
            }
        } else {
            let ids = (0..mat.nrows()).filter(|i| !result.contains(i)).collect::<Vec<_>>();
            for i in ids {
                let info = conditional_information_of(i, result[l - 1], &mat);
                map.insert(i, info);
            }
        }

        let max_info = map.values().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let elem_ids_with_max_info = map.iter().filter(|&item| f_eq(item.1, &max_info)).map(|item| item.0).collect::<Vec<_>>();

        let controlled_elem = {
            let ids_with_units = elem_ids_with_max_info.into_iter().map(|&i| (i, row_num_units(&mat, i)));
            let min_units = ids_with_units.clone().map(|elem| elem.1).min().unwrap_or(n); // TODO: What should we do, if all the items with max info have the same number of zeros in the row? Is such a situation possible?
            // TODO: Maybe if there are several such elements, choose the one with average index
            ids_with_units.filter(|&(_, num)| num == min_units).map(|(i, _)| i).collect::<Vec<_>>()[0].clone()
        };

        result.push(controlled_elem);

        if l > mat.nrows() - 2 {
            break;
        }

        l += 1;
    }


    Ok(result)
}

fn boolean_tuples(n: usize) -> Vec<Vec<bool>> {
    (0..1 << n).map(|i| {
        (0..n).map(|j| (i >> j) & 1 == 1).collect()
    }).collect()
}

pub fn make_tree(mat: &Mat) -> Result<HashMap<Vec<bool>, usize>> {
    let mut elems = get_min_required_elems(mat)?;
    println!("elems: {:?}", elems);
    let tuples = boolean_tuples(elems.len() - 1);
    let mut map = HashMap::new();

    let rows_to_remove = (0..mat.nrows()).filter(|row| !elems.contains(row)).collect::<Vec<_>>();

    let new_mat = remove_rows(mat, rows_to_remove.as_slice())?;
    // println!("new mat: {new_mat:?}");
    // println!("new mat cols: {:?}", new_mat.columns().into_iter().map(|col| col.to_vec()).collect::<Vec<_>>());

    for t in tuples {
        // let count = new_mat.columns().into_iter().filter(|col| col.to_vec().as_slice().iter().cmp(t.as_slice().iter()) == Ordering::Equal).count();

        let mut cols = vec![];
        for col in new_mat.columns() {
            let col_vec = &col.to_vec()[1..col.len()];
            if col_vec.iter().cmp(t.as_slice().iter()) == Ordering::Equal {
                cols.push(col)
            }
        }

        let count = cols.len();

        if count == 0 {
            // return Err(Error::UnresolvedTree);
        } else if count == 1 {
            let (index, _) = mat.columns()
                .into_iter()
                .enumerate()
                .find(|(idx, col)| {
                    let col_vec = col.to_vec().into_iter().enumerate().filter(|&(i, _)| elems.contains(&i)).map(|(_, col)| col).collect::<Vec<_>>();
                    col_vec[1..col_vec.len()].iter().cmp(t.iter()) == Ordering::Equal
                }).unwrap();
            map.insert(t, index);
        } else {
            // TODO: add two bool values corresponding to the last controlled element and repeat all the logic
            println!("{t:?}")
        }
    }

    Ok(map)
}