use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let lo = fs::read_to_string("src/input.txt").unwrap();
    let path = lo.split("\n").collect::<Vec<&str>>();
    let path1 = path[0].split(",").collect::<Vec<&str>>();
    let path2 = path[1].split(",").collect::<Vec<&str>>();
    let mut prev: Point = Point::new(0, 0);
    let mut inst: &str;
    let mut arr2 = HashMap::new();
    let mut mov: i32;

    let mut steps = 0;
    for x in path1 {
        inst = &x[0..1];
        mov = x[1..].parse::<i32>().unwrap();
        match inst {
            "R" => {
                for x in 0..mov {
                    steps += 1;
                    arr2.insert(Point::new(prev.x + 1, prev.y), steps);
                    prev.x = prev.x + 1;
                }
            }
            "L" => {
                for x in 0..mov {
                    steps += 1;
                    arr2.insert(Point::new(prev.x - 1, prev.y), steps);
                    prev.x = prev.x - 1
                }
            }
            "U" => {
                for x in 0..mov {
                    steps += 1;
                    arr2.insert(Point::new(prev.x, prev.y + 1), steps);
                    prev.y = prev.y + 1;
                }
            }
            "D" => {
                for x in 0..mov {
                    steps += 1;
                    arr2.insert(Point::new(prev.x, prev.y - 1), steps);
                    prev.y = prev.y - 1;
                }
            }
            _ => {
                panic!("hello");
            }
        }
    }
    println!("step one completed");
    steps = 0;
    prev = Point::new(0, 0);
    let mut min = 500000;
    for x in path2 {
        inst = &x[0..1];
        mov = x[1..].parse::<i32>().unwrap();

        match inst {
            "R" => {
                for x in 0..mov {
                    steps += 1;
                    prev.x = prev.x + 1;
                    prev.y = prev.y;
                    if arr2.contains_key(&prev) {
                        if *arr2.get(&prev).unwrap() + steps < min {
                            min = *arr2.get(&prev).unwrap() + steps;
                        }
                    }
                }
            }
            "L" => {
                for x in 0..mov {
                    steps += 1;
                    prev.x = prev.x - 1;
                    prev.y = prev.y;

                    if arr2.contains_key(&prev) {
                        if *arr2.get(&prev).unwrap() + steps < min {
                            min = *arr2.get(&prev).unwrap() + steps;
                        }
                    }
                }
            }
            "U" => {
                for x in 0..mov {
                    steps += 1;
                    prev.x = prev.x;
                    prev.y = prev.y + 1;

                    if arr2.contains_key(&prev) {
                        if *arr2.get(&prev).unwrap() + steps < min {
                            min = *arr2.get(&prev).unwrap() + steps;
                        }
                    }
                }
            }
            "D" => {
                for x in 0..mov {
                    steps += 1;
                    prev.x = prev.x;
                    prev.y = prev.y - 1;
                    if arr2.contains_key(&prev) {
                        if *arr2.get(&prev).unwrap() + steps < min {
                            min = *arr2.get(&prev).unwrap() + steps;
                        }
                    }
                }
            }
            _ => {
                panic!("hello");
            }
        }
    }

    println!("{:?}", min);
}
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
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
