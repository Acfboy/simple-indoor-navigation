use serde::{Deserialize, Serialize};

/// 这个模块声明用于存储路线数据的类型，包括找到每一步路径的功能。

use super::map::Node;

/// 生成每一步的路径，并提供返回当前步、下一步和上一步的接口。
#[derive(Default)]
pub struct Guidance {
    path: Route,
    steps: Vec<Route>,
    cur_step: usize,
}

/// `Route` 即一路上的所有点。
#[derive(Default, Clone, Serialize)]
pub struct Route(pub Vec<Node>);

/// 表示显示区域的像素大小。在 `low_bound` 和 `up_bound` 等中也用来表示相对于起点需求的显示区域的大小。
#[derive(Clone, Deserialize)]
pub struct ScreenSize(f64, f64);

impl Node {
    /// 用当前点相对于 `beg` 更新 `lb` 和 `ub` 两个大小限制，返回新的限制。
    pub fn check_bound(
        &self,
        beg: &Node,
        lb: &ScreenSize,
        ub: &ScreenSize,
    ) -> (ScreenSize, ScreenSize) {
        let mut nlb = lb.clone();
        let mut nub = ub.clone();
        nlb.0 = nlb.0.min(self.pos.x - beg.pos.x);
        nlb.1 = nlb.1.min(self.pos.y - beg.pos.y);
        nub.0 = nub.0.max(self.pos.x - beg.pos.x);
        nub.1 = nub.1.max(self.pos.y - beg.pos.y);
        (nlb, nub)
    }
}

impl Guidance {
    pub fn push(&mut self, node: Node) {
        self.path.0.push(node);
    }

    /// 根据显示区域的大小划定每一步要走的路线，并存在 `steps` 内。  
    /// 若有步骤只能包含进单一节点，返回错误。  
    /// 电梯（楼梯）节点单独多加一步。
    pub fn step_by_step(&mut self, size: ScreenSize, scales: Vec<f64>) -> Result<(), String> {
        let mut it = self.path.0.iter().peekable();
        let mut beg = it.next().ok_or("path not exist")?;
        let mut low_bound = ScreenSize(0., 0.);
        let mut up_bound = ScreenSize(0., 0.);
        let mut lst = beg;
        let mut elevated = false;
        self.steps = vec![Route(vec![beg.clone()])];
        while let Some(&nxt) = it.peek() {
            let (n_low_bound, n_up_bound) = nxt.check_bound(beg, &low_bound, &up_bound);
            if n_up_bound.0 - n_low_bound.0 > size.0 * 0.9 / scales[nxt.floor - 1]
                || n_up_bound.1 - n_low_bound.1 > size.1 * 0.9 / scales[nxt.floor - 1]
                || nxt.floor != lst.floor
                || elevated
            {
                if lst.index == beg.index {
                    return Err("screen size too small".to_string());
                }
                if nxt.floor != lst.floor {
                    elevated = true;
                } else {
                    elevated = false;
                }
                beg = lst;
                self.steps.push(Route(vec![beg.clone()]));
            }
            let last_index = self.steps.len() - 1;
            lst = it.next().unwrap();
            self.steps[last_index].0.push(lst.clone());
            (low_bound, up_bound) = (n_low_bound, n_up_bound);
        }
        Ok(())
    }

    pub fn next_step(&mut self) -> Result<(), String> {
        if self.cur_step >= self.steps.len() {
            Err("no next step".to_string())
        } else {
            self.cur_step += 1;
            Ok(())
        }
    }

    pub fn prev_step(&mut self) -> Result<(), String> {
        if self.cur_step == 0 {
            Err("no prev step".to_string())
        } else {
            self.cur_step -= 1;
            Ok(())
        }
    }

    pub fn query(&self) -> Result<Route, String> {
        if self.steps.len() == 0 {
            Err("steps haven't generated".to_string())
        } else {
            Ok(self.steps[self.cur_step].clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::navigator::map::Position;
    use super::*;

    #[test]
    fn test_step_by_step() {
        let mut guide = Guidance::default();
        let nodes = vec![
            Node {
                name: "1".to_string(),
                pos: Position { x: 1.0, y: 1.0 },
                floor: 1,
                elevator: String::new(),
                index: 0,
            },
            Node {
                name: "1".to_string(),
                pos: Position { x: 1.0, y: 2.0 },
                floor: 1,
                elevator: String::new(),
                index: 1,
            },
            Node {
                name: "1".to_string(),
                pos: Position { x: 2.0, y: 3.0 },
                floor: 1,
                elevator: "elevator".to_string(),
                index: 2,
            },
            Node {
                name: "1".to_string(),
                pos: Position { x: 2.0, y: 3.0 },
                floor: 2,
                elevator: "elevator".to_string(),
                index: 3,
            },
            Node {
                name: "1".to_string(),
                pos: Position { x: 2.0, y: 2.0 },
                floor: 2,
                elevator: String::new(),
                index: 4,
            },
        ];
        for node in &nodes {
            guide.push(node.clone());
        }
        guide.step_by_step(ScreenSize(2.0, 2.0), vec![1.0, 1.0]).unwrap();
        let expected_indices = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
        for (expected_0, expected_1) in expected_indices {
            let current = guide.query().unwrap();
            assert_eq!(current.0[0].index, expected_0);
            assert_eq!(current.0[1].index, expected_1);
            guide.next_step().unwrap();
        }
    }
}