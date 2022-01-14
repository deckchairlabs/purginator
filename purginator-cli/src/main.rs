use clap::Parser;
use miette::IntoDiagnostic;
use parcel_css::stylesheet::PrinterOptions;
use purginator::purge;
use tokio::fs;

/// Simple program to greet a person
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

    let html = fs::read_to_string(args.html).await.into_diagnostic()?;
    let css = fs::read_to_string(args.css).await.into_diagnostic()?;

    let stylesheet = purge(&html, &css).into_diagnostic()?;

    let purged_css = stylesheet
        .to_css(PrinterOptions {
            minify: false,
            ..Default::default()
        })
        .map_err(|err| miette::Error::msg(err.reason()))?;

    println!("{}", purged_css.code);

    Ok(())
}
