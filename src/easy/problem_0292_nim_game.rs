use crate::Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        // you get first
        // pick the 4-x stone which x is the number of another player pick
        // your last term and remains 4 stone, win
        n % 4 != 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::can_win_nim(4), false);
        assert_eq!(Solution::can_win_nim(1), true);
    }
}
