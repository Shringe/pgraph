use crate::{timespan::Timespan, wattage::Wattage};

#[derive(PartialEq)]
pub struct Device {
    pub initial_cost: f64,
    pub average_wattage: Wattage,

    /// kWh/$
    pub electricity_rate: f64,
}

impl Device {
    pub fn total_cost(&self, time: &Timespan) -> f64 {
        self.cost(time) + self.initial_cost
    }

    /// Running costs without in initial_cost
    pub fn cost(&self, time: &Timespan) -> f64 {
        (self.average_wattage.kilowatts * time.hours) / self.electricity_rate
    }
}
