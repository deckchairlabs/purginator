use cssparser::ToCss;
use parcel_css::{
    rules::CssRule,
    stylesheet::{ParserOptions, StyleSheet},
};
use scraper::{Html, Selector};

pub use crate::error::Error;

mod error;

pub fn purge(html: &str, css: &str) -> Result<StyleSheet, Error> {
    let mut stylesheet = StyleSheet::parse(
        "styles.css".to_string(),
        css,
        ParserOptions {
            nesting: false,
            css_modules: false,
        },
    )
    .map_err(|_| Error::ParseError)?;

    let document = Html::parse_document(html);
    let select_all: Selector = Selector::parse("html, body, body *").unwrap();

    stylesheet.rules.0.retain(|css_rule| match css_rule {
        CssRule::Style(rule) => {
            let mut all_elements = document.select(&select_all);

            all_elements.any(|element| {
                let mut selector_string = String::new();
                let selectors = rule.selectors.0.iter().rev();

                for component in selectors {
                    component.to_css(&mut selector_string).unwrap();
                }

                let selector = Selector::parse(&selector_string).unwrap();

                selector.matches(&element)
            })
        }
        _ => true,
    });

    Ok(stylesheet)
}

#[cfg(test)]
mod tests {
    use crate::{purge, Error};

    #[test]
    fn it_works() -> Result<(), Error> {
        let html = r#"
            <!DOCTYPE html>
            <html lang="en" class="dark-theme">
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
        "#;

        let css = "
            .dark-theme body {
                background-color: #333;
            }
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
        ";

        let stylesheet = purge(html, css)?;
        // TODO: test stylesheet result

        Ok(())
    }
}
