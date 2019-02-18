use std::fmt;
use std::string::ToString;
use std::time::Duration;

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

impl fmt::Display for FieldValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FieldValue::Str(s) =>
                write!(f, "{}", s),
            FieldValue::UInt32(u)  =>
                write!(f, "{}i", u),
            FieldValue::UInt64(u) =>
                write!(f, "{}i", u),
            FieldValue::Int32(i) =>
                write!(f, "{}i", i),
            FieldValue::Int64(i) =>
                write!(f, "{}i", i),
            FieldValue::Float32(fl) =>
                write!(f, "{}", fl),
            FieldValue::Float64(fl) =>
                write!(f, "{}", fl),
        }
    }
}

impl From<String> for FieldValue {
    fn from(item: String) -> Self {
        FieldValue::Str(item)
    }
}

impl From<u32> for FieldValue {
    fn from(item: u32) -> Self {
        FieldValue::UInt32(item)
    }
}

impl From<u64> for FieldValue {
    fn from(item: u64) -> Self {
        FieldValue::UInt64(item)
    }
}

impl From<f32> for FieldValue {
    fn from(item: f32) -> Self {
        FieldValue::Float32(item)
    }
}

impl From<f64> for FieldValue {
    fn from(item: f64) -> Self {
        FieldValue::Float64(item)
    }
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
    fn time(&self) -> Duration;
    fn measurement(&self) -> String;
    fn fields(&self) -> Vec<Field>;
    fn tags(&self) -> Vec<Tag>;
    fn to_lineproto(&self) -> String;
}

// measurement[,tag=val[,tag=val]] field=value[,field=value]
