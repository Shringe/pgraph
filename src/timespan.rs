pub struct Timespan {
    pub hours: f64,
    pub days: f64,
    pub months: f64,
    pub years: f64,
}

impl Timespan {
    pub fn from_hours(hours: f64) -> Self {
        Self {
            hours,
            days: hours / 24.0,
            months: hours / 720.0,
            years: hours / 8760.0,
        }
    }

    pub fn from_months(months: f64) -> Self {
        Self::from_hours(months * 720.0)
    }
}
