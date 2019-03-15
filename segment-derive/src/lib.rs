
extern crate proc_macro;
extern crate proc_macro2;

use std::fmt;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Field, LitStr, Lit, Ident};
use syn::Meta::{List, NameValue, Word};
use syn::NestedMeta::Meta;
use quote::quote;


/// The type of line protocol element a field represents.
enum SegmentFieldType {
    Unknown,
    Tag,
    Field,
    Time,
}

#[derive(Debug, Clone)]
enum MetricError {
    NoFields(),
}

impl fmt::Display for MetricError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            MetricError::NoFields() => "no fields defined for metric",
        };
        write!(f, "{}", s)
    }
}

struct SegmentField {
    // Field that holds the value of the field
    struct_field: Field,
    // Name of the field in the line protocol
    name: String,
    // What type of value in the line proto is this
    field_type: SegmentFieldType,
}

struct SegmentMetric {
    name: Ident,
    measurement: LitStr,
    fields: Vec<SegmentField>,
    tags: Vec<SegmentField>,
    time_field: Option<SegmentField>,
}

impl SegmentMetric {
    pub fn build(data: DeriveInput) -> Result<SegmentMetric, MetricError> {
        let mut metric: SegmentMetric = SegmentMetric{
            name: data.ident.clone(),
            measurement: syn::LitStr::new(&data.ident.to_string(), data.ident.span()),
            fields: Vec::new(),
            tags: Vec::new(),
            time_field: None,
        };

        // Get Measurement name if present..
        for meta_item in data.attrs.iter().filter_map(get_segment_meta).flat_map(|x| x) {
            match meta_item {
                Meta(NameValue(ref m)) if m.ident == "measurement" =>
                    if let syn::Lit::Str(ref lit) = m.lit {
                        metric.measurement = lit.clone();
                    },
                _ => (),
            }
        }

        // Gather all fields from the metric.
        metric.process_fields(&data.data)?;

        if metric.fields.len() == 0 {
            Err(MetricError::NoFields())
        } else {
            Ok(metric)
        }
    }

