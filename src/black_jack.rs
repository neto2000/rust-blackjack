use rand::Rng;
use std::io;

pub fn blackjack() -> bool
{

    let mut card_stack: Vec<u16>= Vec::new();

    let mut counter: u16 = 1;

    while counter <= 52
    {
        let mut value = counter % 13;

        if value == 0 {
            value = 13;
        }

        card_stack.push(value);

        counter += 1;
    }

    println!("Deine Karten: ");

    let first_card = draw_card(&mut card_stack);

    let second_card = draw_card(&mut card_stack);

    let mut current_value: u16 = 0;

    match first_card 
    {
        1 => {println!("Ass"); current_value += 11;},
        11 => {println!("Bube"); current_value += 10;},
        12 => {println!("Dame"); current_value += 10;},
        13 => {println!("König"); current_value += 10;},
        _ => {println!("{}", first_card); current_value += first_card;},
    }

    match second_card
    {
        1 => {println!("Ass"); current_value += 11;},
        11 => {println!("Bube"); current_value += 10;},
        12 => {println!("Dame"); current_value += 10;},
        13 => {println!("König"); current_value += 10;},
        _ => {println!("{}", second_card); current_value += second_card;},
    }


    println!("Aktueller Karten Wert: {}", current_value);
    println!("----------------------");

    let gebers_first_card = draw_card(&mut card_stack);

    let gebers_second_card = draw_card(&mut card_stack);

    let mut geber_value: u16 = 0;

    println!("Die Karte von dem Geber");

    match gebers_first_card 
    {
        1 => {println!("Ass"); geber_value += 11;},
        11 => {println!("Bube"); geber_value += 10;},
        12 => {println!("Dame"); geber_value += 10;},
        13 => {println!("König"); geber_value += 10;},
        _ => {println!("{}", gebers_first_card); geber_value += gebers_first_card;},
    }

    println!("----------------------");

    match gebers_second_card
    {
        1 => geber_value += 11,
        11 => geber_value += 10,
        12 => geber_value += 10,
        13 => geber_value += 10,
        _ => geber_value += gebers_second_card,
    }

    let mut verloren: bool = false;


    loop
    {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("error");

        match input.trim() {
            "ziehen" => {

                println!("Gezogene Karte:");

                let new_card = draw_card(&mut card_stack);

                match new_card 
                {
                    1 => {println!("Ass"); current_value += 11;},
                    11 => {println!("Bube"); current_value += 10;},
                    12 => {println!("Dame"); current_value += 10;},
                    13 => {println!("König"); current_value += 10;},
                    _ => {println!("{}", new_card); current_value += new_card;},
                }

                println!("Aktueller Karten Wert: {}", current_value);
                println!("----------------------");

                if current_value > 21
                {
                    println!("Verloren Wegen Überziehen");

                    verloren = true;

                    break;
                }
            },
            "stop" => {
                println!("----------------------");

                break;
            },
            _ => println!("falscher Befehl"),
        }

    }

    if verloren == false
    {

        while geber_value < 17
        {
            let new_geber_card = draw_card(&mut card_stack);
    
            println!("Die Karte von dem Geber");
    
            match new_geber_card 
            {
                1 => {println!("Ass"); geber_value += 11;},
                11 => {println!("Bube"); geber_value += 10;},
                12 => {println!("Dame"); geber_value += 10;},
                13 => {println!("König"); geber_value += 10;},
                _ => {println!("{}", new_geber_card); geber_value += new_geber_card;},
            }
    
            println!("Aktueller Wert von dem Geber: {}", geber_value);
    
            println!("----------------------");
    
            if geber_value > 21 
            {
                println!("Gewonnen");
            }
        }
    }



    if verloren == false 
    {
        if geber_value <= current_value
        {
            println!("Gewonnen");
        }
        else if geber_value > current_value && geber_value <= 21
        {
            verloren = true;    

            println!("Verloren");
        }
    }
    

    return verloren;

}

fn draw_card(card_stack: &mut Vec<u16>) -> u16
{
    let card_position: usize = rand::thread_rng().gen_range(0..card_stack.len()-1); 

    let value: u16 = card_stack[card_position];
    
    card_stack.remove(card_position);

    return value
}