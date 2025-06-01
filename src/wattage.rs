#[derive(PartialEq)]
pub struct Wattage {
    pub watts: f64,
    pub kilowatts: f64,
}

impl Wattage {
    pub fn new(watts: f64) -> Self {
        Self {
            watts,
            kilowatts: watts / 1000.0,
        }
    }
}
