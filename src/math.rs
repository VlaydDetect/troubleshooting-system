pub fn f_eq(a: &f64, b: &f64) -> bool {
    a.total_cmp(b) == std::cmp::Ordering::Equal
}

pub fn log2(value: f64) -> f64 {
    if value.total_cmp(&0.0) == std::cmp::Ordering::Equal {
        return 0.0;
    }

    value.log2()
}