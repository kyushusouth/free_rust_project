#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub score: f64,                  // 1st/2nd stageで更新されるスコア
    pub explanation: Option<String>, // 3rd stageで付与
}
