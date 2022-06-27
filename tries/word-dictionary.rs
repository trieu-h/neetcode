use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    children: HashMap<char, Node>,
    is_end: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

struct WordDictionary {
    root: Node,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary { root: Node::new() }
    }

    fn add_word(&mut self, word: String) -> () {
        let mut cur = &mut self.root;
        for c in word.chars() {
            if cur.children.contains_key(&c) == false {
                cur.children.insert(c, Node::new());
            }
            cur = cur.children.get_mut(&c).unwrap();
        }
        cur.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        fn recursive_search(node: &Node, i: usize, word: &String) -> bool {
            let len = word.len();
            let mut cur = node;
            let slice = word.chars().skip(i).take(len).collect::<Vec<char>>();

            for (j, c) in slice.into_iter().enumerate() {
                if c != '.' {
                    if cur.children.contains_key(&c) == false {
                        return false;
                    }
                    cur = cur.children.get(&c).unwrap();
                } else {
                    for (_k, node) in cur.children.iter() {
                        return recursive_search(&node, i + j + 1, word);
                    }
                }
            }

            return cur.is_end;
        }

        recursive_search(&self.root, 0, &word)
    }
}

fn main() {
    let mut dict = WordDictionary::new();
    dict.add_word("trieu".to_string());
    let res = dict.search("t.i.u".to_string());
    println!("{}", res);
}
