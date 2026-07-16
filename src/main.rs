mod settings;
mod errors;
mod fetch;

use crate::errors::FeoError;

use clap::{Parser, ValueEnum, Subcommand};
use std::process::ExitCode;

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

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(fe) => {
            eprintln!("{fe}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), FeoError> {
    let args = Args::parse();
    match args.command {
        Some(Command::Settings) => {
            settings::edit_settings()?
        }
        Some(Command::PrintSettings) => {
            settings::print_settings()?
        }
        None => {
            todo!()
        }
    }
    Ok(())
}