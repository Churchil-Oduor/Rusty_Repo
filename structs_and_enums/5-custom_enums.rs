enum TrafficLight {
    Red,
    Yellow,
    Green
}

impl TrafficLight {

    fn action(&self) -> &str {
        match self {
            TrafficLight::Red => "Stop",
            TrafficLight::Yellow => "Get Ready",
            TrafficLight::Green => "Move",
         }
    }

}


fn main() 
{
    let color: TrafficLight = TrafficLight::Green;
    println!("{}", color.action());
}
