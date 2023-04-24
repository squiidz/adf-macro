pub use adf_template_macro::adf_template;

pub mod prelude {
    pub use super::GenerateADF;
    pub use adf_template_macro::adf_template;
}

#[doc(hidden)]
pub mod __private {
    pub use htmltoadf::convert_html_str_to_adf_str;
    pub use serde;
    pub use tinytemplate;
}

pub trait GenerateADF {
    fn to_adf(&self) -> Result<String, Box<dyn std::error::Error>>;
}
