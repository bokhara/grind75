use std::collections::LinkedList;

struct MinStack {
    stack: LinkedList<i32>,
    min_element: i32,
}
/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: LinkedList::new(),
            min_element: i32::MAX,
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push_back(val);
        if val < self.min_element {
            self.min_element = val;
        }
    }

    fn pop(&mut self) {
        self.stack.pop_back();
        let mut iter = self.stack.iter();
        self.min_element = i32::MAX;
        loop {
            match iter.next() {
                Some(v) => {
                    if *v < self.min_element {
                        self.min_element = *v;
                    }
                }
                None => {
                    break;
                }
            }
        }
    }

    fn top(&mut self) -> i32 {
        return *self.stack.back().unwrap();
    }

    fn get_min(&self) -> i32 {
        return self.min_element;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_stack() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
