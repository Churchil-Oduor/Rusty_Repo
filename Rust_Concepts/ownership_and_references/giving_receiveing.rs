fn main()
{
    let _str = giver();
    println!("I have received a {_str}");

    let _str2 = take_and_give_back(_str);
    println!("Hey I too I have received a {_str2}");

}

fn giver() -> String {
    let s1 = String::from("Gift");
    s1
}


fn take_and_give_back(received_string: String) -> String {
    received_string
}


