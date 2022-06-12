fn my_atoi(s: String) -> i32 {
    let mut ret: i32 = 0;
    let mut leading = true;
    let mut minus = false;

    for c in s.chars() {
        match c {
            ' ' | '\t' if leading => continue,
            ' ' | '\t' if !leading => break,
            '-' if leading => minus = true,
            '+' if leading => minus = false,
            '0'..='9' => {
                let x = (c as i32) - 0x30;

                if std::i32::MAX / 10 < ret || (std::i32::MAX - ret * 10) < x {
                    return match minus {
                        true => std::i32::MIN,
                        false => std::i32::MAX,
                    };
                }
                ret = ret * 10 + x;
            }
            _ => break,
        };

        leading = false;
    }

    if minus {
        ret *= -1;
    }

    ret
}

fn main() {
    assert_eq!(my_atoi("42".to_string()), 42);
    assert_eq!(my_atoi(" -42".to_string()), -42);
    assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(my_atoi("words and 987".to_string()), 0);
    assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
}
