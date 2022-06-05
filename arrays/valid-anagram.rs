fn solve(s1: String, s2: String) -> bool {
    let len = s1.len();
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();

    for i in 0..len/2 {
        if s1[i] != s2[len - 1 - i] {
            return false;
        }
    }

    true
}

fn main() {
    let s1 = String::from("anagram");
    let s2 = String::from("magrana");
    let res = solve(s1, s2);
    println!("{}", res);
}
