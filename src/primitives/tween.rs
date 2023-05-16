use alloc::vec::Vec;

/*
/// Warning! Call sort on the vector before passing
///
/// Get the position of the next closest value to `i`
pub fn get_next_pos(i: f64, array: Vec<f64>) -> Option<usize> {
    array.iter().position(|x| x > &&i)
}

/// Warning! Call sort on the vector before passing
///
/// Get the position of the previous closest value to `i`
pub fn get_prev_pos(i: f64, array: Vec<f64>) -> Option<usize> {
    array.iter().position(|x| x < &&i)
}

/// Warning! Call sort on the vector before passing
///
/// Get the next closest value to `i`
pub fn get_next(i: f64, array: Vec<f64>) -> Option<usize> {
    array.iter().position(|x| x > &&i)
}

/// Warning! Call sort on the vector before passing
///
/// Get the previous closest value to `i`
pub fn get_prev(i: f64, array: Vec<f64>) -> Option<usize> {
    array.iter().position(|x| x < &&i)
}*/

pub fn get_wrapped_range<T: Clone>(pixels: &[T], start: usize, end: usize) -> Vec<T> {
    let len = pixels.len();
    let start_index = start % len;
    let end_index = end % len;

    if start_index <= end_index {
        pixels[start_index..end_index].to_vec()
    } else {
        let mut range = pixels[start_index..].to_vec();
        range.extend_from_slice(&pixels[..end_index]);
        range
    }
}

pub fn get_wrapped_from_len<T: Clone>(pixels: &[T], start: usize, len: usize) -> Vec<T> {
    let pixel_count = pixels.len();
    let start_index = start % pixel_count;
    let end_index = (start_index + len) % pixel_count;

    if start_index <= end_index {
        pixels[start_index..end_index].to_vec()
    } else {
        let mut range = pixels[start_index..].to_vec();
        range.extend_from_slice(&pixels[..end_index]);
        range
    }
}
