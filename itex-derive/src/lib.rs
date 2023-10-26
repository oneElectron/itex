//! Internal only library to create the settings for itex

mod get;
mod print_functions;
mod set;
mod set_functions;

extern crate proc_macro;
use proc_macro::TokenStream;

#[derive(Debug, Clone, Copy)]
enum Type {
    PathBuf,
    String,
    Bool,
    NotSupported,
}

impl Type {
    pub fn as_string(&self) -> String {
        match self {
            Self::PathBuf => "PathBuf".to_string(),
            Self::String => "String".to_string(),
            Self::Bool => "bool".to_string(),

            _ => panic!("unsupported type"),
        }
    }
}

#[derive(Debug, Clone)]
struct SettingsField {
    ty: Type,
    name: String,
}

#[proc_macro_attribute]
pub fn itex_settings(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut output = input.clone();

    let ast = syn::parse_macro_input!(input as syn::ItemStruct);
    let fields = match ast.fields {
        syn::Fields::Named(fields) => {
            let mut output: Vec<SettingsField> = vec![];

            for field in fields.named {
                output.push(parse_field(field));
            }

            output
        }
        _ => {
            panic!("Fields must be named")
        }
    };

    output.extend(print_functions::generate_fmt_display_function(fields.clone()));

    output.extend(get::generate_global_get_function(fields.clone()));

    output.extend(set::generate_global_set_function(fields.clone()));

    for field in fields {
        output.extend(print_functions::generate_display_function(field.clone()));

        output.extend(set_functions::generate_set_function(field));
    }

    output
}

fn parse_field(field: syn::Field) -> SettingsField {
    let name = field.ident.unwrap().to_string();
    let ty = match field.ty {
        syn::Type::Path(p) => {
            let ty = p.path.segments.last().unwrap().ident.to_string();
            if ty.as_str() != "Option" {
                panic!("Must be an optional value");
            }

            let inner_ty = match &p.path.segments.last().unwrap().arguments {
                syn::PathArguments::AngleBracketed(t) => match t.args.last().unwrap() {
                    syn::GenericArgument::Type(syn::Type::Path(p)) => match p.path.segments.last().unwrap().ident.to_string().as_str() {
                        "PathBuf" => Type::PathBuf,
                        "String" => Type::String,
                        "bool" => Type::Bool,
                        _ => Type::NotSupported,
                    },
                    _ => {
                        panic!()
                    }
                },
                _ => {
                    panic!("Must be Angled arguments")
                }
            };

            inner_ty
        }
        _ => {
            panic!("Must be a ")
        }
    };

    SettingsField { ty, name }
}
