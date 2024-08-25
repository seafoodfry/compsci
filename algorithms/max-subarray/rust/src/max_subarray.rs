use std::cmp::PartialOrd;
use std::ops::{Add, AddAssign};

// Derive Debug to print them and PartialEq to test them.
#[derive(Debug, PartialEq)]
pub struct SubArrayResult<N> {
    pub sum: N,
    pub start: usize,
    pub end: usize,
}

pub fn brute_force<N>(numbers: &[N]) -> Option<SubArrayResult<N>>
where
    N: Default + Copy + PartialOrd + Add<Output = N> + AddAssign,
    // Default -> N:default() to provide a zero value.
    // Copy -> needed for += nums[j].
    // PartialOrd -> needed for comparisons.
    // Add<Output = N> -> needed to add values together.
    // AddAssign -> needed for += nums[j].
{
    if numbers.is_empty() {
        return None;
    }

    let mut max_result = SubArrayResult {
        // Using N::default() will result in the wrong result if all entries are negative.
        sum: numbers[0],
        start: 0,
        end: 0,
    };
    for start in 0..numbers.len() {
        let mut current_sum = N::default();

        for (end, _) in numbers.iter().enumerate().skip(start) {
            current_sum += numbers[end];
            if current_sum > max_result.sum {
                max_result = SubArrayResult {
                    sum: current_sum,
                    start,
                    end,
                };
            }
        }
    }

    Some(max_result)
}

pub fn kadane<N>(numbers: &[N]) -> Option<SubArrayResult<N>>
where
    N: Default + Copy + PartialOrd + Add<Output = N> + AddAssign,
    // Default -> N:default() to provide a zero value.
    // Copy -> needed for += nums[j].
    // PartialOrd -> needed for comparisons.
    // Add<Output = N> -> needed to add values together.
    // AddAssign -> needed for += nums[j].
{
    if numbers.is_empty() {
        return None;
    }

    let mut max_result = SubArrayResult {
        sum: numbers[0],
        start: usize::default(),
        end: usize::default(),
    };
    let mut current_sum = N::default();
    let mut current_start = usize::default();

    for (idx, &num) in numbers.iter().enumerate() {
        if current_sum + num >= num {
            current_sum += num;
        } else {
            current_sum = num;
            current_start = idx;
        }

        if current_sum > max_result.sum {
            max_result = SubArrayResult {
                sum: current_sum,
                start: current_start,
                end: idx,
            };
        }
    }

    Some(max_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cases_i32<F>(algorithm: F)
    where
        F: Fn(&[i32]) -> Option<SubArrayResult<i32>>,
    {
        assert_eq!(algorithm(&[]), None);

        assert_eq!(
            algorithm(&[0]),
            Some(SubArrayResult {
                sum: 0,
                start: 0,
                end: 0
            })
        );
        assert_eq!(
            algorithm(&[0, 0, 0]),
            Some(SubArrayResult {
                sum: 0,
                start: 0,
                end: 0
            })
        );

        assert_eq!(
            algorithm(&[1, 2, 3]),
            Some(SubArrayResult {
                sum: 6,
                start: 0,
                end: 2
            })
        );

        let vec1 = vec![0, 1, 2, 3, 4, 5];
        let book = vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];

        assert_eq!(
            algorithm(&vec1),
            Some(SubArrayResult {
                sum: 15,
                start: 0,
                end: 5
            })
        );

        assert_eq!(
            algorithm(&book),
            Some(SubArrayResult {
                sum: 43,
                start: 7,
                end: 10
            })
        );
    }

    fn test_cases_u32(algorithm: fn(&[u32]) -> Option<SubArrayResult<u32>>) {
        assert_eq!(algorithm(&[] as &[u32]), None);
    }

    fn test_cases_f64(algorithm: fn(&[f64]) -> Option<SubArrayResult<f64>>) {
        assert_eq!(
            algorithm(&[1.1, 2.2, 3.3]),
            Some(SubArrayResult {
                sum: 6.6,
                start: 0,
                end: 2
            })
        );

        assert_eq!(
            algorithm(&[-1.1, 2.2, 3.3]),
            Some(SubArrayResult {
                sum: 5.5,
                start: 1,
                end: 2
            })
        );
    }

    #[test]
    fn explicit_test_cases() {
        // We only need to specify the generic type for this next function because that is the only
        // instance in which the compiler won't be able to infer the type.
        assert_eq!(brute_force::<i32>(&[]), None);
        // We could also do...
        assert_eq!(brute_force(&[] as &[u32]), None);
    }

    #[test]
    fn test_brute_force() {
        test_cases_i32(brute_force::<i32>);
        test_cases_u32(brute_force::<u32>);
        test_cases_f64(brute_force::<f64>);
    }

    #[test]
    fn test_kadane() {
        test_cases_i32(kadane::<i32>);
        test_cases_u32(kadane::<u32>);
        test_cases_f64(kadane::<f64>);
    }
}
