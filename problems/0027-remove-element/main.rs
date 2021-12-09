fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut slots = nums.len();

    for idx in 0..nums.len() {
        let pos = nums.len() - idx - 1;

        if nums[pos] == val {
            nums[pos] = nums[slots-1];
            slots -= 1;
        }
    }

    slots as i32
}

fn main() {
    let mut case = vec![4, 4, 4 , 4];
    println!("{} -> {:?}", remove_element(&mut case, 4), case);
}
