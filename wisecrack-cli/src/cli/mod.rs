use clap::{ArgGroup, Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(group(
    ArgGroup::new("action")
        .args(&["quote", "joke"])
        .required(true)
))]
pub struct WiseCrackCli {
    /// Show quote of the day
    #[arg(short, long)]
    pub quote: bool,
    /// Show joke of the day
    #[arg(short, long)]
    pub joke: bool,
    /// Optional output <text|json> defaults to text
    #[arg(short, long)]
    pub output: Option<OutputFormat>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Text,
    Json,
}
