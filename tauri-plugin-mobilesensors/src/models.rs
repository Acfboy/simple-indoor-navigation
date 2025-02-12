use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PingRequest {
//   pub value: Option<String>,
// }

// #[derive(Debug, Clone, Default, Deserialize, Serialize)]
// pub struct PingResponse {
//   pub value: Option<String>,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrientationData {
    pub orientation: f32,
}