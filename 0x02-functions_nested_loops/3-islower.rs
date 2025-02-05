use std::env;

//checks if alphabet entered is lowercase
fn main()
{
    let ascii_val: u8;
    let _str: &String;
    let _char: char;

    let args:  Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 
    {
        eprintln!("Usage: ./3-islower.exe <arg1>");
        return;
    }

    _str = &args[1];


    if _str.chars().count() == 1
    {
        _char = (*_str).chars().next().unwrap();
        ascii_val = _char as u8;
        
        if ascii_val >= 97 && ascii_val <= 122
        {
            println!("Char {} is lowercase", _char);
        } else
        {
            println!("Char {} is not a lowercase", _char);
        }

    } else
    {
        println!("Usage: ./3-islower.exe <char>");
    }
}


