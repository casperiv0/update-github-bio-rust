use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct GrandTotal {
    pub text: String,
}

#[derive(Deserialize)]
pub struct Data {
    pub grand_total: GrandTotal,
}

#[derive(Deserialize)]
pub struct WakatimeData {
    pub data: [Data; 1],
}
