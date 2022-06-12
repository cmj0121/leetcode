use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        let mut slots = 0;

        for pos in 1..nums.len() {
            if nums[slots] == nums[pos] {
                continue;
            }

            slots += 1;
            if slots < pos {
                nums[slots] = nums[pos];
            }
        }

        if !nums.is_empty() {
            slots += 1;
        }
        slots as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 2, 2]), 2);
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
            5
        );
    }
}
