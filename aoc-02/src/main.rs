use std::io;

fn safety_check<I>(iter: &mut I) -> bool
where 
    I: Iterator<Item = i32>,
{
    let mut prev: i32 = iter.next().expect("No value found");
    let mut ascending: Option<bool> = None;

    for num in iter {
        // Set ascending
        if ascending.is_none() {
            ascending = if num > prev { Some(true) } else { Some(false) };
        }
        match ascending {
            Some(true) => {
                if (prev >= num) || (num > (prev + 3)) { return false }
            },
            Some(false) => {
                if (prev <= num) || ((num + 3) < prev) { return false }
            }
            None => { eprintln!("We shouldn't be here!") },
        }
        prev = num;
    }

    true
}

fn main() {
    let mut count: usize = 0;

    for line in io::stdin().lines() {
        let line = line.expect("Failed to read line");
        let mut split = line.split_whitespace()
            .map(|x| x.parse::<i32>().expect("Could not parse number"))
            .into_iter();

        if safety_check(&mut split) { count += 1 };
    }

    println!("{:?}", count);
}
