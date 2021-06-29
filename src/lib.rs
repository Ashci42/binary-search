//! Crate containing implementations of [binary search](https://en.wikipedia.org/wiki/Binary_search_algorithm)

pub mod ranks;
pub mod variations;

mod core;
mod utils;

/// Performs [binary search](https://en.wikipedia.org/wiki/Binary_search_algorithm) on `arr` in order to find the index of `target`
///
/// # Examples
///
/// ```
/// let target = 5;
/// let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9 ,10];
/// let found = binary_search::binary_search(&target, &arr);
///
/// assert_eq!(found, Some(4));
/// ```
///
/// # Panics
///
/// The function panics if the array is not sorted.
pub fn binary_search<T>(target: &T, arr: &[T]) -> Option<usize>
where
    T: Ord,
{
    if !utils::is_sorted(arr) {
        panic!("Binary search encountered an array that is note sorted");
    }

    if arr.is_empty() {
        return None;
    }

    core::binary_search(target, arr)
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn binary_search_returns_none_for_empty_arr() {
        let target = 5;
        let arr = [];
        let found = binary_search(&target, &arr);

        assert_eq!(found, None);
    }

    #[test]
    fn binary_search_returns_none_if_target_not_in_one_element_arr() {
        let target = 5;
        let arr = [4];
        let found = binary_search(&target, &arr);

        assert_eq!(found, None);
    }

    #[test]
    fn binary_search_returns_some_index_if_target_in_one_element_arr() {
        let target = 5;
        let arr = [5];
        let found = binary_search(&target, &arr);

        assert_eq!(found, Some(0));
    }

    #[test]
    fn binary_search_returns_none_if_target_not_in_arr() {
        let target = 12;
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let found = binary_search(&target, &arr);

        assert_eq!(found, None);
    }

    #[test]
    fn binary_search_returns_some_index_if_target_in_arr() {
        let target = 5;
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let found = binary_search(&target, &arr);

        assert_eq!(found, Some(4));
    }

    #[test]
    #[should_panic(expected = "Binary search encountered an array that is note sorted")]
    fn binary_search_panics_when_arr_is_not_sorted() {
        let target = 5;
        let arr = [1, 3, 2, 5];

        binary_search(&target, &arr);
    }
}
