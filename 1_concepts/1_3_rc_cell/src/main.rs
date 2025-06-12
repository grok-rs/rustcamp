use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
struct GlobalStack<T> {
    stack: Rc<RefCell<Vec<T>>>,
}

impl<T> GlobalStack<T> {
    fn new() -> Self {
        GlobalStack {
            stack: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }

    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }

    fn len(&self) -> usize {
        self.stack.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.stack.borrow().is_empty()
    }
}

fn main() {
    let stack = GlobalStack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Stack length: {}", stack.len());

    let stack_clone = stack.clone();

    stack_clone.push(4);

    while let Some(value) = stack.pop() {
        println!(
            "Popped value: {}, length {}, stack is empty? {}",
            value,
            stack.len(),
            stack.is_empty()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let stack = GlobalStack::new();

        // Push elements
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Check size
        assert_eq!(stack.len(), 3);

        // Pop elements
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));

        // Stack should now be empty
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_shared_mutation() {
        let stack = GlobalStack::new();

        // Clone the stack
        let stack_clone = stack.clone();

        // Push elements from the original stack
        stack.push(1);
        stack.push(2);

        // The clone should reflect the changes
        assert_eq!(stack_clone.len(), 2);
        assert_eq!(stack_clone.pop(), Some(2));

        // Mutate from the clone
        stack_clone.push(3);

        // Original stack should reflect the changes
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop(), Some(3));
    }

    #[test]
    fn test_empty_stack() {
        let stack: GlobalStack<i32> = GlobalStack::new();

        // Initially empty
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);

        // Pop from empty stack
        assert_eq!(stack.pop(), None);
    }

        #[test]
    fn test_drop_origin_stack() {
        let stack: GlobalStack<i32> = GlobalStack::new();
        let stack_clone = stack.clone();
        stack.push(1);
        drop(stack);

        stack_clone.push(2);

        assert_eq!(stack_clone.len(), 2);
        assert_eq!(stack_clone.pop(), Some(2));
        assert_eq!(stack_clone.pop(), Some(1));
    }
}
