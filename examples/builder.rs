
use std::time::{SystemTime, Duration};

use segment::Metric;

// Define a metric..
#[derive(Metric)]
#[segment(measurement="cpuinfo")]
pub struct CpuInfo {
    #[segment(time)]
    timestamp: Duration,

    #[segment(tag)]
    descr: & 'static str,

    #[segment(tag, rename="Host")]
    host: String,

    #[segment(tag)]
    cpuid: u32,


    #[segment(field, rename="system")]
    sys_time: u64,
    #[segment(field, rename="user")]
    usr_time: u64,
    #[segment(field, rename="idle")]
    idl_time: u64,
    #[segment(field, rename="load")]
    system_load: f32,
    #[segment(field)]
    some_str: & 'static str,
}

fn main() {
    let m = CpuInfo{
        timestamp: match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d,
            Err(_) => panic!("Unable to get current time, as duration"),
        },
        host: "myhost".to_string(),
        descr: "a wonderful, tag",
        cpuid: 0,
        sys_time: 124,
        usr_time: 256,
        idl_time: 0,
        system_load: 0.75,
        some_str: "\"foo,bar\"",
    };

    println!("Time: {:?}", m.time());

    println!("Tags:");
    for t in m.tags() {
        println!("   - \"{}\" = \"{}\"", t.name, t.value);
    }

    println!("Fields:");
    for f in m.fields() {
        println!("   - \"{}\" = {}", f.name, f.value.to_string());
    }

    let mut s = String::with_capacity(64);
    m.build(&mut s);

    println!("Line Proto: '{}'", s);
}
