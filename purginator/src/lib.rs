use parcel_css::stylesheet::StyleSheet;
use purger::traits::Purger;
pub mod purger;
pub mod stylesheet;

pub fn purge(mut stylesheet: StyleSheet, purgers: &[&dyn Purger]) -> StyleSheet {
    for purger_impl in purgers.iter() {
        purger_impl.purge(&mut stylesheet);
    }

    stylesheet
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::purger::html::PurgeFromHtml;
    use crate::purger::traits::Purger;
    use crate::stylesheet::parse;
    use parcel_css::stylesheet::PrinterOptions;

    fn purge_test(purgers: &[&dyn Purger], css_source: &str, expected_output: &str) {
        let stylesheet = parse(css_source);
        let purged_stylesheet = purge(stylesheet, purgers);
        let purged_css = purged_stylesheet
            .to_css(PrinterOptions {
                ..PrinterOptions::default()
            })
            .unwrap();

        assert_eq!(purged_css.code, expected_output);
    }

    fn purge_from_html_test(html_source: &str, css_source: &str, expected_output: &str) {
        let html_purger = PurgeFromHtml::new(html_source);
        let purgers: [&dyn Purger; 1] = [&html_purger];

        purge_test(&purgers, css_source, expected_output);
    }

    #[test]
    pub fn it_can_purge_simple_style_rules() {
        let css_source = "
            .foo {
                color: red;
            }
        ";

        purge_from_html_test(
            r#"
            <div>
                Hello World!
            </div>
        "#,
            css_source,
            "\n",
        );

        purge_from_html_test(
            r#"
            <div class="foo">
                Hello World!
            </div>
        "#,
            css_source,
            ".foo {\n  color: red;\n}\n",
        );
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

        purge_from_html_test(html_source, css_source, ".bar {\n  color: red;\n}\n");
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

        purge_from_html_test(html_source, css_source, "\n");
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

        let expected_output = ":root {\n  color: red;\n}\n";

        purge_from_html_test(html_source, css_source, expected_output);
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

        let expected_output = "div:not(:empty) {\n  color: #00f;\n}\n";

        purge_from_html_test(html_source, css_source, expected_output);
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

        let expected_output = "div:nth-child(2n+1) {\n  color: #00f;\n}\n";

        purge_from_html_test(html_source, css_source, expected_output);
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

        let expected_output = "\n";

        purge_from_html_test(html_source, css_source, expected_output);
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

        let expected_output = "\n";

        purge_from_html_test(html_source, css_source, expected_output);
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

        let expected_output = "\n";

        purge_from_html_test(html_source, css_source, expected_output);
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

        let expected_output = "\n";

        purge_from_html_test(html_source, css_source, expected_output);
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

        let expected_output = "\n";

        purge_from_html_test(html_source, css_source, expected_output);
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

        let expected_output = "\n";

        purge_from_html_test(html_source, css_source, expected_output);
    }
}
