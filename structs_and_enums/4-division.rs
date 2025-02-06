fn divide(a: i32, b: i32) -> Result<i32, &'static str>
{
    if b == 0 {
        Err("Cannot divide a number with zero")
    } else {
        Ok(a / b)
    }
}

fn main() {

    let a = 10;
    let b = 2;
    match divide(a, b) {
        Ok(result) => println!("{} / {} = {}", a, b, result),
        Err(error) => println!("DivisionByZeroError!")
    }

    match divide(10, 0) {
        Ok(result) => println!("{} / {} = {}", a, b, result),
        Err(error) => println!("DivisionByZeroError!")
    }

}
