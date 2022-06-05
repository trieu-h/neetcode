use std::collections::HashMap;

fn solve(vec: Vec<i32>) -> bool  {
    let mut memo = HashMap::new();
    for v in vec.iter() {
        if !memo.contains_key(v) {
            memo.insert(v, 1);
        } else {
            return true;
        }
    };
    return false;
}

fn main() {
    let input = vec![0, 1, 1, 2, 3];
    let res = solve(input);
    println!("{}", res);
}
