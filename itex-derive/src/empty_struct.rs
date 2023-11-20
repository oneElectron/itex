use super::SettingsField;
use quote::quote;

pub(crate) fn generate_empty_function(fields: Vec<SettingsField>) -> proc_macro::TokenStream {
    let return_statement = gen_return_statement(&fields);

    quote! {
        impl Settings {
            pub fn empty() -> Self {
                #return_statement
            }
        }
    }
    .into()
}

fn gen_return_statement(fields: &Vec<SettingsField>) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();
    output.extend(quote! {
        let mut output_settings: Settings = unsafe { std::mem::zeroed() };
    });

    for field in fields {
        let field_ident = quote::format_ident!("{}", field.name);

        output.extend(quote! {
            output_settings.#field_ident = None;
        })
    }

    output.extend(quote! {
        output_settings
    });

    output
}
