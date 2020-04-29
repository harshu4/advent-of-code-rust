use std::collections::HashMap;
use std::fs;
#[derive(Clone, Debug)]
struct Obj {
    name: String,
    follows: Option<usize>,
}

impl Obj {
    fn new(name: String) -> Obj {
        Obj {
            name,
            follows: Option::None,
        }
    }
    fn add(&mut self, a: usize) -> Result<(), &'static str> {
        self.follows = Some(a);
        Ok(())
    }
}

fn main() {
    let content = fs::read_to_string("src/input.txt").unwrap();
    let mut arr: Vec<Obj> = Vec::new();
    let mut a_found:i32 = -1;
    let mut b_found:i32 = -1;
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
            println!("{}",arr.len());
            a_found = (arr.len() - 1) as i32;
        }

        if b_found == -1 {
            arr.push(Obj::new(x[0..3].to_string()));
            println!("{}",arr.len());
            b_found = (arr.len() - 1) as i32 ;
        }

        arr[a_found as usize].add((b_found) as usize);

        a_found = -1;
        b_found = -1;
       
    }
    println!("{:?}",arr);
    let mut count = 0;
    let mut y = &arr[0];
    for x in &arr {
        y = x;
        while y.follows.is_some() {
            count += 1;
            y = &arr[y.follows.unwrap()]

        }
        
    }
    println!("{}", count)
}
