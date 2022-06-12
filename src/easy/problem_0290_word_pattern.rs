use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut cache: HashMap<char, String> = HashMap::new();
        let mut idx = 0;

        for word in s.split(' ') {
            match pattern.chars().nth(idx) {
                Some(ch) => match cache.get(&ch) {
                    Some(w) if w != word => return false,
                    Some(_) => {}
                    None if cache.values().any(|v| v == word) => return false,
                    None => {
                        cache.insert(ch, word.to_string());
                    }
                },
                None => return false,
            }

            idx += 1
        }

        idx == pattern.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat dog")),
            true
        );
        assert_eq!(
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat fish")),
            false
        );
        assert_eq!(
            Solution::word_pattern(String::from("aaaa"), String::from("dog cat cat dog")),
            false
        );
    }
}
