use clap::Parser;
use miette::IntoDiagnostic;
use parcel_css::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use purginator::html::PurgeFromHtml;
use purginator::purge;
use purginator::purger::Purger;
use tokio::fs;

/// Purge css with speed
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to html file
    #[clap(short, long)]
    html: String,

    /// Path to css file
    #[clap(short, long)]
    css: String,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = Args::parse();

    let html_source = fs::read_to_string(args.html).await.into_diagnostic()?;
    let css_source = fs::read_to_string(args.css).await.into_diagnostic()?;

    let stylesheet = StyleSheet::parse(
        String::from("styles.css"),
        &css_source,
        ParserOptions {
            nesting: true,
            css_modules: false,
        },
    )
    .unwrap();

    let html_purger = Box::new(PurgeFromHtml::new(&html_source)) as Box<dyn Purger>;
    let purgers = vec![html_purger];

    let stylesheet = purge(stylesheet, purgers);

    let purged_css = stylesheet
        .to_css(PrinterOptions {
            minify: false,
            ..Default::default()
        })
        .map_err(|err| miette::Error::msg(err.reason()))?;

    println!("{}", purged_css.code);

    Ok(())
}
