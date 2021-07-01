fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ret: Vec<i32> = nums[.. (k as usize)].to_vec();
    let mut pos = 0;

    for idx in 0..(k as usize) {
        if idx < (k as usize) - 1 && ret[idx] > ret[idx+1] {
            break;
        }
        pos += 1;
    }

    for idx in (k as usize)..nums.len() {
        if pos < (k as usize) {
            ret.push( nums[idx] );
            ret.remove(pos);

            pos = 0;
            for idx in 0..ret.len() {
                if idx < (k as usize) - 1 && ret[idx] > ret[idx+1] {
                    break;
                }
                pos += 1;
            }
        } else if nums[idx] < ret[k as usize - 1] {
            ret[k as usize - 1] = nums[idx];
            if k > 1 && ret[k as usize - 2] > ret[k as usize - 1] {
                pos -= 1;

                while pos > 0 {
                    if ret[pos-1] > ret[pos] {
                        pos -= 1;
                        continue;
                    }
                    break;
                }
            }
        } else {
            // nop
            continue;
        }
    }

    ret
}

fn main() {
    assert_eq!( most_competitive(vec![3, 5, 2, 6], 2), vec![2, 6] );
    assert_eq!( most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4), vec![2, 3, 3, 4]);
    assert_eq!( most_competitive(vec![71, 18, 52, 29, 55, 73, 24, 42, 66, 8, 80, 2], 3), vec![8, 80, 2]);
    assert_eq!( most_competitive(vec![5, 4, 3, 2, 1], 1), vec![1]);
}
