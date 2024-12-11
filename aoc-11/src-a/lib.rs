#[derive (Debug)]
pub struct Stone {
    num: u64
}

impl Stone {
    pub fn new(value: &str) -> Stone {
        let num = value.parse::<u64>()
            .expect("Could not parse stone value");

        Stone{num}
    }

    pub fn blink(&self) -> Vec<Stone> {
        let repr = self.num.to_string();

        // Stone marked 0
        if self.num == 0 {
            return vec![Stone{num: 1}]
        }
        // Stone has even number of digits
        else if repr.len() % 2 == 0 {
            let (first, last) = repr.split_at(repr.len() / 2);

            return vec![
                Stone{num: first.parse::<u64>().expect("Could not parse stone value")},
                Stone{num: last.parse::<u64>().expect("Could not parse stone value")}
            ]
        }
        // Stone doesn't satisfy other rules
        else {
            return vec![Stone{num: self.num * 2024}]
        }
    }
}
