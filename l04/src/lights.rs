#[allow(dead_code)]

pub enum TrafficLight {
    Green,
    Yellow,
    Red
}

const LONGER_DURATION: u8 = 60;
const LONG_DURATION: u8 = 30;
const SHORT_DURATION: u8 = 5;

use super::LightDuration;
impl LightDuration for TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Green => LONG_DURATION,
            TrafficLight::Yellow => SHORT_DURATION,
            TrafficLight::Red => LONGER_DURATION
        }
    }
}

#[cfg(test)]
#[test]
fn lights_test() {
    let green = TrafficLight::Green;
    assert_eq!(green.duration(), LONG_DURATION);

    let yellow = TrafficLight::Yellow;
    assert_eq!(yellow.duration(), SHORT_DURATION);

    let red = TrafficLight::Red;
    assert_eq!(red.duration(), LONGER_DURATION);
}
