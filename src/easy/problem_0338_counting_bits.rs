use crate::Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        match n {
            0 => vec![0],
            1 => vec![0, 1],
            _ => {
                let mut ret = vec![0, 1];
                let mut x = 0;
                let mut y = ret.len();

                for _m in 1..n {
                    ret.push(ret[x] + 1);
                    x += 1;
                    if x == y {
                        x = 0;
                        y *= 2;
                    }
                }

                ret
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::count_bits(0), vec![0]);
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
