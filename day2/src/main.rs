use std::fs;

fn compute(b: &mut Vec<usize>, brute1: usize, brute2: usize) -> usize {
    let mut i: usize = 0;
    b[1] = brute1;
    b[2] = brute2;
    while b[i] != 99 || i >= b.len() {
        let a = b[i + 3];
        match b[i] {
            1 => {
                b[a] = b[b[i + 1]] + b[b[i + 2]];
                i += 4
            }
            2 => {
                b[a] = b[b[i + 1]] * b[b[i + 2]];
                i += 4
            }
            _ => {
                println!("{} {}", brute1, brute2);
                panic!("must be 1 or 2");
            }
        }
    }
    b[0]
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let mut b = Vec::new();

    b.extend(
        input
            .split(",")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<usize>().unwrap()),
    );
    let c = b.clone();
    let mut i = 0;
    let mut j = 0;
    'outer: while i < 100 {
        while j < 100 {
            if compute(&mut b, i, j) == 19_690_720 {
                println!("{}", i * 100 + j);
                break 'outer;
                
            }
            j += 1;
            b = c.clone();
        }
        i += 1;
        j = 0;
    }
}
