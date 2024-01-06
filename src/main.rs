use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
   
    loop{
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        println!("You guessed:{guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
        Ordering::Less=>println!("Too small!"),
        Ordering::Greater=>println!("Too big!"),
        Ordering::Equal=>{println!("You win!");
        break;
        },
        }
    }
    
}

/*
This code contains a lot of information, so let’s go over it line by line. 
To obtain user input and then print the result as output, we need to bring the io input/output library into scope. 
The io library comes from the standard library, known as std:
use std::io;

By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude,

If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement. Using the std::io

On the right of the equal sign is the value that guess is bound to,
 which is the result of calling String::new, a function that returns a new instance of a String. 
 String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String.

In full, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String

The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.

.read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user. 
We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in. 
The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents),
so we therefore pass that string as an argument.

The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references.
like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable. 

*/