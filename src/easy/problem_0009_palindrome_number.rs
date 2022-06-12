use crate::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x >= 0 {
            let mut a = x;
            let mut b = 0;

            while a > 0 {
                b = b * 10 + a % 10;
                a /= 10;
            }

            return b == x;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::is_palindrome(123), false);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
    }
}
