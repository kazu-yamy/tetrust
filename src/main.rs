mod blocks;
mod game;
mod play;
mod ai;
mod ga;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    mode: Option<Mode>,
}

#[derive(Subcommand)]
enum Mode {
    Normal,
    Auto,
    Learning,
}


fn main() {
    let cli = Cli::parse();
    match cli.mode {
        None |
        Some(Mode::Normal) => {
            // normal Mode
            play::normal();
        }
        Some(Mode::Auto) => {
            // auto Mode
            play::auto();
        }
        Some(Mode::Learning) => {
            // GA Learning Mode
            ga::learning();
        }
    }
}
