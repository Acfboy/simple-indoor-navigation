use serde::Deserialize;
use std::{collections::HashMap, hash::Hash, mem::swap};

#[derive(Default, Deserialize, Hash, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Mark {
    pub name: String,
    pub detail: String,
    pub elevator_floor: i32,
}

impl Mark {
    pub fn full_name(&self) -> String {
        let mut last = String::new();
        if self.detail.len() != 0 {
            if self.elevator_floor == 0 {
                last = format!("({last})");
            } else {
                last = format!("({last}, {} 楼)", self.elevator_floor);
            }
        }
        format!("{}{}", self.name, last)
    }
}

pub type Corridor = Vec<Mark>;

#[derive(Default, Deserialize, Hash, PartialEq, Eq)]
pub struct Branch {
    pub direction: usize,
    pub mark: Mark,
}

pub static INF: usize = 0x3f3f3f3f;

pub type Intersection = Vec<Branch>;

#[derive(Default, Deserialize, Hash)]
pub struct Map {
    edges: Vec<Corridor>,
    nodes: Vec<Intersection>,
}


impl Map {
    pub fn identity(mark: &Mark) -> String {
        mark.name.clone() + &mark.detail.clone()
    }

    pub fn elevators(&self) -> HashMap<String, Vec<&Mark>> {
        let mut res = HashMap::new();
        self.nodes
            .iter()
            .flatten()
            .filter_map(|x| {
                if x.mark.elevator_floor != 0 {
                    Some(&x.mark)
                } else {
                    None
                }
            })
            .for_each(|x| {
                let id = Self::identity(x);
                let mut c = res.get_mut(&id);
                if c.is_none() {
                    res.insert(id.clone(), Vec::new());
                    c = res.get_mut(&id);
                }
                c.unwrap().push(x);
            });
        res
    }

    pub fn edge_table(&self) -> HashMap<&Mark, &Mark> {
        let mut res = HashMap::new();
        for edge in &self.edges {
            if edge[0].elevator_floor != 0 || edge[1].elevator_floor != 0 {
                continue;
            }
            res.insert(&edge[0], &edge[edge.len() - 1]);
            res.insert(&edge[edge.len() - 1], &edge[0]);
        }
        res
    }

    pub fn node_table(&self) -> (HashMap<&Mark, &Intersection>, HashMap<&Intersection, usize>) {
        let mut mark2inter = HashMap::new();
        let mut dis = HashMap::new();
        for c in &self.nodes {
            for branch in c {
                mark2inter.insert(&branch.mark, c);
            }
            dis.insert(c, INF);
        }
        (mark2inter, dis)
    }

    pub fn init_intersection<'a>(
        &self,
        mark: &Mark,
        map: &HashMap<&Mark, &'a Intersection>,
    ) -> (Option<&'a Intersection>, Option<&'a Intersection>) {
        let corri = self
            .edges
            .iter()
            .find(|&x| x.iter().find(|&y| y == mark).is_some());
        if let None = corri {
            (None, None)
        } else {
            let c = corri.unwrap();
            let mut res = (map.get(&c[0]).cloned(), map.get(&c[c.len() - 1]).cloned());
            if res.0.is_none() {
                swap(&mut res.0, &mut res.1);
            }
            res
        }
    }

    pub fn find_direction(inter: &Intersection, mark: &Mark) -> usize {
        let res = inter.iter().find(|&x| &x.mark == mark);
        if let None = res {
            0
        } else {
            res.unwrap().direction
        }
    }

    pub fn turn(direction: usize) -> usize {
        (direction + 180) % 360
    }

    pub fn init_direction<'a>(
        &self,
        mark: &Mark,
        inter: &'a Intersection,
    ) -> Option<(usize, &'a Mark)> {
        let corri = self
            .edges
            .iter()
            .find(|&x| x.iter().find(|&y| y == mark).is_some())
            .unwrap();
        if *mark == corri[0] || *mark == corri[corri.len() - 1] {
            return None;
        }
        for c in inter {
            if c.mark == corri[0] || c.mark == corri[corri.len() - 1] {
                return Some((Self::turn(c.direction), &c.mark));
            }
        }
        None
    }

    pub fn inter2marks(inter: &Intersection) -> Vec<String> {
        inter.iter().map(|x| x.mark.name.clone()).collect()
    }
}
