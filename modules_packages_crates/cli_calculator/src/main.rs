mod utils;

use utils::operations::operation;

fn main() {

    let sum: i32 = operation::add(2, 4);

    println!("{sum}");
}
