pub mod guidance;
pub mod map;

use std::collections::VecDeque;

use guidance::Guidance;
use map::Map;

static INF: usize = 0x3f3f3f3f;

impl Map {
    fn relax(
        q: &mut VecDeque<usize>,
        dis: &mut Vec<usize>,
        last: &mut Vec<usize>,
        u: usize,
        v: usize,
    ) {
        if dis[v] > dis[u] + 1 {
            dis[v] = dis[u] + 1;
            q.push_back(v);
            last[v] = u;
        }
    }
    
    /// 在地图从 `s` 出发走到每一个点，并返回每个点的最短路和路径上上一个点。
    fn find_path(&self, s: usize) -> (Vec<usize>, Vec<usize>) {
        let mut dis = vec![INF; self.nodes.len()];
        let mut last = vec![INF; self.nodes.len()];
        let mut q = VecDeque::new();
        dis[s] = 0;
        q.push_back(s);
        while !q.is_empty() {
            let u = q.front().cloned().unwrap();
            q.pop_front();
            if self.nodes[u].elevator.len() > 0 {
                for &v in &self.elevators.get(&self.nodes[u].elevator).unwrap().0 {
                    Self::relax(&mut q, &mut dis, &mut last, u, v);
                }
            }
            for &v in &self.edges[u] {
                Self::relax(&mut q, &mut dis, &mut last, u, v);
            }
        }
        (dis, last)
    }

    /// 倒推出路径。
    fn get_path(&self, dis: Vec<usize>, last: Vec<usize>, mut u: usize) -> Guidance {
        let mut res = Guidance::default();
        res.push(self.nodes[u].clone());
        while last[u] != INF {
            u = last[u];
            res.push(self.nodes[u].clone());
        }
        res
    }

    pub fn navigate(&self, from: usize, to: usize) -> Result<Guidance, String> {
        self.check_node_valid(from)?;
        self.check_node_valid(to)?;
        let (dis, last) = self.find_path(to);
        if dis[from] == INF {
            return Err("no path".to_string());
        }
        Ok(self.get_path(dis, last, from))
    }
}


#[cfg(test)]
mod tests {
    use super::{map::Position, *};

    #[test]
    fn test_find_path() {
        let mut map = Map::default();
        for _ in 0 .. 10 {
            map.add_node( Position{ x: 0.0, y: 0.0 }, 0);
        }
        map.add_edge(1, 2).unwrap();
        map.add_edge(1, 3).unwrap();
        map.add_edge(6, 8).unwrap();        
        map.add_edge(1, 4).unwrap();
        map.add_edge(2, 8).unwrap();
        map.add_edge(3, 5).unwrap();
        map.add_edge(5, 6).unwrap();
        map.add_edge(3, 4).unwrap();
        map.add_edge(4, 6).unwrap();
        let (dis, last) = map.find_path(1);
        assert_eq!(dis, vec![INF, 0, 1, 1, 1, 2, 2, INF, 2, INF]);
        assert_eq!(last, vec![INF, INF, 1, 1, 1, 3, 4, INF, 2, INF]);
    }
}
