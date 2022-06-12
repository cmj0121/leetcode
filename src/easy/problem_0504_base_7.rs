use crate::Solution;

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        match num {
            n if num < 0 => "-".to_owned() + &Solution::convert_to_base7(-n),
            0 => "0".to_string(),
            1 => "1".to_string(),
            2 => "2".to_string(),
            3 => "3".to_string(),
            4 => "4".to_string(),
            5 => "5".to_string(),
            6 => "6".to_string(),
            _ => Solution::convert_to_base7(num / 7) + &Solution::convert_to_base7(num % 7),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::convert_to_base7(100), "202");
        assert_eq!(Solution::convert_to_base7(-7), "-10");
    }
}
