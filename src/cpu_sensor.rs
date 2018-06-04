extern crate futures;
extern crate sysinfo;
extern crate tokio_core;

use sensor::{Metric, ProcessorMetric, Sensor};
use sysinfo::{ProcessorExt, SystemExt};

pub struct CpuSensor {}

impl CpuSensor {
    pub fn new() -> CpuSensor {
        CpuSensor {}
    }
}

impl Sensor for CpuSensor {
    fn sample(&self) -> Vec<Metric> {
        // TODO: Try to make sysinfo a global type or something similar
        // that can be accessed from any place in the program.
        // The refresh_all() method is too expensive to be called on every
        // sampling.
        let mut sys = sysinfo::System::new();
        sys.refresh_system();

        let mut m = Vec::new();
        for processor in sys.get_processor_list() {
            println!("{:?}", processor);
            m.push(Metric::Processor(ProcessorMetric::new(
                processor.get_name().to_string(),
                processor.get_cpu_usage(),
            )));
        }
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
