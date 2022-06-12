use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut counter: HashMap<char, i32> = HashMap::new();
        let mut total_counter: HashMap<char, i32> = HashMap::new();

        for ch in target.chars() {
            *counter.entry(ch).or_insert(0) += 1;
        }

        for ch in s.chars() {
            *total_counter.entry(ch).or_insert(0) += 1;
        }

        let mut ctx = i32::MAX;
        for (key, value) in counter.iter() {
            match total_counter.get(key) {
                Some(c) => ctx = std::cmp::min(ctx, c / value),
                None => return 0,
            };
        }

        ctx
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::rearrange_characters(
                String::from("ilovecodingonleetcode"),
                String::from("code")
            ),
            2
        );
        assert_eq!(
            Solution::rearrange_characters(String::from("abcba"), String::from("abc")),
            1
        );
        assert_eq!(
            Solution::rearrange_characters(String::from("abbaccaddaeea"), String::from("aaaaa")),
            1
        );
    }
}
