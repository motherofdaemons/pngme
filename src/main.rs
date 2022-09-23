use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type MyError = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, MyError>;

fn main() -> Result<()> {
    let cli = args::Cli::try_parse()?;
    commands::run(cli.command)
}

