
// prints lowercase and uppercase letters
fn main()
{
    let first_char: u8 = 97;
    let mut _char = 97;

    while _char < 123
    {
        print!("{}", _char as u8 as char);

        if _char == 122
        {
            _char = 64;
        }
        else if _char == 90
        {
            break;
        }
        _char += 1;
    }
}
