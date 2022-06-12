use crate::Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut ret: String = String::from("");
        let mut stack: String = String::from("");

        for ch in command.chars() {
            match ch {
                'G' => ret.push(ch),
                '(' | 'a' | 'l' => stack.push(ch),
                ')' => {
                    stack.push(ch);
                    match stack.as_str() {
                        "()" => ret.push('o'),
                        "(al)" => ret += "al",
                        _ => panic!("not here: {:?}", stack),
                    }
                    stack = String::from("");
                }
                _ => panic!("not here: {:?}", ch),
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
        assert_eq!(Solution::interpret(String::from("G")), String::from("G"));
        assert_eq!(
            Solution::interpret(String::from("G()(al)")),
            String::from("Goal")
        );
        assert_eq!(
            Solution::interpret(String::from("G()()()()(al)")),
            String::from("Gooooal")
        );
        assert_eq!(
            Solution::interpret(String::from("(al)G(al)()()G")),
            String::from("alGalooG")
        );
    }
}
