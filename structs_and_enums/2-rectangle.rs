#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

impl Rectangle
{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    
    fn create_rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}

fn main()
{
    let rect_1: Rectangle = Rectangle::create_rectangle(23, 45);
    let area_1: u32 = rect_1.area();

    println!("Rect_1 - {rect_1:?}\nArea - {area_1}\n");

    let rect_2: Rectangle = Rectangle::create_rectangle(24, 45);
    let area_2: u32 = rect_2.area();

    println!("Rect_2 - {rect_2:?}\nArea - {area_2}");

    let can_hold = rect_1.can_hold(&rect_2);

    if can_hold
    {
        println!("Rect_1 cn hold Rect_2");
    } else {
        println!("Rect_1 cannot hold Rect_2");
    }
}
