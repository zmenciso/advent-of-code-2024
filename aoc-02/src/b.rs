use std::io;

fn safety_check(v: &Vec<i32>, damper: bool) -> bool {
    let mut prev: i32 = v[0];
    let mut ascending: Option<bool> = None;
    let mut output = true;

    for (i, num) in v.into_iter().enumerate() {
        if i == 0 { continue };

        // Set ascending
        if ascending.is_none() {
            ascending = if *num > prev { Some(true) } else { Some(false) };
        }

        if (ascending.unwrap() && ((prev >= *num) || (*num > (prev + 3)))) ||
            (!ascending.unwrap() && ((prev <= *num) || ((*num + 3) < prev))) {
                match damper {
                    // If damper is already on, return false
                    true => { return false },
                    // If damper is off, retry with each num removed
                    // TODO: This is horrendously inefficient
                    false => { 
                        output = false;
                        for j in 0..v.len() {
                            let mut t: Vec<i32> = Vec::new();
                            t.extend_from_slice(&v[..j]);
                            t.extend_from_slice(&v[j+1..]);
                            if safety_check(&t, true) { return true };
                        }
                    }
                }
            }
        else {
            prev = *num
        }
    }

    output
}

fn main() {
    let mut count: usize = 0;

    for line in io::stdin().lines() {
        let line = line.expect("Failed to read line");
        let split = line.split_whitespace()
            .map(|x| x.parse::<i32>().expect("Could not parse number"))
            .collect();

        if safety_check(&split, false) { 
            count += 1 };
    }

    println!("{:?}", count);
}
