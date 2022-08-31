//#[derive(Debug)]

use std::{io::Read, thread::LocalKey};

struct BattleCharacter {
   health: i32,
    attack: i32,
    defence: i32,
    recovery: i32,
}

impl BattleCharacter{
    
}
fn main() {
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    //let mut vector = Vec::new();
    file.read_to_string(&mut contents).unwrap();

    let vector = contents.lines().map(|i| {
        i.parse::<i32>()
    }).collect::<Vec<_>>();
    
    for x in vector {
        println!("{:?}",x);
    }
    //println!("{:?}",vector);
}

//fn readfile(&x:Vec<i32>) {
//    
//}
