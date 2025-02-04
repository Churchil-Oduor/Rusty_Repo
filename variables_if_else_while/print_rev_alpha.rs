//prints the alphabet in reverse
fn main()
{
    for i in (97..123).rev()
    {
        print!("{}", i as u8 as char);
    }
    print!("{}", '\n');
}

