pub fn remove_occurrences(s: String, part: String) -> String {
    let mut ret = String::from("");

    for ch in s.chars() {
        ret.push(ch);
        if ret.len() >= part.len() && ret[(ret.len() - part.len())..] == part {
            ret.truncate(ret.len() - part.len());
        }
    }

    ret
}

fn main() {
    assert_eq!( remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()), "dab".to_string() );
    assert_eq!( remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string()), "ab".to_string() );
}
