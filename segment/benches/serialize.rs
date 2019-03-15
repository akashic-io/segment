use std::time::{SystemTime, Duration, UNIX_EPOCH};

use segment::Metric;

#[macro_use]
extern crate criterion;

use criterion::{Criterion, BatchSize};

#[derive(Metric)]
#[segment(measurement="stringtest")]
pub struct StringTest {
    #[segment(time)]
    timestamp: Duration,

    #[segment(tag)]
    tag0: String,
    #[segment(tag)]
    tag1: String,

    #[segment(field)]
    field0: String,
    #[segment(field)]
    field1: String,
}

// Define a metric..
#[derive(Metric)]
#[segment(measurement="procstat")]
pub struct ProcStats {
    #[segment(time)]
    timestamp: Duration,

    #[segment(tag, rename="exe")]
    executable: String,

    #[segment(tag, rename="process_name")]
    process: String,

    #[segment(field)]
    cpu_time: u32,
    #[segment(field)]
    cpu_time_guest: f64,
    #[segment(field)]
    cpu_time_guest_nice: f64,
    #[segment(field)]
    cpu_time_idle: f64,
    #[segment(field)]
    cpu_time_iowait: f64,
    #[segment(field)]
    cpu_time_irq: f64,
    #[segment(field)]
    cpu_time_nice: f64,
    #[segment(field)]
    cpu_time_soft_irq: f64,
    #[segment(field)]
    cpu_time_steal: f64,
    #[segment(field)]
    cpu_time_stolen: f64,
    #[segment(field)]
    cpu_time_system: f64,
    #[segment(field)]
    cpu_time_user: f64,
    #[segment(field)]
    cpu_usage: f64,
    #[segment(field)]
    involuntary_context_switches: u32,
    #[segment(field)]
    memory_data: u64,
    #[segment(field)]
    memory_locked: u64,
    #[segment(field)]
    memory_rss: u64,
    #[segment(field)]
    memory_stack: u64,
    #[segment(field)]
    memory_swap: u64,
    #[segment(field)]
    memory_vms: u64,
    #[segment(field)]
    nice_priority: u32,
    #[segment(field)]
    num_fds: u32,
    #[segment(field)]
    num_threads: u32,
    #[segment(field)]
    pid: u32,
    #[segment(field)]
    read_bytes: u64,
    #[segment(field)]
    read_count: u64,
    #[segment(field)]
    realtime_priority: u32,
    #[segment(field)]
    rlimit_cpu_time_hard: u32,
    #[segment(field)]
    rlimit_cpu_time_soft: u32,
    #[segment(field)]
    rlimit_file_locks_hard: u32,
    #[segment(field)]
    rlimit_file_locks_soft: u32,
    #[segment(field)]
    rlimit_memory_data_hard: u32,
    #[segment(field)]
    rlimit_memory_data_soft: u32,
    #[segment(field)]
    rlimit_memory_locked_hard: u32,
    #[segment(field)]
    rlimit_memory_locked_soft: u32,
    #[segment(field)]
    rlimit_memory_rss_hard: u32,
    #[segment(field)]
    rlimit_memory_rss_soft: u32,
    #[segment(field)]
    rlimit_memory_stack_hard: u32,
    #[segment(field)]
    rlimit_memory_stack_soft: u32,
    #[segment(field)]
    rlimit_memory_vms_hard: u32,
    #[segment(field)]
    rlimit_memory_vms_soft: u32,
    #[segment(field)]
    rlimit_nice_priority_hard: u32,
    #[segment(field)]
    rlimit_nice_priority_soft: u32,
    #[segment(field)]
    rlimit_num_fds_hard: u32,
    #[segment(field)]
    rlimit_num_fds_soft: u32,
    #[segment(field)]
    rlimit_realtime_priority_hard: u32,
    #[segment(field)]
    rlimit_realtime_priority_soft: u32,
    #[segment(field)]
    rlimit_signals_pending_hard: u32,
    #[segment(field)]
    rlimit_signals_pending_soft: u32,
    #[segment(field)]
    signals_pending: u32,
    #[segment(field)]
    voluntary_context_switches: u32,
    #[segment(field)]
    write_bytes: u32,
    #[segment(field)]
    write_count: u32,
}



fn criterion_benchmark(c: &mut Criterion) {
    let t = SystemTime::now().duration_since(UNIX_EPOCH).expect("unable to generate now()");
    let procstats = ProcStats {
        timestamp: t,
        executable: "bash".to_string(),
        process: "bash".to_string(),
        cpu_time: 0,
        cpu_time_guest: 0_f64,
        cpu_time_guest_nice: 0_f64,
        cpu_time_idle: 0_f64,
        cpu_time_iowait: 0_f64,
        cpu_time_irq: 0_f64,
        cpu_time_nice: 0_f64,
        cpu_time_soft_irq: 0_f64,
        cpu_time_steal: 0_f64,
        cpu_time_stolen: 0_f64,
        cpu_time_system: 0_f64,
        cpu_time_user: 0.02_f64,
        cpu_usage: 0_f64,
        involuntary_context_switches: 2,
        memory_data: 1576960,
        memory_locked: 0,
        memory_rss: 5103616,
        memory_stack: 139264,
        memory_swap: 0,
        memory_vms: 21659648,
        nice_priority: 20,
        num_fds: 4,
        num_threads: 1,
        pid: 29417,
        read_bytes: 0,
        read_count: 259,
        realtime_priority: 0,
        rlimit_cpu_time_hard: 2147483647,
        rlimit_cpu_time_soft: 2147483647,
        rlimit_file_locks_hard: 2147483647,
        rlimit_file_locks_soft: 2147483647,
        rlimit_memory_data_hard: 2147483647,
        rlimit_memory_data_soft: 2147483647,
        rlimit_memory_locked_hard: 65536,
        rlimit_memory_locked_soft: 65536,
        rlimit_memory_rss_hard: 2147483647,
        rlimit_memory_rss_soft: 2147483647,
        rlimit_memory_stack_hard: 2147483647,
        rlimit_memory_stack_soft: 8388608,
        rlimit_memory_vms_hard: 2147483647,
        rlimit_memory_vms_soft: 2147483647,
        rlimit_nice_priority_hard: 0,
        rlimit_nice_priority_soft: 0,
        rlimit_num_fds_hard: 4096,
        rlimit_num_fds_soft: 1024,
        rlimit_realtime_priority_hard: 0,
        rlimit_realtime_priority_soft: 0,
        rlimit_signals_pending_hard: 78994,
        rlimit_signals_pending_soft: 78994,
        signals_pending: 0,
        voluntary_context_switches: 42,
        write_bytes: 106496,
        write_count: 35,
    };
    c.bench_function("procstats-2tags-52fields", move |b| {
        b.iter_batched_ref(
            || String::with_capacity(3048),
            |buffer: &mut String| {
                procstats.build(buffer);
                buffer.clear();
            },
            BatchSize::SmallInput
        )
    });

    let strings = StringTest{
        timestamp: t,
        tag0: "Taggy Tag Tag".to_string(),
        tag1: "tag\ntag tag".to_string(),
        field0: "Hello world, hello world".to_string(),
        field1: "hello \n World, hello there".to_string(),
    };
    c.bench_function("stringfields", move |b| {
        b.iter_batched_ref(
            || String::with_capacity(128),
            |buffer: &mut String| {
                strings.build(buffer);
                buffer.clear();
            },
            BatchSize::SmallInput
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
