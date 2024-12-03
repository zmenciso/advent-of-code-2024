#[derive(Debug)]
pub struct Mult {
    a: i32,
    b: i32
}

impl Mult {
    pub fn new(a: &str, b: &str) -> Mult {
        Mult {
            a: a.parse().expect("Could not parse string"),
            b: b.parse().expect("Could not parse string")
        }
    }

    pub fn product(&self) -> i32 {
        self.a * self.b
    }
}
