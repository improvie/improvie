#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

use std::str::FromStr;

macro_rules! token {
    ($expr:expr) => {
        proc_macro::TokenStream::from_str($expr).unwrap()
    };
}

#[proc_macro_attribute]
pub fn ts(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut arg = String::from("export");
    if attr.is_empty() {
        panic!("The export_to argument is required. Please provide a path by string.");
    }
    let raw_arg_str = attr.to_string();
    let arg_str = raw_arg_str.trim_matches('"');
    if arg_str == raw_arg_str {
        panic!("The export_to argument must be a string");
    }
    if arg_str.is_empty() {
        panic!("The export_to argument must not be empty");
    }
    for c in arg_str.chars() {
        if c.is_ascii_uppercase() {
            panic!("The export_to argument must not contain uppercase letters");
        }
        if c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '/' {
            continue;
        }
        panic!("The export_to argument must be a valid a path. Use cabab-case for path.");
    }
    if !arg_str.ends_with(".ts") {
        panic!("The export_to argument must end with .ts");
    }
    arg = format!(r#"{arg}, export_to = "{arg_str}""#);

    let mut base = token!(&format!(
        "
#[derive(ts_rs::TS)]
#[ts({arg})]
"
    ));

    base.extend(item);

    base
}

fn module_concat(attr: proc_macro::TokenStream, file: &'static str) -> proc_macro::TokenStream {
    let attr = attr.to_string();
    let attr = attr.trim_matches('"');
    if attr.is_empty() {
        panic!("The module argument must not be empty");
    }
    token!(&format!(r#""{attr}/{file}.ts""#))
}

fn require_ident_ends_with(item: &proc_macro::TokenStream, suffix: &str) -> String {
    let item = item.clone();
    let input =
        syn::parse::<syn::DeriveInput>(item).expect("Failed to parse item as syn::DeriveInput");
    let ident = input.ident.to_string();
    if !ident.ends_with(suffix) {
        panic!("The item must end with {suffix}, but got {ident}");
    }
    ident
}

#[proc_macro_attribute]
pub fn dto(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = module_concat(attr, "request");
    let ident = require_ident_ends_with(&item, "Dto").replace("Dto", "Request");
    let mut base = token!(&format!(r#"#[ts(rename = "{ident}")]"#));
    base.extend(item);

    ts(attr, base)
}

// When writing to the same file across different crates in the workflow, it gets overwritten, so improvie-command and improvie-app should write to different files
#[proc_macro_attribute]
pub fn command(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = module_concat(attr, "command");
    ts(attr, item)
}

#[proc_macro_attribute]
pub fn response(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = module_concat(attr, "response");
    require_ident_ends_with(&item, "Response");
    ts(attr, item)
}