    pub fn measurement_fn(&self) -> proc_macro2::TokenStream {
        let measurement = &self.measurement;
        quote!(fn measurement(&self) -> String { #measurement.to_string() })
    }

    pub fn time_fn(&self) -> proc_macro2::TokenStream {
        match self.time_field {
            Some(ref t) => {
                let time = &t.struct_field.ident;
                quote!(fn time(&self) -> std::time::Duration { self.#time })
            },
            None =>
                panic!("no field flagged as metric time"),
        }
    }

    pub fn tags_fn(&self) -> proc_macro2::TokenStream {
        let names: Vec<String> = self.tags.iter().map(|t| t.name.clone()).collect();
        let vals = self.tags.iter().map(|t| t.struct_field.ident.clone());
        quote!{
            fn tags(&self) -> Vec<segment::Tag> {
                vec!(#(
                        segment::Tag{
                            name: #names.to_string(),
                            value: segment::escape_tagstr(&self.#vals.to_string()),
                        },
                )*)
            }
        }
    }

    pub fn fields_fn(&self) -> proc_macro2::TokenStream {
        let names = self.fields.iter().map(|f| f.name.clone());
        let vals = self.fields.iter().map(|f| {
            f.struct_field.ident.clone()
        });
        quote!{
            fn fields(&self) -> Vec<segment::Field> {
                vec!(#(segment::Field{
                    name: #names.to_string(),
                    value: segment::FieldValue::from(self.#vals.clone()),
                }, )*)
            }
        }
    }


    fn tag_vals(&self) -> proc_macro2::TokenStream {
        let tags = self.tags.iter().map(|t| {
            let n = &t.name;
            let v = &t.struct_field.ident;
            let ty = &t.struct_field.ty;
            quote!{
                s.push_str(concat!(",", #n, "="));
                segment::segment_write!(s, self.#v, #ty, tag);
            }
        });

        quote!{
            #( #tags )*
        }
    }

    fn field_vals(&self) -> proc_macro2::TokenStream {
        let mut fields = self.fields.iter().map(|f| {
            let n = &f.name;
            let v = &f.struct_field.ident;

            let ty = &f.struct_field.ty;

            (
                quote!(concat!(#n,"=")),
                quote!(segment::segment_write!(s, self.#v, #ty, field);)
            )
        });

        let fields_head = match fields.next() {
            Some((n, v)) => {
                Some(quote!{
                    s.push_str(#n);
                    #v
                })
            },
            None =>
                None
        };

        let fields_tail = fields.map(|(n, v)| {
            quote!{
                s.push_str(concat!(",", #n));
                #v
            }
        });
        quote!{
            #fields_head
            #( #fields_tail )*
        }
    }

    pub fn lineproto_fn(&self) -> proc_macro2::TokenStream {
        // <measurement>,<tags> <fields> <time>
        let measurement = &self.measurement;
        let push_tags = self.tag_vals();
        let push_fields = self.field_vals();
        match &self.time_field {
            None => panic!("no field declared as time of metric"),
            Some(t) => {
                let tfield = &t.struct_field.ident;
                let ns = quote!{
                    let sec_ns = self.#tfield.as_secs() * 1e9 as u64;
                    let ns = sec_ns + self.#tfield.subsec_nanos() as u64;
                };
                quote!{
                    fn to_lineproto(&self) -> String {
                        let mut s = String::with_capacity(64);
                        self.build(&mut s);
                        s
                    }

                    fn build(&self, s: &mut String) -> std::io::Result<usize> {
                        s.push_str(#measurement);
                        #push_tags
                        s.push(' ');
                        #push_fields
                        s.push(' ');
                        #ns
                        unsafe {
                            let mut bytes = s.as_mut_vec();
                            itoa::write(&mut bytes, ns);
                        }
                        Ok(s.len())
                    }
                }
            }
        }
    }

    fn process_fields(&mut self, input: &Data) -> Result<(), MetricError> {
        if let Data::Struct(ref data) = *input {
            let fields = match data.fields {
                Fields::Named(ref fields) => fields.named.clone(),
                Fields::Unnamed(ref fields) => fields.unnamed.clone(),
                _ => panic!("unknown fields")
            };

            for (field_idx, field) in fields.iter().enumerate() {
                match make_field(field, field_idx) {
                    None => (),
                    Some(seg_field) =>
                        match seg_field.field_type {
                            SegmentFieldType::Tag =>
                                self.tags.push(seg_field),
                            SegmentFieldType::Field =>
                                self.fields.push(seg_field),
                            SegmentFieldType::Time =>
                                self.time_field = Some(seg_field),
                            SegmentFieldType::Unknown =>
                                (),
                        }
                }
            }
        } else {
            panic!("only structs supported");
        }

        // Sort our tags lexographically, per Influx Data recommendation.
        self.tags.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

        Ok(())
    }
}

#[proc_macro_derive(Metric, attributes(segment))]
pub fn metric_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let metric = match SegmentMetric::build(input) {
        Ok(m) => m,
        Err(e) => panic!("Error"),
    };

    let name = &metric.name;
    let measurement = metric.measurement_fn();
    let time = metric.time_fn();
    let tags = metric.tags_fn();
    let fields = metric.fields_fn();
    let to_lineproto = metric.lineproto_fn();

    TokenStream::from(quote!{
        impl Metric for #name {
            #time
            #measurement
            #tags
            #fields
            #to_lineproto
        }
    })
}

// TODO: Sanity check types: tags need to be strings.. time needs to be duration.
fn make_field(field: &Field, field_idx: usize) -> Option<SegmentField> {
    let mut seg_field: SegmentField = SegmentField{
        struct_field: field.clone(),
        field_type: SegmentFieldType::Unknown,
        name: match &field.ident {
            Some(id) => format!("{}", id).to_string(),
            None => format!("{}", field_idx).to_string(),
        }
    };

    for meta_item in field.attrs.iter().filter_map(get_segment_meta).flat_map(|x| x) {
        match meta_item {
            Meta(Word(ref w)) if w == "tag" =>
                seg_field.field_type = SegmentFieldType::Tag,
            Meta(Word(ref w)) if w == "field" =>
                seg_field.field_type = SegmentFieldType::Field,
            Meta(Word(ref w)) if w == "time" =>
                seg_field.field_type = SegmentFieldType::Time,
            Meta(NameValue(ref n)) if n.ident == "rename" => match &n.lit {
                Lit::Str(s) =>
                    seg_field.name = s.value(),
                _ =>
                    println!("Other Lit"),
            },
            _ =>
                println!("Unexpected attribute value"),
        }
    }
    match seg_field.field_type {
        SegmentFieldType::Unknown => None,
        _ => Some(seg_field),
    }
}

fn get_segment_meta(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "segment" {
        match attr.interpret_meta() {
            Some(List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
            _ => None
        }
    } else {
        None
    }
}
