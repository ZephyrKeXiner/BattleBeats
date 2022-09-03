//#[derive(Debug)]

use std::io;

struct BattleCharacter {
    health: i32,
    normal_attack: i32,
    defence: i32,
    recovery: i32,
}

impl BattleCharacter{
    fn output(&self) {       //A debug tool
        let healthy = &self.health;
        let attacky = &self.normal_attack;
        let defencey = &self.defence;
        let recoveryy = &self.recovery;
        println!("Your character is {},{},{},{}",healthy,attacky,defencey,recoveryy);
    }
}


fn main() {
    println!("Please choose your character: Atri/Tifa");
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Fail to read");

    let character = if input.trim() == "atri" {
        create_atri()
    }else {
        create_tifa()
    };
    
    character.output();
}

fn create_atri()-> BattleCharacter {         //Create a atri
    BattleCharacter { 
        health: 1000, 
        normal_attack: 200, 
        defence: 100, 
        recovery: 75
    }
}

fn create_tifa()-> BattleCharacter {        //Create a tifa
    BattleCharacter { 
        health: 1300, 
        normal_attack: 150, 
        defence: 146, 
        recovery: 87 
    }
}