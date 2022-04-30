fn convert_to_base7(num: i32) -> String {
    match num {
        n if num < 0 => "-".to_owned() + &convert_to_base7(n*-1),
        0 => "0".to_string(),
        1 => "1".to_string(),
        2 => "2".to_string(),
        3 => "3".to_string(),
        4 => "4".to_string(),
        5 => "5".to_string(),
        6 => "6".to_string(),
        _ => {
            convert_to_base7(num / 7) + &convert_to_base7(num % 7)
        }
    }
}

fn main() {
    assert_eq!(convert_to_base7(100), "202");
    assert_eq!(convert_to_base7(-7), "-10");
}
