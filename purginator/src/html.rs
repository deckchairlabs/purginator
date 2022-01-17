use cssparser::ToCss;
use parcel_css::rules::style::StyleRule;
use scraper::{Html, Selector};

use crate::purger::Purger;

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
        let mut selector_string = String::new();
        let selectors = style.selectors.0.iter().rev();

        for component in selectors.rev() {
            component.to_css(&mut selector_string).unwrap();
        }

        let selector = Selector::parse(&selector_string).unwrap();
        let elements = self.document.select(&selector);

        elements.count() == 0
    }
}
