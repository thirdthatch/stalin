//! Classic Stalin sort intentionally discards out-of-order items.
//! Equal values are retained. Input is never mutated.

use std::cmp::Ordering;

/// Retain a nondecreasing subsequence in O(n) time and O(n) result space.
pub fn stalin_sort<T: Ord + Clone>(values: &[T]) -> Vec<T> {
    stalin_sort_by(values, T::cmp)
}

/// Comparator version, for example `stalin_sort_by(&floats, f64::total_cmp)`.
/// The comparator must define a consistent total ordering.
pub fn stalin_sort_by<T: Clone, F: Fn(&T, &T) -> Ordering>(values: &[T], compare: F) -> Vec<T> {
    let mut retained: Vec<T> = Vec::new();
    for value in values {
        if retained
            .last()
            .is_none_or(|last| compare(value, last) != Ordering::Less)
        {
            retained.push(value.clone());
        }
    }
    retained
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn javascript_reference_fixtures() {
        let parse = |text: &str| -> Vec<i64> {
            if text.is_empty() || text == "[]" {
                Vec::new()
            } else {
                text.split(',')
                    .map(|value| value.parse().unwrap())
                    .collect()
            }
        };
        for line in include_str!("../../tests/fixtures/javascript.tsv")
            .lines()
            .skip(1)
        {
            let fields: Vec<_> = line.split('\t').collect();
            assert_eq!(
                stalin_sort(&parse(fields[1])),
                parse(fields[2]),
                "{}",
                fields[0]
            );
        }
    }

    #[test]
    fn examples_and_input_preservation() {
        let input = vec![3, 1, 4, 2, 5, 0, 2, 6];
        let before = input.clone();
        assert_eq!(stalin_sort(&input), [3, 4, 5, 6]);
        assert_eq!(input, before);
        assert_eq!(stalin_sort::<i32>(&[]), []);
        assert_eq!(stalin_sort(&[3, 2, 1]), [3]);
        assert_eq!(stalin_sort(&[2, 2, 1, 2]), [2, 2, 2]);
        assert_eq!(
            stalin_sort_by(&[-2.5_f64, -3.0, 0.5, 0.5], f64::total_cmp),
            [-2.5, 0.5, 0.5]
        );
    }

    fn check(input: &[i32]) {
        let expected: Vec<_> = input
            .iter()
            .enumerate()
            .filter(|(i, value)| **value == *input[..=*i].iter().max().unwrap())
            .map(|(_, value)| *value)
            .collect();
        assert_eq!(stalin_sort(input), expected);
    }

    fn permute(items: &mut [i32], index: usize) {
        if index == items.len() {
            check(items);
            return;
        }
        for i in index..items.len() {
            items.swap(index, i);
            permute(items, index + 1);
            items.swap(index, i);
        }
    }

    #[test]
    fn exhaustive_permutations_and_duplicates() {
        permute(&mut [0, 1, 2, 3, 4, 5, 6], 0);
        for mut code in 0..729 {
            let mut input = [0; 6];
            for value in &mut input {
                *value = code % 3;
                code /= 3;
            }
            check(&input);
        }
    }

    #[test]
    fn comparator_preserves_equal_records() {
        let rows = [(2, "a"), (1, "b"), (2, "c"), (3, "d")];
        let calls = Cell::new(0);
        let output = stalin_sort_by(&rows, |a, b| {
            calls.set(calls.get() + 1);
            a.0.cmp(&b.0)
        });
        assert_eq!(output, [rows[0], rows[2], rows[3]]);
        assert_eq!(calls.get(), rows.len() - 1);
    }
}
