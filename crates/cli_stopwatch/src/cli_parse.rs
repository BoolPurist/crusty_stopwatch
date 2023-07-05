use clap::{Args, Parser, Subcommand};
#[derive(Parser)]
pub struct CliCommands {
    #[command(subcommand)]
    pub(crate) sub_commands: CliSubCommands,
}

#[derive(Debug, Subcommand)]
pub enum CliSubCommands {
    Create(Create),
    Clear,
    List,
}

#[derive(Debug, Args)]
pub struct Create {
    #[clap(short, long)]
    pub title: Option<String>,
}
