use std::collections::HashMap;

fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut count: i32 = 0;
    let mut cache: HashMap<i32, usize> = HashMap::new();

    for n in nums.iter() {
        match cache.get_mut( &(k - *n) ) {
            Some(v) if *v > 0 => {
                *v -= 1;
                count += 1;
            },
            _ => {
                cache.insert(*n, 1 + cache.get(n).unwrap_or(&0));
            },
        };
    }

    count
}

fn main() {
    assert_eq!( max_operations( vec![1, 2, 3, 4], 5), 2);
    assert_eq!( max_operations( vec![3, 1, 3, 4, 3], 6), 1);
    assert_eq!( max_operations( vec![2,5,4,4,1,3,4,4,1,4,4,1,2,1,2,2,3,2,4,2], 3), 4);
}
