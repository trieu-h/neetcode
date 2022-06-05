use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn equal(s1: &Vec<char>, s2: &Vec<char>) -> bool {
    let mut memo = HashMap::new();

    if s1.len() != s2.len() {
        return false;
    }

    for c in s1.iter() {
        memo.insert(c, 1);
    }

    for c in s2.iter() {
        if !memo.contains_key(c) {
            return false;
        }
    }

    true
}


fn solve(input: Vec<String>) -> Vec<Vec<String>> {
    let mut words_m: HashMap<Vec<char>, Vec<String>> = HashMap::new();

    for word in input {
        let keys: Vec<Vec<char>> = words_m.keys().cloned().collect();

        let chars: Vec<char> = word.chars().collect();

        let found = keys
                .iter()
                .find(|key| equal(key, &chars));

        match found {
            None => {
                words_m.insert(chars, vec![word.to_string()]);
            },
            Some(key) => {
                match words_m.entry(key.to_vec()) {
                    Entry::Occupied(mut words) => {
                        words.get_mut().push(word.to_string());
                    },
                    Entry::Vacant(e) => {
                        e.insert(vec![word.to_string()]);
                    }
                }
            }
        }
    }

    words_m.values().cloned().collect()
}

fn main() {
    let input = vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat"),
    ];
    let res = solve(input);
    println!("{:?}", res);
}
