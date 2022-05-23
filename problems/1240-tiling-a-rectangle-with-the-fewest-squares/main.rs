use std::collections::HashMap;

fn tiling_rectangle(n: i32, m: i32) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    tiling_rectangle_ex(n, m, &mut map)
}

fn tiling_rectangle_ex(n: i32, m: i32, map: &mut HashMap<(i32, i32), i32>) -> i32 {
    if n > m {
        return tiling_rectangle_ex(m, n, map);
    }

    match (n, m) {
        (1, _) => m,
        (_, 1) => n,
        _ => {
            if n == m {
                return 1;
            }

            if m % n == 0 {
                return m / n;
            }

            match map.get(&(n, m)) {
                Some(ret) => *ret,
                None => {
                    let mut dp_min: i32 = i32::MAX;

                    for idx in 1..n {
                        let left = tiling_rectangle_ex(idx, m, map);
                        let right = tiling_rectangle_ex(n-idx, m, map);

                        let ans = left + right;
                        if ans < dp_min {
                            dp_min = ans;
                        }
                    }

                    for idx in 1..m {
                        let up = tiling_rectangle_ex(n, idx, map);
                        let bottom = tiling_rectangle_ex(n, m-idx, map);

                        let ans = up + bottom;
                        if ans < dp_min {
                            dp_min = ans;
                        }
                    }

                    for x in 1..(n-1) {
                        for y in 1..(m-1) {
                            let left_up = tiling_rectangle_ex(x, y+1, map);
                            let right_up = tiling_rectangle_ex(n-x, y, map);
                            let left_bottom = tiling_rectangle_ex(x+1, m-y-1, map);
                            let right_bottom = tiling_rectangle_ex(n-x-1, m-y, map);

                            let ans = left_up + right_up + left_bottom + right_bottom + 1;
                            if ans < dp_min {
                                dp_min = ans;
                            }
                        }
                    }

                    map.insert((n, m), dp_min);
                    dp_min
                }
            }
        }
    }
}

fn main() {
    assert_eq!(tiling_rectangle(11, 13), 6);
}
