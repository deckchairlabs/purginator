use parcel_css::stylesheet::{ParserOptions, StyleSheet};

pub fn parse(css_source: &str) -> StyleSheet {
    StyleSheet::parse(
        "style.css",
        css_source,
        ParserOptions {
            nesting: true,
            custom_media: true,
            ..ParserOptions::default()
        },
    )
    .unwrap()
}
