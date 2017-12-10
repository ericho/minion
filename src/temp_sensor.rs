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
use std::path::Path;
use regex::Regex;


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
}

pub fn sample_interval(dur: Duration,
                       handle: &Handle)
                       -> Box<Future<Item = (), Error = io::Error>> {
    let interval = Interval::new(dur, handle).unwrap();
    let int_stream = interval.for_each(|_| {
        let temp = TempSensor::new();
        println!("{}", temp.sample());
        Ok(())
    });

    Box::new(int_stream)
}

impl TempSensor {
    pub fn new() -> TempSensor {
        TempSensor {
            match_temp_file: Regex::new("temp[0-9]+_(input|label)").unwrap(),
            match_hwmon_entry: Regex::new("hwmon[0-9]+").unwrap(),
            metric_name_expander: Regex::new("[a-z]+([0-9]+)_(input|label)").unwrap(),
        }
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

    fn get_metrics_group_name(&self, p: &Path) -> Result<String, io::Error> {
        let name_path = p.join("name");
        let mut file = File::open(name_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content.trim().to_owned())
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
                    println!("id {}", id);
                }
                println!("File {:?} in {}", f, name);
            }
    }
}

impl Sensor for TempSensor {
    fn sample(&self) -> String {
        for entry in WalkDir::new("/sys/class/hwmon")
            .follow_links(true)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| self.is_hwmon_entry(e.path())) {
                self.get_metrics_from_sysfs(entry.path());
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
