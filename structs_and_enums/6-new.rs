#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow
}

impl TrafficLight {
    fn new() -> Self {
        TrafficLight::Yellow
    }
}

fn main() {
    let color = TrafficLight::new();
    println!("{:?}", color);

}
