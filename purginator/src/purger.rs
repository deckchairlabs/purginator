use cssparser::ToCss;
use parcel_css::{
    rules::{
        document::MozDocumentRule, media::MediaRule, nesting::NestingRule, style::StyleRule,
        supports::SupportsRule, CssRule, CssRuleList,
    },
    stylesheet::StyleSheet,
};
use scraper::Selector;

pub trait Purger {
    fn should_purge_selector(&self, selector: &Selector) -> bool;

    fn should_purge_style(&self, style: &mut StyleRule) -> bool {
        for selector in style.selectors.0.drain(..) {
            let parsed_selector = Selector::parse(&selector.to_css_string()).unwrap();
            if self.should_purge_selector(&parsed_selector) {
                continue;
            }
        }

        style.selectors.0.is_empty()
            && style.rules.0.is_empty()
            && style.declarations.declarations.is_empty()
            && style.declarations.important_declarations.is_empty()
    }

    fn should_purge_media(&self, media: &MediaRule) -> bool {
        media.rules.0.is_empty()
    }

    fn should_purge_supports(&self, supports: &SupportsRule) -> bool {
        supports.rules.0.is_empty()
    }

    fn should_purge_nesting(&self, nesting: &NestingRule) -> bool {
        nesting.style.rules.0.is_empty()
    }

    fn should_purge_document(&self, document: &MozDocumentRule) -> bool {
        document.rules.0.is_empty()
    }

    fn should_purge_rule(&self, rule: &mut CssRule) -> bool {
        match rule {
            CssRule::Style(style) => {
                self.purge_css_rules(&mut style.rules);
                self.should_purge_style(style)
            }
            CssRule::Media(media) => {
                self.purge_css_rules(&mut media.rules);
                self.should_purge_media(media)
            }
            CssRule::Supports(supports) => {
                self.purge_css_rules(&mut supports.rules);
                self.should_purge_supports(supports)
            }
            CssRule::Nesting(nesting) => {
                self.purge_css_rules(&mut nesting.style.rules);
                self.should_purge_nesting(nesting)
            }
            CssRule::MozDocument(document) => {
                self.purge_css_rules(&mut document.rules);
                self.should_purge_document(document)
            }
            _ => false,
        }
    }

    fn purge_css_rules(&self, rules: &mut CssRuleList) {
        for mut rule in rules.0.drain(..) {
            if self.should_purge_rule(&mut rule) {
                continue;
            }
        }
    }

    fn purge(&self, stylesheet: &mut StyleSheet) {
        self.purge_css_rules(&mut stylesheet.rules);
    }
}
