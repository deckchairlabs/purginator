use parcel_css::stylesheet::StyleSheet;
use purger::Purger;
pub mod html;
pub mod purger;

pub fn purge(mut stylesheet: StyleSheet, purgers: Vec<Box<dyn Purger>>) -> StyleSheet {
    let mut rules = Vec::new();

    for purger_impl in purgers.iter() {
        rules = purger_impl.purge_css_rules(&mut stylesheet.rules);
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
                ..Default::default()
            })
            .unwrap();

        assert_eq!(purged_css.code, expected_output);
    }

    #[test]
    pub fn it_works() {
        let html_source = r#"
            <div>
                Hello World!
            </div>
        "#;

        let css_source = "
            .foo {
                color: red;
            }

            @media (min-width: 400px) {
                .bar {
                    color: blue;
                }
            }
        ";

        let html_purger = Box::new(PurgeFromHtml::new(html_source)) as Box<dyn Purger>;
        let purgers = vec![html_purger];

        let expected_output = "\n";

        purge_test(purgers, css_source, expected_output);
    }
}
