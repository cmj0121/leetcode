fn my_sqrt(x: i32) -> i32 {
    match x {
        0 | 1 => x,
        2 | 3 => 1,
        _ => {
            let mut start = 1;
            let mut end = x / 2 + 1;

            while start < end {
                let idx = (start + end) / 2;
                let sqrt = x / idx;

                if sqrt == idx {
                    return idx;
                } else if sqrt < idx {
                    end = idx - 1;
                } else {
                    start = idx + 1;
                }
            }
            loop {
                if start > (x / start) {
                    start -= 1;
                    continue;
                }

                break;
            }

            start
        }
    }
}

fn main() {
    assert_eq!(my_sqrt(1), 1);
    assert_eq!(my_sqrt(2), 1);
    assert_eq!(my_sqrt(3), 1);
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(8), 2);
    assert_eq!(my_sqrt(10), 3);
    assert_eq!(my_sqrt(2147483647), 46340);
}
