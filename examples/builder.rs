
use std::time::SystemTime;

use segment::{Builder, Metric};

// Define a metric..
#[derive(Metric)]
#[segment(measurement="cpuinfo", foo="bar")]
pub struct CpuInfo {
    #[segment(time)]
    timestamp: SystemTime,

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
}

fn main() {
    let m = CpuInfo{
        timestamp: SystemTime::now(),
        host: "myhost".to_string(),
        cpuid: 0,
        sys_time: 124,
        usr_time: 256,
        idl_time: 0,
    };

    println!("Tags:");
    for t in m.tags() {
        println!("   - \"{}\" = \"{}\"", t.name, t.value);
    }

    println!("Fields:");
    for f in m.fields() {
        println!("   - \"{}\" = ", f.name);
    }

    println!("Line Proto: '{}'", m.to_lineproto());
}
