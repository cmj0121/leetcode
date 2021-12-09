use std::collections::HashMap;

fn word_pattern(pattern: String, s: String) -> bool {
    let mut cache: HashMap<char, String> = HashMap::new();
    let mut idx = 0;

    for word in s.split(" ") {
        match pattern.chars().nth(idx) {
            Some(ch) => match cache.get(&ch) {
                Some(w) if w != word => return false,
                Some(_) => {}
                None if cache.values().any(|v| v == word) => return false,
                None => {
                    cache.insert(ch, word.to_string());
                }
            },
            None => return false,
        }

        idx += 1
    }

    idx == pattern.len()
}

fn main() {
    assert_eq!(
        word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
        true
    );
    assert_eq!(
        word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
        false
    );
    assert_eq!(
        word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
        false
    );
    assert_eq!(
        word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
        false
    );
}
