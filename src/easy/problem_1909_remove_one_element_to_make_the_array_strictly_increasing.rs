use crate::Solution;

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut pick: bool = false;
        let mut prev: i32 = std::i32::MIN;

        for idx in 1..nums.len() {
            if pick && nums[idx] <= prev && (idx > 2 && nums[idx - 1] <= nums[idx - 3]) {
                return false;
            }
            if nums[idx] <= nums[idx - 1] {
                if pick {
                    return false;
                }
                pick = true;
            }

            prev = nums[idx - 1];
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::can_be_increasing(vec![1, 2, 10, 5, 7]), true);
        assert_eq!(Solution::can_be_increasing(vec![2, 3, 1, 2]), false);
        assert_eq!(Solution::can_be_increasing(vec![1, 1, 1]), false);
        assert_eq!(Solution::can_be_increasing(vec![105, 924, 32, 968]), true);
    }
}
