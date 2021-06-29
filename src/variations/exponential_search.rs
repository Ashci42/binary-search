use std::cmp::min;

use crate::{core, utils};

/// Performs [exponential search](https://en.wikipedia.org/wiki/Exponential_search) on `arr` in order to find the index of `target`
///
/// # Examples
///
/// ```
/// use binary_search::variations;
///
/// let target = 5;
/// let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9 ,10];
/// let found = variations::exponential_search(&target, &arr);
///
/// assert_eq!(found, Some(4));
/// ```
///
/// # Panics
///
/// The function panics if the array is not sorted.
pub fn exponential_search<T>(target: &T, arr: &[T]) -> Option<usize>
where
    T: Ord,
{
    if !utils::is_sorted(arr) {
        panic!("Exponential search encountered an array that is note sorted");
    }

    if arr.is_empty() {
        return None;
    }

    let arr_size = arr.len();
    let mut bound = 1;

    while bound < arr_size && &arr[bound] < target {
        bound *= 2;
    }

    let left_bound = bound / 2;
    let right_bound = min(bound + 1, arr_size);
    let slice_index = core::binary_search(target, &arr[left_bound..right_bound])?;
    let index = slice_index + left_bound;

    Some(index)
}

#[cfg(test)]
mod exponential_search_tests {
    use super::exponential_search;

    #[test]
    #[should_panic(expected = "Exponential search encountered an array that is note sorted")]
    fn exponential_search_panics_when_arr_is_not_sorted() {
        let target = 5;
        let arr = [1, 3, 2, 5];

        exponential_search(&target, &arr);
    }

    #[test]
    fn exponential_search_returns_none_for_empty_arr() {
        let target = 5;
        let arr = [];
        let found = exponential_search(&target, &arr);

        assert_eq!(found, None);
    }

    #[test]
    fn exponential_search_returns_none_if_target_not_in_one_element_arr() {
        let target = 5;
        let arr = [4];
        let found = exponential_search(&target, &arr);

        assert_eq!(found, None);
    }

    #[test]
    fn exponential_search_returns_some_index_if_target_in_one_element_arr() {
        let target = 5;
        let arr = [5];
        let found = exponential_search(&target, &arr);

        assert_eq!(found, Some(0));
    }

    #[test]
    fn exponential_search_returns_none_if_target_not_in_arr() {
        let target = 12;
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let found = exponential_search(&target, &arr);

        assert_eq!(found, None);
    }

    #[test]
    fn exponential_search_returns_some_index_if_target_in_arr() {
        let target = 5;
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let found = exponential_search(&target, &arr);

        assert_eq!(found, Some(4));
    }
}
