fn fibonacci(x: u32)
{
    let mut start: u32 = 1;
    let mut prev: u32 = 0;

    for number in (1..x).rev()
    {
        println!("{start}");
        prev = start;
        start += prev;

    }
}

fn main()
{
    fibonacci(4);
}
