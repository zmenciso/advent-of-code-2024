use std::io;
// use std::collections::HashSet;

fn operate(v: &[u64], opr: &[char]) -> u64 {
    let mut result = v[0];

    for (i, &op) in opr.iter().enumerate() {
        match op {
            '+' => result += v[i+1],
            '*' => result *= v[i+1],
            _ => panic!("Invalid operation"),
        }
    }

    result
}

fn gen_ops(n: usize) -> Vec<Vec<char>> {
    let mut opr: Vec<Vec<char>> = Vec::new();

    for i in 0 .. 1 << (n-1) {
        let mut o = Vec::with_capacity(n-1);
        for j in 0 .. (n-1) {
            if (i & (1 << j)) == 0 {
                o.push('*');
            }
            else {
                o.push('+');
            }
        }

        opr.push(o);
    }

    opr
}

fn main() {
    let mut count: u64 = 0;

    for line in io::stdin().lines() {
        let line = line.expect("Could not parse line");

        let (val, eq) = line.split_once(':')
            .expect("Invalid line");

        let val = val.parse::<u64>()
            .expect("Could not parse value");

        let eq: Vec<u64> = eq.trim().split_whitespace()
            .map(|x| x.parse::<u64>().expect("Could not parse value"))
            .collect();

        for opr in gen_ops(eq.len()) {
            if operate(&eq, &opr) == val {
                count += val;
                break;
            }
        }
    }

    println!("{:?}", count);
}
