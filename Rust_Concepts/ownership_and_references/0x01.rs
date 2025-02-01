fn take_ownership(some_string: String)
{
    println!("String passed is {some_string}")
}

fn main()
{
    let _str: String = String::from("Rustaceann"); //_str is the owner and it exists within the
                                                   //scope of main function
                                                   


    take_ownership(_str); //_str looses ownership and the function ->
                          //take_ownership is the new ownw
    println!("{_str}");
}
