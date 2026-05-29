use crate::{Field, Packet};

pub fn build() -> Packet {
    Packet {
        title: Some("TCP Packet".to_string()),
        fields: vec![
            Field::Fixed {
                label: "Source Port".to_string(),
                bits: 16,
            },
            Field::Fixed {
                label: "Destination Port".to_string(),
                bits: 16,
            },
            Field::Fixed {
                label: "Sequence Number".to_string(),
                bits: 32,
            },
            Field::Fixed {
                label: "Acknowledgment Number".to_string(),
                bits: 32,
            },
            Field::Fixed {
                label: "Data Offset".to_string(),
                bits: 4,
            },
            Field::Fixed {
                label: "Reserved".to_string(),
                bits: 6,
            },
            Field::Fixed {
                label: "URG".to_string(),
                bits: 1,
            },
            Field::Fixed {
                label: "ACK".to_string(),
                bits: 1,
            },
            Field::Fixed {
                label: "PSH".to_string(),
                bits: 1,
            },
            Field::Fixed {
                label: "RST".to_string(),
                bits: 1,
            },
            Field::Fixed {
                label: "SYN".to_string(),
                bits: 1,
            },
            Field::Fixed {
                label: "FIN".to_string(),
                bits: 1,
            },
            Field::Fixed {
                label: "Window".to_string(),
                bits: 16,
            },
            Field::Fixed {
                label: "Checksum".to_string(),
                bits: 16,
            },
            Field::Fixed {
                label: "Urgent Pointer".to_string(),
                bits: 16,
            },
            Field::Fixed {
                label: "Options".to_string(),
                bits: 24,
            },
            Field::Fixed {
                label: "Padding".to_string(),
                bits: 8,
            },
            Field::Variable {
                label: "Data".to_string(),
            },
        ],
    }
}
