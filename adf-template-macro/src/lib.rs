use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(adf_template, attributes(path))]
pub fn adf_template(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let ident = input.ident;
    let path = "template/comment.html";

    let output = quote! {

        impl #ident {
            fn new(title: String, body: String) -> Self {
                Self { title, body }
            }

            fn generate(&self) -> String {
                let template_str = include_str!(#path);
                let mut template = ::adf_template_lib::__private::tinytemplate::TinyTemplate::new();
                template.add_template(stringify!(#ident), &template_str).unwrap();
                let rendered = template.render(stringify!(#ident), &self).unwrap();
                let adf_str = ::adf_template_lib::__private::convert_html_str_to_adf_str(rendered);
                adf_str
            }
        }
    };

    output.into()
}
