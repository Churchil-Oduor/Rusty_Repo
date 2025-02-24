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

    pub fn print_list(head: &mut Option<Box<Node>>) {

        let mut current = head.as_mut();

        while let Some(n) = current {
            print!(" {} ->", n.data);
            current = n.next.as_mut();
        }

        println!(" NULL");
    }

    pub fn delete_node_at(head: &mut Option<Box<Node>>, position: i32) {
        let num_nodes = Self::count_nodes(head);
        let mut current = head.as_mut();
        let mut count = 0;
        let mut help_pos = position;


        if help_pos < 0 {
            help_pos += num_nodes as i32;
        }

        if help_pos == 0 {
            if let Some(n) = current {
                *head = n.next.take();
                return;
            }
        }

        while let Some(n) = current {
            if count + 1 == help_pos {
                n.next = n.next.take().and_then(|node| node.next);
                return;
            }
            count += 1;
            current = n.next.as_mut();
        }
    }

    pub fn count_nodes(head:&mut Option<Box<Node>>) -> u32 {
        let mut current = head.as_mut();
        let mut count = 0;

        while let Some(n) = current {
            count += 1;
            current = n.next.as_mut();
        }

        count

    }
}
