fn main() {
    serde_json::to_writer(
        std::fs::File::create("../../src/static/default_theme.json").unwrap(),
        &improvie_builtin::themes::black_theme(),
    )
    .unwrap();
}
