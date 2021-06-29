use std::cmp::Ordering;

/// Core implementation of binary search with no additional checks
pub fn binary_search<T>(target: &T, arr: &[T]) -> Option<usize>
where
    T: Ord,
{
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let middle = (left + right) / 2;

        match arr[middle].cmp(target) {
            Ordering::Equal => return Some(middle),
            Ordering::Greater => right = middle - 1,
            Ordering::Less => left = middle + 1,
        }
    }

    None
}
