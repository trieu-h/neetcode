use std::collections::HashMap;

fn solve(nums: Vec<i64>, target: i64) -> Option<(i64, i64)> {
    let mut memo = HashMap::new();

    for num in nums.iter() {
        if memo.contains_key(num) {
            return Some((*num, *memo.get(num).unwrap()))
        } else {
            let rest = target - num;
            memo.insert(rest, *num);
        }
    }

    None
}

fn main() {
    let input = vec![3,3];
    let res = solve(input, 6);
    match res {
        Some((x, y)) => println!("Found pairs: ({}, {})",x,y),
        None   => println!("No pairs found")
    }
}
