use std::ops::{Add, AddAssign};
use std::cmp::PartialOrd;


pub fn brute_force<N>(nums: &[N]) -> Option<N> 
where
    N: Default + Copy + PartialOrd + Add<Output = N> + AddAssign,
    // Default -> N:default() to provide a zero value.
    // Copy -> needed for += nums[j].
    // PartialOrd -> needed for comparisons.
    // Add<Output = N> -> needed to add values together.
    // AddAssign -> needed for += nums[j].
{
    let n = nums.len();
    if n == 0 {
        return None;
    }

    let mut max_sum = nums[0];
    for i in 0..n {
        let mut current_sum = N::default();
        for j in i..n {
            current_sum += nums[j];
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
    }

    Some(max_sum)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brute_force() {
        let vec1 = vec![0, 1, 2, 3, 4, 5];
        let book = vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];

        // We only need to specify the generic type for this next function because that is the only
        // instance in which the compiler won't be able to infer the type.
        assert_eq!(brute_force::<i32>(&[]), None);
        // We could also do...
        assert_eq!(brute_force(&[] as &[u32]), None);

        assert_eq!(brute_force(&[0]), Some(0));
        assert_eq!(brute_force(&[0, 0, 0]), Some(0));

        assert_eq!(brute_force(&[1, 2, 3]), Some(6));
        assert_eq!(brute_force(&vec1), Some(15));
        assert_eq!(brute_force(&[-1, -2, -3]), Some(-1));
        assert_eq!(brute_force(&book), Some(43));
    }
}