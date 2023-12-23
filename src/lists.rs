#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next:Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct MyLinkedList<T> {
    pub head:Link<T>
}

impl<T> MyLinkedList<T> {

    pub fn new() -> MyLinkedList<T> {
        MyLinkedList { head: None }
    }

    pub fn push(&mut self, value:T) {
        let old_head:Option<Box<Node<T>>> =  std::mem::replace(&mut self.head, None);
        //also works with
        //let old_head = self.head.take();

        let new_head:Box<Node<T>> = Box::new(
            Node {
                value,
                next: old_head
            }
        );

        self.head = Some(new_head);
    }

    pub fn put(&mut self, value:T) {

    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head:Option<Box<Node<T>>> = std::mem::replace(&mut self.head, None);

        match old_head {
            Some(n) => {
                self.head = n.next;
                Some(n.value)
            },
            None => None,
        }
        //same as below
        // old_head.map(|n| {
        //     self.head = n.next;
        //     n.value
        // }) 
    }

    pub fn drop(&mut self) {

    }

    pub fn peek(&mut self) -> Option<&T> {
        match &self.head {
            Some(n) => {
                Some(&n.value)
            },
            None => None,
        }
    }

    pub fn empty(&mut self) {

    }

}

