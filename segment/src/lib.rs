//! A library for serializing metric data into InfluxData's Line Protocol for
//! ingestion into influxdb.

use std::string::ToString;
use std::time::Duration;

pub use segment_derive::*;

#[macro_export]
macro_rules! segment_write {
    // Type suffix for influx integers, etc.
    ( @type_suff, $b:ident, f32, $i:ident ) => { };
    ( @type_suff, $b:ident, f64, $i:ident ) => { };
    ( @type_suff, $b:ident, $t:tt, tag) => {  };
    ( @type_suff, $b:ident, $t:tt, field ) => {  $b.push('i'); };

    // Determine serialization for numerics
    ( @num_ser, f32, $b:ident, $($i:ident).+ ) => { dtoa::write(&mut $b, $($i).*)?; };
    ( @num_ser, f64, $b:ident, $($i:ident).+ ) => { dtoa::write(&mut $b, $($i).*)?; };
    ( @num_ser, $t:tt, $b:ident, $($i:ident).+)  =>   { itoa::write(&mut $b, $($i).*)?; };

    // Serialization for string types
    ( @str_ser, $b:ident, $val:expr, tag ) => {
        segment::build_escapedtagstr($val, $b);
    };
    ( @str_ser, $b:ident, $val:expr, field ) => {
        segment::build_escapedfieldstr($val, $b);
    };

    // Main Entry
    ( $b:ident, $($i:ident).+, String, $lf:ident ) => { segment::segment_write!(@str_ser, $b, &$($i).*, $lf); };
    ( $b:ident, $($i:ident).+, &str, $lf:ident ) => { segment::segment_write!(@str_ser, $b, $($i).*, $lf); };
    ( $b:ident, $($i:ident).+, &'static str, $lf:ident ) => { segment::segment_write!(@str_ser, $b, $($i).*, $lf) };
    ( $b:ident, $($i:ident).+, $t:tt, $lf:ident ) => {
        unsafe {
            let mut bytes = $b.as_mut_vec();
            segment::segment_write!(@num_ser, $t, bytes, $($i).*);
        }
        segment::segment_write!(@type_suff, $b, $t, $lf);
    };
}

pub enum MetricError {
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

impl FieldValue {
    pub fn build(&self, sb: &mut String) {
        // NOTE: The unsafe sections below are unsafe due to manipulation of
        //       the Vec returned by String::as_mut_vec(), since there is no
        //       UTF8 validation. itoa, and dtoa, write in UTF8 compatible
        //       encoding, so the unsafes are safe.
        match self {
            FieldValue::Str(s) => {
                sb.push('"');
                build_escapedfieldstr(s, sb);
                sb.push('"');
            },
            FieldValue::UInt32(u)  => {
                unsafe {
                    let mut bytes = sb.as_mut_vec();
                    itoa::write(&mut bytes, *u).expect("cannot write u32");
                }
                sb.push('i');
            },
            FieldValue::UInt64(u) => {
                unsafe {
                    let mut bytes = sb.as_mut_vec();
                    itoa::write(&mut bytes, *u).expect("cannot write u64");
                }
                sb.push('i');
            },
            FieldValue::Int32(i) => {
                unsafe {
                    let mut bytes = sb.as_mut_vec();
                    itoa::write(&mut bytes, *i).expect("cannot write i32");
                }
                sb.push('i');
            },
            FieldValue::Int64(i) => {
                unsafe {
                    let mut bytes = sb.as_mut_vec();
                    itoa::write(&mut bytes, *i).expect("cannot write i64");
                }
                sb.push('i');
            },
            FieldValue::Float32(fl) => {
                unsafe {
                    let mut bytes = sb.as_mut_vec();
                    dtoa::write(&mut bytes, *fl).expect("cannot write f32");
                }
            },
            FieldValue::Float64(fl) => {
                unsafe {
                    let mut bytes = sb.as_mut_vec();
                    dtoa::write(&mut bytes, *fl).expect("cannot write f64");
                }
            }
        };
    }
}

impl ToString for FieldValue {
    fn to_string(&self) -> String {
        let mut sret = String::new();
        self.build(&mut sret);
        sret
    }
}

impl From<String> for FieldValue {
    #[inline]
    fn from(item: String) -> Self {
        FieldValue::Str(item)
    }
}

impl From<&str> for FieldValue {
    #[inline]
    fn from(item: &str) -> Self {
        FieldValue::Str(String::from(item))
    }
}

impl From<u32> for FieldValue {
    #[inline]
    fn from(item: u32) -> Self {
        FieldValue::UInt32(item)
    }
}

impl From<u64> for FieldValue {
    #[inline]
    fn from(item: u64) -> Self {
        FieldValue::UInt64(item)
    }
}

impl From<f32> for FieldValue {
    #[inline]
    fn from(item: f32) -> Self {
        FieldValue::Float32(item)
    }
}

impl From<f64> for FieldValue {
    #[inline]
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
    fn build(&self, buffer: &mut String) -> std::io::Result<usize>;
}

// measurement[,tag=val[,tag=val]] field=value[,field=value]

pub fn build_escapedtagstr(s: &str, buff: &mut String) {
    for c in s.chars() {
        match c {
            ',' | ' ' | '=' => {
                buff.push('\\');
                buff.push(c);
            },
            _ =>
                buff.push(c)
        }
    }
}

// NOTE: Source for this is an adaptation from std::String::replace
pub fn build_escapedfieldstr(s: &str, buff: &mut String) {
    let from = "\"";
    let to = "\\\"";
    let mut last_end = 0;
    for (start, part) in s.match_indices(from) {
        buff.push_str(unsafe { s.get_unchecked(last_end..start) });
        buff.push_str(to);
        last_end = start + part.len();
    }
    buff.push_str(unsafe { s.get_unchecked(last_end..s.len()) });
}

pub fn escape_tagstr(s: &str) -> String {
    let mut new_s = String::with_capacity(s.len()+16);
    build_escapedtagstr(s, &mut new_s);
    new_s
}

pub fn escape_fieldstr(s: &str) -> String {
    let mut result = String::new();
    build_escapedfieldstr(s, &mut result);
    result
}
