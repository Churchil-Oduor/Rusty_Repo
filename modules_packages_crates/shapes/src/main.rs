mod shapes;
use crate::shapes::Rectangle;

fn main() {
    let h: u32 = 10;
    let w: u32 = 3;

    let rectangle: Rectangle = Rectangle::new(h, w);
    let area: u32 = rectangle.area();
    
    println!("{area}");
}
