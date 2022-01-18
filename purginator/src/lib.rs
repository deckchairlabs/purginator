use parcel_css::stylesheet::StyleSheet;
use purger::Purger;
pub mod html;
pub mod purger;

pub fn purge(mut stylesheet: StyleSheet, purgers: &[&dyn Purger]) -> StyleSheet {
    let mut rules = Vec::new();

    for purger_impl in purgers.iter() {
        rules.extend(purger_impl.purge_css_rules(&mut stylesheet.rules, None));
    }

    stylesheet.rules.0 = rules;
    stylesheet
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::PurgeFromHtml;
    use crate::purger::Purger;
    use parcel_css::stylesheet::{ParserOptions, PrinterOptions};

    fn purge_test(purgers: &[&dyn Purger], css_source: &str, expected_output: &str) {
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
                ..Default::default()
            })
            .unwrap();

        assert_eq!(purged_css.code, expected_output);
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

        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        let expected_output = "\n";

        purge_test(&purgers, css_source, expected_output);
    }

    #[test]
    pub fn it_can_purge_multiple_style_rules() {
        let html_source = r#"
            <div class="bar">
                Hello World!
            </div>
        "#;

        let css_source = "
            .foo, .bar {
                color: red;
            }
        ";

        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        let expected_output = ".bar {\n  color: red;\n}\n";

        purge_test(&purgers, css_source, expected_output);
    }

    #[test]
    pub fn it_can_handle_pseudo_selectors() {
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

        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        let expected_output = "\n";

        purge_test(&purgers, css_source, expected_output);
    }

    // https://drafts.csswg.org/selectors/#the-root-pseudo
    #[test]
    pub fn it_can_handle_root_pseudos() {
        let html_source = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>Hello World!</body>
            </html>
        "#;

        let css_source = "
            :root {
                color: red;
            }
        ";

        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        let expected_output = ":root {\n  color: red;\n}\n";

        purge_test(&purgers, css_source, expected_output);
    }

    // https://drafts.csswg.org/selectors/#the-empty-pseudo
    #[test]
    pub fn it_can_handle_empty_pseudos() {
        let html_source = r#"
            <div>Hello World!</div>
        "#;

        let css_source = "
            div:empty {
                color: red;
            }
            div:not(:empty) {
                color: blue;
            }
        ";

        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        let expected_output = "div:not(:empty) {\n  color: #00f;\n}\n";

        purge_test(&purgers, css_source, expected_output);
    }

    // https://drafts.csswg.org/selectors/#the-empty-pseudo
    #[test]
    pub fn it_can_handle_nth_child_pseudos() {
        let html_source = r#"
            <div>Hello World!</div>
        "#;

        let css_source = "
            div:nth-child(even) {
                color: red;
            }

            div:nth-child(odd) {
                color:blue;
            }
        ";

        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        let expected_output = "div:nth-child(2n+1) {\n  color: #00f;\n}\n";

        purge_test(&purgers, css_source, expected_output);
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

        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        let expected_output = "\n";

        purge_test(&purgers, css_source, expected_output);
    }

    #[test]
    pub fn it_can_purge_tailwind_style_rules() {
        let html_source = r#"
            <div>
                Hello World!
            </div>
        "#;

        let css_source = "
            .dark\\:bg-red {
                color: red;
            }
        ";

        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        let expected_output = "\n";

        purge_test(&purgers, css_source, expected_output);
    }

    #[test]
    pub fn it_can_purge_nest_rules() {
        let html_source = r#"
            <div>
                Hello World!
            </div>
        "#;

        let css_source = "
            .foo {
                color: red;
                @nest .parent & {
                    color: blue;
                }
            }
        ";

        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        let expected_output = "\n";

        purge_test(&purgers, css_source, expected_output);
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

        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        let expected_output = "\n";

        purge_test(&purgers, css_source, expected_output);
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

        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        let expected_output = "\n";

        purge_test(&purgers, css_source, expected_output);
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

        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        let expected_output = "\n";

        purge_test(&purgers, css_source, expected_output);
    }
}
