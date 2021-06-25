fn roman_to_int(s: String) -> i32 {
    fn rtoi(ch: char) -> i32 {
        match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }

    let mut ret: i32 = 0;

    for (idx, ch) in s.chars().enumerate() {
        ret += match idx {
            0 => rtoi(ch),
            _ => {
                let prev = rtoi(s.chars().nth(idx-1).unwrap());
                let mut curr = rtoi(ch);

                if prev < curr {
                    curr = curr - prev - prev;
                }

                curr
            }
        };
    }

    ret
}


fn main() {
    assert_eq!(roman_to_int("MCMXCIV".to_string()), 58);
}
