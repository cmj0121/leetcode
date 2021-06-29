fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut cache: Vec<Vec<String>> = vec![ vec!["".to_string()] ];

    for x in 1..(n+1) {
        let mut cache_c: Vec<String> = vec!{};

        for c in 0..x {
            for left in cache[ c as usize ].iter() {
                for right in cache[ (x-c-1) as usize ] .iter() {
                    cache_c.push(format!("({}){}", left, right));
                }
            }

        }
        cache.push(cache_c);
    }

    cache.last().unwrap().to_vec()
}

fn main() {
    assert_eq!( generate_parenthesis(1), vec![ "()".to_string() ] );
    assert_eq!( generate_parenthesis(2), vec![ "()()".to_string(), "(())".to_string() ]);
    assert_eq!( generate_parenthesis(3), vec![ "()()()".to_string(), "()(())".to_string(), "(())()".to_string(), "(()())".to_string(), "((()))".to_string() ] );
}
