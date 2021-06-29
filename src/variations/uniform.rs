//! [Uniform binary search](https://en.wikipedia.org/wiki/Uniform_binary_search)

use crate::utils;
use std::cmp::Ordering;

const MAX_LOOKUP_TABLE_SIZE: usize = 64;

/// Struct used for performing uniform binary search
pub struct UniformBinarySearch {
    /// The size of the last array used for searching
    last_arr_size: Option<usize>,
    /// The lookup table to be used for searching the array
    lookup_table: [usize; MAX_LOOKUP_TABLE_SIZE],
}

impl UniformBinarySearch {
    /// Returns a new `UniformBinarySearch` struct
    ///
    /// # Examples
    ///
    /// ```
    /// use binary_search::variations::UniformBinarySearch;
    ///
    /// let unifrom_binary_search = UniformBinarySearch::new();
    /// ```
    pub fn new() -> Self {
        Self {
            last_arr_size: None,
            lookup_table: [0; MAX_LOOKUP_TABLE_SIZE],
        }
    }

    /// Performs [uniform binary search](https://en.wikipedia.org/wiki/Uniform_binary_search) on `arr` in order to find the index of `target`
    ///
    /// # Examples
    ///
    /// ```
    /// use binary_search::variations::UniformBinarySearch;
    ///
    /// let target = 5;
    /// let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9 ,10];
    /// let mut uniform_binary_search = UniformBinarySearch::default();
    /// let found = uniform_binary_search.search(&target, &arr);
    ///
    /// assert_eq!(found, Some(4));
    /// ```
    ///
    /// # Panics
    ///
    /// The function panics if the array is not sorted.
    pub fn search<T>(&mut self, target: &T, arr: &[T]) -> Option<usize>
    where
        T: Ord,
    {
        if !utils::is_sorted(&arr) {
            panic!("Uniform binary search encountered an array that is note sorted");
        }

        if arr.is_empty() {
            return None;
        }

        let arr_len = arr.len();

        if let Some(last_arr_size) = self.last_arr_size {
            if last_arr_size != arr_len {
                self.update_lookup_table(arr_len);
            }
        } else {
            self.update_lookup_table(arr_len);
        }

        self.inner_search(target, arr)
    }

    /// Perform binary search using the current lookup table
    fn inner_search<T>(&self, target: &T, arr: &[T]) -> Option<usize>
    where
        T: Ord,
    {
        let mut index = self.lookup_table[0] - 1;
        let mut lookup_table_index = 0;

        loop {
            if self.lookup_table[lookup_table_index] == 0 {
                return None;
            }

            match arr[index].cmp(target) {
                Ordering::Less => {
                    lookup_table_index += 1;
                    index += self.lookup_table[lookup_table_index];
                }
                Ordering::Equal => return Some(index),
                Ordering::Greater => {
                    lookup_table_index += 1;
                    index -= self.lookup_table[lookup_table_index];
                }
            }
        }
    }

    /// Updates lookup table
    fn update_lookup_table(&mut self, len: usize) {
        let mut power = 1;
        let mut i = 0;

        loop {
            let half = power;

            power <<= 1;
            self.lookup_table[i] = (len + half) / power;

            if self.lookup_table[i] == 0 {
                break;
            }

            i += 1;
        }
    }
}

impl Default for UniformBinarySearch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod uniform_tests {
    use crate::variations::uniform::MAX_LOOKUP_TABLE_SIZE;

    use super::UniformBinarySearch;

    #[test]
    #[should_panic(expected = "Uniform binary search encountered an array that is note sorted")]
    fn search_panics_if_arr_is_not_sorted() {
        let target = 5;
        let arr = [1, 3, 2, 5];
        let mut uniform_binary_search = UniformBinarySearch::default();

        uniform_binary_search.search(&target, &arr);
    }

    #[test]
    fn search_returns_none_for_empty_arr() {
        let target = 5;
        let arr = [];
        let mut uniform_binary_search = UniformBinarySearch::default();
        let found = uniform_binary_search.search(&target, &arr);

        assert_eq!(found, None);
    }

    #[test]
    fn update_lookup_table_with_zero_len() {
        let mut uniform_binary_search = UniformBinarySearch::default();

        uniform_binary_search.update_lookup_table(0);

        assert_eq!(
            uniform_binary_search.lookup_table,
            [0; MAX_LOOKUP_TABLE_SIZE]
        );
    }

    #[test]
    fn update_lookup_table_with_one_len() {
        let mut uniform_binary_search = UniformBinarySearch::default();

        uniform_binary_search.update_lookup_table(1);

        assert_eq!(uniform_binary_search.lookup_table[0], 1);
    }

    #[test]
    fn update_lookup_table() {
        let mut uniform_binary_search = UniformBinarySearch::default();

        uniform_binary_search.update_lookup_table(8);

        assert_eq!(uniform_binary_search.lookup_table[..4], [4, 2, 1, 1]);
    }

    #[test]
    fn search_returns_none_if_target_not_in_one_element_arr() {
        let target = 5;
        let arr = [4];
        let mut uniform_binary_search = UniformBinarySearch::default();
        let found = uniform_binary_search.search(&target, &arr);

        assert_eq!(found, None);
    }

    #[test]
    fn search_returns_some_index_if_target_in_one_element_arr() {
        let target = 5;
        let arr = [5];
        let mut uniform_binary_search = UniformBinarySearch::default();
        let found = uniform_binary_search.search(&target, &arr);

        assert_eq!(found, Some(0));
    }

    #[test]
    fn search_returns_none_if_target_not_in_arr() {
        let target = 12;
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut uniform_binary_search = UniformBinarySearch::default();
        let found = uniform_binary_search.search(&target, &arr);

        assert_eq!(found, None);
    }

    #[test]
    fn search_returns_some_index_if_target_in_arr() {
        let target = 5;
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut uniform_binary_search = UniformBinarySearch::default();
        let found = uniform_binary_search.search(&target, &arr);

        assert_eq!(found, Some(4));
    }

    #[test]
    fn inner_search() {
        let target = 5;
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut uniform_binary_search = UniformBinarySearch::default();
        let found = uniform_binary_search.search(&target, &arr);

        assert_eq!(found, Some(4));

        let found = uniform_binary_search.inner_search(&3, &arr);

        assert_eq!(found, Some(2));
    }
}
