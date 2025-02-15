mod guidance;
pub mod map;

use std::collections::{HashMap, VecDeque};

use guidance::Route;
use map::INF;
use map::{Map, Mark};

#[derive(Default)]
pub struct Navigator {
    map: Map,
    route: Route,
}

impl Navigator {
    pub fn init(&mut self, map: Map) {
        self.map = map;
    }

    pub fn navigate(&mut self, start: &Mark, finish: &Mark) -> Result<(), ()> {
        let graph = self.map.edge_table();
        let (mark2inter, mut dis) = self.map.node_table();
        let begin = self.map.init_intersection(&start, &mark2inter);
        let end = self.map.init_intersection(&finish, &mark2inter);
        let elevators = self.map.elevators();
        let mut q = VecDeque::new();

        if let Some(v) = end.0 {
            q.push_back(v);
            dis.entry(v).and_modify(|v| *v = 0);
        }
        if let Some(v) = end.1 {
            q.push_back(v);
            dis.entry(v).and_modify(|v| *v = 0);
        }

        let mut last_inter = HashMap::new();
        while !q.is_empty() {
            let u = q.front().cloned().unwrap();
            q.pop_front();
            let d = dis.get(u).cloned().unwrap();
            for c in u {
                let op_exit = graph.get(&c.mark);
                if op_exit.is_none() {
                    if c.mark.elevator_floor != 0 {
                        let set = elevators.get(&Map::identity(&c.mark)).unwrap();
                        for &floor in set {
                            let &v = mark2inter.get(floor).unwrap();
                            let od = dis.get_mut(v).unwrap();
                            if *od > d + 1 {
                                *od = d + 1;
                                last_inter.entry(v).insert_entry(&c.mark);
                                q.push_back(v);
                            }
                        }
                    }
                } else {
                    let exit = op_exit.unwrap();
                    let v = *(mark2inter.get(exit).unwrap());
                    let od = dis.get_mut(v).unwrap();
                    if *od > d + 1 {
                        *od = d + 1;
                        last_inter.entry(v).insert_entry(&c.mark);
                        q.push_back(v);
                    }
                }
            }
        }

        if begin.0.is_none() {
            return Err(());
        }
        let mut fst = begin.0.unwrap();
        if let Some(v) = begin.1 {
            if dis.get(&v).unwrap() < dis.get(fst).unwrap() {
                fst = v;
            }
        }
        if *dis.get(fst).unwrap() == INF {
            return Err(());
        }

        let init_direction = self.map.init_direction(&start, &fst);
        if let Some(c) = init_direction {
            let next_marks = Map::inter2marks(fst);
            let prompt = format!("参考给出的参考方向，移动到{}附近路口", c.1.name);
            self.route
                .emplace_back(c.0, prompt, c.1.name.clone(), next_marks);
        }

        let mut u = fst;
        while let Some(exit) = last_inter.get(u).cloned() {
            let port = graph.get(exit).cloned().unwrap();
            let nxt = mark2inter.get(port).cloned().unwrap();
            let di = Map::find_direction(u, exit);
            let target_mark: String = exit.name.clone();
            let prompt: String;
            if port.elevator_floor != 0 {
                prompt = format!("由{}前往 {} 楼。", target_mark, port.elevator_floor);
            } else {
                prompt = format!("往{}方向前进至下一个路口。", target_mark);
            }
            self.route
                .emplace_back(di, prompt, target_mark, Map::inter2marks(nxt));
            u = nxt;
        }

        let last_direction = self.map.init_direction(&finish, u);
        if let Some(c) = last_direction {
            let prompt = format!("向{}方向往前走，直到你到达目的地。", c.1.name);
            self.route
                .emplace_back(Map::turn(c.0), prompt, c.1.name.clone(), Vec::new());
        }
        Ok(())
    }

    pub fn route(&mut self) -> &mut Route {
        &mut self.route
    }
}
