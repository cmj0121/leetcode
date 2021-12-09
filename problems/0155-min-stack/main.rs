struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        let stack: Vec<(i32, i32)> = vec![];
        Self { stack: stack }
    }

    fn push(&mut self, val: i32) {
        let min: i32 = match self.stack.last() {
            Some(v) => std::cmp::min(val, v.1),
            None => val,
        };
        self.stack.push((val, min))
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

fn main() {
    let mut obj = MinStack::new();
    obj.push(12);
    obj.pop();
    obj.push(1);
    obj.top();
    obj.get_min();
}
