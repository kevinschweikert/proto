use std::process::exit;

use clap::{Parser, ValueEnum};
use proto::{Mermaid, Packet, Render, RfcDiagram, Style, registry, render::rfc::Junctions};
use thiserror::Error;

#[derive(Clone, ValueEnum)]
enum OutputType {
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
    #[arg(value_enum, short = 's', long, default_value_t = OutputType::Ascii)]
    style: OutputType,
    #[arg(short, long)]
    /// remove crosses for each bit in a field for cleaner rendering
    clean: bool,
    /// number of bits per row
    #[arg(short = 'b', long = "bits-per-row", default_value_t = 32)]
    bits_per_row: usize,
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
        println!();
        for (category, definitions) in registry::list() {
            println!("{}:", category);
            for def in definitions {
                println!("  {}    {}", def.name, def.description);
            }
            println!()
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

    let junctions = match cli.clean {
        true => Junctions::Boundaries,
        false => Junctions::All,
    };

    let renderer: Box<dyn Render> = match cli.style {
        OutputType::Ascii => Box::new(
            RfcDiagram::new(cli.bits_per_row)
                .with_style(Style::ascii().with_junctions(junctions))
                .with_ruler(!cli.no_ruler),
        ),
        OutputType::Unicode => Box::new(
            RfcDiagram::new(cli.bits_per_row)
                .with_style(Style::unicode().with_junctions(junctions))
                .with_ruler(!cli.no_ruler),
        ),
        OutputType::Mermaid => Box::new(Mermaid {}),
    };

    println!("{}", renderer.render(&packet));

    Ok(())
}
