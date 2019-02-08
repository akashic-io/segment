
use std::time::SystemTime;

pub use segment_derive::*;


pub struct Builder {
    // bytes..
}

pub enum FieldValue {
    Str(String),
    UInt32(u32),
    UInt64(u64),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
}

pub struct Field {
    pub name: String,
    pub value:  FieldValue,
}

pub struct Tag {
    pub name: String,
    pub value: String,
}


// A metric represents a single point in a measurement.
pub trait Metric {
    fn time(&self) -> SystemTime;
    fn measurement(&self) -> String;
    fn fields(&self) -> Vec<Field>;
    fn tags(&self) -> Vec<Tag>;
    fn to_lineproto(&self) -> String;
}

// measurement[,tag=val[,tag=val]] field=value[,field=value]

impl Metric {
    pub fn to_string() -> String {
        "foo".to_string()
    }
}
