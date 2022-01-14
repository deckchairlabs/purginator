use cssparser::ToCss;
use parcel_css::rules::CssRule::{self, Style};
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
                <h1 class="unused">
                    This is unused
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
        "style.css".to_string(),
        &css,
        ParserOptions {
            nesting: true,
            css_modules: false,
        },
    );

    let mut rules: Vec<CssRule> = Vec::new();

    if let Ok(mut stylesheet) = parsed {
        rules.append(&mut stylesheet.rules.0);
    } else {
        panic!("Parsing failed")
    }

    let document: Html = Html::parse_document(&html);
    let select_all: Selector = Selector::parse("*").unwrap();

    let all_elements = document.select(&select_all);

    for element_ref in all_elements {
        for rule in rules.iter() {
            if let Style(rule) = rule {
                let element_name = element_ref.value().name();

                let mut selector_string = String::new();
                let iter = rule.selectors.0.iter().clone().rev();

                for component in iter {
                    component.to_css(&mut selector_string)?
                }

                let selector = Selector::parse(&selector_string).unwrap();

                if selector.matches(&element_ref) {
                    println!("{} Matches! {}", element_name, selector_string);
                } else {
                    println!("{} Doesn't match! {}", element_name, selector_string);
                }
            }
        }
    }

    Ok(())
}
