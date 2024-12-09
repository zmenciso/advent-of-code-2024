use itertools::Itertools;
use std::fmt;

#[derive (Debug)]
pub struct Disk {
    d: Vec<u16>
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut message = String::from("");

        for data in self.d.iter() {
            if *data == u16::MAX {
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

        let mut disk: Vec<u16> = Vec::new();

        // HERP DERP
        for (id, (blocks, free)) in dense.enumerate() {
            let file = vec![id as u16; blocks as usize];
            let space = vec![u16::MAX; free as usize];

            disk.extend(file);
            disk.extend(space);
        }

        Disk {
            d: disk
        }
    }

    /// Returns a tuple representing the start and end index of the block
    fn find_block(v: &[u16]) -> Option<(usize, usize)> {
        let end = v.iter().rposition(|&x| x < u16::MAX)
            .expect("No data left");
        let id = v[end];

        let mut start = end;
        while v[start] != id {
            let chk = start.checked_sub(1);
            match chk {
                Some(x) => { start = x; }
                None => { return None }
            }
        }

        return Some((start, end))
    }

    /// Returns a tuple represeting the start and end index of free space starting at bound
    fn find_free(v: &[u16], bound: usize) -> Option<(usize, usize)> {
        if let Some(start) = v[bound+1..].iter().position(|&x| x == u16::MAX) {
            let start = start + bound + 1;
            let mut end = start;

            while v[end] != u16::MAX {
                if end >= v.len() { return Some((start, end-1)) }

                let chk = end.checked_add(1);
                match chk {
                    Some(x) => { end = x; }
                    None => { return None }
                }
            }

            return Some((start, end))
        }
        else {
            return None
        }
    }

    /// Returns true if a block was moved
    fn move_block(&mut self, loc: usize) -> usize {
        // TODO: Kill this derpy, horrible code with fire
        let slice = &self.d[0..loc];

        let block = Self::find_block(slice);
        if block.is_none() { return 0 }
        let block = block.unwrap();
        let block_size = block.1 - block.0 + 1;

        let id = slice[block.0];

        let free = Self::find_free(slice, 0);
        if free.is_none() { return 0 }
        let mut search = free.unwrap();
        let mut free_size = search.1 - search.0 + 1;

        while free_size < block_size {
            let free = Self::find_free(slice, search.1);
            if free.is_none() { return 0 }
            search = free.unwrap();
            free_size = search.1 - search.0 + 1;
        }

        for i in search.0 .. search.0 + block_size {
            self.d[i] = id;
        }

        for i in block.0 .. block.0 + block_size {
            self.d[i] = u16::MAX;
        }

        block.0
    }

    pub fn defrag(&mut self) {
        let mut i = self.d.len();
        while i > 0 {
            i = Self::move_block(self, i);
        }
    }

    pub fn checksum(&self) -> u64 {
        self.d.iter().enumerate()
            .filter(|(_, &val)| val != u16::MAX)
            .map(|(i, &x)| i as u64 * x as u64)
            .sum()
    }
}
