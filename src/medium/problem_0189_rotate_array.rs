use crate::Solution;

impl Solution {
    pub fn rotate(nums: &mut [i32], k: i32) {
        let len = nums.len();
        let mut runs = nums.len() - 1;
        let mut idx = 0;

        while runs > 0 {
            let mut pos = idx;

            while runs > 0 {
                let next = (pos + 2 * len - (k as usize)) % len;

                if next == idx {
                    runs -= 1;
                    break;
                }

                nums.swap(pos, next);
                runs -= 1;
                pos = next;
            }

            idx += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        let mut v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];

        Solution::rotate(&mut v, 3);
        assert_eq!(v, vec![5, 6, 7, 1, 2, 3, 4]);
    }
}
