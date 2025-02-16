pub mod guidance;
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

    pub fn navigate(&mut self, start: &Mark, finish: &Mark) -> Result<(), String> {
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
                        let set = elevators.get(&Map::identity(&c.mark)).ok_or("err")?;
                        for &floor in set {
                            let &v = mark2inter.get(floor).ok_or("err")?;
                            let od = dis.get_mut(v).unwrap();
                            if *od > d + 1 {
                                *od = d + 1;
                                last_inter.entry(v).insert_entry((floor, u));
                                q.push_back(v);
                            }
                        }
                    }
                } else {
                    let exit = op_exit.unwrap();
                    let v = *(mark2inter.get(exit).ok_or("err")?);
                    let od = dis.get_mut(v).unwrap();
                    if *od > d + 1 {
                        *od = d + 1;
                        last_inter.entry(v).insert_entry((exit, u));
                        q.push_back(v);
                    }
                }
            }
        }

        if begin.0.is_none() {
            return Err("err".to_string());
        }
        let mut fst = begin.0.unwrap();
        if let Some(v) = begin.1 {
            if dis.get(&v).unwrap() < dis.get(fst).unwrap() {
                fst = v;
            }
        }
        if *dis.get(fst).unwrap() == INF {
            return Err("dis = inf".to_string());
        }

        let init_direction = self.map.init_direction(&start, &fst);
        if let Some(c) = init_direction {
            let next_marks = Map::inter2marks(fst);
            let prompt = format!("参考给出的参考方向，移动到{}附近路口", c.1.name);
            self.route
                .emplace_back(c.0, prompt, c.1.name.clone(), next_marks);
        }

        let mut u = fst;
        while let Some((exit, nxt)) = last_inter.get(u).cloned() {
            let di = Map::find_direction(u, exit);
            let target_mark: String = exit.name.clone();
            let prompt: String;
            if exit.elevator_floor != 0 {
                prompt = format!("由{}前往 {} 楼。", target_mark, Map::floor_number(nxt));
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

#[test]
fn test_nav() {
    let map = serde_json::from_str::<Map>(
r#"{"nodes":[[{"direction":303,"mark":{"name":"4","detail":"","elevatorFloor":0}},{"direction":7,"mark":{"name":"3","detail":"","elevatorFloor":0}},{"direction":94,"mark":{"name":"2","detail":"","elevatorFloor":0}}],[{"direction":196,"mark":{"name":"7","detail":"","elevatorFloor":0}},{"direction":102,"mark":{"name":"6","detail":"","elevatorFloor":0}}],[{"direction":295,"mark":{"name":"1","detail":"","elevatorFloor":0}}]],"edges":[[{"name":"1","detail":"","elevatorFloor":0},{"name":"2","detail":"","elevatorFloor":0}],[{"name":"3","detail":"","elevatorFloor":0},{"name":"7","detail":"","elevatorFloor":0}]]}
"#).unwrap();
    let start =
        serde_json::from_str::<Mark>(r#"{"name":"1","detail":"","elevatorFloor":0}"#).unwrap();
    let dest =
        serde_json::from_str::<Mark>(r#"{"name":"7","detail":"","elevatorFloor":0}"#).unwrap();
    let mut navi = Navigator::default();
    navi.init(map.clone());
    navi.navigate(&start, &dest).unwrap();
    let (mark2inter, _) = map.node_table();
    let end = map.init_intersection(&dest, &mark2inter);
    assert!(mark2inter.contains_key(&dest));
    let end_ins = end.0.unwrap();
    assert_eq!(end_ins[0].mark.name, "7");
    let route  = &navi.route().path;
    assert_eq!(route.len(), 2);

}
