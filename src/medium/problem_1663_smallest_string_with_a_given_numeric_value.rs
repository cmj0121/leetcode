use crate::Solution;

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut ret: Vec<u8> = vec![0; n as usize];
        let mut remains = k - n;

        for idx in (0..n as usize).rev() {
            if remains > 25 {
                ret[idx] += 25_u8;
                remains -= 25;
            } else {
                ret[idx] += remains as u8;
                remains = 0;
            }

            if remains < 0 || (remains > 0 && idx == 0) {
                panic!("should not be here: {} {}: {:?}", remains, idx, ret);
            }
        }

        let mut s = "".to_string();
        for ch in ret {
            s += &((0x61 + ch) as char).to_string()
        }

        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::get_smallest_string(3, 27), String::from("aay"));
        assert_eq!(Solution::get_smallest_string(5, 73), String::from("aaszz"));
    }
}
