use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut cache: HashMap<i32, Vec<i32>> = HashMap::new();

    for i in 0..nums.len() {
        // save in cache
        if cache.get(&nums[i]).is_none() {
            cache.insert(nums[i], vec![i as i32]);
        } else {
            cache.get_mut(&nums[i]).unwrap().push(i as i32);
        }

        match cache.get(&(target - nums[i])) {
            Some(v) => match v.len() {
                1 if (i as i32) != v[0] => return vec![i as i32, v[0]],
                2 => return v.to_vec(),
                _ => {}
            },
            _ => {}
        }
    }
    vec![0, 0]
}

fn main() {
    println!("{:?}", two_sum(vec![1, 2, 3, 4], 3));
    println!("{:?}", two_sum(vec![3, 3], 6));
    println!("{:?}", two_sum(vec![3, 2, 4], 6));
}
