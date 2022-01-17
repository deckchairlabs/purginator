use crate::purger::Purger;
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
        let selector_string = self.style_to_selector_string(style);

        dbg!(&selector_string);

        let selector = Selector::parse(&selector_string).unwrap();
        let elements = self.document.select(&selector);

        elements.count() == 0
    }
}
