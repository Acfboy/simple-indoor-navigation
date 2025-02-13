#[derive(Default, Clone)]
pub struct Guidance {
    pub target_direction: usize,
    pub target_mark: String,
    pub next_marks: Vec<String>,
}

#[derive(Default)]
pub struct Route {
    path: Vec<Guidance>,
    current_guidance: usize,
}

impl Route {
    pub fn emplace_back(
        &mut self,
        target_direction: usize,
        target_mark: String,
        next_marks: Vec<String>,
    ) {
        self.path.push(Guidance {
            target_direction,
            target_mark,
            next_marks,
        });
    }

    pub fn query(&self) -> Guidance {
        self.path[self.current_guidance].clone()
    }

    pub fn next_guidance(&mut self) -> Result<(), ()> {
        if self.current_guidance + 1 >= self.path.len() {
            Err(())
        } else {
            self.current_guidance += 1;
            Ok(())
        }
    }

    pub fn prev_guidance(&mut self) -> Result<(), ()> {
        if self.current_guidance - 1 < 0 {
            Err(())
        } else {
            self.current_guidance -= 1;
            Ok(())
        }
    }
}
