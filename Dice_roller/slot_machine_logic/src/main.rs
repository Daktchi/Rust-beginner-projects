use std::fmt::format;
use std::{fs, io};
use rand::Rng;
use std::fs::File;
use std::io::{Read};
use std::io::Write;

fn gen_reel() -> i32{
    rand::thread_rng().gen_range(0..5)
}//Function for generate random number from 0 to 5 (1-6)

fn open_balance_from_file()-> Result<String, io::Error>{/*The function is returning a value of the type Result<T, E>,
                                                        where the generic parameter T has been filled in with the concrete type String
                                                        and the generic type E has been filled in with the concrete type io::Error. */

    let balance_file = File::open("balance.txt");

    let mut balance_file = match balance_file {
        Ok(file ) => file,/*If File::open succeeds, the file handle in the pattern variable
                                file 4 becomes the value in the mutable variable balance_file 3 and
                                the function continues*/
        Err(e) => return Err(e),
    };

    let mut balance = String::new();

    match balance_file.read_to_string(&mut balance) {/*calls the read_to_string method on the file handle in balance_file
                                                              to read the contents of the file into balance*/

        Ok(_) => Ok(balance),/*If this function succeeds without any problems, the code that calls this
                            function will receive an Ok value that holds a String—the balance that this
                            function read from the file*/
        Err(e) => Err(e),/*If this function encounters any problems,
                                the calling code will receive an Err value that holds an instance of io::Error
                                that contains more information about what the problems were*/
    }
}

fn save_balance(balance : f32){
    let path = "balance.txt";
    fs::write(path, balance.to_string()).unwrap();
}

fn main() {

    let reels = ["❌", "❌", "❌", "🌟", "🌟", "🎰"];//The different type of reels
    let mut bet : f32;// The amount to bet or that has been bet

    let balance = match open_balance_from_file() {
        Ok(file) => file,
        Err(err) => err.to_string()
    };
    let mut amount : f32  = balance.parse().unwrap();//The amount in my account

    println!("WELCOME TO SLOT MACHINE LOGIC GAME");

        loop {//Loop choose between bet and exit the game for choose different options to bet

            println!("\t1- Play \n \t2- Exit the game");// Available and allowed options menu

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("You must enter a value here !");//Asking user input

            let choice : u32 = match choice.trim().parse(){//For convert and extract number input from user
                Ok(num) => {
                    num
                },
                Err(_) => {
                    println!("Invalid option. Only a number is allowed.");
                    continue
                }
            };

        let mut reel_vec : Vec<i32> =  Vec::new(); //The different types of selected reel push and saved

        if choice == 1 || choice == 2{// If the choice is 1 or 2

            if choice == 1{//We want to play the game

                loop {// Loop for the bet amount

                    let mut to_bet = String::new();

                    println!("Amount = {} \n Place your bet amount: ", amount);//Dashboard amount

                    let _ = io::stdin().read_line(&mut to_bet);//Take the amount from the user
                    let to_bet  : f32 = match to_bet.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue
                    };

                    if to_bet as usize >= 0 && to_bet as usize <= amount as usize {// If the amount correspond
                        bet = to_bet;
                        amount = amount-bet as f32;
                        break;
                    }else {//If not, continue to ask to retype a valid amount
                        println!("The amount is not sufficient or too higher than what you have");
                        continue
                    }

                }

                println!("Bet : {}. Amount : {}", bet, amount);//Dashboard

                //TABLEAU THAT DISPLAY THE REELS GENERATED
                let mut count = 0;

                loop {

                    reel_vec.push(gen_reel());

                    loop {

                        reel_vec.push(gen_reel());

                        loop {
                            reel_vec.push(gen_reel());
                            break;
                        }
                        break;
                    }

                    if count == 3 { break; }
                    //Top reel
                    let t_l = {
                        if reel_vec[0]-1 < 0 {
                            5 //Vector index element
                        }else{
                            reel_vec[0]-1
                        }
                    };
                    let t_c = {
                        if reel_vec[1]-1 < 0 {
                            5
                        }else{
                            reel_vec[1]-1
                        }
                    };
                    let t_r = {
                        if reel_vec[2]-1 < 0 {
                            5
                        }else{
                            reel_vec[2]-1
                        }
                    };

                    println!("\t\t[ {} | {} | {} ]", reels[t_l as usize], reels[t_c as usize], reels[t_r as usize]);
                    println!("\t\t[ {} | {} | {} ]", reels[reel_vec[0] as usize], reels[reel_vec[1] as usize], reels[reel_vec[2] as usize]);//center reels

                    let b_l = {
                        if reel_vec[0]+1 > 5 {
                            0 //Vector index element
                        }else{
                            reel_vec[0]+1
                        }
                    };
                    let b_c = {
                        if reel_vec[1]+1 > 5 {
                            0
                        }else{
                            reel_vec[1]+1
                        }
                    };
                    let b_r = {
                        if reel_vec[2]+1 > 5 {
                            0
                        }else{
                            reel_vec[2]+1
                        }
                    };

                    println!("\t\t[ {} | {} | {} ]",reels[b_l as usize], reels[b_c as usize], reels[b_r as usize]);
                    //Bottom reels




                    //Check win and attribution recompense
                    if reels[reel_vec[0] as usize] == "🎰" &&  reels[reel_vec[1] as usize] == "🎰" &&  reels[reel_vec[2] as usize] == "🎰"{
                        println!("PERFECT WIN!");
                        amount = amount + bet as f32 *3 as f32;//Add to amount the bet win
                    }else if reels[reel_vec[0] as usize] == "🌟" && reels[reel_vec[1] as usize] == "🌟" && reels[reel_vec[2] as usize] == "🌟" {
                        println!("WIN!");
                        amount += bet as f32 *0.5 as f32;
                    }else {
                        println!("LOOSE!");
                        amount += bet *0 as f32;
                    }

                    save_balance(amount);

                    count+=1;
                    break;
                }


            }else {//We want to exit
                break;
            }

        }else { //If not, we reask user to input and allowed and available one
            println!("Invalid option. Enter a valid option form the menu");
            continue
        }
    }

}
