//prints 0 - 9 separated by comma
fn main()
{
    let num: u8 = 0;

    for i in num..10
    {
        if i == 9
        {
            print!("{}", i);
        } else {
            print!("{}, ", i);
        }
    }
    print!("{}", '\n');
}
