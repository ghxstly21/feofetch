mod settings;

use clap::{Parser, ValueEnum, Subcommand};
use owo_colors::colors::*;

#[derive(Clone, Debug, Subcommand, ValueEnum)]
enum Command {
    Settings,
    PrintSettings
}

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
#[command(name = "feofetch", version, about, long_about = "Showing your system information... but made in Rust")]
struct Args {
    /// The color to use for ascii art
    #[arg(short, long, default_value = "auto")]
    color: Color,

    /// List of information to hide
    #[arg(long, num_args = 1..)]
    hide: Vec<settings::Setting>,

    #[command(subcommand)]
    command: Option<Command>
}
fn main() {
    let args = Args::parse();
    match args.command {
        Some(command) => {
            match command {
                Command::Settings => {
                    settings::edit_settings();
                    return;
                },
                Command::PrintSettings => {
                    settings::print_settings();
                    return;
                },
            }
        }
        None => {}
    }
}