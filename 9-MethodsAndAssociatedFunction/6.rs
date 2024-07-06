#[derive(Debug)]
enum TrafficLightColor{
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
    fn color(&self) -> &str {
        match self {
            Self::Yellow => "yellow",
            Self::Red => "red",
            Self::Green => "green",
        }
    }
}
fn main(){
    let c: TrafficLightColor = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}