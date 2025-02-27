pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
            Rectangle {
                width,
                height
            }
    }

    pub fn area(&self) -> u32 {
            self.width * self.height
    }
}
