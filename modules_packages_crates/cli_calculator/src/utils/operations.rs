pub mod operation {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn sub(x: i32, y: i32) -> i32 {
        x - y
    }

    pub fn div(x: i32, y: i32) -> Result <i32, &'static str> {
        if y == 0 {
            Err("MathError")
        }
        else {
            Ok(x / y)
        }
    }

    pub fn mult(x: i32, y: i32) -> i32 {
        x * y
    }
}
