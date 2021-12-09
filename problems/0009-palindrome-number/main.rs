fn is_palindrome(x: i32) -> bool {
    if x >= 0 {
        let mut a = x;
        let mut b = 0;

        while a > 0 {
            b = b * 10 + a % 10;
            a /= 10;
        }

        return b == x;
    }

    false
}

fn main() {
    println!("{} {}", 123, is_palindrome(123));
    println!("{} {}", 121, is_palindrome(121));
    println!("{} {}", -121, is_palindrome(-121));
}
