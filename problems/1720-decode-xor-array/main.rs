fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![first; encoded.len()+1 ];

    for index in 0..encoded.len() {
        ret[index+1] =  ret[index] ^ encoded[index];
    }

    ret
}

fn main() {
    assert_eq!( decode( vec![1, 2, 3], 1 ), vec![1, 0, 2, 1] );
    assert_eq!( decode( vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4] );
}
