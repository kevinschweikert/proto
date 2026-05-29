use crate::{Packet, definitions};

static REGISTRY: &[(&str, fn() -> Packet)] = &[
    ("udp", definitions::udp::build),
    ("tcp", definitions::tcp::build),
];

pub fn get(name: &str) -> Option<Packet> {
    REGISTRY.iter().find(|(n, _)| *n == name).map(|(_, f)| f())
}

pub fn list() -> Vec<String> {
    REGISTRY.iter().map(|(n, _)| n.to_string()).collect()
}
