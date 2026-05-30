use proto::{Field, kaitai};

const SPEC: &'static str = r#"
meta:
  id: udp_datagram
  title: some packet
seq:
  - id: src_port
    type: u2
  - id: dst_port
    type: b2
  - id: length
    type: u2le
  - id: checksum
    type: u2
  - id: body
    size: length - 8
"#;

#[test]
fn parse_spec_to_packet() {
    let packet = kaitai::parse_kaitai(SPEC).unwrap();
    assert_eq!(packet.title, Some("some packet".into()));

    assert_eq!(packet.fields.len(), 5);
    assert_eq!(packet.fields.first().unwrap().bits(), Some(16));
    assert!(matches!(packet.fields.last(), Some(Field::Variable { .. })))
}
