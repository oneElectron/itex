use super::SettingsField;
use super::Type;
use proc_macro::TokenStream;
use quote::quote;

pub(super) fn generate_global_set_function(fields: Vec<SettingsField>) -> TokenStream {
    let mut match_statement = proc_macro2::TokenStream::new();

    for field in fields {
        let field_name = quote::format_ident!("set_{}", field.name);
        let field_name_string = field.name;

        match field.ty {
            Type::Bool => {
                match_statement.extend(quote! {
                    #field_name_string => build_settings.#field_name(match value.to_ascii_lowercase().as_str() {
                        "true" | "t" | "yes" | "y" => Some(true),
                        "false" | "f" | "no" | "n" => Some(false),
                        _ => {
                            println!("Invalid value");
                            exit!(0);
                        }
                    }),
                });
            }

            Type::PathBuf => {
                match_statement.extend(quote! {
                    #field_name_string => build_settings.#field_name(Some(std::path::PathBuf::from(value))),
                });
            }

            _ => {
                match_statement.extend(quote! {
                    #field_name_string => build_settings.#field_name(Some(value)),
                });
            }
        }
    }

    quote! {
        pub fn set(setting: Option<String>, value: Option<String>) {
            if setting.is_none() {
                println!("{}", style("No value given for setting").red().bold());
                exit!(0);
            }
            if value.is_none() {
                println!("{}", style("No value given for setting").red().bold());
                exit!(0);
            }

            let setting = setting.unwrap();
            let value = value.unwrap();

            let mut build_settings = Settings::find_and_parse_toml();

            match setting.as_str() {
                #match_statement
                _ => {
                    println!("{}", style("Invalid setting name").red().bold());
                    exit!(0);
                }
            }

            let build_settings_str: Result<String, toml::ser::Error> = toml::to_string_pretty(&build_settings);
            let build_settings_str: String = build_settings_str.unwrap();

            let mut path = std::env::current_dir().unwrap();
            path.push("itex-build.toml");

            let mut path_with_dot = path.clone();
            path_with_dot.push("/.itex-build.toml");

            if path_with_dot.is_file() {
                if std::fs::write(path_with_dot, build_settings_str).is_err() {
                    println!("{}", style("Failed to write to .itex-build.toml").red().bold());
                }
            } else if std::fs::write(path, build_settings_str).is_err() {
                println!("{}", style("Failed to write to .itex-build.toml").red().bold());
            }
        }
    }
    .into()
}
