
use std::time::Duration;

use segment::Metric;

#[derive(Metric)]
#[segment(measurement="cpu")]
struct Minimal {
    #[segment(time)]
    timestamp: Duration,
    #[segment(field)]
    value: f32,
}

#[test]
fn minimal_test() {
    let metric = Minimal {
        timestamp: Duration::from_nanos(0),
        value: 42.0,
    };
    let mut s = String::with_capacity(64);
    let _ = metric.build(&mut s);

    assert_eq!(s, "cpu value=42.0 0");
}

#[derive(Metric)]
#[segment(measurement="cpu")]
struct MultiTag {
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
    let metric = MultiTag {
        timestamp: Duration::from_nanos(0),
        host: "localhost".to_string(),
        cpu: "CPU0".to_string(),
        value: 42.0,
    };


    let mut s = String::with_capacity(64);
    let _ = metric.build(&mut s);

    assert_eq!(s, "cpu,cpu=CPU0,host=localhost value=42.0 0");
}


#[derive(Metric)]
#[segment(measurement="cpu")]
struct MultiField {
    #[segment(time)]
    timestamp: Duration,
    #[segment(field)]
    x: f32,
    #[segment(field)]
    y: u32,
}

#[test]
fn multiple_fields() {
    let metric = MultiField {
        timestamp: Duration::from_nanos(0),
        x: std::f32::NAN,
        y: 42,
    };

    let mut s = String::new();
    let _ = metric.build(&mut s);

    // TODO: Don't serialize NaN and Inf floats.
    assert_eq!(s, "cpu y=42i 0");
}

#[derive(Metric)]
#[segment(measurement="cpu")]
struct StringNewline {
    #[segment(time)]
    timestamp: Duration,
    #[segment(field)]
    value: String,
}

#[test]
fn string_newlines() {
    let metric = StringNewline{
        timestamp: Duration::from_nanos(0),
        value: "x\ny".to_string(),
    };

    let mut s = String::new();
    let _ = metric.build(&mut s);

    assert_eq!(s, "cpu value=\"x\\ny\" 0");
}

#[derive(Metric)]
#[segment(measurement="cpu")]
struct TagNewline {
    #[segment(time)]
    timestamp: Duration,
    #[segment(tag)]
    host: String,
    #[segment(field)]
    value: u32,
}

#[test]
fn tag_newlines() {
    let metric = TagNewline{
        timestamp: Duration::from_nanos(0),
        host: "x\ny".to_string(),
        value: 42,
    };

    let mut s = String::new();
    let _ = metric.build(&mut s);

    assert_eq!(s, "cpu,host=x\\ny value=42i 0");
}

#[derive(Metric)]
#[segment(measurement="cpu")]
struct StringField {
    #[segment(time)]
    timestamp: Duration,
    #[segment(field)]
    value: String,
}

#[test]
fn string_fields() {
    let metric = StringField{
        timestamp: Duration::from_nanos(0),
        value: "howdy".to_string(),
    };

    let mut s = String::new();
    let _ = metric.build(&mut s);

    assert_eq!(s, "cpu value=\"howdy\" 0");
}
