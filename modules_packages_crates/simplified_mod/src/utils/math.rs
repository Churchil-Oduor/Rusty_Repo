pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn sub(x: i32, y: i32) -> i32 {
    x - y
}

pub fn div(x: i32, y: i32) -> Result<i32, &'str> {
    if y == 0 {
        Err("Division By Zero Error")
    }
    else {
        Ok(x / y)
    }
}
