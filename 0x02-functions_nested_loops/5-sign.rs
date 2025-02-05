use std::env;




fn main()
{
    let _str: &String;
    let args: Vec<String> = env::args().collect();
    let digit: i32;
    let res: i8;

    
    if args.len() != 2 {
        eprintln!("Usage: ./5-sign.exe <u64>");
        return;
    }


    digit =  args[1].parse();
    res = print_sign(digit);

    if res > 0
    {
        println!("{digit} is positive");
    } else if res < 0
    {
        println!("{digit} is negative");
    } else
    {
        println!("{digit} is zero");
    }

}
//checks the sign of entered number
//if number > 0, returns 1, i number == 0, returns 0 else -1
fn print_sign(n: i32) -> i8 {
    if n > 0
    {
        1
    } else if n < 0
    {
        -1
    } else 
    {
        0
    }
}

