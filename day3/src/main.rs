use std::fs;
use std::collections::HashSet;

fn main() {
    let lo = fs::read_to_string("src/input.txt").unwrap();
    let path = lo.split("\n").collect::<Vec<&str>>();
    let path1 = path[0].split(",").collect::<Vec<&str>>();
    let path2 = path[1].split(",").collect::<Vec<&str>>();
    let mut prev: Point = Point::new(0, 0);
    let mut inst: &str;
    let mut arr = HashSet::new();
    let mut mov: i32;
 


    for x in path1 {
        inst = &x[0..1];
        mov = x[1..].parse::<i32>().unwrap();
        match inst {
            "R" => {
                
                for x in 0..mov {
                    arr.insert(Point::new(prev.x + 1, prev.y));
                    prev.x = prev.x+1;
                    
                }
            }
            "L" => {
                
                for x in 0..mov {
                    arr.insert(Point::new(prev.x - 1, prev.y));
                    prev.x = prev.x-1
                    
                }
            }
            "U" => {
                
                for x in 0..mov {
                    arr.insert(Point::new(prev.x, prev.y+1));
                    prev.y = prev.y+1;
                    
                }
            },
            "D" => {
                
                for x in 0..mov {
                    arr.insert(Point::new(prev.x, prev.y-1));
                    prev.y = prev.y-1;
                    
                }
            },
            _ => {
                panic!("hello");
            }

            
        }
    }
    println!("step one completed");
    prev = Point::new(0,0);
    let mut min = 50000;
    for x in path2 {
        

        inst = &x[0..1];
        mov = x[1..].parse::<i32>().unwrap();
        
      
        match inst {
            "R" => {
                
                for x in 0..mov {
                    prev.x = prev.x+1;
                    prev.y = prev.y;
                    if arr.contains(&prev) {
                       
                       
                        prev.distfromsrc();
                        if prev.dis.unwrap() < min {
                            min = prev.dis.unwrap()
                        }
                        prev.dis = Option::None;
                        
                    }
                    
                }
            }
            "L" => {
                
                for x in 0..mov {

                    prev.x = prev.x-1;
                    prev.y = prev.y;
                
                    if arr.contains(&prev) {
                       
                        prev.distfromsrc();
                        if prev.dis.unwrap() < min {
                            min = prev.dis.unwrap()
                        }
                        prev.dis = Option::None;
                    }
                    
                }
            }
            "U" => {
                
                for x in 0..mov {
                    prev.x = prev.x;
                    prev.y = prev.y+1;
                  
                   if arr.contains(&prev) {
                    
                    prev.distfromsrc();
                    if prev.dis.unwrap() < min {
                        min = prev.dis.unwrap()
                    }
                    prev.dis = Option::None;
                }
                    
                }
            },
            "D" => {
                
                for x in 0..mov {
                    prev.x = prev.x;
                    prev.y = prev.y-1;
                    if arr.contains(&prev) {
                      
                        prev.distfromsrc();
                        if prev.dis.unwrap() < min {
                            min = prev.dis.unwrap()
                        }
                        prev.dis = Option::None;
                    }
                    
                }
            },
            _ => {
                panic!("hello");
            }

            
        }
      
    }
  
    println!("{:?}",min);
    
}
#[derive(Debug,PartialEq,Clone,Eq,Hash)]
struct Point {
    x: i32,
    y: i32,
    dis: Option<u32>,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
            dis: Option::None,
        }
    }

    fn distfromsrc(&mut self) {
        if self.dis.is_none() {
            self.dis = Some(((self.x).abs() + (self.y).abs()) as u32);
            
        }
    }
}
