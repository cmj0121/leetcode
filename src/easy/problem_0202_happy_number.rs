use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let res: HashMap<i32, bool> = HashMap::new();
        Solution::is_happy_inner(n, res)
    }

    pub fn is_happy_inner(n: i32, mut res: HashMap<i32, bool>) -> bool {
        match res.get(&n) {
            Some(_) => false,
            None => {
                match n {
                    1 => true,
                    7 => true,  // 49 -> 97 -> 130 -> 10 -> 1
                    13 => true, // 10 -> 1
                    28 => true, // 68 -> 100 -> 1
                    31 => true, // 10 -> 1
                    49 => true, // 130 -> 10 -> 1
                    68 => true, // 100 -> 1
                    82 => true, // 68 -> 100 -> 1
                    86 => true, // 100 -> 1
                    94 => true, // 97 -> 130 -> 10 -> 1
                    97 => true, // 130 -> 10 -> 1

                    2 => false,
                    3 => false,
                    4 => false,
                    5 => false,
                    6 => false,
                    8 => false,
                    9 => false,

                    _ => {
                        let mut sum = 0;
                        let mut x = n;

                        while x > 0 {
                            sum += (x % 10) * (x % 10);
                            x /= 10;
                        }

                        res.insert(n, true);
                        Solution::is_happy_inner(sum, res)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(2), false);
    }
}
