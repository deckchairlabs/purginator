use crate::purger::Purger;
use cssparser::ToCss;
use parcel_css::rules::{style::StyleRule, CssRule};
use scraper::{Html, Selector};

pub struct PurgeFromHtml {
    document: Html,
}

impl PurgeFromHtml {
    pub fn new(html_source: &str) -> Self {
        let document = Html::parse_document(html_source);
        Self { document }
    }
}

impl Purger for PurgeFromHtml {
    fn should_purge_style(&self, style: &mut StyleRule) -> bool {
        let parent_selector = style.selectors.to_css_string();

        // If we have nested rules, we should match those selectors instead
        if !style.rules.0.is_empty() {
            style.rules.0.iter().all(|rule| match rule {
                CssRule::Style(child) => {
                    let selector_string = child
                        .selectors
                        .to_css_string()
                        .replace("&", &parent_selector);

                    dbg!(&selector_string);

                    let selector = Selector::parse(&selector_string).unwrap();
                    let elements = self.document.select(&selector);

                    elements.count() == 0
                }
                _ => false,
            })
        } else {
            let selector = Selector::parse(&parent_selector).unwrap();
            let elements = self.document.select(&selector);

            elements.count() == 0
        }
    }
}
