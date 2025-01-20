fn main()
{
    let x: u32 = 20;
    let y = plus_two(x);
    println!("The summation of 20 by two is {y}");
    println!("{x}");
}

fn plus_two(x: u32) -> u32 {
    x + 2
}

