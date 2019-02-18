
use std::time::{SystemTime, Duration};

use segment::{Builder, Metric};

// Define a metric..
#[derive(Metric)]
#[segment(measurement="cpuinfo")]
pub struct CpuInfo {
    #[segment(time)]
    timestamp: Duration,

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
}

fn main() {
    let m = CpuInfo{
        timestamp: match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d,
            Err(_) => panic!("Unable to get current time, as duration"),
        },
        host: "myhost".to_string(),
        cpuid: 0,
        sys_time: 124,
        usr_time: 256,
        idl_time: 0,
        system_load: 0.75,
    };

    println!("Time: {:?}", m.time());

    println!("Tags:");
    for t in m.tags() {
        println!("   - \"{}\" = \"{}\"", t.name, t.value);
    }

    println!("Fields:");
    for f in m.fields() {
        println!("   - \"{}\" = {}", f.name, f.value);
    }

    println!("Line Proto: '{}'", m.to_lineproto());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_lineproto(b: &mut Bencher) {
        let m = CpuInfo {
            timestamp: match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(d) => d,
                Err(_) => panic!("Unable to get current time, as duration"),
            },
            host: "myhost".to_string(),
            cpuid: 0,
            sys_time: 124,
            usr_time: 256,
            idl_time: 0,
        };
        b.iter(move || {
            let line = m.to_lineproto();
            if line.len() == 0 {
                println!("Invalid line protocol");
            }
        });
    }
}
