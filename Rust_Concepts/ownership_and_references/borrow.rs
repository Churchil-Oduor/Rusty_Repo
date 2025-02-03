fn main(){

    let s1 = String::from("Churchil");
    println!("{}", str_len(&s1));
}

fn str_len(_str: &String) -> usize {
    _str.len()
}

