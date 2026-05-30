mod definitions;
pub mod kaitai;
pub mod packet;
pub mod registry;
pub mod render;

pub use packet::{Field, Packet};
pub use render::Render;
pub use render::mermaid::Mermaid;
pub use render::rfc::{RfcDiagram, Style};

use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("ivalid field: {0}, Expected label:bits")]
    InvalidField(String),
    #[error("invalid bit size: {0}")]
    InvalidBits(String),
    #[error("invalid kaitai spec")]
    InvalidKaitaiSpec,
}

pub fn render_terminal(packet: &Packet, width: usize, style: Option<Style>) -> String {
    let style = match style {
        Some(style) => style,
        None => Style::ascii(),
    };
    RfcDiagram::new(width).with_style(style).render(packet)
}

impl FromStr for Packet {
    type Err = PacketError;
    fn from_str(packet_def: &str) -> Result<Self, Self::Err> {
        let fields = packet_def
            .split(",")
            .map(|field_def| {
                let (label, bits) = field_def
                    .trim()
                    .split_once(":")
                    .ok_or_else(|| PacketError::InvalidField(field_def.to_string()))?;
                match bits {
                    "*" => Ok(Field::Variable {
                        label: label.trim().to_string(),
                    }),
                    bits => {
                        let bits = bits
                            .trim()
                            .parse()
                            .map_err(|_| PacketError::InvalidBits(bits.to_string()))?;
                        Ok(Field::Fixed {
                            label: label.trim().to_string(),
                            bits,
                        })
                    }
                }
            })
            .collect::<Result<Vec<Field>, PacketError>>()?;
        Ok(Packet {
            title: None,
            fields,
        })
    }
}
