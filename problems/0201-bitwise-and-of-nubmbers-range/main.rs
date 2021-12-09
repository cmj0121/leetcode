fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let mut diff = right - left;
    let mut ret = left;
    let mut bits = 0;

    while ret > 0 && diff > 0 {
        ret = ret >> 1;
        diff /= 2;
        bits += 1;
    }

    ret = ret << bits;

    ret & right
}

fn main() {
    assert_eq!(range_bitwise_and(5, 7), 4);
    assert_eq!(range_bitwise_and(0, 0), 0);
    assert_eq!(range_bitwise_and(1, 2147483647), 0);
    assert_eq!(range_bitwise_and(3, 4), 0);
}
