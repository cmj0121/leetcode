pub fn reverse(x: i32) -> i32 {
    let mut minus: bool = false;
    let mut val: i32 = x;
    let mut ret: i32 = 0;

    if x < 0 {
        minus = true;
        if x == std::i32::MIN {
            return 0;
        }
        val *= -1;
    }

    while val > 0 {
        if ret > (std::i32::MAX - (val%10)) / 10 {
            // too large
            return 0;
        }

        ret = ret * 10;
        ret += val % 10;
        val /= 10;
    }

    if minus {
        ret *= -1;
    }

    ret
}

fn main() {
    println!("{} -> {}", 123, reverse(123));
    println!("{} -> {}", -123, reverse(-123));
    println!("{} -> {}", std::i32::MAX, reverse(std::i32::MAX));
    println!("{} -> {}", std::i32::MIN, reverse(std::i32::MIN));
}
