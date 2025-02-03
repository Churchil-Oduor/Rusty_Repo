struct Rectangle {
    height: u32,
    width: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

fn main()
{
    let rect: Rectangle = Rectangle {
        height: 23,
        width: 89,
    };

    println!("Area is {}" , area(&rect));
}
