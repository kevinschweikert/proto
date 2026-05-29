use std::process::exit;

use clap::{Parser, ValueEnum};
use proto::{Mermaid, Packet, Render, Style, Terminal, registry};
use thiserror::Error;

#[derive(Clone, ValueEnum)]
enum OutputStyle {
    /// RFC Style ASCII characters
    Ascii,
    /// Modern unicode characters
    Unicode,
    /// Mermaid spec
    Mermaid,
}

#[derive(Parser)]
struct Cli {
    /// Protocol name or inline definition
    definition: Option<String>,
    /// Output style
    #[arg(value_enum, short = 's', long, default_value_t = OutputStyle::Ascii)]
    style: OutputStyle,
    /// number of bits per row
    #[arg(short, long = "bits-per-row", default_value_t = 32)]
    b: usize,
    /// omit the bit number header
    #[arg(short, long)]
    no_ruler: bool,
    #[arg(short, long)]
    list: bool,
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("invalid definition")]
    InvalidDefinition,
    #[error("missing definition")]
    MissingDefinition,
    #[error(transparent)]
    Packet(#[from] proto::PacketError),
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), CliError> {
    let cli = Cli::parse();

    if cli.list {
        println!("Available Protocols:");
        println!("");
        for (name, description) in registry::list() {
            println!("  {}    {}", name, description);
        }
        exit(0)
    }

    let definition = match cli.definition {
        None => return Err(CliError::MissingDefinition),
        Some(def) => def,
    };

    let packet = match registry::get(definition.as_str()) {
        Some(p) => p,
        None => definition.parse::<Packet>()?,
    };

    let renderer: Box<dyn Render<_>> = match cli.style {
        OutputStyle::Ascii => Box::new(
            Terminal::new(cli.b)
                .with_style(Style::ascii())
                .set_ruler(!cli.no_ruler),
        ),
        OutputStyle::Unicode => Box::new(
            Terminal::new(cli.b)
                .with_style(Style::unicode())
                .set_ruler(!cli.no_ruler),
        ),
        OutputStyle::Mermaid => Box::new(Mermaid {}),
    };

    println!("{}", renderer.render(&packet));

    Ok(())
}
