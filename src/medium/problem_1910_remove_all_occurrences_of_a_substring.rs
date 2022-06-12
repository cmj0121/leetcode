use crate::Solution;

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut ret = String::from("");

        for ch in s.chars() {
            ret.push(ch);
            if ret.len() >= part.len() && ret[(ret.len() - part.len())..] == part {
                ret.truncate(ret.len() - part.len());
            }
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
            Solution::remove_occurrences(String::from("daabcbaabcbc"), String::from("abc")),
            String::from("dab")
        );
        assert_eq!(
            Solution::remove_occurrences(String::from("axxxxyyyyb"), String::from("xy")),
            String::from("ab")
        );
    }
}
