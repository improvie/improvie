#![allow(unnameable_test_items, dead_code)]

// This test is used to check if the `bind` macro compiles correctly
// but dose not check if the generated code is correct.

#[test]
fn export() {
    #[bind::ts]
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
    #[bind::ts("playlist.ts")]
    struct Test {
        a: i32,
        b: String,
    }

    let _ = Test {
        a: 1,
        b: "test".to_string(),
    };
}
