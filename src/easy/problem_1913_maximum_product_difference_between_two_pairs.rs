use crate::Solution;

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut ret = nums;

        ret.sort_unstable();
        let len = ret.len();
        ret[len - 1] * ret[len - 2] - ret[0] * ret[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::max_product_difference(vec![5, 6, 2, 7, 4]), 34);
        assert_eq!(
            Solution::max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]),
            64
        );
    }
}
