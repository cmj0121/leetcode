fn get_smallest_string(n: i32, k: i32) -> String {
    let mut ret: Vec<u8> = vec![0; n as usize];
    let mut remains = k - n;

    for idx in (0..n as usize).rev() {
        if remains > 25 {
            ret[idx] += 25 as u8;
            remains -= 25;
        } else {
            ret[idx] += remains as u8;
            remains = 0;
        }

        if remains < 0 || ( remains > 0 && idx == 0) {
            panic!("should not be here: {} {}: {:?}", remains, idx, ret);
        }
    }

    let mut s = "".to_string();
    for ch in ret {
        s += &((0x61 + ch) as char).to_string()
    }

    s
}

fn main() {
    assert_eq!( get_smallest_string(3, 27), "aay".to_string() );
    assert_eq!( get_smallest_string(5, 73), "aaszz".to_string() );
}
