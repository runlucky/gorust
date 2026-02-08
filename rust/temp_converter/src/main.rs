mod temperature;

use temperature::Temperature;

fn main() {
    let temp = Temperature::new(40.0, temperature::TemperatureUnit::Celsius);


    println!("celsius: {}", temp.get_value(temperature::TemperatureUnit::Celsius));
    println!("fahrenheit: {}", temp.get_value(temperature::TemperatureUnit::Fahrenheit));
}
