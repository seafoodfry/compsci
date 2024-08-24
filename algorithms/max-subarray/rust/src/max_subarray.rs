use std::cmp::PartialOrd;
use std::ops::{Add, AddAssign};

// Derive Debug to print them and PartialEq to test them.
#[derive(Debug, PartialEq)]
pub struct SubArrayResult<N> {
    pub sum: N,
    pub start: usize,
    pub end: usize,
}

pub fn brute_force<N>(nums: &[N]) -> Option<SubArrayResult<N>>
where
    N: Default + Copy + PartialOrd + Add<Output = N> + AddAssign,
    // Default -> N:default() to provide a zero value.
    // Copy -> needed for += nums[j].
    // PartialOrd -> needed for comparisons.
    // Add<Output = N> -> needed to add values together.
    // AddAssign -> needed for += nums[j].
{
    if nums.is_empty() {
        return None;
    }

    let mut max_result = SubArrayResult {
        // Using N::default() will result in the wrong result if all entries are negative.
        sum: nums[0],
        start: 0,
        end: 0,
    };
    for start in 0..nums.len() {
        let mut current_sum = N::default();

        for (end, _) in nums.iter().enumerate().skip(start) {
            current_sum += nums[end];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brute_force() {
        let vec1 = vec![0, 1, 2, 3, 4, 5];
        let book = vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];

        // We only need to specify the generic type for this next function because that is the only
        // instance in which the compiler won't be able to infer the type.
        assert_eq!(brute_force::<i32>(&[]), None);
        // We could also do...
        assert_eq!(brute_force(&[] as &[u32]), None);

        assert_eq!(
            brute_force(&[0]),
            Some(SubArrayResult {
                sum: 0,
                start: 0,
                end: 0
            })
        );
        assert_eq!(
            brute_force(&[0, 0, 0]),
            Some(SubArrayResult {
                sum: 0,
                start: 0,
                end: 0
            })
        );

        assert_eq!(
            brute_force(&[1, 2, 3]),
            Some(SubArrayResult {
                sum: 6,
                start: 0,
                end: 2
            })
        );
        assert_eq!(
            brute_force(&vec1),
            Some(SubArrayResult {
                sum: 15,
                start: 0,
                end: 5
            })
        );
        assert_eq!(
            brute_force(&[-1, -2, -3]),
            Some(SubArrayResult {
                sum: -1,
                start: 0,
                end: 0
            })
        );
        assert_eq!(
            brute_force(&book),
            Some(SubArrayResult {
                sum: 43,
                start: 7,
                end: 10
            })
        );
    }
}
