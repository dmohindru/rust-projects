use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct WiseCrackCli {
    #[command(subcommand)]
    pub command: Commands,
    /// Optional output <text|json> defaults to text
    #[arg(short, long)]
    pub output: Option<OutputFormat>,
    /// Optional config file
    #[arg(short, long)]
    pub config: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get Quote of the day
    Quote(QuoteOfDayArgs),
    /// Get Dad joke
    Joke(DadJokeArgs),
}

#[derive(Args, Debug)]
pub struct DadJokeArgs {
    #[arg(short, long)]
    pub joke: bool,
}

#[derive(Args, Debug)]
pub struct QuoteOfDayArgs {
    #[arg(short, long)]
    pub quote: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Text,
    Json,
}
