use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "eternalMac")]
#[command(about = "Turn a Mac Mini into a personal devserver")]
pub struct Cli {}

pub fn run() -> Result<()> {
    let _cli = Cli::parse();
    Ok(())
}
