//#[derive(Debug)]

use std::{io};
use std::cmp::Ordering; 
struct BattleCharacter {
    name: String,
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

    fn rocket_fists(&self)-> i32 {
        300
    }
}

struct Enemy{
    health: i32,
    normal_attack: i32,
    defence: i32,
}

impl Enemy{
    fn output(&self) {       //A debug tool
        let healthy = &self.health;
        let attacky = &self.normal_attack;
        let defencey = &self.defence;
        println!("Your character is {},{},{}",healthy,attacky,defencey);
    }

    fn rain(&self) -> i32 {
        14
    }
}   


fn main() {
    println!("Please choose your character: Atri/Tifa");
    let mut input = String::new();

    io::stdin()         //process user's inputs
        .read_line(&mut input)
        .expect("Fail to read");

    let mut character = if input.trim() == "atri" {
        create_atri()
    }else {
        create_tifa()
    };
    
    let mut monsters = create_monsters(); 
    character.output();
    monsters.output();

    while character.health > 0 && monsters.health > 0 {         //main process part
        let mut i = 1;
        while i % 2 == 1 {
            let mut user_input = String::new();

            println!("It's your turn");
            if character.name == String::from("atri") {
                println!("You can use:\n1.normal attack\n2.rocket fists");
            }else {
                println!("You can use:\n1.normal attack");
            }

            io::stdin()
                .read_line(&mut user_input)
                .expect("Fail to read");

            if character.name == String::from("atri") {
                if user_input.trim() == "1" || user_input.trim() == "normal attack" {
                    monsters.health -= character.normal_attack - monsters.defence;
                }else if user_input.trim() == "2" || user_input.trim() == "rocket fists"{
                    monsters.health -= character.rocket_fists() - monsters.defence;
                }

                println!("The monster remains:{}",monsters.health);

            }else {
                if user_input.trim() == "1" || user_input.trim() == "normal attack" {
                    monsters.health -= character.normal_attack - monsters.defence;
                }

                println!("The monster remains:{}",monsters.health);
            }

            character.health -= monsters.normal_attack - character.defence;
            println!("You remains:{}",character.health);

            i += 1;
        } 
    }

    match character.health.cmp(&monsters.health) {
        Ordering::Greater => println!("The monster has been beaten.YOU WIN!"),
        Ordering::Less => println!("YOU has been beaten.YOU LOSE!"),
        Ordering::Equal => println!("BOOMMMMM!!!!,nobody win"),
    }
    
    
}



fn create_atri()-> BattleCharacter {         //Create a atri
    BattleCharacter { 
        name: String::from("atri"),
        health: 100, 
        normal_attack: 100, 
        defence: 10, 
        recovery: 50
    }
}

fn create_tifa()-> BattleCharacter {        //Create a tifa
    BattleCharacter { 
        name: String::from("tifa"),
        health: 130, 
        normal_attack: 70, 
        defence: 15, 
        recovery: 40 
    }
}

fn create_monk()-> Enemy {
    Enemy { 
        health: 300,
        normal_attack: 40,
        defence: 5
    }
}

fn create_shaman()-> Enemy {
    
}