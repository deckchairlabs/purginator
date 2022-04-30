use cssparser::ToCss;
use parcel_css::rules::{CssRule, CssRuleList};
use parcel_css::stylesheet::{PrinterOptions, StyleSheet};
use scraper::{Html, Selector};
use stylesheet::parse;
use wasm_bindgen::prelude::*;
pub mod stylesheet;

trait Purge<'a> {
    fn purge(self, stylesheet: &PurgeableStyleSheet<'a>) -> Self;
}

trait ShouldPurge<'a, F>
where
    F: Fn(&String) -> bool,
{
    fn should_purge(&mut self, predicate: F) -> bool;
}

struct PurgeableStyleSheet<'a> {
    pub stylesheet: StyleSheet<'a>,
    document: Html,
}

impl<'a> PurgeableStyleSheet<'a> {
    pub fn new(stylesheet: StyleSheet<'a>, html_source: &str) -> Self {
        let document = Html::parse_document(html_source);
        Self {
            stylesheet,
            document,
        }
    }

    pub fn purge(self) -> StyleSheet<'a> {
        let predicate = |selector_string: &String| -> bool {
            let selector = Selector::parse(selector_string).unwrap();
            let elements = self.document.select(&selector);
            elements.count() == 0
        };

        let rules = self.stylesheet.rules.clone();
        let mut new_rules = Vec::new();

        for mut rule in rules.0 {
            if !rule.should_purge(predicate) {
                new_rules.push(rule.clone());
            }
        }

        StyleSheet::new(
            self.stylesheet.sources,
            CssRuleList(new_rules),
            Default::default(),
        )
    }
}

impl<'a, F> ShouldPurge<'a, F> for CssRule<'a>
where
    F: Fn(&String) -> bool,
{
    fn should_purge(&mut self, predicate: F) -> bool {
        match self {
            CssRule::Style(style) => {
                style.selectors.0.retain(|selector| {
                    let selector_string = selector.to_css_string();
                    !predicate(&selector_string)
                });

                style.selectors.0.is_empty()
                    || style.is_empty()
                    || style.declarations.declarations.is_empty()
                        && style.declarations.important_declarations.is_empty()
            }
            CssRule::Media(media) => media.rules.0.is_empty(),
            CssRule::Supports(supports) => supports.rules.0.is_empty(),
            CssRule::Nesting(nesting) => nesting.style.rules.0.is_empty(),
            CssRule::MozDocument(document) => document.rules.0.is_empty(),
            _ => false,
        }
    }
}

#[wasm_bindgen]
pub fn purge(css_bytes: &[u8], html_bytes: &[u8]) -> Vec<u8> {
    let css_source = match std::str::from_utf8(css_bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let html_source = match std::str::from_utf8(html_bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let stylesheet = parse(css_source);
    let transformed = stylesheet.to_css(Default::default()).unwrap();
    let transformed_source = parse(&transformed.code);

    let purgeable_stylesheet = PurgeableStyleSheet::new(transformed_source, html_source);
    let purged_stylesheet = purgeable_stylesheet.purge();

    let purged_css = purged_stylesheet
        .to_css(PrinterOptions {
            ..PrinterOptions::default()
        })
        .unwrap();

    purged_css.code.into_bytes()
}
