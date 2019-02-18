
extern crate proc_macro;
extern crate proc_macro2;


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


enum ParsingError {
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
    pub fn build(data: DeriveInput) -> Result<SegmentMetric, ParsingError> {
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
        metric.process_fields(&data.data);

        Ok(metric)
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
        let names = self.tags.iter().map(|t| t.name.clone());
        let vals = self.tags.iter().map(|t| t.struct_field.ident.clone());
        quote!{
            fn tags(&self) -> Vec<segment::Tag> {
                vec!(#( segment::Tag{name: #names.to_string(), value: self.#vals.to_string()}, )*)
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
                    value: segment::FieldValue::from(self.#vals),
                }, )*)
            }
        }
    }

    // Returns the name, or index, of the field which contains the Metric's timestamp
    fn find_fields(data: &Data) -> Vec<SegmentField> {
        if let Data::Struct(ref data) = *data {
            let fields = match data.fields {
                Fields::Named(ref fields) => fields.named.clone(),
                Fields::Unnamed(ref fields) => fields.unnamed.clone(),
                _ => panic!("Unknown fields")
            };
            let mut segment_fields: Vec<SegmentField> = Vec::new();
            for (field_idx, field) in fields.iter().enumerate() {
                match make_field(field, field_idx) {
                    None => (),
                    Some(seg_field) =>
                        segment_fields.push(seg_field)
                }
            }
            segment_fields
        } else {
            panic!("We only handle structs here..");
        }
    }

    fn process_fields(&mut self, input: &Data) -> Result<(), ParsingError> {
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
        Ok(())
    }
}

#[proc_macro_derive(Metric, attributes(segment))]
pub fn metric_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let metric = match SegmentMetric::build(input) {
        Ok(m) => m,
        Err(e) => panic!(e),
    };

    let name = &metric.name;
    let measurement = metric.measurement_fn();
    let time = metric.time_fn();
    let tags = metric.tags_fn();
    let fields = metric.fields_fn();

    TokenStream::from(quote!{
        impl Metric for #name {
            #time
            #measurement
            #tags
            #fields

            fn to_lineproto(&self) -> String {
                "not_implemented".to_string()
            }
        }
    })

// Returns the name, or index, of the field which contains the Metric's timestamp
fn find_fields(data: &Data) -> Vec<SegmentField> {
    if let Data::Struct(ref data) = *data {
        let fields = match data.fields {
            Fields::Named(ref fields) => fields.named.clone(),
            Fields::Unnamed(ref fields) => fields.unnamed.clone(),
            _ => panic!("Unknown fields")
        };
        let mut segment_fields: Vec<SegmentField> = Vec::new();
        for (field_idx, field) in fields.iter().enumerate() {
           match make_field(field, field_idx) {
               None => (),
               Some(seg_field) =>
                   segment_fields.push(seg_field)
           }
        }
        segment_fields
    } else {
        panic!("We only handle structs here..");
    }
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
