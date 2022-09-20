use lightningcss::stylesheet::{ParserOptions, StyleSheet};

pub fn parse(css_source: &str) -> StyleSheet {
  StyleSheet::parse(
    css_source,
    ParserOptions {
      filename: "style.css".to_string(),
      nesting: true,
      custom_media: true,
      ..ParserOptions::default()
    },
  )
  .unwrap()
}
