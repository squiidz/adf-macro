pub use adf_template_macro::adf_template;

#[doc(hidden)]
pub mod __private {
    pub use htmltoadf::convert_html_str_to_adf_str;
    pub use tinytemplate;
    pub use serde;
}

