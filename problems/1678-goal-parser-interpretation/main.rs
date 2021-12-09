fn interpret(command: String) -> String {
    let mut ret: String = String::from("");
    let mut stack: String = String::from("");

    for ch in command.chars() {
        match ch {
            'G' => ret.push(ch),
            '(' | 'a' | 'l' => stack.push(ch),
            ')' => {
                stack.push(ch);
                match stack.as_str() {
                    "()" => ret.push('o'),
                    "(al)" => ret += "al",
                    _ => panic!("not here: {:?}", stack),
                }
                stack = String::from("");
            }
            _ => panic!("not here: {:?}", ch),
        }
    }

    ret
}

fn main() {
    assert_eq!(interpret("G".to_string()), "G".to_string());
    assert_eq!(interpret("G()(al)".to_string()), "Goal".to_string());
    assert_eq!(
        interpret("G()()()()(al)".to_string()),
        "Gooooal".to_string()
    );
    assert_eq!(
        interpret("(al)G(al)()()G".to_string()),
        "alGalooG".to_string()
    );
}
