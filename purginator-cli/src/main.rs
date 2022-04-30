use clap::Parser;
use miette::IntoDiagnostic;
use purginator::purge;
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

    let result = purge(css_source, html_source);

    println!("{}", result);

    Ok(())
}
