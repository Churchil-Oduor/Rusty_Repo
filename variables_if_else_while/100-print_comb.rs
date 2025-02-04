// prints double digit numbers 01, 02 ... 89
fn main()
{

    for fdigit in 0..9
    {
        for ldigit in 0..10
        {
            print!("{}{} ", fdigit, ldigit);
        }
    }

    print!("{}", '\n');
    
}
