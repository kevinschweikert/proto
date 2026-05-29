use crate::{Field, Packet};

pub fn build() -> Packet {
    Packet {
        title: Some("UDP Packet".to_string()),
        fields: vec![
            Field::Fixed {
                bits: 16,
                label: "Source Port".to_string(),
            },
            Field::Fixed {
                bits: 16,
                label: "Destination Port".to_string(),
            },
            Field::Fixed {
                bits: 16,
                label: "Length".to_string(),
            },
            Field::Fixed {
                bits: 16,
                label: "Checksum".to_string(),
            },
            Field::Variable {
                label: "Data".to_string(),
            },
        ],
    }
}
