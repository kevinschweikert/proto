#[derive(Debug)]
pub struct Packet {
    pub title: Option<String>,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug)]
pub enum Field {
    Fixed { bits: usize, label: String },
    Variable { label: String },
}

impl Field {
    pub fn label(&self) -> &str {
        match self {
            Field::Fixed { label, .. } => label,
            Field::Variable { label } => label,
        }
    }
}
