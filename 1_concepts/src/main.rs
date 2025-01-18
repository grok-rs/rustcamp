use std::fmt;
use std::sync::{Arc, Mutex};

pub struct Node<T> {
    value: Arc<T>,
    next: Mutex<Option<Arc<Node<T>>>>,
    prev: Mutex<Option<Arc<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Arc<Self> {
        Arc::new(Self {
            value: Arc::new(value),
            next: Mutex::new(None),
            prev: Mutex::new(None),
        })
    }
}

#[derive(Clone)]
pub struct DoublyLinkedList<T> {
    head: Arc<Mutex<Option<Arc<Node<T>>>>>,
    tail: Arc<Mutex<Option<Arc<Node<T>>>>>,
    length: Arc<Mutex<usize>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: Arc::new(Mutex::new(None)),
            tail: Arc::new(Mutex::new(None)),
            length: Arc::new(Mutex::new(0)),
        }
    }

    pub fn push_front(&self, value: T) {
        let node = Node::new(value);

        let mut head = self.head.lock().unwrap();
        let mut tail = self.tail.lock().unwrap();
        let mut lenght = self.length.lock().unwrap();

        match head.take() {
            Some(old_head) => {
                *node.next.lock().unwrap() = Some(old_head.clone());
                *old_head.prev.lock().unwrap() = Some(node.clone());
                *head = Some(node);
            }
            None => {
                *tail = Some(node.clone());
                *head = Some(node);
            }
        }

        *lenght += 1;
    }

    pub fn push_back(&self, value: T) {
        let node = Node::new(value);

        let mut head = self.head.lock().unwrap();
        let mut tail = self.tail.lock().unwrap();
        let mut length = self.length.lock().unwrap();

        match tail.take() {
            Some(old_tail) => {
                *node.prev.lock().unwrap() = Some(old_tail.clone());
                *old_tail.next.lock().unwrap() = Some(node.clone());
                *tail = Some(node);
            }
            None => {
                *head = Some(node.clone());
                *tail = Some(node);
            }
        }

        *length += 1;
    }

    pub fn pop_front(&self) -> Option<T> {
        let mut head = self.head.lock().unwrap();
        let mut tail = self.tail.lock().unwrap();
        let mut length = self.length.lock().unwrap();

        head.take()
            .map(|old_head| {
                if let Some(next) = old_head.next.lock().unwrap().take() {
                    next.prev.lock().unwrap().take();
                    *head = Some(next);
                } else {
                    tail.take();
                }

                *length -= 1;
                Arc::try_unwrap(old_head).ok().unwrap().value
            })
            .map(|arc| Arc::try_unwrap(arc).ok().unwrap())
    }

    pub fn pop_back(&self) -> Option<T> {
        let mut head = self.head.lock().unwrap();
        let mut tail = self.tail.lock().unwrap();
        let mut length = self.length.lock().unwrap();

        tail.take()
            .map(|old_tail| {
                if let Some(prev) = old_tail.prev.lock().unwrap().take() {
                    prev.next.lock().unwrap().take();
                    *tail = Some(prev);
                } else {
                    head.take();
                }

                *length -= 1;
                Arc::try_unwrap(old_tail).ok().unwrap().value
            })
            .map(|arc| Arc::try_unwrap(arc).ok().unwrap())
    }

    pub fn len(&self) -> usize {
        let length = self.length.lock().unwrap();
        *length
    }

    pub fn is_empty(&self) -> bool {
        let length = self.length.lock().unwrap();
        *length == 0
    }
}

// Implement Debug for visualization
impl<T: fmt::Debug> fmt::Debug for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut curr = self.head.lock().unwrap().clone();
        let mut result = Vec::new();

        while let Some(node) = curr {
            result.push(node.value.clone());
            curr = node.next.lock().unwrap().clone();
        }

        f.debug_list().entries(result.iter()).finish()
    }
}

fn main() {
    let list = DoublyLinkedList::new();

    list.push_front(1);
    list.push_back(2);
    list.push_front(0);

    println!("{:?}", list);

    list.pop_front();
    list.pop_back();

    println!("{:?}", list);

    // Test multi-threaded operations
    let list = DoublyLinkedList::new();

    let list_clone1 = list.clone();
    let handle1 = std::thread::spawn(move || {
        list_clone1.push_front(1);
        list_clone1.push_front(2);
    });

    let list_clone2 = list.clone();
    let handle2 = std::thread::spawn(move || {
        list_clone2.push_back(3);
        list_clone2.push_back(4);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{:?}", list);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn single_thread_operations() {
        let list = DoublyLinkedList::new();

        list.push_front(1);
        list.push_back(2);
        list.push_front(0);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn multi_thread_operations() {
        let list = DoublyLinkedList::new();

        let list_clone1 = list.clone();
        let handle1 = thread::spawn(move || {
            list_clone1.push_front(1);
            list_clone1.push_front(2);
        });

        let list_clone2 = list.clone();
        let handle2 = thread::spawn(move || {
            list_clone2.push_back(3);
            list_clone2.push_back(4);
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        assert_eq!(list.len(), 4);
    }
}
