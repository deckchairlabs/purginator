use crate::purger::Purger;
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
    fn should_purge_selector(&self, selector: &Selector) -> bool {
        let elements = self.document.select(selector);
        dbg!(selector);

        elements.count() == 0
    }
}
