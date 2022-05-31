use std::collections::HashMap;

pub fn rearrange_characters(s: String, target: String) -> i32 {
    let mut counter: HashMap<char, i32> = HashMap::new();
    let mut total_counter: HashMap<char, i32> = HashMap::new();

    for ch in target.chars() {
        match counter.get(&ch) {
            Some(c) => counter.insert(ch, c+1),
            None => counter.insert(ch, 1),
        };
    }

    for ch in s.chars() {
        match total_counter.get(&ch) {
            Some(c) => total_counter.insert(ch, c+1),
            None => total_counter.insert(ch, 1),
        };
    }


    let mut ctx = i32::MAX;
    for (key, value) in counter.iter() {
        match total_counter.get(&key) {
            Some(c) => ctx = std::cmp::min(ctx, c / value),
            None => return 0,
        };
    }

    ctx
}

fn main() {
    assert_eq!(rearrange_characters(String::from("ilovecodingonleetcode"), String::from("code")), 2);
    assert_eq!(rearrange_characters(String::from("abcba"), String::from("abc")), 1);
    assert_eq!(rearrange_characters(String::from("abbaccaddaeea"), String::from("aaaaa")), 1);
}
