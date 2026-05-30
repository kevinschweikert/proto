pub mod mermaid;
pub mod rfc;

use crate::packet::Packet;

pub trait Render {
    fn render(&self, packet: &Packet) -> String;
}
