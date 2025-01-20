fn main()
{
    let p: f32 = 10000.0;
    const R: f32 = 0.4;
    let t: f32 = 3.0;

    let simple_interest: f32 = {
        p * R * t
    };

    println!("Simple Interest -> {simple_interest}");
}
