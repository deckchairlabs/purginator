use clap::Parser;
use miette::IntoDiagnostic;
use parcel_css::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use purginator::purge;
use purginator::purger::html::PurgeFromHtml;
use purginator::purger::traits::Purger;
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

    /// Path to css file
    #[clap(short, long)]
    minify: bool,
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

    let html_purger = PurgeFromHtml::new(&html_source);
    let purgers: [&dyn Purger; 1] = [&html_purger];

    let mut stylesheet = purge(stylesheet, &purgers);

    if args.minify {
        stylesheet.minify(MinifyOptions {
            ..Default::default()
        });
    }

    let purged_css = stylesheet
        .to_css(PrinterOptions {
            minify: args.minify,
            ..Default::default()
        })
        .map_err(|err| miette::Error::msg(err.reason()))?;

    println!("{}", purged_css.code);

    Ok(())
}
