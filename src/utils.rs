struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: self.head });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => Some(node.data),
            None => None,
        }
    }
}
