use serde::Deserialize;

#[derive(Default, Deserialize, Hash, Clone)]
pub struct Mark {
    pub name: String,
    pub detail: String,
    pub elevator_floor: usize,
}

pub type Corridor = Vec<Mark>;

#[derive(Default, Deserialize, Hash)]
pub struct Branch {
    pub direction: usize, 
    pub mark: Mark,
}

pub type Intersection = Vec<Branch>;

#[derive(Default, Deserialize, Hash)]
pub struct Map {
    edges: Vec<Corridor>,
    nodes: Vec<Intersection>,
}
