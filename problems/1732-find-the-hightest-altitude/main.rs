fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut max: i32 = 0;
    let mut cur: i32 = 0;

    for item in gain.iter() {
        cur += item;
        max = std::cmp::max(cur, max);
    }

    max
}

fn main() {
    assert_eq!( largest_altitude( vec![-5, 1, 5, 0, -7] ), 1 );
    assert_eq!( largest_altitude( vec![-4, -3, -2, -1, 4, 3, 2] ), 0 );
}
