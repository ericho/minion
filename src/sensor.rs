extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;

pub trait Sensor {
    fn sample(&self) -> String;
}

#[test]
fn smoke() {
    // Basically if this tests compiles and run we are good.
    let mut t = TempSensor::new("TestSensor");
    let new_rate = Duration::from_millis(1000);
}
