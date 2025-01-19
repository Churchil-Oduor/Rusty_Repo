fn main()
{
    let principle: f32 = 1200.00;
    const INTEREST: f32 = 0.01;
    let time: f32 = 3.0;

    let simple_interest = principle * INTEREST * time;
    println!("The simple interest is -> {simple_interest}");
}
