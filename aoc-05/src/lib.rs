use std::collections::HashMap;

#[derive (Debug)]
pub struct Rules {
    r: HashMap<usize, Vec<usize>>,
}

impl Rules {
    pub fn new() -> Rules {
        Rules {
            r: HashMap::new()
        }
    }

    pub fn insert(&mut self, rule: &str) {
        match rule.split_once('|') {
            Some((x, y)) => { 
                let before = x.parse::<usize>()
                    .expect("Could not parse num");
                let after = y.parse::<usize>()
                    .expect("Could not parse num");

                if let Some(v) = self.r.get_mut(&after) {
                    v.push(before);
                } 
                else {
                    self.r.insert(after, vec![before]);
                }
            },

            None => return
        }
    }

    /// Returns a list of all numbers that must come before num
    pub fn retrieve(&self, num: usize) -> Option<&Vec<usize>> {
        self.r.get(&num)
    }
}

#[derive (Debug)]
pub struct Update {
    p: Vec<usize>,
    pub mid: usize
}

impl Update {
    pub fn new(update: &str) -> Update {
        let p: Vec<usize> = update.split(',')
            .map(|x| x.parse::<usize>()
                .expect("Could not parse num"))
            .collect();
        let mid = p[p.len() / 2];

        Update {p, mid}
    }

    pub fn check_valid(&self, rules: &Rules) -> bool {
        for i in 0 .. self.p.len() {
            let num = self.p[i];

            if let Some(before) = rules.retrieve(num) {
                // Check to make sure none of before comes after
                if before.into_iter().
                    any(|x| self.p[i..].contains(x)) { return false }
            }
            else {
                continue
            }

        }
        true
    }
}
