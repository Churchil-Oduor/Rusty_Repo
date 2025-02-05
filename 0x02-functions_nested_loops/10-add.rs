//Adds the two digits passed onto it
//Returns the sum of the digits
fn add(x: i32, y: i32) -> i32 {
    x + y
}


fn main()
{
    let x: i32 = -34;
    let y: i32 = 56;

    let sum: i32 = add(x, y);
    println!("{x} + {y} = {sum}");
}
