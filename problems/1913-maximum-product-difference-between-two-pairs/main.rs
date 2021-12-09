pub fn max_product_difference(nums: Vec<i32>) -> i32 {
    let mut ret = nums.clone();

    ret.sort();
    let len = ret.len();
    ret[len - 1] * ret[len - 2] - ret[0] * ret[1]
}

fn main() {
    assert_eq!(max_product_difference(vec![5, 6, 2, 7, 4]), 34);
    assert_eq!(max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]), 64);
}
