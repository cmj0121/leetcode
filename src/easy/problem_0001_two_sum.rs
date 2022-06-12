use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache: HashMap<i32, Vec<i32>> = HashMap::new();

        for (i, item) in nums.iter().enumerate() {
            // save in cache
            if cache.get(item).is_none() {
                cache.insert(*item, vec![i as i32]);
            } else {
                cache.get_mut(item).unwrap().push(i as i32);
            }

            if let Some(v) = cache.get(&(target - item)) {
                match v.len() {
                    1 if (i as i32) != v[0] => return vec![v[0], i as i32],
                    2 => return v.to_vec(),
                    _ => {}
                }
            }
        }
        vec![0, 0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
