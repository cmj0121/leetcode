fn is_ugly(n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    match n {
        1 => true,
        mut x => {
            while x != 0 && x % 2 == 0 {
                x /= 2
            }

            while x != 0 && x % 3 == 0 {
                x /= 3
            }

            while x != 0 && x % 5 == 0 {
                x /= 5
            }

            println!("{}", x);
            x == 1 || x == -1 || x == 0
        }
    }
}

fn main() {
    assert_eq!(is_ugly(6), true);
    assert_eq!(is_ugly(8), true);
    assert_eq!(is_ugly(14), false);
    assert_eq!(is_ugly(1), true);
    assert_eq!(is_ugly(-2147483648), true);
}
