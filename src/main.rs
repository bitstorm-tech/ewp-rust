use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
	let rnd_number = rand::thread_rng().gen_range(1, 101);
	println!("Random number is {}", rnd_number);

	loop {
		println!("Please enter a number: ");
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Error while reading stdin");

		if guess.trim().to_lowercase().eq("exit") || guess.trim().to_lowercase().eq("quit") {
			println!("Bye bye");
			break;
		}

		let guess: u32 = match guess.trim().parse() {
			Ok(number) => number,
			Err(_) => continue,
		};

		println!("You guessed: {}", guess);
		if number_found(guess, rnd_number) {
			break;
		}
	}
}

fn number_found(guess: u32, rnd_number: u32) -> bool {
	match guess.cmp(&rnd_number) {
		Ordering::Less => {
			println!("Too small!");
			false
		}
		Ordering::Greater => {
			println!("Too big!");
			false
		}
		Ordering::Equal => {
			println!("Got it!");
			true
		}
	}
}
