extern crate futures;
extern crate tokio_core;

use sensor::Sensor;

use temp_sensor::Metric;

pub struct CpuSensor {}

impl CpuSensor {
    pub fn new() -> CpuSensor {
        CpuSensor {}
    }
}

impl Sensor for CpuSensor {
    fn sample(&self) -> Vec<Metric> {
        let mut m = Vec::new();
        m.push(Metric{
            name: "dummy".to_string(),
            value : "fake".to_string()
        });
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_cpu_metrics() {
        let cpu = CpuSensor::new();
        let _metrics = cpu.sample();
    }
}
