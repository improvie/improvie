#![allow(clippy::panic, clippy::unwrap_used)]

use std::str::FromStr;

#[proc_macro_attribute]
pub fn ts(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut arg = String::from("export");
    if !attr.is_empty() {
        let arg_str = attr.to_string();
        arg = format!("{arg}, export_to = {arg_str}");
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
