use proto::packet::Field;

pub fn f(bits: usize, label: &str) -> Field {
    Field::Fixed {
        bits,
        label: label.to_string(),
    }
}

pub fn v(label: &str) -> Field {
    Field::Variable {
        label: label.to_string(),
    }
}
