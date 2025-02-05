// this function prints the alphabet in lowercase 10 times
fn main()
{
    for _i in 1..11 {
        for x in 97..123
        {
            print!("{}", x as u8 as char);
        }

        print!("{}", '\n');
    }
}
