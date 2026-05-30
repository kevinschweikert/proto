use proto::Packet;

use crate::common::{f, p, v};
mod common;

#[test]
fn fixed_field() {
    assert_eq!(
        "Payload:16".parse::<Packet>().unwrap(),
        p(None, vec![f(16, "Payload")])
    )
}

#[test]
fn fixed_field_with_whitespace() {
    assert_eq!(
        " Payload: 16 ".parse::<Packet>().unwrap(),
        p(None, vec![f(16, "Payload")])
    )
}

#[test]
fn variable_field() {
    assert_eq!(
        "Payload:*".parse::<Packet>().unwrap(),
        p(None, vec![v("Payload")])
    )
}

#[test]
fn space_in_label_is_conserved() {
    assert_eq!(
        "Source Port:*".parse::<Packet>().unwrap(),
        p(None, vec![v("Source Port")])
    )
}

#[test]
fn many_fixed_fields() {
    assert_eq!(
        "Source Port:16,Destination Port:16"
            .parse::<Packet>()
            .unwrap(),
        p(None, vec![f(16, "Source Port"), f(16, "Destination Port")])
    )
}

#[test]
fn many_fixed_fields_with_whitespace() {
    assert_eq!(
        " Source Port : 16 , Destination Port   : 16 "
            .parse::<Packet>()
            .unwrap(),
        p(None, vec![f(16, "Source Port"), f(16, "Destination Port")])
    )
}

#[test]
fn many_variable_fields() {
    assert_eq!(
        "Source Port:*,Destination Port:*"
            .parse::<Packet>()
            .unwrap(),
        p(None, vec![v("Source Port"), v("Destination Port")])
    )
}

#[test]
fn many_mixed_fields() {
    assert_eq!(
        "Source Port:16,Destination Port:*"
            .parse::<Packet>()
            .unwrap(),
        p(None, vec![f(16, "Source Port"), v("Destination Port")])
    )
}
