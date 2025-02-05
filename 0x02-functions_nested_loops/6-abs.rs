//computes the absolute value of an integer.
//returns the absolute value of number entered
fn _abs(i: i32) -> i32 {
    if i < 0
    {
        -i
    } else 
    {
        i
    }
}

fn main()
{
    let num: i32;

    num = _abs(23);
    println!("{num}");
    
}
