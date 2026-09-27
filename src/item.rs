#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub score: f64,                  // 1st/2nd stageで更新されるスコア
    pub explanation: Option<String>, // 3rd stageで付与
}
