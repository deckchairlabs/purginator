use parcel_css::rules::{CssRule, CssRuleList};
use parcel_css::stylesheet::{PrinterOptions, StyleSheet};
use scraper::{Html, Selector};
use stylesheet::parse;
use wasm_bindgen::prelude::*;
pub mod stylesheet;

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

    pub fn purge(mut self) -> StyleSheet<'a> {
        self.stylesheet.rules = Self::retain_used_rules(self.stylesheet.rules.0, &self.document);

        StyleSheet::new(
            self.stylesheet.sources,
            self.stylesheet.rules,
            Default::default(),
        )
    }

    fn retain_used_rules(rules: Vec<CssRule<'a>>, document: &Html) -> CssRuleList<'a> {
        let mut new_rules = Vec::new();

        for mut rule in rules {
            if Self::should_retain_rule(&mut rule, document) {
                new_rules.push(rule);
            }
        }

        CssRuleList(new_rules)
    }

    fn should_retain_rule(rule: &mut CssRule<'a>, document: &Html) -> bool {
        match rule.to_owned() {
            CssRule::Style(style) => {
                let selector = style.selectors.to_string();
                let result = Selector::parse(&selector);

                let selector_is_retainable = match result {
                    Ok(result) => {
                        let elements = document.select(&result);
                        let matched_elements_count = elements.count();
                        matched_elements_count > 0
                    }
                    Err(_) => true,
                };

                selector_is_retainable
            }
            CssRule::Media(mut media) => {
                media.rules = Self::retain_used_rules(media.rules.0, document);
                !media.rules.0.is_empty()
            }
            CssRule::Supports(mut supports) => {
                supports.rules = Self::retain_used_rules(supports.rules.0, document);
                !supports.rules.0.is_empty()
            }
            CssRule::Nesting(mut nesting) => {
                nesting.style.rules = Self::retain_used_rules(nesting.style.rules.0, document);
                !nesting.style.rules.0.is_empty()
            }
            CssRule::MozDocument(mut document_rule) => {
                document_rule.rules = Self::retain_used_rules(document_rule.rules.0, document);
                !document_rule.rules.0.is_empty()
            }
            _ => false,
        }
    }
}

#[wasm_bindgen]
pub fn purge(css_bytes: &[u8], html_bytes: &[u8], minify: Option<bool>) -> Vec<u8> {
    let minify = minify.unwrap_or(false);

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
            minify,
            ..PrinterOptions::default()
        })
        .unwrap();

    purged_css.code.into_bytes()
}
