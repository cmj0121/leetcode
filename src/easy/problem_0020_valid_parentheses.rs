use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];

        for c in s.chars() {
            match c {
                '{' | '[' | '(' => stack.push(c),
                x => match stack.pop() {
                    Some('{') if x == '}' => {}
                    Some('[') if x == ']' => {}
                    Some('(') if x == ')' => {}
                    _ => return false,
                },
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }
}
