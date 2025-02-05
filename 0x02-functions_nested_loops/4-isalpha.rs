use std::env;

//checks if the entered value is a valid alphabet character A-Z, a-z
fn main()
{
    let ascii_value: u8;
    let _str: &String;
    let _char: char;
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2
    {
        eprintln!("Usage: ./4-isalpha.exe <arg1>");
        return;
    }
     _str = &args[1];

     if _str.chars().count() == 1
     {
         _char = (*_str).chars().next().unwrap();
         ascii_value = _char as u8;

         if ascii_value >= 65 && ascii_value <= 90
             || ascii_value >= 97 && ascii_value <= 122{ 
                 
                 println!("char {} is exists within AZaz", _char);
         }
         else {
             println!("char {} does not exist within AZaz!!", _char);
         }

     } else 
     {
         println!("Usage: ./4-isalpha.exe <arg1>")
     }

}
