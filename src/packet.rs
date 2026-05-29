#[derive(Debug)]
pub struct Packet {
    pub title: Option<String>,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub bits: usize,
    pub label: String,
}
