struct Circle {
    radius: f32,
}

impl Circle
{
    fn area(&self) -> f32 {
        3.142 * self.radius * self.radius
    }
}


fn main()
{
    let radius: f32 = 7.0;
    let circle: Circle = Circle {
        radius
    };

    println!("The area is {}", circle.area());
}
