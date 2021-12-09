fn my_pow(x: f64, n: i32) -> f64 {
    if x == 1.0 {
        return 1.0;
    } else if x == -1.0 {
        if  n % 2 == 0 {
            return 1.0;
        }
        return -1.0;
    } else if n == std::i32::MIN {
        return 0.0;
    } else if n < 0 {
        return 1.0 / my_pow(x, -n);
    }

    let mut ret: f64 = 1.0;
    let mut tmp: f64 = x;
    let mut exp: i32 = n;

    while exp > 0 {
        if exp % 2 == 1 {
            ret *= tmp;
        }
        tmp *= tmp;
        exp /= 2;
    }

    ret
}

fn main() {
    assert!( (my_pow(2.0, 10) - 1024.00000).abs() < 0.00001);
    assert!( (my_pow(2.10000, 3) -9.26100).abs() < 0.00001);
    assert!( (my_pow(2.00000, -2) - 0.25000).abs() < 0.00001);
}
