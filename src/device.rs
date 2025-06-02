use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::{timespan::Timespan, wattage::Wattage};

#[derive(Serialize, Deserialize)]
pub struct Rgb(pub u8, pub u8, pub u8);

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub initial_cost: f64,
    pub average_wattage: Wattage,

    /// kWh/$
    pub electricity_rate: f64,

    pub color: Rgb,
    pub name: String,
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.initial_cost == other.initial_cost
            && self.average_wattage == other.average_wattage
            && self.electricity_rate == other.electricity_rate
        // intentionally ignore color and name
    }
}

impl Device {
    pub fn total_cost(&self, time: &Timespan) -> f64 {
        self.cost(time) + self.initial_cost
    }

    /// Running costs without in initial_cost
    pub fn cost(&self, time: &Timespan) -> f64 {
        (self.average_wattage.kilowatts * time.hours) / self.electricity_rate
    }

    /// Used because Color doesn't support serialization
    pub fn get_color(&self) -> Color {
        Color::Rgb(self.color.0, self.color.1, self.color.2)
    }
}
