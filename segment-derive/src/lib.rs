
extern crate proc_macro;
extern crate proc_macro2;


use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Field, LitStr, Lit};
use syn::Meta::{List, NameValue, Word};
use syn::NestedMeta::Meta;
use quote::quote;

enum SegmentField {
    Tag(Field, String),
    Field(Field, String),
    Time(Field),
}

struct Options {
    measurement: Option<LitStr>,
}

#[proc_macro_derive(Metric, attributes(segment))]
pub fn metric_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident.clone();

    let opts = get_options(&input);
    let fields = find_fields(&input.data);
    let time_fn = fields.iter().find_map(|f| match f {
            SegmentField::Time(field) => {
                let name = &field.ident;
                Some(quote!(fn time(&self) -> std::time::SystemTime { self.#name }))
            },
            _ =>
                None,
    });
    let measure_fn = opts.measurement.map(|m| {
        quote!(fn measurement(&self) -> String { #m.to_string() })
    });
    let tags = fields.iter().filter_map(|f| match f {
        SegmentField::Tag(t, n) => {
            let name = syn::LitStr::new(n, proc_macro2::Span::call_site());
            let id = t.ident.clone();
            Some(quote!{ segment::Tag{name: #name.to_string(), value: self.#id.to_string()} })
        },
        _ => None
    });
    let tags_fn = quote! {
        fn tags(&self) -> Vec<segment::Tag> {
            vec!(#( #tags, )*)
        }
    };

    let mfields = fields.iter().filter_map(|f| match f {
        SegmentField::Field(f, n) => {
            let name = syn::LitStr::new(n, proc_macro2::Span::call_site());
            let id = f.ident.clone();
            match f.ty {
                syn::Type::Path(ref tp) => {
                    println!("PATH: {}", tp.path.segments[0].ident);

                },
                _ => ()
            };
            Some(quote!{
                segment::Field{name: #name.to_string(), value: segment::FieldValue::UInt64(self.#id)}
            })
        },
        _ => None
    });
    let fields_fn = quote! {
        fn fields(&self) -> Vec<segment::Field> {
            vec!(#( #mfields, )*)
        }
    };

    let expanded = quote! {
        impl segment::Metric for #name {
            #measure_fn
            #time_fn
            #tags_fn
            #fields_fn

        }
    };

    TokenStream::from(expanded)
}

fn get_options(data: &DeriveInput) -> Options {
    let mut opts = Options{measurement: None};
    for meta_items in data.attrs.iter().filter_map(get_segment_meta) {
        for meta_item in meta_items {
            match meta_item {
                Meta(NameValue(ref m)) if m.ident == "measurement" =>
                    if let syn::Lit::Str(ref lit) = m.lit {
                        opts.measurement = Some(lit.clone());
                    },
                _ => (),
            }
        }
    }
    opts
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

// TODO: Clean this up.. SegmentField isn't ergonomic enough..
fn make_field(field: &Field, field_idx: usize) -> Option<SegmentField> {
    let mut field_tpe: u8 = 0; // 0 = Unknown, 1 = Tag, 2 = Field, 3 = Time
    let mut seg_field: Option<SegmentField> = None;
    let mut field_name = match &field.ident {
        Some(id) => format!("{}", id).to_string(),
        None => format!("{}", field_idx).to_string(),
    };

    for meta_items in field.attrs.iter().filter_map(get_segment_meta) {
        for meta_item in meta_items {
            match meta_item {
                Meta(Word(ref w)) if w == "tag" =>
                    field_tpe = 1,
                Meta(Word(ref w)) if w == "field" =>
                    field_tpe = 2,
                Meta(Word(ref w)) if w == "time" =>
                    field_tpe = 3,
                Meta(NameValue(ref n)) if n.ident == "rename" => match &n.lit {
                    Lit::Str(s) =>
                        field_name = s.value(),
                    _ =>
                        println!("Other Lit"),
                },
                _ =>
                    println!("Unexpected attribute value"),
            }
        }
    }
    match field_tpe {
        0 =>
            None,
        1 =>
            Some(SegmentField::Tag(field.clone(), field_name)),
        2 =>
            Some(SegmentField::Field(field.clone(), field_name)),
        3 =>
            Some(SegmentField::Time(field.clone())),
        _ => None,
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
