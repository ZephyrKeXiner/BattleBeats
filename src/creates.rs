mod creates{
    pub mod create_characters{

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
        
    }

    pub mod create_enemies{
        
        fn create_monsters()-> Enemy {
            Enemy { 
                health: 500,
                normal_attack: 30,
                defence: 10 
            }
        }
        
    }
}