use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn _guessing_game() {
    println!("Guess the number!");

    // cargo doc --open command = documentation provided by all dependencies locally and open it in your browser
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        // let a = [3; 5];
        // let mut i: i32 = 0;
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                // guesses[i] = &secret_number;
            },
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn _test_plus_one() {
    let x = _plus_one(5);

    println!("The value of x is: {x}");
}

// Learning: You can't put a semicolon, if you do, use return.
fn _plus_one(x: i32) -> i32 {
    x + 1
}

fn _get_index_array(){
    println!("Enter the index of which value you want: ");

    let a: [u32;10] = [1,2,3,4,5,6,7,8,9,10];

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");
    
    let index: usize = index
                .trim()
                .parse()
                .expect("Couldn't parse it into int");
    
    let element = a[index];
    // Thread panics when out of bound memory is accessed - Rust's Memory Safety Principle.
    println!("The value of Array a at index {index} is {element}");
}

fn main(){
    
}
