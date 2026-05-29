use super::Render;
use crate::{Field, Packet};

pub struct Mermaid {}

impl Render<String> for Mermaid {
    fn render(&self, packet: &Packet) -> String {
        let mut lines = vec!["packet".to_string()];
        if let Some(title) = &packet.title {
            lines.push(format!("title {}", title))
        };
        let mut bits_used = 0;
        for field in &packet.fields {
            match field {
                Field::Fixed { bits, label } => {
                    bits_used += bits;
                    lines.push(format!("+{}: \"{}\"", bits, label));
                }
                Field::Variable { label } => {
                    let remaining_in_row = 32 - (bits_used % 32);
                    lines.push(format!("+{}: \"{}\"", remaining_in_row, label));
                }
            };
        }

        lines.join("\n")
    }
}
