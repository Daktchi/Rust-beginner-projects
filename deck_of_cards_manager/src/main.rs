use rand::Rng;

#[derive(Debug)]
struct Card{//card structure
    heart : String,
    diamond: String,
    spade : String,
    club : String
}
fn shuffle_deck(deck: Vec<i32>) -> Vec<String>{

    // card initialization type
    let card = Card {
        heart : String::from("♥") ,
        diamond : String::from("♦"),
        spade : String::from("♠"),
        club : String::from("♣")
    };

    let mut shuffled_deck = vec![];
    let card_type = vec![card.heart, card.diamond, card.spade, card.club];

    while shuffled_deck.len() < 52 {

        let type_no = rand::thread_rng().gen_range(0..4);//The range generated number for the 4 types of card
        let card_no = rand::thread_rng().gen_range(0..13); //Generative range for 1 to 13 for select the number of card from ace to king
        let text: String;

        text = format!("{}{}", deck[card_no], card_type[type_no]);
        // println!("{text}");
        if ! shuffled_deck.contains(&text) {
            shuffled_deck.push(text.clone());
        }

    }

    shuffled_deck
}

fn main() {

    //Deck of card. Deck card is created here
    let deck  = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];//The number for each type
    let mut count : i32 = 0;//Counter

    for val in shuffle_deck(deck) {

        count += 1;

        if val == "11♥" {
            println!("{count} => J{}", &val[2..5]);//For take the end, the type of it
        }else if val == "11♠" {
            println!("{count} => J{}", &val[2..5]);//For take the end, the type of it
        }else if val == "11♦" {
            println!("{count} => J{}", &val[2..5]);//For take the end, the type of it
        }else if val == "11♣" {
            println!("{count} => J{}", &val[2..5]);//For take the end, the type of it
        }


        if val == "12♥" {
            println!("{count} => Q{}", &val[2..5]);//For take the end, the type of it
        }else if val == "12♠" {
            println!("{count} => Q{}", &val[2..5]);//For take the end, the type of it
        }else if val == "12♦" {
            println!("{count} => Q{}", &val[2..5]);//For take the end, the type of it
        }else if val == "12♣" {
            println!("{count} => Q{}", &val[2..5]);//For take the end, the type of it
        }


        if val == "13♥" {
            println!("{count} => K{}", &val[2..5]);//For take the end, the type of it
        }else if val == "13♠" {
            println!("{count} => K{}", &val[2..5]);//For take the end, the type of it
        }else if val == "13♦" {
            println!("{count} => K{}", &val[2..5]);//For take the end, the type of it
        }else if val == "13♣" {
            println!("{count} => K{}", &val[2..5]);//For take the end, the type of it
        }
        //     Others card
        else {
            println!("{count} => {val}");
        }


    }


}
