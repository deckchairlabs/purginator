use cssparser::ToCss;
use parcel_css::{
    rules::{
        document::MozDocumentRule, media::MediaRule, nesting::NestingRule, style::StyleRule,
        supports::SupportsRule, CssRule, CssRuleList,
    },
    stylesheet::StyleSheet,
};

pub trait Purger {
    fn should_purge_selector(&self, selector: &str) -> bool;

    fn should_purge_style(&self, style: &mut StyleRule) -> bool {
        style.selectors.0.retain(|selector| {
            let selector_string = selector.to_css_string();
            !self.should_purge_selector(&selector_string)
        });

        style.selectors.0.is_empty() && style.rules.0.is_empty()
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
                style.rules = self.purge_css_rules(&mut style.rules);
                self.should_purge_style(style)
            }
            CssRule::Media(media) => {
                media.rules = self.purge_css_rules(&mut media.rules);
                self.should_purge_media(media)
            }
            CssRule::Supports(supports) => {
                supports.rules = self.purge_css_rules(&mut supports.rules);
                self.should_purge_supports(supports)
            }
            CssRule::Nesting(nesting) => {
                nesting.style.rules = self.purge_css_rules(&mut nesting.style.rules);
                self.should_purge_nesting(nesting)
            }
            CssRule::MozDocument(document) => {
                document.rules = self.purge_css_rules(&mut document.rules);
                self.should_purge_document(document)
            }
            _ => false,
        }
    }

    fn purge_css_rules(&self, rules: &mut CssRuleList) -> CssRuleList {
        let mut new_rules = Vec::new();
        for mut rule in rules.0.drain(..) {
            if !self.should_purge_rule(&mut rule) {
                new_rules.push(rule)
            }
        }

        CssRuleList(new_rules)
    }

    fn purge<'a>(
        &self,
        stylesheet: &'a mut StyleSheet,
    ) -> &'a mut parcel_css::stylesheet::StyleSheet {
        stylesheet.rules = self.purge_css_rules(&mut stylesheet.rules);
        stylesheet
    }
}
