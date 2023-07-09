use std::io;
use rand::Rng;

pub mod black_jack;

fn main() {

    bj_with_money(10);

}

fn bj_with_money(mut money: i32)
{
    loop
    {
        if money <= 0
        {
            println!("Du bist Pleite!");
    
            break;
        }

        println!("Dein Geld: {money}$");

        println!("----------------------");

        println!("Wie viel möchtest du einsetzen?");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("error");
    
        let guess: i32 = guess.trim().parse().expect("Please type a number");

        println!("----------------------");


        if money >= guess
        {
            money -= guess;


            let verloren = black_jack::blackjack();

            if verloren == false
            {
                money = money + guess * 2;

                println!("+{}$", guess * 2);
            }
            else if verloren
            {
                println!("-{}$", guess);
            }
        }
        else
        {
            println!("Du hast nicht genug Geld!");
        }


    }
}


#[allow(dead_code)]
fn guessing_game()
{
    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Type your guess: ");

    for i in 1..5 {

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("error");
    
        let guess: u32 = guess.trim().parse().expect("Please type a number");
    
        if guess == secret_number
        {
            println!("You guessed the Number!");

            break;
        }
        else
        {
            if guess > secret_number
            {
                println!("Too High!");
            }
            else if guess < secret_number {
                println!("Too Low!");
            }
        }

        if i == 4
        {
            println!("You lost! Too many guesses!");

            break;
        }
    }
}



