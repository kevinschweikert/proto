use proto::packet::Field;

pub fn f(bits: usize, label: &str) -> Field {
    Field {
        bits: bits,
        label: label.to_string(),
    }
}
