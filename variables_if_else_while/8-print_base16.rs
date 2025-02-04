//prints 0123456789abcdef followed by a new line
fn main()
{
    let mut lower_limit: u8 = 48;
    let mut upper_limit: u8 = 57;

    while lower_limit <= upper_limit
    {

        print!("{}", lower_limit as char);

        if lower_limit == upper_limit {
            lower_limit = 96;
            upper_limit = 122;
        }
        else if upper_limit == 122 {
            break;
        }
        lower_limit += 1;
    }
}
