use crate::{Packet, definitions};

static REGISTRY: &[(&str, &str, fn() -> Packet)] = &[
    ("udp", "User Datagram Protocol", definitions::udp::build),
    (
        "tcp",
        "Transmission Control Protocol",
        definitions::tcp::build,
    ),
];

pub fn get(name: &str) -> Option<Packet> {
    REGISTRY
        .iter()
        .find(|(n, _, _)| *n == name)
        .map(|(_, _, f)| f())
}

pub fn list() -> Vec<(String, String)> {
    REGISTRY
        .iter()
        .map(|(n, d, _)| (n.to_string(), d.to_string()))
        .collect()
}
