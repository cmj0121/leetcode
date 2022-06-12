use crate::Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut cache: Vec<i32> = vec![1];

        for (idx, ch) in s.chars().enumerate() {
            match idx {
                0 => match ch {
                    '1'..='9' => cache.push(1),
                    _ => cache.push(0),
                },
                _ => {
                    let prev = s.chars().nth(idx - 1).unwrap();
                    let len = cache.len();

                    match (prev, ch) {
                        (('1'..='2'), '0') => cache.push(cache[len - 2]),
                        ('1', ('1'..='9')) => cache.push(cache[len - 1] + cache[len - 2]),
                        ('2', ('1'..='6')) => cache.push(cache[len - 1] + cache[len - 2]),
                        (_, ('1'..='9')) => cache.push(cache[len - 1]),
                        _ => cache.push(0),
                    }
                }
            }
        }

        *cache.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::num_decodings(String::from("12")), 2);
        assert_eq!(Solution::num_decodings(String::from("226")), 3);
        assert_eq!(Solution::num_decodings(String::from("06")), 0);
    }
}
