pub mod ascii;

use crate::packet::Packet;

pub trait Render<T> {
    fn render(&self, packet: &Packet) -> T;
}
