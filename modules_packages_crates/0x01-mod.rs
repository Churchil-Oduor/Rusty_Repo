mod math_utils {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn sub(x:i32, y: i32) -> i32 {
        x - y
    }

    pub fn div(x: i32, y: i32) -> Result<i32, &'static str> {
        if y == 0 {
            Err("MathError, Division by Zero!")
        } else {
            Ok(x / y)
        }
    }

    pub fn mult(x: i32, y: i32) -> i32 {
        x * y
    }
}


fn main()
{
    let res: i32 = math_utils::div(10, 5).unwrap();
    println!("{}", res);
}
