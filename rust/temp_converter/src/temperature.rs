

pub struct Temperature {
    value: f64,
    unit: TemperatureUnit,
}

impl Temperature {
    pub fn new(value: f64, unit: TemperatureUnit) -> Self {
        Temperature { value, unit }
    }

    pub fn get_value(&self, unit: TemperatureUnit) -> f64 {
        use TemperatureUnit::*;

        match (self.unit, unit) {
            (Celsius, Celsius) => self.value,
            (Fahrenheit, Fahrenheit) => self.value,
            (Celsius, Fahrenheit) => self.value * 1.8 + 32.0,
            (Fahrenheit, Celsius) => (self.value - 32.0) / 1.8,
        }
    }
}

#[derive(Copy, Clone)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}
