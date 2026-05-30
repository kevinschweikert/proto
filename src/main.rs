use std::{fs, path::PathBuf, process::exit};

use clap::{Parser, ValueEnum};
use proto::{Mermaid, Packet, Render, RfcDiagram, Style, kaitai, registry, render::rfc::Junctions};
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
    #[arg(value_enum, short = 's', long, default_value_t = OutputType::Ascii, hide = true)]
    style: OutputType,
    /// Output in unicode style
    #[arg(short, long, conflicts_with = "mermaid", overrides_with = "style")]
    unicode: bool,
    /// Output a mermaid.js spec
    #[arg(short, long, conflicts_with = "unicode", overrides_with = "style")]
    mermaid: bool,
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
    /// list available protocols
    list: bool,
    #[arg(short, long)]
    /// path to a kaitai spec file
    from_kaitai: Option<PathBuf>,
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

    let packet = if let Some(kaitai_path) = cli.from_kaitai {
        let content = fs::read_to_string(kaitai_path).map_err(|_| CliError::InvalidDefinition)?;
        kaitai::parse_kaitai(&content)?
    } else {
        let definition = cli.definition.ok_or(CliError::MissingDefinition)?;

        match registry::get(definition.as_str()) {
            Some(p) => p,
            None => definition.parse::<Packet>()?,
        }
    };

    let junctions = match cli.clean {
        true => Junctions::Boundaries,
        false => Junctions::All,
    };

    let output_type = if cli.unicode {
        OutputType::Unicode
    } else if cli.mermaid {
        OutputType::Mermaid
    } else {
        cli.style.clone()
    };

    let renderer: Box<dyn Render> = match output_type {
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
