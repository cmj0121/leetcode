use crate::Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        /*
            // Slow version
            let len = words.len();
            let mut cache: HashMap<char, usize> = HashMap::new();

            for word in words.iter() {
                for ch in word.chars() {
                    cache.insert( ch, cache.get(&ch).unwrap_or(&0)+1 );
                }
            }

            for (_, count) in cache.iter() {
                if count % len != 0 {
                    return false;
                }
            }
        */

        // still the slow version
        let len = words.len();
        let s = words.join("");
        let mut chars = s.chars().collect::<Vec<char>>();

        chars.sort_unstable();
        for idx in (0..chars.len()).step_by(len) {
            if idx + len > chars.len() || chars[idx] != chars[idx + len - 1] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::make_equal(vec![
                String::from("abc"),
                String::from("aabc"),
                String::from("bc")
            ]),
            true
        );
        assert_eq!(
            Solution::make_equal(vec![String::from("ab"), String::from("b")]),
            false
        );
    }
}
