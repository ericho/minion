extern crate futures;
extern crate serde_json;
extern crate sysinfo;
extern crate tokio;
extern crate tokio_core;

use sensor::{Metric, Sensor, TempMetric};
use sysinfo::{ComponentExt, SystemExt};

pub struct TempSensor {}

impl TempSensor {
    pub fn new() -> TempSensor {
        TempSensor {}
    }
}

impl Sensor for TempSensor {
    fn sample(&self) -> Vec<Metric> {
        let mut sys = sysinfo::System::new();
        sys.refresh_system();

        // TODO: Seems possible to get first all the components and then just
        // update each of them on every sample, by this way it shouldn't be
        // necessary to run the whole refresh_all() method.
        let mut samples = Vec::new();
        for i in sys.get_components_list() {
            // TODO: Print data only in debug mode (env_logger?)
            println!("{:?}", i);
            let m = Metric::Temperature(TempMetric::new(
                i.get_label().to_string(),
                i.get_max(),
                i.get_critical(),
                i.get_temperature(),
            ));
            samples.push(m);
        }
        samples
    }
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
