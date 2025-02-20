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
            print!("[{}]--->", n.data);
            tray = n.next.as_ref();
        }
        print!("Null");
        println!("{}", '\n');
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

    pub fn add_node_end(head: &mut Option<Box<Node>>, data: String) {
        
        let new_node = Some(Box::new(Self::new(data)));
        let mut node = head.as_mut();

        while let Some(n) = node {
            if n.next.is_none() {
                n.next = new_node;
                return;
            }
            node = n.next.as_mut();
        }
        *head = new_node;
    }

    pub fn remove_node_in(head: &mut Option<Box<Node>>, position: i32) {

        let num_nodes: u32 = Self::count_nodes(head);
        let mut current = head.as_mut();
        let mut count: u32 = 0;
        let mut help_pos = position;

        if position >= num_nodes as i32 {
            println!("Error! Out of Bounds Error");
            return;
        } else if position < 0 {
            help_pos = position + num_nodes as i32;
        }

        if help_pos == 0 {
            *head = head.take().and_then(|node| node.next);
            return;
        }

        while let Some(n) = current {

            if (count + 1) as i32 == help_pos {
                n.next = n.next.take().and_then(|next_node| next_node.next);
                return;
            }

            count += 1;
            current = n.next.as_mut();
        }
    }
}





