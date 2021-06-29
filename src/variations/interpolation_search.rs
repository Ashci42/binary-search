use crate::utils;
use std::{
    cmp::Ordering,
    ops::{Div, Sub},
};

/// Performs [interpolation search](https://en.wikipedia.org/wiki/Interpolation_search) on `arr` in order to find the index of `target`.
/// This function uses [linear interpolation](https://en.wikipedia.org/wiki/Linear_interpolation).
///
/// # Examples
///
/// ```
/// use binary_search::variations;
///
/// let target = 5u16;
/// let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9 ,10];
/// let found = variations::linear_interpolation_search(&target, &arr);
///
/// assert_eq!(found, Some(4));
/// ```
///
/// # Panics
///
/// The function panics if the array is not sorted.
pub fn linear_interpolation_search<T>(target: &T, arr: &[T]) -> Option<usize>
where
    T: Ord + Sub + Copy,
    <T as Sub>::Output: Div,
    <<T as Sub>::Output as Div>::Output: Into<usize>,
{
    interpolation_search(target, arr, |t, l, r| ((*t - *l) / (*r - *l)).into())
}

/// Performs [interpolation search](https://en.wikipedia.org/wiki/Interpolation_search) on `arr` in order to find the index of `target`
/// The function's `interpolation_fn` should calculate the midpoint.
///
/// # Examples
///
/// ```
/// use binary_search::variations;
///
/// let target = 5u16;
/// let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let found = variations::interpolation_search(&target, &arr, |t, l, r| ((*t - *l) / (*r - *l)).into());
///
/// assert_eq!(found, Some(4));
/// ```
///
/// # Panics
///
/// The function panics if the array is not sorted.
pub fn interpolation_search<T, InterpolationFn>(
    target: &T,
    arr: &[T],
    interpolation_fn: InterpolationFn,
) -> Option<usize>
where
    InterpolationFn: Fn(&T, &T, &T) -> usize,
    T: Ord,
{
    if !utils::is_sorted(arr) {
        panic!("Interpolation search encountered an array that is note sorted");
    }

    if arr.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right && target < &arr[right] && target > &arr[left] {
        let middle = left + interpolation_fn(target, &arr[left], &arr[right]);

        match arr[middle].cmp(target) {
            Ordering::Less => left = middle + 1,
            Ordering::Equal => return Some(middle),
            Ordering::Greater => right = middle - 1,
        }

        if &arr[left] == target {
            return Some(left);
        }
    }

    None
}

#[cfg(test)]
mod interpolation_search_tests {
    use super::interpolation_search;

    #[test]
    #[should_panic(expected = "Interpolation search encountered an array that is note sorted")]
    fn interpolation_search_panics_when_arr_is_not_sorted() {
        let target = 5u16;
        let arr = [1, 3, 2, 5];

        interpolation_search(&target, &arr, |t, l, r| ((*t - *l) / (*r - *l)).into());
    }

    #[test]
    fn interpolation_search_returns_none_if_arr_is_empty() {
        let target = 5u16;
        let arr = [];
        let found = interpolation_search(&target, &arr, |t, l, r| ((*t - *l) / (*r - *l)).into());

        assert_eq!(found, None);
    }

    #[test]
    fn interpolation_search_returns_some_index_if_target_in_arr() {
        let target = 5u16;
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let found = interpolation_search(&target, &arr, |t, l, r| ((*t - *l) / (*r - *l)).into());
        assert_eq!(found, Some(4));
    }

    #[test]
    fn interpolation_search_returns_none_if_target_not_in_arr() {
        let target = 11u16;
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let found = interpolation_search(&target, &arr, |t, l, r| ((*t - *l) / (*r - *l)).into());
        assert_eq!(found, None);
    }
}
