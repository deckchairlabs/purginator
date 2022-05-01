use purginator::purge;
use std::str;
use std::{env, fs};

pub fn run_test(test_name: &str) -> String {
    let current_dir = env::current_dir().unwrap();

    let html_source = fs::read_to_string(format!(
        "{}/tests/fixtures/{}.html",
        current_dir.to_str().unwrap(),
        test_name
    ))
    .unwrap();

    let css_source = fs::read_to_string(format!(
        "{}/tests/fixtures/{}.css",
        current_dir.to_str().unwrap(),
        test_name
    ))
    .unwrap();

    let result: &[u8] = &purge(css_source.as_bytes(), html_source.as_bytes());

    let s = match str::from_utf8(result) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    s.to_owned()
}
