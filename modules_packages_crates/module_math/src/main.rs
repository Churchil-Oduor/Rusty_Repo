mod math_utils;

fn main() {
    let result = math_utils::add(3, 5);
    println!("3 + 5 = {}", result);

    let sub_res = math_utils::sub(30, 2);
    println!("30 - 2 = {}", sub_res);
}
