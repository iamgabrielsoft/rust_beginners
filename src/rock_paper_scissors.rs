
use std::fmt; 
use std::io::{self, Write}; 
use std::cmp::{Ord, Ordering};
use rand::seq::SliceRandom; 




/// The chosen move: either rock, paper, or scissors
#[derive(Eq)]
pub struct RPS {
    chosen: String,
}



impl RPS {
    /// Rock constant so I don't have to write rock every time
    pub const ROCK: &'static str = "rock";
    /// Paper constant so I don't have to write paper every time
    pub const PAPER: &'static str = "paper";
    /// scissors constant so I don't have to write scissors every time
    pub const SCISSORS: &'static str = "scissors";


    pub fn new(chosen: String) -> Option<RPS>{
        if[RPS::ROCK, RPS::PAPER, RPS::SCISSORS].contains(&chosen.as_ref())  {
            return Some(RPS { chosen });
        }
        
        None
    }


    /// Selects rock paper or scissors at random and sets it to `chosen`
    pub fn default() -> RPS {
        let mut rng = rand::thread_rng(); 
        RPS {
            chosen: [RPS::ROCK, RPS::PAPER, RPS::SCISSORS].choose(&mut rng).unwrap().to_string()
        }
      
    }


    //function to get user input 
    pub fn get_input(&mut self) -> bool {
        let mut player_move = String::new(); 
        print!("ROCK, Paper, or scissors? [\"q\" to quit]:"); // Make sure `print!` actually prints to screen

        io::stdout().flush().unwrap(); // Get user input
        io::stdin().read_line(&mut player_move).unwrap(); // Clean up the user's input
        let player_move = player_move.trim().to_lowercase(); 

        if [RPS::ROCK, RPS::PAPER, RPS::SCISSORS].contains(&player_move.as_ref()) {
            self.chosen = player_move;
            true

        }else if player_move == "q" {
            std::process::exit(0); 
        
        }else {
            false 
        }
    }
}



impl Ord for RPS {
    fn cmp(&self, other: &RPS) -> Ordering{
        if self.chosen == RPS::ROCK {
            if other.chosen == RPS::ROCK {
                return Ordering::Equal; 
            
            }else if other.chosen == RPS::PAPER  {
                return Ordering::Less; 
            
            }
            else {
                return Ordering::Equal; 
            }
            
             
            
        }

        if self.chosen == RPS::PAPER {
            if other.chosen == RPS::PAPER{
                return Ordering::Greater; 
            }

            else if other.chosen == RPS::PAPER {
                return Ordering::Equal; 
            }

            else {
                return Ordering::Less; 
            }

        }



        if self.chosen == RPS::SCISSORS {
            if other.chosen == RPS::ROCK {
                return Ordering::Less; 
            }

            else if other.chosen == RPS::PAPER {
                return Ordering::Greater; 
            }

            else {
                return Ordering::Equal; 
            }
        }

        return Ordering::Equal; 

        return Ordering::Equal; 
    }
}


impl fmt::Display for RPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.chosen)
    }
}




impl PartialEq for RPS {
    fn eq(&self, other: &RPS) -> bool {
        self.chosen == other.chosen
    }
}



impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &RPS) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}





pub fn play() {
    let mut c_tally = 0; 
    let mut p_tally = 0; 

    let mut player_move = RPS::default(); 
    
    if !player_move.get_input(){ // Get input from user
        print!("Invalid move: must be one of \"rock\", \"paper\", or \"scissors\" [\"q\" to quit]");     
        
    }    

    let computer = RPS::default(); // Get a random choice for the computer
    print!("Computer: {}\nPlayer: {}", computer, player_move); // This is allowed because RPS implements the `Display` trait
    if computer > player_move {
        println!("Computer Wins!");
        c_tally += 1; 

    }else if computer < player_move {
        println!("Player wins!");
        p_tally += 1; 

    }else {

    }

}


