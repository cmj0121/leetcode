pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![0; code.len()];

    match k {
        _ if k > 0 => {
            let mut tmp = code[1..(k as usize + 1)].iter().sum();

            for index in 0..code.len() {
                ret[index] = tmp;
                tmp = tmp - code[(index+1) % code.len()] + code[(index + 1 + (k as usize)) % code.len()];
            }
        },
        _ if k < 0 => {
            let mut tmp = code[(code.len() as i32 + k) as usize % code.len() ..].iter().sum();

            for index in 0..code.len() {
                ret[index] = tmp;
                tmp = tmp - code[((code.len()+index) as i32 + k) as usize % code.len()] + code[(code.len()+index) % code.len()];
            }
        },
        _ => {},
    }

    ret
}

fn main() {
    assert_eq!( decrypt( vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0,] );
    assert_eq!( decrypt( vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13] );
    assert_eq!( decrypt( vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13] );
}
