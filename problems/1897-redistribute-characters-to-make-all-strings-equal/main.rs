fn make_equal(words: Vec<String>) -> bool {
    /*
        // Slow version
        let len = words.len();
        let mut cache: HashMap<char, usize> = HashMap::new();

        for word in words.iter() {
            for ch in word.chars() {
                cache.insert( ch, cache.get(&ch).unwrap_or(&0)+1 );
            }
        }

        for (_, count) in cache.iter() {
            if count % len != 0 {
                return false;
            }
        }
    */

    // still the slow version
    let len = words.len();
    let s = words.join("");
    let mut chars = s.chars().collect::<Vec<char>>();

    chars.sort_by(|x, y| x.cmp(y));
    for idx in (0..chars.len()).step_by(len) {
        if idx + len - 1 >= chars.len() {
            return false;
        } else if chars[idx] != chars[idx + len - 1] {
            return false;
        }
    }

    true
}

fn main() {
    assert_eq!(
        make_equal(vec![
            "abc".to_string(),
            "aabc".to_string(),
            "bc".to_string()
        ]),
        true
    );
    assert_eq!(make_equal(vec!["ab".to_string(), "b".to_string()]), false);
}
