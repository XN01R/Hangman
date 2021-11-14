use std::io;
use std::io::Write;

fn main() {

    let mut limbs = 5;
    let secret_word = "hello";
    let mut guess = String::new();

    println!("\t\t== !Hangman! ==\n");
    println!("Word is {} characters long", secret_word.len());
    println!("You have {} limbs remaining\n", limbs);

    loop {
        print!("Enter guess: ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = guess.trim().to_lowercase();


        if guess == secret_word
        {
            println!("You guessed it!");
            break;
        }

        else if guess != secret_word
        {
            limbs -= 1;

            if limbs == 0
            {
                println!("You died!");
                break;
            }

            println!("Incorrect guess, you have {} limbs remaining\n", limbs);
        }
    }
}
