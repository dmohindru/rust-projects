use clap::{Parser, ValueEnum};
use glyph_lib::AnimationMode;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(group(
    clap::ArgGroup::new("action")
        .required(true)
        .args(&["char", "string", "info"]),
))]
pub struct GlyphCli {
    /// Generate bitmap for a single character (requires `--grid-size` and `--file`).
    #[arg(short, long, group = "action", requires_all = ["grid_size", "file"])]
    pub char: Option<char>,

    /// Generate bitmap for a string (requires `--mode`, `--grid-size` and `--file`).
    #[arg(short, long, group = "action", requires_all = ["mode", "grid_size", "file"])]
    pub string: Option<String>,

    /// Mode to render strings: `scroll` or `next`.
    #[arg(short, long, value_enum)]
    pub mode: Option<CliAnimationMode>,

    /// Print information about an existing glyph binary file (requires `--grid-size` and `--file`).
    #[arg(short, long, group = "action", requires_all = ["grid_size", "file"])]
    pub info: bool,

    /// Square grid size (e.g., 5, 8). Present only when required by the action.
    #[arg(short = 'g', long, value_name = "N")]
    pub grid_size: u8,

    /// File to read from or write to. Present only when required by the action.
    #[arg(short, long, value_name = "FILE")]
    pub file: String,
}

#[derive(ValueEnum, Clone)]
pub enum CliAnimationMode {
    Next,
    Scroll,
}

impl From<CliAnimationMode> for AnimationMode {
    fn from(value: CliAnimationMode) -> Self {
        match value {
            CliAnimationMode::Next => AnimationMode::Next,
            CliAnimationMode::Scroll => AnimationMode::Scroll,
        }
    }
}

// if let Some(mode) = cli.mode.map(Into::into) {
//     run_string_generation(mode);
// }
