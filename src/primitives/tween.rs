pub use easer::functions::*;

/// Warning! Call sort on the vector before passing
pub fn get_next(i: f64, array: Vec<f64>) -> Option<usize> {
    array.iter().position(|x| x > &&i).map(|x| x.to_owned())
}

/// Warning! Call sort on the vector before passing
pub fn get_prev(i: f64, array: Vec<f64>) -> Option<usize> {
    array.iter().position(|x| x < &&i).map(|x| x.to_owned())
}
