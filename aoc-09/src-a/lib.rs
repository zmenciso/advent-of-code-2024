use itertools::Itertools;
use std::fmt;

#[derive (Debug)]
pub struct Disk {
    d: Vec<i16>
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut message = String::from("");

        for data in self.d.iter() {
            if *data < 0 {
                message = format!("{}.", message);
            }
            else {
                message = format!("{}{}", message, *data);
            }
        }

        write!(f, "{}", message)
    }
}

impl Disk {
    pub fn new(line: &str) -> Disk {
        let dense = line.chars()
            .filter_map(|c| c.to_digit(10))
            .map(|x| x as u8)
            .tuples();

        let mut disk: Vec<i16> = Vec::new();

        // HERP DERP
        for (id, (blocks, free)) in dense.enumerate() {
            let file = vec![id as i16; blocks as usize];
            let space = vec![-1_i16; free as usize];

            disk.extend(file);
            disk.extend(space);
        }

        Disk {
            d: disk
        }
    }

    /// Returns false when no blocks moved (fully defragged)
    fn move_block(&mut self) -> bool {
        let i = self.d.iter().rposition(|&x| x != -1)
            .expect("No data");

        if !self.d[0..i].contains(&-1) {
            // Disk fully defragged
            return false
        }
        
        let free = self.d.iter().position(|&x| x == -1)
            .expect("No free space");

        let block = self.d.remove(i);
        self.d[free] = block;
        self.d.push(-1_i16);

        true
    }

    pub fn defrag(&mut self) {
        while Self::move_block(self) {
            // println!("{}", self);
        }
    }

    pub fn checksum(&self) -> u64 {
        let mut sum: u64 = 0;

        for (i, val) in self.d.iter().filter(|&x| *x >= 0).enumerate() {
            sum += i as u64 * *val as u64;
        }

        sum
    }
}
