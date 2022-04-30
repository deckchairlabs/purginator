use purginator::purge;
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

    let result = purge(css_source, html_source);

    println!("{}", result);
    result
}
