use crate::Solution;

impl Solution {
    pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
        let mut slots = nums.len();

        for idx in 0..nums.len() {
            let pos = nums.len() - idx - 1;

            if nums[pos] == val {
                nums[pos] = nums[slots - 1];
                slots -= 1;
            }
        }

        slots as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);
        assert_eq!(
            Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2),
            5
        );
    }
}
