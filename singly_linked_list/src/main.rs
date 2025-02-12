mod utils;
use utils::node_utils::List_S;

fn main() {
    let mut head = None;
    List_S::add_node(&mut head, "Node 1".to_string());
    List_S::add_node(&mut head, "Node 2".to_string());
    List_S::add_node(&mut head, "Node 3".to_string());

    List_S::print_node(&head);
    let count: u32 = List_S::list_len(&head);
    println!("{}", count);
}
