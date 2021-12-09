fn rotate(nums: &mut Vec<i32>, k: i32) {
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

fn main() {
    let mut vec: Vec<i32> = vec![-1, -100, 3, 99];
    rotate(&mut vec, 2);
    assert_eq!(vec, vec![3, 99, -1, -100]);
}
