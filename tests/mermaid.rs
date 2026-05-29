use proto::{Mermaid, Packet, Render};
mod common;
use common::f;

#[test]
fn udp() {
    let packet = Packet {
        title: None,
        fields: vec![
            f(16, "Source Port"),
            f(16, "Destination Port"),
            f(16, "Length"),
            f(16, "Checksum"),
            f(32, "Data"),
        ],
    };

    let renderer = Mermaid {};
    insta::assert_snapshot!(renderer.render(&packet), @r#"
    packet
    +16: "Source Port"
    +16: "Destination Port"
    +16: "Length"
    +16: "Checksum"
    +32: "Data"
    "#);
}

#[test]
fn udp_with_title() {
    let packet = Packet {
        title: Some("UDP Packet".to_string()),
        fields: vec![
            f(16, "Source Port"),
            f(16, "Destination Port"),
            f(16, "Length"),
            f(16, "Checksum"),
            f(32, "Data"),
        ],
    };

    let renderer = Mermaid {};
    insta::assert_snapshot!(renderer.render(&packet), @r#"
    packet
    title UDP Packet
    +16: "Source Port"
    +16: "Destination Port"
    +16: "Length"
    +16: "Checksum"
    +32: "Data"
    "#);
}
