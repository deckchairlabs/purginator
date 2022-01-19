use super::traits::Purger;
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
    fn should_purge_selector(&self, selector_string: &str) -> bool {
        let selector = Selector::parse(selector_string).unwrap();
        let elements = self.document.select(&selector);
        elements.count() == 0
    }
}
