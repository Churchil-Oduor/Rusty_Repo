mod utils;

use utils::operations::Node;

fn main() {

    let mut node = Some(Box::new(Node::new("Churchil".to_string())));    
    Node::add_node(&mut node, "Hope".to_string());
    Node::add_node(&mut node, "Brandy".to_string());
    Node::print_list(&mut node);
    Node::delete_node_at(&mut node, -2);
    Node::print_list(&mut node);
/*
    let count = Node::count_nodes(&node);
    println!("{}", count);
    Node::add_node_end(&mut node, "Okech".to_string());
    Node::add_node(&mut node, "Victor".to_string());
    Node::print_list(&mut node);
    Node::print_list(&mut node);
    //Node::remove_node_in(&mut node, -4);
    let mut new_node = Some(Box::new(Node::new("Lingala".to_string())));
    Node::insert_into_pos(&mut node, new_node, 4);
    Node::delete_list(&mut node);
    Node::print_list(&mut node);*/
}
