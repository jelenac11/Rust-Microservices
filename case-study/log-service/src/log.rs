#[derive(Deserialize, Serialize, Debug)]
pub struct Log {
    pub datetime: String,
    pub level: String,
    pub target: String,
    pub message: String
}