//#[derive(Debug)]

//use std::{io::Read, thread::LocalKey};

struct BattleCharacter {
   health: i32,
    attack: i32,
    defence: i32,
    recovery: i32,
}

impl BattleCharacter{
    fn output(&self) {
        let healthy = &self.health;
        let attacky = &self.attack;
        let defencey = &self.defence;
        let recoveryy = &self.recovery;
        println!("Your character is {},{},{},{}",healthy,attacky,defencey,recoveryy);
    }
}
fn main() {
    let atri = create();
    atri.output();
}

fn create()-> BattleCharacter {
    BattleCharacter { health: 1000, 
        attack: 300, 
        defence: 200, 
        recovery: 100 }
}