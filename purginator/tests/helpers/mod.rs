use parcel_css::stylesheet::PrinterOptions;
use purginator::{
    purge,
    purger::{html::PurgeFromHtml, traits::Purger},
    stylesheet::parse,
};
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

    let html_purger = PurgeFromHtml::new(&html_source);
    let purgers: [&dyn Purger; 1] = [&html_purger];

    let stylesheet = parse(&css_source);
    let purged_stylesheet = purge(stylesheet, &purgers);
    let purged_css = purged_stylesheet
        .to_css(PrinterOptions {
            ..PrinterOptions::default()
        })
        .unwrap();

    purged_css.code
}
