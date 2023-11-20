use super::SettingsField;
use quote::quote;

pub(crate) fn generate_merge_global_and_local(fields: Vec<SettingsField>) -> proc_macro::TokenStream {
    let match_statements = gen_match_statements(&fields);

    quote! {
        impl Settings {
            fn merge_global_and_local(global: Option<Settings>, local: Settings) -> Self {
                if global.is_none() {
                    return local;
                }

                let global = global.unwrap();

                let mut output_settings: Settings = Settings::empty();

                #match_statements

                output_settings
            }
        }
    }
    .into()
}

fn gen_match_statements(fields: &Vec<SettingsField>) -> proc_macro2::TokenStream {
    let mut body = proc_macro2::TokenStream::new();

    for field in fields {
        let field_name = quote::format_ident!("{}", field.name.clone());
        body.extend(quote! {
            output_settings.#field_name = match local.#field_name {
                Some(value) => Some(value),
                None => {
                    match global.#field_name {
                        Some(value) => Some(value),
                        None => None,
                    }
                },
            };
        });
    }

    body
}
