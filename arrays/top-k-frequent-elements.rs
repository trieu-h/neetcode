use std::collections::HashMap;

fn solve(nums: Vec<u64>, k: usize) -> Vec<u64> {
    let mut memo = HashMap::new();

    for num in nums {
        memo
            .entry(num)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    let mut memo_vec: Vec<(&u64, &u32)> = memo.iter().collect();

    memo_vec
        .sort_by(|a, b| b.1.cmp(a.1));

    memo_vec
        .iter()
        .map(|entry| entry.0)
        .cloned()
        .take(k)
        .collect::<Vec<_>>()
}

fn main() {
    let input = vec![1,1,1,2,2,3];
    let result = solve(input, 2);
    println!("{:?}", result);
}
