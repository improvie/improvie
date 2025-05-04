const FRONTEND_STATIC_DIR: &str = env!("FRONTEND_STATIC_DIR");

fn main() {
    let theme_file_path = format!("{}/default_theme.json", FRONTEND_STATIC_DIR);
    serde_json::to_writer(
        std::fs::File::create(theme_file_path).unwrap(),
        &improvie_builtin::themes::black_theme(),
    )
    .unwrap();
}
