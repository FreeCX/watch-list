use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    mode: Option<Mode>,
}

#[derive(Debug, Default, Copy, Clone, ValueEnum)]
pub enum Format {
    /// Old text file format
    Text,
    #[default]
    /// Current file format
    Yaml,
    /// Shikimori export file format (json)
    Shikimori,
}

#[derive(Args)]
pub struct ImportArgs {
    /// File format to import
    #[arg(short = 'f', long, default_value = "yaml")]
    format: Format,
    #[arg(short = 'i', long, value_name = "FILE")]
    /// Input file name
    file: String,
}

#[derive(Args)]
pub struct ExportArgs {
    #[arg(short = 'o', long, value_name = "FILE")]
    /// Output file name
    file: String,
}

#[derive(Subcommand)]
pub enum Mode {
    /// Run app in command line interface
    Cli,
    /// Execute an old style command
    Cmd { command: String },
    /// Import data from other formats
    Import(ImportArgs),
    /// Export database to another format
    Export(ExportArgs),
}
