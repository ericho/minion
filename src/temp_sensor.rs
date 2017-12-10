extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;
extern crate walkdir;

use std::time::Duration;
use sensor::Sensor;
use futures::Future;
use futures::stream::Stream;
use tokio_core::reactor::{Handle, Interval};
use std::io;
use std::io::{Read, Error, ErrorKind};
use std::fs::File;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use regex::Regex;
use std::collections::HashMap;

// Get the list of metrics available:
//   - name of the sensor : coretemp, acpitz, fan, etc.
//   - name of the metric: core0, core1, etc.
//     - if it's not available, create metric from <sensor>_<id>
//   - The path of the metric: /sys/class/hwmon/hwmon1/temp1_input
// On every sample:
//   - For each sensor:
//     - Get value from defined path. (need to open files on every read?)



#[derive(Debug, PartialEq)]
enum MetricKind {
    Input,
    Label,
    Unknown,
}

pub struct TempSensor {
    match_temp_file: Regex,
    match_hwmon_entry: Regex,
    metric_name_expander: Regex,
    metrics_path: HashMap<String, Vec<HashMap<String, PathBuf>>>,
}

pub fn sample_interval(dur: Duration,
                       handle: &Handle)
                       -> Box<Future<Item = (), Error = io::Error>> {
    let interval = Interval::new(dur, handle).unwrap();
    let temp = TempSensor::new();
    let int_stream = interval.for_each(move |_| {
        println!("{}", temp.sample());
        Ok(())
    });

    Box::new(int_stream)
}

impl TempSensor {
    pub fn new() -> TempSensor {
        let mut sensor = TempSensor {
            match_temp_file: Regex::new("temp[0-9]+_input").unwrap(),
            match_hwmon_entry: Regex::new("hwmon[0-9]+").unwrap(),
            metric_name_expander: Regex::new("[a-z]+([0-9]+)_(input|label)").unwrap(),
            metrics_path: HashMap::new(),
        };
        sensor.init_sensor();
        sensor
    }

    fn init_sensor(&mut self) {
        let mut metrics_path = HashMap::new();
        for entry in WalkDir::new("/sys/class/hwmon")
            .follow_links(true)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| self.is_hwmon_entry(e.path())) {
                let name = self.get_metrics_group_name(entry.path())
                    .unwrap_or(String::from("noname"));
                let mut v = Vec::new();
                for f in WalkDir::new(entry.path())
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| self.is_temp_file(e.path())) {
                        if let Ok((id, kind)) = self.expand_metric_filename(f.file_name().to_str().unwrap()) {
                            let label = self.get_metric_label(f.path()).unwrap_or(format!("{}_{}", name, id));
                            let mut m = HashMap::new();
                            m.insert(label, f.path().to_path_buf());
                            v.push(m);
                        }
                    }
                metrics_path.insert(name, v);
            }
        self.metrics_path = metrics_path;
    }

    fn get_metric_label(&self, p: &Path) -> Result<String, io::Error> {
        let path_str = p.to_str().unwrap().replace("_input", "_label");
        let label = self.read_file_content(PathBuf::from(path_str))?;
        Ok(label)
    }

    fn is_hwmon_entry(&self, p: &Path) -> bool {
        // Convert OsStr to str :/
        let s = p.file_name().unwrap().to_str().unwrap();
        self.match_hwmon_entry.is_match(s)
    }

    fn is_temp_file(&self, p: &Path) -> bool {
        let s = p.file_name().unwrap().to_str().unwrap();
        self.match_temp_file.is_match(s)
    }

    fn read_file_content(&self, p: PathBuf) -> Result<String, io::Error> {
        let mut file = File::open(p)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content.trim().to_owned())
    }

    fn get_metrics_group_name(&self, p: &Path) -> Result<String, io::Error> {
        let name_path = p.join("name");
        let name = self.read_file_content(name_path)?;
        Ok(name)
    }

    fn expand_metric_filename(&self, name: &str) -> Result<(u32, MetricKind), io::Error> {
        let id: u32;
        let m: MetricKind;
        if let Some(res) = self.metric_name_expander.captures(name) {
            if res.len() == 3 {
                // Getting cpu number and convert it to u32
                let got_id = res.get(1).map_or("", |m| m.as_str());
                id = got_id.parse::<u32>().unwrap_or(0);

                match res.get(2).map_or("", |m| m.as_str()) {
                    "input" => m = MetricKind::Input,
                    "label" => m = MetricKind::Label,
                    _ => m = MetricKind::Unknown,
                }
                return Ok((id, m));
            } else {
                return Err(Error::new(ErrorKind::InvalidInput, "Invalid metric name"));
            }
        }
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid metric name"));
    }

    fn get_metrics_from_sysfs(&self, p: &Path) {
        let name = self.get_metrics_group_name(p).unwrap_or(String::from("noname"));
        for f in WalkDir::new(p)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| self.is_temp_file(e.path())) {
                if let Ok((id, kind)) = self.expand_metric_filename(f.file_name().to_str().unwrap()) {
                    let content = self.read_file_content(f.path().to_path_buf()).unwrap();
                    println!("Sensor {} with id {} has {}", name, id, content);
                }
            }
    }
}

impl Sensor for TempSensor {
    fn sample(&self) -> String {
        for (name, metric_path) in &self.metrics_path {
            for metric in metric_path {
                for (label, path) in metric {
                    let content = self.read_file_content(path.clone()).unwrap();
                    println!("{} {:?} {}", name, label, content);
                }
            }

        }
        format!("Sampling temp sensor")
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

    #[test]
    fn check_if_folder_is_hwmon() {
        let temp = TempSensor::new();
        let p = Path::new("/sys/class/hwmon");
        assert_eq!(temp.is_hwmon_entry(&p), false);
        let p = Path::new("/some/bad/path");
        assert_eq!(temp.is_hwmon_entry(&p), false);
        let p = Path::new("/sys/class/hwmon0");
        assert_eq!(temp.is_hwmon_entry(&p), true);
        let p = Path::new("/sys/class/hwmon10000");
        assert_eq!(temp.is_hwmon_entry(&p), true);
        let p = Path::new("/sys/class/hwmon1/some/other");
        assert_eq!(temp.is_hwmon_entry(&p), false);
    }

    #[test]
    fn check_if_file_is_temp_file() {
        let temp = TempSensor::new();
        let p = Path::new("/sys/class/hwmon/hwmon0/temp1_input");
        assert_eq!(temp.is_temp_file(p), true);
        let p = Path::new("/sys/class/hwmon/hwmon0/temp1_label");
        assert_eq!(temp.is_temp_file(p), true);
        let p = Path::new("/sys/class/hwmon/hwmon0/tempA_input");
        assert_eq!(temp.is_temp_file(p), false);
    }

    #[test]
    fn check_expand_filenames() {
        let temp = TempSensor::new();
        match temp.expand_metric_filename("temp1_input") {
            Ok((id, kind)) => {
                assert_eq!(id, 1);
                assert_eq!(kind, MetricKind::Input);
            },
            Err(_) => assert!(false),
        }
        match temp.expand_metric_filename("temp2_label") {
            Ok((id, kind)) => {
                assert_eq!(id, 2);
                assert_eq!(kind, MetricKind::Label);
            },
            Err(_) => assert!(false),
        }
        match temp.expand_metric_filename("temp3_some") {
            Ok((id, kind)) => {
                assert_eq!(id, 3);
                assert_eq!(kind, MetricKind::Unknown);
            },
            Err(_) => assert!(true),
        }
    }
}
