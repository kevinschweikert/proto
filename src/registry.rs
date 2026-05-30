use std::fmt;

use crate::{Packet, definitions};

#[derive(PartialEq)]
pub enum Category {
    Layer3,
    Layer4,
}

impl Category {
    fn all() -> &'static [Self] {
        &[Category::Layer3, Category::Layer4]
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Layer3 => write!(f, "Layer 4 (Network)"),
            Category::Layer4 => write!(f, "Layer 4 (Transport)"),
        }
    }
}

pub struct PacketDef {
    pub name: &'static str,
    pub description: &'static str,
    pub category: Category,
    build: fn() -> Packet,
}
static REGISTRY: &[PacketDef] = &[
    PacketDef {
        name: "ip",
        description: "Internet Protocol Version 4",
        category: Category::Layer3,
        build: definitions::ip::build,
    },
    PacketDef {
        name: "udp",
        description: "User Datagram Protocol",
        category: Category::Layer4,
        build: definitions::udp::build,
    },
    PacketDef {
        name: "tcp",
        description: "Transmission Control Protocol",
        category: Category::Layer4,
        build: definitions::tcp::build,
    },
];

pub fn get(name: &str) -> Option<Packet> {
    REGISTRY
        .iter()
        .find(|pd| pd.name == name)
        .map(|pd| (pd.build)())
}

pub fn list() -> Vec<(&'static Category, Vec<&'static PacketDef>)> {
    Category::all()
        .iter()
        .map(|category| {
            let definitions = REGISTRY
                .iter()
                .filter(|pd| &pd.category == category)
                .collect();
            (category, definitions)
        })
        .collect()
}
