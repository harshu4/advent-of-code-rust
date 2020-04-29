use std::collections::HashMap;
use std::fs;
#[derive(Clone, Debug)]
pub struct Obj {
    name: String,
    follows: Option<usize>,
    followers: Vec<usize>,
}

impl Obj {
    fn new(name: String) -> Obj {
        Obj {
            name,
            follows: Option::None,
            followers: vec![],
        }
    }
    fn add(&mut self, a: usize) -> Result<(), &'static str> {
        self.follows = Some(a);
        Ok(())
    }
    fn addf(&mut self, a: usize) -> () {
        self.followers.push(a)
    }
}

fn main() {
    let content = fs::read_to_string("src/input.txt").unwrap();
    let mut arr: Vec<Obj> = Vec::new();
    let mut visited: Vec<usize> = Vec::new();
    let mut a_found: i32 = -1;
    let mut b_found: i32 = -1;
    let mut index = 0;

    for x in content.lines() {
        for y in &arr {
            if y.name == &x[0..3] {
                b_found = index;
            }

            if y.name == &x[4..7] {
                a_found = index;
            }

            index += 1;
        }
        index = 0;
        if a_found == -1 {
            arr.push(Obj::new(x[4..7].to_string()));
            a_found = (arr.len() - 1) as i32;
        }

        if b_found == -1 {
            arr.push(Obj::new(x[0..3].to_string()));
            b_found = (arr.len() - 1) as i32;
        }

        arr[a_found as usize].add((b_found) as usize);
        arr[b_found as usize].addf((a_found) as usize);
        a_found = -1;
        b_found = -1;
    }

    let mut count = 0;
    rec_count(&mut visited, &arr, 1359, count);
}

pub fn rec_count(visited: &mut Vec<usize>, main: &Vec<Obj>, curr: usize, count: i32) -> i32 {
    visited.push(curr);

    if (main[curr].name == "SAN") {
        println!("san reached {}", count - 2);
        return count + 1;
    }

    let mut arre = vec![];
    let mut follows: Option<usize> = Option::None;

    for x in &main[curr].followers {
        if !visited.contains(x) {
            arre.push(x);
        }
    }
    if (!main[curr].follows.is_none()) {
        if (!visited.contains(&main[curr].follows.unwrap())) {
            follows = Option::Some(main[curr].follows.unwrap());
        }
    }

    if (follows != Option::None) {
        rec_count(visited, main, follows.unwrap(), count + 1);
    }
    if (arre.len() != 0) {
        for x in &arre {
            if (!visited.contains(x)) {
                rec_count(visited, main, **x, count + 1);
            }
        }
    }
    return 0;
}
