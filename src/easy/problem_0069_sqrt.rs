use crate::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        match x {
            0 | 1 => x,
            2 | 3 => 1,
            _ => {
                let mut start = 1;
                let mut end = x / 2 + 1;

                while start < end {
                    let idx = (start + end) / 2;
                    let sqrt = x / idx;

                    match sqrt.cmp(&idx) {
                        std::cmp::Ordering::Equal => return idx,
                        std::cmp::Ordering::Less => end = idx - 1,
                        std::cmp::Ordering::Greater => start = idx + 1,
                    };
                }
                loop {
                    if start > (x / start) {
                        start -= 1;
                        continue;
                    }

                    break;
                }

                start
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
