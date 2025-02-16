use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct TimelineResponse {
    pub start: String,
    pub dest: String,
    pub path: Vec<String>,
}

#[derive(Serialize, Clone)]
pub struct IntersectionResponse {
    pub landmarks: Vec<String>
}