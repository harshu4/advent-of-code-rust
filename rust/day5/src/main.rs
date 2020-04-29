use std::fs;

fn compute(b: &mut Vec<i32>) -> usize {
    let mut i: usize = 0;
    while b[i] != 99 || i >= b.len() {
        let a = b[i as usize + 3] as usize;
        let mut ls: Vec<u32> = b[i as usize]
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
            .into_iter()
            .collect();
        ls.reverse();
        for x in 0..5 - ls.len() {
            ls.push(0);
        }
        println!("{:?}", ls);
        let lo;
        let ko;
        let bo;

        if (ls[2] == 1) {
            ko = i + 1
        } else {
            ko = b[i + 1] as usize
        }
        if (ls[3] == 1) {
            bo = i + 2
        } else {
            bo = b[i + 2] as usize
        }
        if (ls[4] == 1) {
            lo = i as usize + 3
        } else {
            lo = b[i + 3] as usize
        }
        
        match ls[0] {
            1 => {
                b[lo] = b[ko] + b[bo];
                i += 4
            }
            2 => {
                b[lo] = b[ko] * b[bo];
                i += 4
            }
            3 => {
                b[ko] = 1;
                i += 2
            }
            4 => {
                
                println!("{}", b[ko]);
                i += 2
            }
           
            _ => {
                panic!("must be 1 or 2");
            }
        }
    }
    b[0] as usize
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut b = Vec::new();

    b.extend(
        input
            .split(",")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>(),
    );
    compute(&mut b);
}
