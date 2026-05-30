use crate::{Field, Packet};

pub fn build() -> Packet {
    Packet {
        title: Some("IP Packet".to_string()),
        fields: vec![
            Field::Fixed {
                label: "Version".to_string(),
                bits: 4,
            },
            Field::Fixed {
                label: "IHL".to_string(),
                bits: 4,
            },
            Field::Fixed {
                label: "Type of Service".to_string(),
                bits: 8,
            },
            Field::Fixed {
                label: "Total Length".to_string(),
                bits: 16,
            },
            Field::Fixed {
                label: "Identification".to_string(),
                bits: 16,
            },
            Field::Fixed {
                label: "Flags".to_string(),
                bits: 3,
            },
            Field::Fixed {
                label: "Fragment Offset".to_string(),
                bits: 13,
            },
            Field::Fixed {
                label: "Time to Live".to_string(),
                bits: 8,
            },
            Field::Fixed {
                label: "Protocol".to_string(),
                bits: 8,
            },
            Field::Fixed {
                label: "Header Checksum".to_string(),
                bits: 16,
            },
            Field::Fixed {
                label: "Source Address".to_string(),
                bits: 32,
            },
            Field::Fixed {
                label: "Destination Address".to_string(),
                bits: 32,
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
