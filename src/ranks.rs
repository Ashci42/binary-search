//! Contains functions for checking leftmost and rightmost rank

use crate::utils;

/// Calculates the leftmost rank of the given target in the array.
///
/// # Examples
/// ```
/// use binary_search::ranks;
///
/// let target = 4;
/// let arr = [1, 2, 4, 4, 4, 5, 6, 7];
/// let rank = ranks::leftmost_rank(&target, &arr);
///
/// assert_eq!(rank, 2);
/// ```
///
/// # Panics
///
/// The function panics if the array is not sorted.
pub fn leftmost_rank<T>(target: &T, arr: &[T]) -> usize
where
    T: Ord,
{
    if !utils::is_sorted(&arr) {
        panic!("Binary search encountered an array that is note sorted");
    }

    if arr.is_empty() {
        return 0;
    }

    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let middle = (left + right) / 2;

        if arr[middle] < *target {
            left = middle + 1;
        } else {
            right = middle;
        }
    }

    left
}

/// Calculates the rightmost rank of the given target in the array.
///
/// # Examples
/// ```
/// use binary_search::ranks;
///
/// let target = 4;
/// let arr = [1, 2, 4, 4, 4, 5, 6, 7];
/// let rank = ranks::rightmost_rank(&target, &arr);
///
/// assert_eq!(rank, 4);
/// ```
///
/// # Panics
///
/// The function panics if the array is not sorted.
pub fn rightmost_rank<T>(target: &T, arr: &[T]) -> usize
where
    T: Ord,
{
    if !utils::is_sorted(&arr) {
        panic!("Binary search encountered an array that is note sorted");
    }

    if arr.is_empty() {
        return 0;
    }

    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let middle = (left + right) / 2;

        if arr[middle] > *target {
            right = middle;
        } else {
            left = middle + 1;
        }
    }

    right - 1
}

#[cfg(test)]
mod ranks_tests {
    use super::{leftmost_rank, rightmost_rank};

    #[test]
    #[should_panic(expected = "Binary search encountered an array that is note sorted")]
    fn leftmost_rank_panics_if_the_arr_is_not_sorted() {
        let target = 4;
        let arr = [1, 2, 5, 4, 4, 6];

        leftmost_rank(&target, &arr);
    }

    #[test]
    fn leftmost_rank_returns_zero_if_arr_is_empty() {
        let target = 4;
        let arr: [i32; 0] = [];
        let rank = leftmost_rank(&target, &arr);

        assert_eq!(rank, 0);
    }

    #[test]
    fn leftmost_rank_returns_the_correct_leftmost_rank_if_target_in_arr() {
        let target = 4;
        let arr = [1, 2, 4, 4, 4, 5, 6, 7];
        let rank = leftmost_rank(&target, &arr);

        assert_eq!(rank, 2);
    }

    #[test]
    fn leftmost_rank_returns_the_correct_leftmost_rank_if_target_not_in_arr() {
        let target = 3;
        let arr = [1, 2, 4, 4, 4, 5, 6, 7];
        let rank = leftmost_rank(&target, &arr);

        assert_eq!(rank, 2);
    }

    #[test]
    #[should_panic(expected = "Binary search encountered an array that is note sorted")]
    fn rightmost_rank_panics_if_the_arr_is_not_sorted() {
        let target = 4;
        let arr = [1, 2, 5, 4, 4, 6];

        rightmost_rank(&target, &arr);
    }

    #[test]
    fn rightmost_rank_returns_zero_if_arr_is_empty() {
        let target = 4;
        let arr: [i32; 0] = [];
        let rank = rightmost_rank(&target, &arr);

        assert_eq!(rank, 0);
    }

    #[test]
    fn rightmost_rank_returns_the_correct_rightmost_rank_if_target_in_arr() {
        let target = 4;
        let arr = [1, 2, 4, 4, 4, 5, 6, 7];
        let rank = rightmost_rank(&target, &arr);

        assert_eq!(rank, 4);
    }

    #[test]
    fn rightmost_rank_returns_the_correct_rightmost_rank_if_target_not_in_arr() {
        let target = 10;
        let arr = [1, 2, 4, 4, 4, 5, 6, 7];
        let rank = rightmost_rank(&target, &arr);

        assert_eq!(rank, 7);
    }
}
