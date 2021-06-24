fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut slots = 0;

    for pos in 1..nums.len() {
        if nums[slots] == nums[pos] {
            continue;
        }

        slots += 1;
        if slots < pos {
            nums[slots] = nums[pos];
        }
    }

    if nums.len() > 0 {
        slots += 1;
    }
    slots as i32
}

fn main() {
    let mut case: Vec<i32> = vec![1, 2, 2];
    println!("{} {:?}", remove_duplicates(&mut case), case);
}
