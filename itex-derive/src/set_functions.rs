use super::SettingsField;
use proc_macro::TokenStream;
use quote::quote;

pub(super) fn generate_set_function(field: SettingsField) -> TokenStream {
    let func_name = quote::format_ident!("set_{}", field.name);
    let to_ty = quote::format_ident!("{}", field.ty.as_string());
    let field_name = quote::format_ident!("{}", field.name);

    quote! {
        impl Settings {
            pub fn #func_name(&mut self, to: Option<#to_ty>) {
                self.#field_name = to;
            }
        }
    }
    .into()
}
