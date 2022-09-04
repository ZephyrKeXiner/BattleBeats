//#[derive(Debug)]

use std::{io};
use std::cmp::Ordering; 
struct Atri {
    name: String,
    health: i32,
    normal_attack: i32,
    defence: i32,
    recovery: i32,
}

impl Atri {

    fn output(&self) {       //A debug tool
        println!("Your character is {},{},{},{}",
        &self.health,
        &self.normal_attack,
        &self.defence,
        &self.recovery);
    }

    fn rocket_fists(&self)-> i32 {
        300
    }
}

struct Tifa {
    name: String,
    health: i32,
    normal_attack: i32,
    defence: i32,
    recovery: i32,
}

impl Tifa {

    fn output(&self) {       //A debug tool
        println!("Your character is {},{},{},{}",
        &self.health,
        &self.normal_attack,
        &self.defence,
        &self.recovery);
    }
}
struct Monk{
    health: i32,
    normal_attack: i32,
    defence: i32,
}

impl Monk{

    fn new()-> Self {
        Monk { 
            health: 300,
            normal_attack: 40,
            defence: 5
        }
    }

    fn output(&self) {
        println!("Your enemy is Monk,{},{},{}", &self.health, &self.normal_attack, &self.defence);
    }
}   

struct Shaman {
    health: i32,
    normal_attack: i32,
    defence: i32,
    recovery: i32
}

struct Challenger {
    health: i32,
    normal_attack: i32,
    defence: i32,
}

struct Expect {
    
}

fn main() {
    println!("Please choose your character: Atri/Tifa");

    let mut input = String::new();

    io::stdin()         //process user's inputs
        .read_line(&mut input)
        .expect("Fail to read");
    
    let mut character = if input.trim() == "atri" || input.trim() == "Atri" {
        create_atri()
    }else {
        create_tifa()
    };
    
    let mut enemies = Monk::new(); 
    enemies.output();

    while character.health > 0 && enemies.health > 0 {         //main process part

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
                enemies.health -= character.normal_attack - enemies.defence;
            }else if user_input.trim() == "2" || user_input.trim() == "rocket fists"{
                enemies.health -= character.rocket_fists() - enemies.defence;
            }

            println!("The monster remains:{}",enemies.health);

        }else {
            if user_input.trim() == "1" || user_input.trim() == "normal attack" {
                enemies.health -= character.normal_attack - enemies.defence;
            }

            println!("The monster remains:{}",enemies.health);
        }

        character.health -= enemies.normal_attack - character.defence;
        println!("You remains:{}",character.health);

    }

    match character.health.cmp(&enemies.health) {
        Ordering::Greater => println!("The monster has been beaten.YOU WIN!"),
        Ordering::Less => println!("YOU has been beaten.YOU LOSE!"),
        Ordering::Equal => println!("BOOMMMMM!!!!,nobody win"),
    }
    
    
}

fn create_atri() -> Atri {
    Atri { 
        name: String::from("atri"),
        health: 100, 
        normal_attack: 100, 
        defence: 10, 
        recovery: 50 
    }
}

fn create_tifa() -> Tifa {
    Tifa { 
        name: String::from("tifa"),
        health: 130, 
        normal_attack: 70, 
        defence: 15, 
        recovery: 40
    }
}

fn create_monk() -> Monk {
    Monk { 
        health: 130, 
        normal_attack: 70, 
        defence: 15, 
    }
}