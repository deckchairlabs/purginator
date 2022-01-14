use cssparser::ToCss;
use parcel_css::rules::CssRule::Style;
use parcel_css::stylesheet::{ParserOptions, StyleSheet};
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en>
            <head>
                <meta charset="UTF-8">
                <title>Test</title>
            </head>
            <body>
                <div class="foo">
                    This is foo
                </div>
                
                <h1 class="undefined">
                    This is undefined
                </h1>
                
                <section id="main" class="multiple selectors">
                    This has multiple selectors!
                </section>
                
                <ul>
                    <li class="list-item">Nested</li>
                </ul>
                
                <div>Broken html</>
            </body>
        </html>
    "#
    .to_string();

    let css = "
        .foo {
            color: red;
        }

        .bar {
            color: green;
        }

        .multiple {
            color: maroon;
        }

        .multiple.selectors {
            color: purple;
        }

        ul .list-item {
            color: blue;
        }
    "
    .to_string();

    let parsed = StyleSheet::parse(
        "styles.css".to_string(),
        &css,
        ParserOptions {
            nesting: false,
            css_modules: false,
        },
    );

    if let Ok(stylesheet) = parsed {
        let document: Html = Html::parse_document(&html);
        let select_all: Selector = Selector::parse("*:not(head)").unwrap();

        for (index, css_rule) in stylesheet.rules.0.iter().enumerate() {
            let mut matched = false;

            if let Style(rule) = css_rule {
                let all_elements = document.select(&select_all);

                for element_ref in all_elements {
                    let mut selector_string = String::new();
                    let iter = rule.selectors.0.iter().clone().rev();

                    for component in iter {
                        component.to_css(&mut selector_string)?
                    }

                    let selector = Selector::parse(&selector_string).unwrap();

                    matched = selector.matches(&element_ref);

                    if matched {
                        break;
                    }
                }
            }

            if !matched {
                // How can we remove the rule at this index
                // stylesheet.rules.0.remove(index);
                dbg!(index);
            }
        }
    } else {
        panic!("Parsing failed")
    }

    Ok(())
}
