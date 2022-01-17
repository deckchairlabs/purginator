use parcel_css::stylesheet::StyleSheet;
use purger::Purger;
pub mod html;
pub mod purger;

pub fn purge(mut stylesheet: StyleSheet, purgers: Vec<Box<dyn Purger>>) -> StyleSheet {
    let mut rules = Vec::new();

    for purger_impl in purgers.iter() {
        rules = purger_impl.purge_css_rules(&mut stylesheet.rules, None);
    }

    stylesheet.rules.0 = rules;
    stylesheet
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::PurgeFromHtml;
    use crate::purger::Purger;
    use parcel_css::{
        stylesheet::{ParserOptions, PrinterOptions},
        targets::Browsers,
    };

    fn purge_test(purgers: Vec<Box<dyn Purger>>, css_source: &str, expected_output: &str) {
        let stylesheet = StyleSheet::parse(
            "test.css".into(),
            css_source,
            ParserOptions {
                nesting: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();

        let purged_stylesheet = purge(stylesheet, purgers);
        let purged_css = purged_stylesheet
            .to_css(PrinterOptions {
                targets: Some(Browsers::default()),
                ..Default::default()
            })
            .unwrap();

        assert_eq!(purged_css.code, expected_output);
    }

    fn create_html_purger(html_source: &str) -> std::boxed::Box<dyn purger::Purger> {
        Box::new(PurgeFromHtml::new(html_source)) as Box<dyn Purger>
    }

    #[test]
    pub fn it_can_purge_simple_style_rules() {
        let html_source = r#"
            <div>
                Hello World!
            </div>
        "#;

        let css_source = "
            .foo {
                color: red;
            }
        ";

        let html_purger = create_html_purger(html_source);
        let purgers = vec![html_purger];

        let expected_output = "\n";

        purge_test(purgers, css_source, expected_output);
    }

    #[test]
    pub fn it_handles_pseudo_selectors() {
        let html_source = r#"
            <a href="\#">
                Hello World!
            </a>
        "#;

        let css_source = "
            a:hover {
                color: red;
            }
        ";

        let html_purger = create_html_purger(html_source);
        let purgers = vec![html_purger];

        let expected_output = "\n";

        purge_test(purgers, css_source, expected_output);
    }

    #[test]
    pub fn it_can_purge_simple_media_rules() {
        let html_source = r#"
            <div>
                Hello World!
            </div>
        "#;

        let css_source = "
            @media (min-width: 400px) {
                .foo {
                    color: red;
                }
            }
        ";

        let html_purger = create_html_purger(html_source);
        let purgers = vec![html_purger];

        let expected_output = "\n";

        purge_test(purgers, css_source, expected_output);
    }

    #[test]
    pub fn it_can_purge_simple_supports_rule() {
        let html_source = r#"
            <div>Hello World</div>
        "#;

        let css_source = "
            @supports (display: grid) {
                .foo {
                    display: grid;
                }
            }
        ";

        let html_purger = create_html_purger(html_source);
        let purgers = vec![html_purger];

        let expected_output = "\n";

        purge_test(purgers, css_source, expected_output);
    }

    #[test]
    pub fn it_can_purge_nested_rule() {
        let html_source = r#"
            <div>Hello World</div>
        "#;

        let css_source = "
            .nesting {
                & .nested {
                    color: blue;
                }
            }
        ";

        let html_purger = create_html_purger(html_source);
        let purgers = vec![html_purger];

        let expected_output = "\n";

        purge_test(purgers, css_source, expected_output);
    }

    #[test]
    pub fn it_can_purge_deeply_nested_rule() {
        let html_source = r#"
            <div>Hello World</div>
        "#;

        let css_source = "
            .nesting {
                & .nested {
                    & .nested-deeper {
                        color: blue;
                    }
                }
            }
        ";

        let html_purger = create_html_purger(html_source);
        let purgers = vec![html_purger];

        let expected_output = "\n";

        purge_test(purgers, css_source, expected_output);
    }
}
