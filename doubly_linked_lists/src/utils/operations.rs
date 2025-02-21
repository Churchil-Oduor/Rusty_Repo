#[derive(Debug)]
pub struct Node {
    data: String,
    prev: Option<Box<Node>>,
    next: Option<Box<Node>>
}

impl Node{
    pub fn new(data: String) -> Node {
        Node {
            data,
            prev: None,
            next: None,
        }
    }

    pub fn add_node(head: &mut Option<Box<Node>>, data: String) {

        let mut new_node = Box::new(Self::new(data));
        new_node.next = head.take();
        *head = Some(new_node);
    }
}
