use std::collections::HashMap;

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

struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Trie { root: Node::new() }
    }

    fn insert(&mut self, word: String) {
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
        let mut cur = &self.root;
        for c in word.chars() {
            if cur.children.contains_key(&c) == true {
                cur = cur.children.get(&c).unwrap();
            } else {
                return false;
            }
        }
        return cur.is_end;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = &self.root;
        for c in prefix.chars() {
            if cur.children.contains_key(&c) == true {
                cur = cur.children.get(&c).unwrap();
            } else {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("abc".to_string());
    trie.insert("efd".to_string());
    trie.insert("trieu".to_string());
    assert!(trie.search("trieu".to_string()));
    assert!(trie.starts_with("trie".to_string()));
}
