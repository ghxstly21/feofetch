use std::process::exit;
use clap::{Parser, ValueEnum};
use owo_colors::OwoColorize;
use owo_colors::colors::*;

#[derive(Clone, Debug, ValueEnum)]
enum Color {
    // normal colors
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    // bright colors
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    // chooses the color based on current config
    Auto,
}

/// Shows system information
#[derive(Parser, Debug)]
#[command(version, about, long_about = "Showing your system information... but made in Rust")]
struct Args {
    /// The color to use for ascii art
    #[arg(short, long, default_value = "auto")]
    color: Color,

    /// List of information to hide
    #[arg(long)]
    hide: Vec<String>,
}
fn main() {
    let args = Args::parse();
    let supported_platforms = ["macos", "windows", "linux"];
}