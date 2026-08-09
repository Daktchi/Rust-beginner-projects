use std::io;
use rand::Rng;

fn main() {
    println!("--DICE ROLLER GAME--");
    loop{
        println!("Enter a bet [1-6]");
        let mut user_bet = String::new();
        io::stdin().read_line(&mut user_bet).expect("Failed to read!");
        let user_bet : i32= match user_bet.trim().parse(){
            Ok(num) => //{
               // if num >= 1 && num <= 6 {
                    num,
              //  }else{
                    //println!("Enter a valid number betwen [1-6]");
                   // continue
                //}
           // },
            
            Err(_) => {
                println!("Enter a valid number range");
                continue
            }
        };
        
        let dice = roll_dice();
        if user_bet >=1 && user_bet <=6 {
            
                if user_bet == dice {
                    println!("You win. your bet : {user_bet} = dice rolled {dice}");
                }else{
                    println!("You lose. your bet : {user_bet} ≠ dice rolled {dice}");
                }
               // break;
        }else{
            println!("Restart! Enter a valid number between [1-6]");
        }
    }
}

fn roll_dice()-> i32{
    let dice = rand::thread_rng().gen_range(1..=6);
    dice
}