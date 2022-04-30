fn is_power_of_four(n: i32) -> bool {
    match n {
        d if d <= 0 => false,
        1 => true,
        _ => {
            let mut res: i32 = n;
            let mut cnt: i32 = 0;

            while res > 1 {
                if res % 2 == 1 {
                    return false;
                }

                res /= 2;
                cnt += 1;
            }

            cnt % 2 == 0
        }
    }
}

fn main() {
    assert_eq!(is_power_of_four(64), true);
    assert_eq!(is_power_of_four(32), false);
    assert_eq!(is_power_of_four(16), true);
    assert_eq!(is_power_of_four(8), false);
    assert_eq!(is_power_of_four(5), false);
    assert_eq!(is_power_of_four(1), true);
    assert_eq!(is_power_of_four(0), false);
    assert_eq!(is_power_of_four(-4), false);
}
