use parcel_css::rules::{
    document::MozDocumentRule, media::MediaRule, nesting::NestingRule, style::StyleRule,
    supports::SupportsRule, CssRule, CssRuleList,
};

pub trait Purger {
    fn should_purge_style(&self, style: &mut StyleRule) -> bool {
        let has_own_declarations = !style.declarations.declarations.is_empty()
            || !style.declarations.important_declarations.is_empty();

        if has_own_declarations {
            false
        } else {
            style.rules.0.is_empty()
        }
    }

    fn should_purge_media(&self, media: &mut MediaRule) -> bool {
        media.rules.0.is_empty()
    }

    fn should_purge_supports(&self, supports: &mut SupportsRule) -> bool {
        supports.rules.0.is_empty()
    }

    fn should_purge_nesting(&self, nesting: &mut NestingRule) -> bool {
        nesting.style.rules.0.is_empty()
    }

    fn should_purge_document(&self, document: &mut MozDocumentRule) -> bool {
        document.rules.0.is_empty()
    }

    fn purge_css_rules(&self, css_rule_list: &mut CssRuleList) -> Vec<CssRule> {
        let mut rules = Vec::new();
        for mut rule in css_rule_list.0.drain(..) {
            match &mut rule {
                CssRule::Style(style) => {
                    if self.should_purge_style(style) {
                        continue;
                    }

                    style.rules.0 = self.purge_css_rules(&mut style.rules);
                }
                CssRule::Media(media) => {
                    media.rules.0 = self.purge_css_rules(&mut media.rules);

                    if self.should_purge_media(media) {
                        continue;
                    }
                }
                CssRule::Supports(supports) => {
                    supports.rules.0 = self.purge_css_rules(&mut supports.rules);

                    if self.should_purge_supports(supports) {
                        continue;
                    }
                }
                CssRule::Nesting(nesting) => {
                    nesting.style.rules.0 = self.purge_css_rules(&mut nesting.style.rules);

                    if self.should_purge_nesting(nesting) {
                        continue;
                    }
                }
                CssRule::MozDocument(document) => {
                    document.rules.0 = self.purge_css_rules(&mut document.rules);

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
