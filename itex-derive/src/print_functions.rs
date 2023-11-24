use super::SettingsField;
use super::Type;
use proc_macro::TokenStream;
use quote::quote;

pub(super) fn generate_display_function(field: SettingsField) -> TokenStream {
    let func_name = quote::format_ident!("print_{}", field.name);
    let field_name = quote::format_ident!("{}", field.name);
    let field_name_string = field.name;

    #[allow(unused_assignments)]
    let mut inherited_value = proc_macro2::TokenStream::new();
    match field.ty {
        Type::PathBuf => {
            inherited_value = quote! { self.#field_name().display() };
        }
        _ => {
            inherited_value = quote! { self.#field_name() };
        }
    };

    quote! {
        impl Settings {
            pub fn #func_name(&self, local_settings: Option<&Settings>) {
                let mut optional_settings: Settings;
                let mut local_settings = match local_settings {
                    Some(v) => v,
                    None => {
                        optional_settings = Settings::from_local();
                        &optional_settings
                    },
                };

                match &self.#field_name {
                    Some(value) => {
                        match local_settings.#field_name.clone() {
                            Some(v) => println!("{} = {}", console::style(#field_name_string).bright().blue(), console::style(#inherited_value)),
                            None => println!("{} = {} (global)", console::style(#field_name_string).bright().blue(), console::style(#inherited_value)),
                        }
                    },
                    None => {
                        println!("{} = {} (inherited)", console::style(stringify!(#field_name)).bright().blue(), #inherited_value);
                    },
                }
            }
        }
    }
    .into()
}

pub(super) fn generate_fmt_display_function(fields: Vec<SettingsField>) -> TokenStream {
    let mut body = proc_macro2::TokenStream::new();

    for field in fields {
        let print_command = quote::format_ident!("print_{}", field.name);
        body.extend::<proc_macro2::TokenStream>(quote! {
            self.#print_command(Some(&local_settings));
        });
    }

    quote! {
        impl std::fmt::Display for Settings {
            fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                let local_settings = Settings::from_local();
                #body

                fmt::Result::Ok(())
            }
        }

    }
    .into()
}
