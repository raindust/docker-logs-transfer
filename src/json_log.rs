#[derive(Debug, Serialize, Deserialize)]
pub struct JsonLog {
    pub log: String,
    pub stream: String,
    pub time: String,
}
