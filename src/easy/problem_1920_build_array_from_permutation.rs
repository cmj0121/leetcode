use crate::Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = nums.clone();

        for (idx, value) in nums.iter().enumerate() {
            ret[idx] = nums[(*value) as usize];
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::build_array(vec![0, 2, 1, 5, 3, 4]),
            vec![0, 1, 2, 4, 5, 3]
        );
    }
}
