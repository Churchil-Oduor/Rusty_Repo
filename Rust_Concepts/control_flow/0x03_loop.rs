fn main()
{
    let mut counter = 10;

    loop
    {
        counter -= 1;

        if counter == 0 
        {
            break;
        }
        println!("Countdown -> {counter}")
    }
}
