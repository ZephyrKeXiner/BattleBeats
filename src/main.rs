#[derive(Debug)]

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
    let atri = create();
    println!("{:?}", &atri);
}

fn create()-> BattleCharacter {
    BattleCharacter { health: 1000, 
        attack: 300, 
        defence: 200, 
        recovery: 100 }
}