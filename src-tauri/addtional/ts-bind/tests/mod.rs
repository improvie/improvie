#![allow(unnameable_test_items, dead_code)]

#[test]
fn export() {
    #[ts_bind::ts]
    struct Test {
        a: i32,
        b: String,
    }

    let _ = Test {
        a: 1,
        b: "test".to_string(),
    };
}

#[test]
fn export_to() {
    #[ts_bind::ts("playlist.rs")]
    struct Test {
        a: i32,
        b: String,
    }

    let _ = Test {
        a: 1,
        b: "test".to_string(),
    };
}
