fn majority_element(nums: Vec<i32>) -> i32 {
    // the majority element in the arrary which appears more than [n / 2] times.
    match nums.len() {
        1 => nums[0],
        // the same of the two element, assume the answer exists
        2 => nums[0],
        _ => {
            let mut remains = nums[0];
            let mut count = 1;
            let mut idx = 1;

            while idx < nums.len() {
                remains = match nums[idx] == remains {
                    true => {
                        count += 1;
                        remains
                    }
                    false => {
                        count -= 1;
                        if count == 0 {
                            idx += 1;

                            remains = nums[idx];
                            count = 1;
                        }
                        remains
                    }
                };

                idx += 1;
            }

            remains
        }
    }
}

fn main() {
    assert_eq!(majority_element(vec![1]), 1);
    assert_eq!(majority_element(vec![1, 1]), 1);
    assert_eq!(majority_element(vec![3, 2, 3]), 3);
    assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
