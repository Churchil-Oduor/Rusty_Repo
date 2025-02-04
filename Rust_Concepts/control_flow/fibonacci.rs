fn fibonacci(range: u32)
{
    let mut present_fib: u32 = 1;
    let mut _previous_fib: u32 = 0;
    let mut count: u32 = 0;

    while count <= (range / 2) {

        print!("{present_fib}, ");
        _previous_fib += present_fib;
        present_fib += _previous_fib;
        print!("{_previous_fib}, ");
        count += 1;
    }
}

fn main()
{
    let range: u32 = 4;

    fibonacci(range);
}
