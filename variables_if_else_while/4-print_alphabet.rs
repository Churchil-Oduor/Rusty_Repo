// prints alphabet in lowercase except q and e
fn main()
{

    for i in 97..123
    {
        if i != 101 && i != 113
        {
            print!("{}", i as u8 as char);
        }
    }
    print!("{}", '\n');
}
