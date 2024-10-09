 use rand::Rng;

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess < secret {
        return -1
    }
     
    if guess > secret {
        return 1
    }

    return 0

}

fn main() {
    let mut secret = 69;

    let mut attempts = 0;

    let mut rng = rand::thread_rng();

    let mut low = i32::MIN;
    let mut high = i32::MAX;

    let mut guess: i32;
    let mut result_string;

    loop {
        guess = rng.gen_range(low..high);

        let result = check_guess(guess, secret);

        attempts += 1;

        if result == 0 {
            println!("Correctly guessed {}", guess);
            break;
        }

        if result == 1 {
            result_string = "high";
            high = guess;

        } else {
            result_string = "low";
            low = guess;
        }

        println!("{} is too {}", guess, result_string);
    }

    println!("Number of guesses: {}", attempts);
}
