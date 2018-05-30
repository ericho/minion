extern crate futures;
extern crate tokio;
extern crate tokio_core;
extern crate sysinfo;
extern crate serde_json;

use sensor::Sensor;
use sysinfo::{SystemExt, ComponentExt};

pub struct TempSensor {}

impl TempSensor {
    pub fn new() -> TempSensor {
        TempSensor{}
    }
}

impl Sensor for TempSensor {
    fn sample(&self) -> Vec<Metric> {
        let mut sys = sysinfo::System::new();
        sys.refresh_all();

        // TODO: Seems possible to get first all the components and then just
        // update each of them on every sample, by this way it shouldn't be
        // necessary to run the whole refresh_all() method.
        let mut samples = Vec::new();
        for i in sys.get_components_list() {
            // TODO: Print data only in debug mode (env_logger?)
            println!("{:?}", i);
            let m = Metric{
                name: i.get_label().to_string(),
                value: i.get_temperature().to_string(),
            };
            samples.push(m);
        }
        samples
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metric {
    pub name: String,
    // TODO: Make the value a generic type.
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_temp_metrics() {
        let temp = TempSensor::new();
        let _samples = temp.sample();
    }
}
