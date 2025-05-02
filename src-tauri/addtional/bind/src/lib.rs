#![allow(clippy::panic, clippy::unwrap_used)]

use std::str::FromStr;

#[proc_macro_attribute]
pub fn ts(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut arg = String::from("export");
    if !attr.is_empty() {
        let arg_str = attr.to_string();
        if !arg_str.starts_with('"') {
            panic!("The export_to argument must be a string");
        }
        let arg_str = arg_str.trim_matches('"');
        if arg_str.is_empty() {
            panic!("The export_to argument must not be empty");
        }
        if !arg_str.ends_with(".ts") {
            panic!("The export_to argument must end with .ts");
        }
        arg = format!(r#"{arg}, export_to = "{arg_str}""#);
    }

    let mut base = proc_macro::TokenStream::from_str(&format!(
        "
#[derive(ts_rs::TS)]
#[ts({arg})]
"
    ))
    .unwrap();

    base.extend(item);

    base
}
