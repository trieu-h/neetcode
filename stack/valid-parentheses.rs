use std::collections::HashMap;

fn solve(str: String) -> bool {
    let mut stack = Vec::<char>::new();
    let mut closing_parens = HashMap::new();
    closing_parens.insert(')', '(');
    closing_parens.insert('}', '{');
    closing_parens.insert(']', '[');
    for char in str.chars() {
        if closing_parens.contains_key(&char) == false {
            stack.push(char);
            continue;
        }

        if let Some(last) = stack.last() {
            let matched_open_paren = closing_parens.get(&char).unwrap();
            if matched_open_paren == last {
                stack.pop();
            } else {
                stack.push(char);
            }
        } else {
            stack.push(char);
        }
    }

    stack.len() == 0
}

fn main() {
    let res = solve("{}()[]".to_string());
    assert!(res);
}
