use std::io;
use rand::Rng;


fn main() {

    let mut limbs = 4;
    let num_gen = rand::thread_rng().gen_range(0..3);
    let secret_word = words()[num_gen];

    println!("\n\t\t== !Hangman! ==\n");
    println!("Random word generated.");
    println!("You have {} limbs.", limbs);

    loop {

        let mut guess = String::new();

        for _c in secret_word.chars() {
            print!(" _");
        }

        println!("\nEnter guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().to_lowercase();


        if guess.len() == 1 {

            if secret_word.contains(&guess) {
                println!("{} is in the word!", guess);
                continue; 
            }

            else if !secret_word.contains(&guess) {
                println!("Not in word");
                limbs -= 1;

                println!("{} not in word. {} limbs remaining", guess, limbs);

                if limbs == 0 {
                        println!("You died!");
                        break;
                }
            }
        }


        else if guess.len() > 1 {
            if guess == secret_word {
                println!("You guessed it!");
                break;
            }

            else if guess != secret_word {
                limbs -= 1;
                println!("Incorrect guess! {} limbs remaining", limbs);
            

                if limbs == 0 {
                    println!("You died!");
                    break;
                }
            }
        }
    }
}


fn words() -> [&'static str; 4] {


    ["hello", "bye", "morning", "midnight"]

    
}
