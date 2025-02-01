fn str_len(str_: String) -> usize {
    str_.len()
}

fn main()
{
    let name = String::from("Churchil");
    let len_name = str_len(name);
    println!("{len_name}");
}
