#[derive(Debug)]
pub struct Node {

    data: String,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: String) -> Node {
        Node {
            data,
            next: None,
        }
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }

    pub fn get_next(&self) -> &Option<Box<Node>> {
        &self.next
    }

    pub fn add_node(head: &mut Option<Box<Node>>, data: String)
    {
        let mut new_head = Box::new(Self::new(data));
        new_head.next = head.take();
        *head = Some(new_head);
    }

    pub fn print_list(head: &Option<Box<Node>>) {

        let mut tray = head.as_ref();
        while let Some(n) = tray {
            println!("{}", n.data);
            tray = n.next.as_ref();
        }
    }

    pub fn count_nodes(head: &Option<Box<Node>>) -> u32 {
        let mut count: u32 = 0;
        let mut tray = head.as_ref();

        while let Some(n) = tray {
            count += 1;
            tray = n.next.as_ref();
        }

        count
    }

    pub fn add_node_end(head: &Option<Box<Node>>, data: String) {
        let mut last_node = head.as_ref();
        let new_node = Box::new(Self::new(data));

        while let Some(mut n) = last_node {

            match n.next.as_ref() {
                None => {
                    n = &new_node;
                    break;
                },
                Some(_) => {}
            }
            last_node = n.next.as_ref();
        }
    }
}
