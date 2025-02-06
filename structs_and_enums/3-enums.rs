//checks if a number is valid
//describes the number as being null or Type T
fn describe_option(value: Option<i32>) {
    match value {
        Some(value) => println!("Number is {}", value),
        None => println!("Number is Null")
    }
}
fn main()
{
    let a = Some(19);
    let b: Option<i32> = None;
    describe_option(a);
    describe_option(b);
}
