use std::env;




fn main()
{
    let args: Vec<String> = env::args().collect();
}
//checks the sign of entered number
//if number > 0, returns 1, i number == 0, returns 0 else -1
fn print_sign(n: u32) -> i32 {
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

