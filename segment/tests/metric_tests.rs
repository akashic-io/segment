
use std::time::Duration;

use segment::Metric;

#[derive(Metric)]
#[segment(measurement="cpu")]
struct minimal {
    #[segment(time)]
    timestamp: Duration,
    #[segment(field)]
    value: f32,
}

#[test]
fn minimal_test() {
    let metric = minimal{
        timestamp: Duration::from_nanos(0),
        value: 42.0,
    };
    let mut s = String::with_capacity(64);
    metric.build(&mut s);

    assert_eq!(s, "cpu value=42.0 0");
}

#[derive(Metric)]
#[segment(measurement="cpu")]
struct multi_tag {
    #[segment(time)]
    timestamp: Duration,

    #[segment(tag)]
    host: String,

    #[segment(tag)]
    cpu: String,

    #[segment(field)]
    value: f32,
}

#[test]
fn multiple_tags() {
    let metric = multi_tag{
        timestamp: Duration::from_nanos(0),
        host: "localhost".to_string(),
        cpu: "CPU0".to_string(),
        value: 42.0,
    };


    let mut s = String::with_capacity(64);
    metric.build(&mut s);

    assert_eq!(s, "cpu,cpu=CPU0,host=localhost value=42.0 0");
}


#[derive(Metric)]
#[segment(measurement="cpu")]
struct multi_field {
    #[segment(time)]
    timestamp: Duration,
    #[segment(field)]
    x: f32,
    #[segment(field)]
    y: u32,
}

#[test]
fn multiple_fields() {
    let metric = multi_field {
        timestamp: Duration::from_nanos(0),
        x: std::f32::NAN,
        y: 42,
    };

    let mut s = String::new();
    metric.build(&mut s);

    assert_eq!(s, "cpu y=42i 0");
}
