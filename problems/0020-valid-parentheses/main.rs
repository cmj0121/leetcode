fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec!{};

    for c in s.chars() {
        match c {
            '{' | '[' | '(' => stack.push(c),
            x => {
                match stack.pop() {
                    Some('{') if x == '}' => {},
                    Some('[') if x == ']' => {},
                    Some('(') if x == ')' => {},
                    _ => return false,
                }
            }
        }
    }

    stack.len() == 0
}

fn main() {
    println!("{} -> {}", "{}", is_valid("{}".to_string()));
}
