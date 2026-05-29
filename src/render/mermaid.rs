use super::Render;
use crate::packet::Packet;

pub struct Mermaid {}

impl Render<String> for Mermaid {
    fn render(&self, packet: &Packet) -> String {
        let mut lines = vec!["packet".to_string()];
        if let Some(title) = &packet.title {
            lines.push(format!("title {}", title))
        };
        for field in packet.fields.clone() {
            lines.push(format!("+{}: \"{}\"", field.bits, field.label));
        }

        lines.join("\n")
    }
}
