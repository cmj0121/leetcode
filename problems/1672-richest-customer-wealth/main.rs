fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut max: i32 = 0;

    for customer in accounts.iter() {
        let sum: i32 = customer.iter().sum();
        max = std::cmp::max(sum, max);
    }

    max
}

fn main() {
    assert_eq!( maximum_wealth( vec![ vec![1,2,3], vec![3,2,1] ]), 6);
    assert_eq!( maximum_wealth( vec![ vec![1,5], vec![7,3], vec![3,5] ]), 10);
}
