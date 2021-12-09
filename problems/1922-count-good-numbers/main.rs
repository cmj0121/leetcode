pub fn count_good_numbers(n: i64) -> i32 {
    match n {
        1 => 5,
        _ => {
            let mut m = n / 2;
            let mut ret: i64 = 1;
            let mut pow: i64 = 20;
            let module: i64 = (10 as i64).pow(9) + 7;

            while m > 0 {
                if m % 2 == 1 {
                    ret = ret * pow % module;
                }
                pow = pow * pow % module;
                m = m / 2;
            }

            if n % 2 == 1 {
                ret = ret * 5 % module;
            }

            ret as i32
        }
    }
}

fn main() {
    assert_eq!(count_good_numbers(1), 5);
    assert_eq!(count_good_numbers(3), 100);
    assert_eq!(count_good_numbers(4), 400);
    assert_eq!(count_good_numbers(5), 2000);
    assert_eq!(count_good_numbers(50), 564908303);
}
