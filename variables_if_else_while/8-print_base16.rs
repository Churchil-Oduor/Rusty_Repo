//prints 0123456789abcdef followed by a new line
fn main()
{
    let mut lower_limit: u8 = 48;
    let mut upper_limit: u8 = 57;

    while lower_limit <= upper_limit
    {
        print!("{}", lower_limit as char);
        lower_limit += 1;
    }

    lower_limit = 97;
    upper_limit = 102;

    while lower_limit <= upper_limit
    {
        print!("{}", lower_limit as char);
        lower_limit += 1;
    }
    print!("{}", '\n');
}
