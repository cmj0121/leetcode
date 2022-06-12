use crate::Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut x: i32 = n;
        let mut ret: i32 = 0;

        while x > 0 {
            ret += x / 5;
            x /= 5;
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
        assert_eq!(Solution::trailing_zeroes(5), 1);
        assert_eq!(Solution::trailing_zeroes(0), 0);
    }
}
