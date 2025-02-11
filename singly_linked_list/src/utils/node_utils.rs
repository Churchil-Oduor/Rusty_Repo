#[derive(Debug)]
pub struct List_S {
    _str: String,
    len: usize,
    next: Option<Box<List_S>>,
}



impl List_S {

    pub fn new(_str: String, next: Option<Box<List_S>>) -> List_S {
        let len = _str.len();
        List_S {
            _str,
            len,
            next,
        }
    }

    pub fn add_node(head_node: &mut Option<Box<List_S>>, _str: String)
    {
        let mut new_node = Box::new(Self::new(_str, head_node.take()));
        *head_node = Some(new_node);
    }

    pub fn print_node(head_node: &Option<Box<List_S>>)
    {
        let mut node = head_node.as_ref();

        while let Some(n) = node {
            println!("{}", n._str);
            node = n.next.as_ref(); //moves to the next node safely
        }
    }
}
