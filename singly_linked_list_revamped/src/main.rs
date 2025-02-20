mod utils;

use utils::singly_linked_list::Node;

fn main() {

    let mut node = None;
   // let mut node = Some(Box::new(Node::new("Churchil".to_string())));    
    Node::add_node(&mut node, "Hope".to_string());
    Node::add_node(&mut node, "Brandy".to_string());
    Node::print_list(&mut node);
    let count = Node::count_nodes(&node);
    println!("{}", count);
    Node::add_node_end(&mut node, "Okech".to_string());
    Node::add_node(&mut node, "Victor".to_string());
    Node::print_list(&mut node);
    Node::print_list(&mut node);
    Node::remove_node_in(&mut node, -1);
    Node::print_list(&mut node);
}
