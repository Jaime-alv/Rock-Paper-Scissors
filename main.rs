use std::io;

use rand::Rng;

fn main() {
    println!("Hello!");
    println!("Rock, paper, scissors!");

    let mut game = Game {
        ..Default::default()
    };

    println!("Tell me your name");
    let binding = input();
    let name = capitalize(binding.trim());

    let mut human: Player = Player {
        name,
        roll: Roll::Rock,
    };

    loop {
        println!("Round {} of {}", game.round + 1, game.max_game);

        let mut pc: Player = Player {
            name: String::from("pc"),
            roll: Roll::Rock,
        };

        let secret = pc.get_secret();
        pc.roll = pc.roll.resolve_trait(secret);

        // println!("pc= {:?}", pc.roll);

        let roll = human.ask_for_guess();
        human.roll = human.roll.resolve_trait(roll);

        let victory = game.compare_results(&human, &pc);

        game.resolve_victory(&victory);

        victory.show_victory(&human);

        game.show_victory_chart();

        if !game.active {
            game.show_results(&human);
            break;
        }
    }
}


#[derive(Debug, PartialEq)]
enum Roll {
    Rock,
    Paper,
    Scissors,
}

enum Victory {
    Win,
    Loose,
    Tie,
}

impl Roll {    
    fn resolve_trait(&mut self, int: u8) -> Roll {
        match int {
            1 => Self::Rock,
            2 => Self::Paper,
            _ => Self::Scissors,
        }
    }
}

impl Victory {
    fn show_victory(&self, human: &Player) {
        match self {
            Victory::Loose => {
                println!("You Loose: {}!", human.name)
            }
            Victory::Win => {
                println!("You win: {}!", human.name)
            }
            Victory::Tie => {
                println!("It's a tie!")
            }
        }
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    roll: Roll,
}

struct Game {
    active: bool,
    max_game: u16,
    tie: u16,
    round: u16,
    human_wins: u16,
    pc_wins: u16,
}

impl Default for Game {
    fn default() -> Game {
        Game {
            active: true,
            max_game: 5,
            round: 0,
            tie: 0,
            human_wins: 0,
            pc_wins: 0,
        }
    }
}

impl Game {
    fn resolve_victory(&mut self, victory: &Victory) {
        match victory {
            Victory::Loose => {
                self.pc_wins += 1;
            }
            Victory::Tie => {
                self.tie += 1;
            }
            Victory::Win => {
                self.human_wins += 1;
            }
        }
    }

    fn show_victory_chart(&mut self) {
        if self.round < self.max_game {
            println!("")
        } else {
            println!("Game over\n ");
            self.active = false;
        }
    }

    fn show_results(&self, human: &Player) {
        println!("Score for {}:", human.name);
        println!("wins = {}", self.human_wins);
        println!("looses = {}", self.pc_wins);
        println!("ties = {}", self.tie);

        println!("\nOverall winner:");
        if self.tie > self.human_wins && self.tie > self.pc_wins {
            println!("Nobody, it's a tie")
        } else if self.human_wins > self.pc_wins {
            println!("{}", human.name)
        } else {
            println!("pc")
        }
    }

    fn compare_results(&mut self, human: &Player, pc: &Player) -> Victory {
        self.round += 1;
        if human.roll == pc.roll {
            Victory::Tie
        } else if human.roll == Roll::Paper && pc.roll == Roll::Rock
            || human.roll == Roll::Rock && pc.roll == Roll::Scissors
            || human.roll == Roll::Scissors && pc.roll == Roll::Paper
        {
            Victory::Win
        } else {
            Victory::Loose
        }
    }
}

impl Player {
    fn ask_for_guess(&self) -> u8 {
        println!("Choose between 1.Rock, 2.Paper, 3.Scissors");

        let number: u8 = match input().trim().parse() {
            Ok(num) => num,
            Err(_) => self.ask_for_guess(),
        };
        if number > 0 && number < 4 {
            number
        } else {
            self.ask_for_guess()
        }
    }

    fn get_secret(&self) -> u8 {
        rand::thread_rng().gen_range(1..=3)
    }
}

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn capitalize(string: &str) -> String {
    let mut c = string.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
