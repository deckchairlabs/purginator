use parcel_css::rules::{
    document::MozDocumentRule, media::MediaRule, nesting::NestingRule, style::StyleRule,
    supports::SupportsRule, CssRule, CssRuleList,
};

pub trait Purger {
    fn should_purge_style(&self, style: &StyleRule) -> bool;

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

    fn purge_css_rules(
        &self,
        css_rule_list: &mut CssRuleList,
        _context: Option<&mut StyleRule>,
    ) -> Vec<CssRule> {
        let mut rules = Vec::new();
        for mut rule in css_rule_list.0.drain(..) {
            match &mut rule {
                CssRule::Style(style) => {
                    if !style.rules.0.is_empty() {
                        style.rules.0 = self.purge_css_rules(&mut style.rules, None);
                    }

                    if self.should_purge_style(style) {
                        continue;
                    }
                }
                CssRule::Media(media) => {
                    media.rules.0 = self.purge_css_rules(&mut media.rules, None);

                    if self.should_purge_media(media) {
                        continue;
                    }
                }
                CssRule::Supports(supports) => {
                    supports.rules.0 = self.purge_css_rules(&mut supports.rules, None);

                    if self.should_purge_supports(supports) {
                        continue;
                    }
                }
                // CssRule::Nesting(nesting) => {
                //     nesting.style.rules.0 = self.purge_css_rules(&mut nesting.style.rules, None);

                //     if self.should_purge_nesting(nesting) {
                //         continue;
                //     }
                // }
                CssRule::MozDocument(document) => {
                    document.rules.0 = self.purge_css_rules(&mut document.rules, None);

                    if self.should_purge_document(document) {
                        continue;
                    }
                }
                _ => {}
            }

            rules.push(rule)
        }

        rules
    }
}
