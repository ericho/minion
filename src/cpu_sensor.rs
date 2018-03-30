extern crate futures;
extern crate tokio_core;
extern crate walkdir;

use sensor::Sensor;

use temp_sensor::Metric;

use walkdir::WalkDir;
use regex::Regex;

pub struct CpuSensor {
    name: String,
}

impl CpuSensor {
    pub fn new() -> CpuSensor {
        CpuSensor { name: "FreqSensor".to_string() }
    }

    fn get_metrics(&self, entry: &walkdir::DirEntry) {
        let p = entry.path().to_path_buf();
        println!("The path : {:?}", p)
    }

    fn is_cpu_entry(&self, entry: &walkdir::DirEntry) -> bool {
        let re = Regex::new("cpu[0-9]+").unwrap();
        let name = entry.file_name().to_str().unwrap();
        if re.is_match(name) { true } else { false }
    }
}

impl Sensor for CpuSensor {
    fn sample(&self) -> Vec<Metric> {
        for entry in WalkDir::new("/sys/bus/cpu/devices")
            .follow_links(true)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok()) {
            if self.is_cpu_entry(&entry) {
                self.get_metrics(&entry);
            }
        }
        format!("Sampling {} sensor", self.name);
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
        let cpu = CpuSensor::new("TestSensor");
        let _metrics = cpu.sample();
    }
}
