use std::io; // brings the io library in (io meaning the input/output library)
use rand::Rng;
use std::cmp::Ordering;
fn main() {
	println!("Guess the number!");
	let secret_num = rand::thread_rng().gen_range(1..=100);
	// String::new() creates a new empty string
	// Java equivalent: String guess = ""; or String guess = new String();
	// mut : variable is mutable (can change)
	// otherwise, if mut not specified, variable is immutable (can't be changed);	
	loop{
		println!("Please Input your guess.");
		let mut guess = String::new(); // Declare variables with let

		io::stdin()
			.read_line(&mut guess)// Calls read_line from io::stdin, and stores the user
					// input into guess
			.expect("Failed to read line");
		let guess: u32 = match guess.trim().parse(){
		//Using a match instead of expect expression allows for input checking 
			Ok(num) => num,
			Err(_) => continue,
		};	
		match guess.cmp(&secret_num) {
			Ordering::Less => println!("Too small"),
			Ordering::Greater => println!("Too big"),
			Ordering::Equal => {
				println!("C'est vrai! Bonne!"); 
				break;
			} 	
		}
	}
}
