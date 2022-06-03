struct MyQueue {
    push_stack: Vec<i32>,
    pop_stack: Vec<i32>,
}
/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 */
impl MyQueue {
    fn new() -> Self {
        return MyQueue {
            push_stack: Vec::new(),
            pop_stack: Vec::new(),
        };
    }

    fn push(&mut self, x: i32) {
        while !self.pop_stack.is_empty() {
            self.push_stack.push(self.pop_stack.pop().unwrap());
        }
        self.push_stack.push(x);
        while !self.push_stack.is_empty() {
            self.pop_stack.push(self.push_stack.pop().unwrap());
        }
    }

    fn pop(&mut self) -> i32 {
        self.pop_stack.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.pop_stack.last().cloned().unwrap()
    }

    fn empty(&mut self) -> bool {
        self.pop_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_myqueue() {
        let mut myQueue = MyQueue::new();
        myQueue.push(1); // queue is: [1]
        myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
        assert_eq!(myQueue.peek(), 1); // return 1
        assert_eq!(myQueue.pop(), 1);
        assert_eq!(myQueue.empty(), false);
    }
}
