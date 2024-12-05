use std::collections::HashMap;

type Rule = Vec<usize>;

#[derive (Debug)]
pub struct Rules {
    r: HashMap<usize, Rule>,
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
    pub fn retrieve(&self, num: usize) -> Option<&Rule> {
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

    pub fn find_fault(&self, rules: &Rules) -> Option<usize> {
        for i in 0 .. self.p.len() {
            let num = self.p[i];

            if let Some(before) = rules.retrieve(num) {
                // Check to make sure none of before comes after
                if before.into_iter().
                    any(|x| self.p[i..].contains(x)) { return Some(i) }
            }
            else {
                continue
            }

        }
        None
    }

    /// Returns true if the update was invalid
    pub fn fix_invalid(&mut self, rules: &Rules) -> bool {
        if let Some(loc) = Self::find_fault(self, rules) {
            let num = self.p[loc];
            // Get violating Rule
            // TODO: Have find_fault also return the Rule
            let before = rules.retrieve(num).unwrap();

            // Scan right and place the number after all items in before
            assert_eq!(self.p.remove(loc), num);
            let mut i = loc;
            while before.into_iter().any(|x| self.p[i..].contains(x)) {
                i += 1;
            }
            self.p.insert(i, num);

            // Do it again
            Self::fix_invalid(self, rules);
        }
        else {
            return false
        }

        // Update midpoint
        self.mid = self.p[self.p.len() / 2];
        true
    }
}
