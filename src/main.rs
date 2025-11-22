
trait Sensor {
    fn generate_reading(&mut self) -> String;
    fn get_id(&self) -> i32;
}

#[derive(Debug)]
enum State {
    Active,
    Inactive,
}

#[derive(Debug)]
enum Parameters {
    Celsius,
    Fahrenheit,
}

struct TemperatureSensor {
    id: i32,
    state: State,
    parameters: Parameters,
    reading: f64,
}

impl Sensor for TemperatureSensor {
    fn generate_reading(&mut self) -> String {
        let base_reading = self.reading;
        let fluctuation = vec![0.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.5, 3.5, 1.0, 1.0, 1.1, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let reading = base_reading + fluctuation[10];
        reading.to_string()
    }
    fn get_id(&self) -> i32 {
        self.id
    }
}

fn main() {
    let mut sensor = TemperatureSensor {
        id: 1,
        state: State::Active,
        parameters: Parameters::Celsius,
        reading: 25.0,
    };
    
    println!("Sensor state: {:?}", sensor.state);
    println!("Sensor Reading: {} {:#?}", sensor.generate_reading(), sensor.parameters);
}
