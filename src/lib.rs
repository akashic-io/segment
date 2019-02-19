use std::fmt;
use std::string::ToString;
use std::time::Duration;

pub use segment_derive::*;


pub enum MetricError {
}

// TODO: Implement a macro thatt handles implementing Display for FieldValue
//macro_rules! enum_str {
//    (pub enum $name:ident {
//        $($variant:ident(type)),*,
//    }) => {
//        enum $name {
//            $($variant = $val),*
//        }
//
//        impl fmt::Display for $name {
//            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//                let val = match self {
//                    $($name::$variant => stringify!($variant)),*
//                };
//                write!(f, "{}", val)
//            }
//        }
//    };
//}

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
        match self {
            FieldValue::Str(s) => {
                sb.push('"');
                sb.push_str(&s.replace("\"", "\\\""));
                sb.push('"');
            },
            FieldValue::UInt32(u)  => {
                fmt::write(sb, format_args!("{}i", u));
            },
            FieldValue::UInt64(u) => {
                fmt::write(sb, format_args!("{}i", u));
            },
            FieldValue::Int32(i) => {
                fmt::write(sb, format_args!("{}i", i));
            },
            FieldValue::Int64(i) => {
                fmt::write(sb, format_args!("{}i", i));
            },
            FieldValue::Float32(fl) =>
                sb.push_str(&fl.to_string()),
            FieldValue::Float64(fl) =>
                sb.push_str(&fl.to_string()),
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
    fn build(&self, buffer: &mut String) -> Result<(), MetricError>;
}

// measurement[,tag=val[,tag=val]] field=value[,field=value]

pub fn build_escapedtagstr(s: String, buff: &mut String) {
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

pub fn escape_tagstr(s: String) -> String {
    let mut new_s = String::with_capacity(s.len()+16);
    build_escapedtagstr(s, &mut new_s);
    new_s
}

pub fn escape_fieldstr(s: String) -> String {
    s.replace("\"", "\\\"")
}
