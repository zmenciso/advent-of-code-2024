use regex::Regex;

#[derive (Debug)]
pub struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    x: i64,
    y: i64
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            x: -1,
            y: -1,
            a: (-1,-1),
            b: (-1,-1),
        }
    }

    pub fn read_line(&mut self, line: &str) {
        if line.contains("Button") {
            let re = Regex::new(r"Button ([AB]): X\+([0-9]+), Y\+([0-9]+)")
                .expect("Could not compile regex");

            let (_, [button, x, y]) = re.captures(line)
                .expect("Could not match pattern to line")
                .extract();

            match button {
                "A" => { 
                    self.a = (
                        x.parse::<i64>().expect("Parse failure"), 
                        y.parse::<i64>().expect("Parse failure")
                    )},
                "B" => { 
                    self.b = (
                        x.parse::<i64>().expect("Parse failure"), 
                        y.parse::<i64>().expect("Parse failure")
                    )},
                _ => {}
            }
        }
        else if line.contains("Prize") {
            let re = Regex::new(r"X=([0-9]+), Y=([0-9]+)")
                .expect("Could not compile regex");

            let (_, [x, y]) = re.captures(line)
                .expect("Could not match pattern to line")
                .extract();

            self.x = x.parse::<i64>().expect("Parse failure");
            self.y = y.parse::<i64>().expect("Parse failure");
        }
    }

    pub fn solve(&self) -> Option<(i64, i64)> {
        let det = self.a.0 * self.b.1 - self.a.1 * self.b.0;

        if det == 0 { return None }

        let det_x = self.x * self.b.1 - self.y * self.b.0;
        let det_y = self.a.0 * self.y - self.a.1 * self.x;
        let (x, y) = (det_x / det, det_y / det);

        // Check fractional solutions
        if (det_x as f64 / det as f64).fract() != 0.0 ||
           (det_y as f64 / det as f64).fract() != 0.0 {
               return None
        }

        return if x > 0 && y > 0 { Some((x,y)) } else { None }
    }

    pub fn complete(&self) -> bool {
        self.x > 0 &&
        self.y > 0 &&
        self.a.0 > 0 && self.a.1 > 0 &&
        self.b.0 > 0 && self.b.1 > 0
    }
}
