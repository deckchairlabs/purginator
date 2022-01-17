use crate::purger::Purger;
use cssparser::ToCss;
use parcel_css::rules::style::StyleRule;
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
        let selector_string = style.selectors.to_css_string();

        // TODO: If we have nested rules, we should match from the deepest level up?
        if !style.rules.0.is_empty() {
            dbg!(&style);
            false
        } else {
            dbg!(&selector_string);
            let selector = Selector::parse(&selector_string).unwrap();
            let elements = self.document.select(&selector);

            elements.count() == 0
        }
    }
}
