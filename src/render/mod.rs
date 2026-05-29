pub mod ascii;
pub mod mermaid;

use crate::packet::Packet;

pub trait Render<T> {
    fn render(&self, packet: &Packet) -> T;
}
