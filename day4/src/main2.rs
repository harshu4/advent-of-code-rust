fn main() {
    let mut ins = 0;
    for x in 357253..892942 {
        let mut list = x
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        let mut x = list.len();
        if !is_sorted(&list) {
            continue;
        }

        if !is_adjacent(&list) {
            continue;
        }
        ins += 1;
    }
    println!("{}", is_adjacent(&vec![1, 1, 1, 1, 2, 2]));
    println!("{}", ins);
}

fn is_sorted(ro: &Vec<u32>) -> bool {
    let mut prev = ro[0];
    for x in ro {
        if *x >= prev {
            prev = *x
        } else {
            return false;
        }
    }
    true
}

fn is_adjacent(ro: &Vec<u32>) -> bool {
    let mut prev = 20000;
    let mut repeat = 1;
    let mut cur: i32 = -1;
    let mut vec = vec![];
    for x in ro {
        if *x == prev {
            repeat += 1;
            vec[cur as usize] = vec[cur as usize] + 1;
        } else {
            repeat = 1;
            vec.push(repeat);
            cur += 1;
        }
        prev = *x;
    }

    vec.contains(&2)
}
