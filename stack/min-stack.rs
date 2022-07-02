use std::collections::VecDeque;

struct MinStack {
    stack: VecDeque<i32>,
    min_stack: Vec<i32>,
}
// [2, 1, 0, -1];
// [2, 1, 0, 3, 5, 6, -1];

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: VecDeque::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push_back(val);
        if &val <= self.stack.back().unwrap() || self.min_stack.is_empty() {
            self.min_stack.push(val);
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if let Some(last) = self.stack.pop_back() {
            if self.min_stack.last() == Some(&last) {
                self.min_stack.pop();
            }
        }
        None
    }

    fn top(&self) -> Option<&i32> {
        self.stack.back()
    }

    fn get_min(&self) -> Option<&i32> {
        self.min_stack.last()
    }

    fn print(&self) {
        for i in self.stack.iter() {
            println!("{}", i);
        }
    }
}

fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(1);
    min_stack.push(2);
    min_stack.push(4);
    min_stack.push(-1);
    println!("{}", min_stack.get_min().unwrap());
    println!("{}", min_stack.top().unwrap());
}
