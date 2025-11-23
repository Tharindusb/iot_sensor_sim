use chrono::prelude::*;
use rand::Rng;

trait Sensor {
    fn generate_reading(&mut self) -> f64;
    fn get_id(&self) -> i32;
}

#[derive(Debug, PartialEq)]
enum State {
    Active,
    Inactive,
}

#[derive(Debug)]
enum Units {
    Celsius,
    Fahrenheit,
}

struct TemperatureSensor {
    id: i32,
    state: State,
    units: Units,
    reading: f64,
}

impl Sensor for TemperatureSensor {
    fn generate_reading(&mut self) -> f64 {
        let base_reading = self.reading;
        let mut rng = rand::rng();
        let fluctuation = rng.random_range(-2.0..=2.0);
        self.reading = base_reading + fluctuation;
        self.reading
    }
    fn get_id(&self) -> i32 {
        self.id
    }
}

fn main() {
    let mut sensor = TemperatureSensor {
        id: 1,
        state: State::Active,
        units: Units::Celsius,
        reading: 25.0,
    };

    println!("Sensor state: {:?}", sensor.state);

    while sensor.state == State::Active {
        let now = Local::now();
        println!(
            "{} - Sensor Reading: {:.4} {:#?}",
            now.format("%Y-%m-%d %H:%M:%S"),
            sensor.generate_reading(),
            sensor.units
        );
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
