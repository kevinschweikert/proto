use std::process::exit;

use clap::Parser;
use proto::{Field, Mermaid, Packet, Render, Style, Terminal};
use thiserror::Error;

#[derive(Parser)]
struct Cli {
    definition: String,
    #[arg(short, long)]
    ascii: bool,
    #[arg(short, long)]
    unicode: bool,
    #[arg(short, long)]
    mermaid: bool,
    #[arg(short, long, default_value_t = 32)]
    b: usize,
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("invalid definition")]
    InvalidDefinition,
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

    let packet = match cli.definition.as_str() {
        "udp" => udp(),
        def => def.parse::<Packet>()?,
    };

    let renderer: Box<dyn Render<_>> = match (cli.ascii, cli.unicode, cli.mermaid) {
        (true, _, _) => Box::new(Terminal::new(cli.b).with_style(Style::ascii())),
        (_, true, _) => Box::new(Terminal::new(cli.b).with_style(Style::unicode())),
        (_, _, true) => Box::new(Mermaid {}),
        _ => Box::new(Terminal::new(cli.b)),
    };

    println!("{}", renderer.render(&packet));

    Ok(())
}

fn udp() -> Packet {
    Packet {
        title: Some("UDP Packet".to_string()),
        fields: vec![
            Field {
                bits: 16,
                label: "Source Port".to_string(),
            },
            Field {
                bits: 16,
                label: "Destination Port".to_string(),
            },
            Field {
                bits: 16,
                label: "Length".to_string(),
            },
            Field {
                bits: 16,
                label: "Checksum".to_string(),
            },
            Field {
                bits: 32,
                label: "Data".to_string(),
            },
        ],
    }
}
