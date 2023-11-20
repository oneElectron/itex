use proc_macro::TokenStream;
use quote::quote;

use super::SettingsField;

pub(super) fn generate_global_get_function(fields: Vec<SettingsField>) -> TokenStream {
    let mut match_statement = proc_macro2::TokenStream::new();

    for field in fields {
        let field_name = quote::format_ident!("print_{}", field.name);
        let field_name_string = field.name;

        match_statement.extend(quote! {
            #field_name_string => itex_build_toml.#field_name(),
        });
    }

    quote! {
        pub fn get(setting: Option<String>) -> std::result::Result<(), u32> {
            let itex_build_toml = Settings::from_global();

            if setting.is_none() {
                println!("{}", itex_build_toml);
                return Ok(());
            }
            let setting = setting.unwrap();
            let setting = setting.as_str();

            match setting {
                #match_statement
                _ => {
                    println!("{}", console::style("Invalid setting name").red().bold());
                    exit!(0);
                }
            };

            Ok(())
        }

    }
    .into()
}
