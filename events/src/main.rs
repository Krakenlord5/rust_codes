use std::io;
use rand::Rng;

struct Player {
    hp: i32,
    stamina: i32,
    power: i32,
    gold: i32,
    max_hp: i32
}

#[derive(Clone)]
struct Enemy {
    name: String,
    hp: i32,
    stamina: i32,
    power:i32
}

enum Encounter {
    Nothing,
    Bush,
    Meat,
    Water,
    Herb,
    Iron,
    Enemy(Enemy)
}

impl Player {
    fn new() -> Self {
        Player {
            hp: 100,
            stamina: 50,
            power: 10,
            gold: 0,
            max_hp: 100
        }
    }

    fn show_stats(&self) {
        println!("HP: {}, Stamina: {}, Power: {}, Gold: {}", self.hp, self.stamina, self.power, self.gold);
    }

    fn get_encounter(e: &[Enemy; 5]) -> Encounter {
        let n = rand::thread_rng().gen_range(1..=100);
        match n {
            1..=17 => Encounter::Nothing,
            18..=25 => Encounter::Bush,
            26..=40 => Encounter::Meat,
            41..=55 => Encounter::Water,
            56..=70 => Encounter::Herb,
            71..=75 => Encounter::Iron,
            _ => {
                let enemy: Enemy = random_enemy(e);
                Encounter::Enemy(enemy)
            }
        }
    }

    fn apply_encounter(&mut self, en: Encounter) {
        match en {
            Encounter::Nothing => println!("Nothing, go next."),
            Encounter::Bush => {
                println!("You stumble upon a bush, stamina decreases by 1.");
                self.stamina -= 1
            }
            Encounter::Meat => {
                println!("You found a meat, hp increases by 4. YUM!");
                self.hp += 4
            }
            Encounter::Water => {
                println!("You found water, stamina increases by 2.");
                self.stamina += 2
            }
            Encounter::Herb => {
                println!("You found a herb, power increases by 1.");
                self.power += 1
            }
            Encounter::Iron => {
                println!("You found iron, power increases by 10!");
                self.power += 10
            }
            Encounter::Enemy(enemy) => {
                println!("You found an enemy!");
                println!("Name: {} HP {} Stamina {} Power {}", enemy.name, enemy.hp, enemy.stamina, enemy.power);
                self.fight_or_flee(enemy)
            }
        }
    }

    fn fight_or_flee(&mut self, mut e: Enemy) {
        let mut n = rand::thread_rng();
        let winning_gold: i32 = e.hp;
        loop {
            let mut choice: String = String::new();
            println!("(1) Fight! (2) Flee");
            io::stdin().read_line(&mut choice).expect("Failed to read line");
            match choice.trim().parse() {
                Ok(1) => {
                    self.stamina -= 1;
                    if self.stamina <= 0 {
                        break
                    }
                    if n.gen_range(1..=100) <= 100 - e.stamina / 2 {
                        println!("You hit the enemy! Dropping their HP by {}!", self.power);
                        e.hp -= self.power
                    } else {
                        println!("You missed them!");
                    }

                    if e.hp > 0 {
                        if n.gen_range(1..=100) <= 100 - self.stamina / 2 {
                            println!("The enemy hit you! Dropping your HP by {}!", e.power);
                            self.hp -= e.power;
                            if self.hp <= 0 {
                                break
                            }
                        } else {
                            println!("They missed you!")
                        }
                    } else {
                        println!("The enemy is dead! You gain {} gold!", winning_gold);
                        self.gold += winning_gold;
                        break
                    }
                }
                Ok(2) => {
                    self.stamina -= 2;
                    if n.gen_range(1..=100) <= 90 + self.stamina - e.stamina {
                        println!("You successfully escaped the {}!", e.name);
                        break
                    } else {
                        println!("You couldn't escape them!");
                        if n.gen_range(1..=100) <= 100 - self.stamina / 2 {
                            println!("The enemy hit you! Dropping your HP by {}!", e.power);
                            self.hp -= e.power;
                            if self.hp <= 0 {
                                break
                            }
                        } else {
                            println!("They missed you!")
                        }
                    }
                }
                Ok(_) => {
                    println!("Invalid choice");
                    continue
                }
                Err(_) => {
                    println!("Invalid choice");
                    continue
                }
            }

            print!("YOU: ");
            self.show_stats();
            println!("ENEMY: HP: {}, Stamina: {}, Power: {}", e.hp, e.stamina, e.power);
            
        }
    }
}

fn main() {
    let mut player_1: Player = Player::new();
    let enemies: [Enemy; 5] = [
        Enemy {
            name: "Rat".to_string(),
            hp: 10,
            stamina: 10,
            power: 2
        },
        Enemy {
            name: "Wolf".to_string(),
            hp: 20,
            stamina: 20,
            power: 10
        },
        Enemy {
            name: "Boar".to_string(),
            hp: 30,
            stamina: 40,
            power: 20
        },
        Enemy {
            name: "Tiger".to_string(),
            hp: 40,
            stamina: 50,
            power: 30
        },
        Enemy {
            name: "Dragon".to_string(),
            hp: 60,
            stamina: 60,
            power: 40
        },
    ];

    loop {
        let mut n: String = String::new();
        println!("Which direction do you want to walk?");
        println!("(1) North ,(2) South ,(3) East ,(4) West ,(9) Exit the game");
        io::stdin().read_line(&mut n).expect("Failed to read line");
        match n.trim().parse() {
            Ok(1..=4) => {
                player_1.stamina -= 1;
                if player_1.stamina <= 0 {
                    println!("GAME OVER, YOU RAN OUT OF STAMINA");
                    player_1.show_stats();
                    break
                }
                let encounter: Encounter = Player::get_encounter(&enemies);
                player_1.apply_encounter(encounter);
                if player_1.stamina <= 0 {
                    println!("GAME OVER, YOU RAN OUT OF STAMINA");
                    player_1.stamina = 0;
                    player_1.show_stats();
                    break
                }
                if player_1.hp <= 0 {
                    println!("GAME OVER, YOU DIED");
                    player_1.hp = 0;
                    player_1.show_stats();
                    break
                }
                if player_1.gold >= 200 {
                    println!("YOU WIN THE GAME, CONGRATS!");
                    player_1.show_stats();
                    break
                }
                if player_1.hp > player_1.max_hp {
                    player_1.hp = player_1.max_hp
                }
                player_1.show_stats();
            },
            Ok(9) => {
                println!("BYE BYE"); 
                break
            },
            Ok(_) => {
                println!("Invalid command");
                continue
            },
            Err(_) => {
                println!("Invalid command");
                continue
            }
        }
    }
}

fn random_enemy(e: &[Enemy; 5]) -> Enemy {
    let n = rand::thread_rng().gen_range(1..=100);
    match n {
        1..=45 => e[0].clone(),
        46..=70 => e[1].clone(),
        71..=85 => e[2].clone(),
        86..=95 => e[3].clone(),
        _ => e[4].clone()
    }
}