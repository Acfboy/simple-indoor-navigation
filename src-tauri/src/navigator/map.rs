/// 这个模块实现地图的存储和加边加点删边删点，获得地图上导航路线的功能在 `navigator.rs` 里实现。

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// 一个节点里存储下面信息：
/// - 节点名称，可以为空。
/// - 在地图上的位置，单位是像素，未经缩放。
/// - 在第几层，用于处理电梯和楼梯。
/// - 电梯备注，用于区分是哪个电梯。
/// - 节点编号。
#[derive(Serialize, Deserialize, Default)]
pub struct Node {
    pub name: String,
    pub pos: Position,
    pub floor: i32,
    pub elevator: String,
    pub index: usize,
}


/// 电梯（楼梯）存储该电梯中每个节点的编号。
#[derive(Serialize, Deserialize)]
pub struct Elevator(pub HashSet<usize>);

/// 地图包括点，边，电梯，和因为删除空出来的节点下标，用于回收被删除的节点.
/// - 存图采用一个存一个编号的点对应的所有出边。
/// - `elevators` 将电梯备注对应到相应的电梯。
#[derive(Serialize, Deserialize, Default)]
pub struct Map {
    pub nodes: Vec<Node>,
    pub edges: Vec<HashSet<usize>>,
    pub elevators: HashMap<String, Elevator>,
    node_garbage: HashSet<usize>,
}

impl Map {
    fn new_node_id(&mut self) -> usize {
        if !self.node_garbage.is_empty() {
            let res = self.node_garbage.iter().nth(0).cloned().unwrap();
            self.node_garbage.remove(&res);
            res
        } else {
            self.nodes.push(Node::default());
            self.edges.push(HashSet::new());
            self.nodes.len() - 1
        }
    }

    /// 加点，同时加入电梯。
    pub fn add_node(&mut self, name: String, pos: Position, floor: i32, elevator: String) {
        let index = self.new_node_id();
        if !elevator.is_empty() {
            self.elevators
                .entry(elevator.clone())
                .and_modify(|x| {
                    x.0.insert(index);
                })
                .or_insert(Elevator(HashSet::from([index])));
        }
        self.nodes[index] = Node {
            name,
            pos,
            floor,
            elevator,
            index,
        };
    }

    pub fn check_node_valid(&self, index: usize) -> Result<(), String> {
        if index >= self.nodes.len() {
            Err("node index too large".to_string())
        } else if self.node_garbage.contains(&index) {
            Err("the node was removed".to_string())
        } else {
            Ok(())
        }
    }

    /// 加边。如果边已经存在，则返回 `Ok(())` 但图不改变。
    pub fn add_edge(&mut self, from: usize, to: usize) -> Result<(), String> {
        self.check_node_valid(from)?;
        self.check_node_valid(to)?;
        self.edges[from].insert(to);
        self.edges[to].insert(from);
        Ok(())
    }

    /// 标记删除一个点，同时删除该点涉及的边和电梯中涉及的该点。但不改变那个点存储的数据。
    /// 点不存在时返回错误。
    pub fn remove_node(&mut self, index: usize) -> Result<(), String> {
        self.check_node_valid(index)?;
        self.node_garbage.insert(index);
        self.edges[index].clear();
        self.edges
            .iter_mut()
            .filter(|x| x.contains(&index))
            .for_each(|x| {
                x.remove(&index);
            });
        if !self.nodes[index].elevator.is_empty() {
            self.elevators
                .get_mut(&self.nodes[index].elevator.clone())
                .unwrap()
                .0
                .remove(&index);
        }
        Ok(())
    }

    /// 删边。边不存在时返回 `Ok`，但端点不合法时返回 `Err`。
    pub fn remove_edge(&mut self, from: usize, to: usize) -> Result<(), String> {
        self.check_node_valid(from)?;
        self.check_node_valid(to)?;
        self.edges[from].remove(&to);
        self.edges[to].remove(&from);
        Ok(())
    }

    /// 从 serde_json 构造地图文件中的地图后，判断地图是否合法。由于格式已经正确，不合法情况只有：
    /// - 电梯或边中出现不合法的点。
    pub fn is_valid(&self) -> bool {
        let edge_invalid = self
            .edges
            .iter()
            .find(|&x| {
                x.iter()
                    .find(|&y| self.check_node_valid(*y).is_err())
                    .is_some()
            })
            .is_some();
        let elevator_invalid = self
            .elevators
            .iter()
            .find(|x| {
                x.1 .0
                    .iter()
                    .find(|&y| self.check_node_valid(*y).is_err())
                    .is_some()
            })
            .is_some();
        !(elevator_invalid || edge_invalid)
    }
}
