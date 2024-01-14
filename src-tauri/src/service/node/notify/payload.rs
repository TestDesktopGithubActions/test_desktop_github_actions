#[derive(Debug, serde::Serialize, Clone)]
#[serde(untagged)]
pub enum Payload {
    Data { data: String },
}

impl Payload {
    pub(crate) fn data(data: String) -> Self {
        Payload::Data { data }
    }
}
