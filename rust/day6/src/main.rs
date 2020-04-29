use std::fs;
use std::collections::HashMap;

struct Obj<'a> {
    name: String,
    follows: Option<&'a Obj<'a>>,
}

impl<'a> Obj<'a> {
    fn new(name: String) -> Obj<'a> {
        Obj {
            name,
            follows: Option::None,
        }
    }
    fn add(&mut self, a: &'a Obj) -> Result<(), &'static str> {
        self.follows = Option::Some(a);
        Ok(())
    }
}

fn main() {
    let content = fs::read_to_string("src/input.txt").unwrap();
    let mut arr = HashMap::new();
    
    for x in content.lines(){
        if !arr.contains_key(&x[4..7]){
            arr.insert(x[4..7].to_string(),Obj::new(x[4..7].to_string()));
        }
        if !arr.contains_key(&x[0..3]){
            arr.insert(x[0..3].to_string(),Obj::new(x[0..3].to_string()));
        }
        let mut abc = arr.get_mut(&x[4..7]).unwrap();
        let obj = *arr.get(&x[0..3]).unwrap().clone();
        


    }
}
