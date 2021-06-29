//! Utilities module

/// Checks if `arr` is sorted.
pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: Ord,
{
    arr.windows(2).all(|pair| pair[0] <= pair[1])
}

#[cfg(test)]
mod utils_tests {
    use super::is_sorted;

    #[test]
    fn is_sorted_returns_true_for_empty_arr() {
        let arr: [i32; 0] = [];
        let result = is_sorted(&arr);

        assert_eq!(result, true);
    }

    #[test]
    fn is_sorted_returns_true_for_one_element_arr() {
        let arr = [1];
        let result = is_sorted(&arr);

        assert_eq!(result, true);
    }

    #[test]
    fn is_sorted_returns_true_for_sorted_arr() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = is_sorted(&arr);

        assert_eq!(result, true);
    }

    #[test]
    fn is_sorted_returns_false_for_unsorted_arr() {
        let arr = [1, 2, 3, 5, 4, 6, 7, 8, 9, 10];
        let result = is_sorted(&arr);

        assert_eq!(result, false);
    }
}
