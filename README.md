# note: this is for practicing rust only, do not use this in actual production programs!

really, don't.

```toml
[dependencies]
simple-std = "0.1.1"
```

simple-std is a little extension to the standard library, 
providing additional helpers for getting input or creating random numbers.

`std` is very useful, but it's lacking for little beginner exercises 
(for a good reason), so I made this library to help with that.

Every function from this library has a little section on why this function isn't in `std`, to help you understand
the reasoning behind including something in `std`.

# Examples

Greeting

```rust
use simple_std::input;

fn main() {
    println!("What is your name?");
    let name = input();
    println!("Hello {}!", name)
}
```

Guessing game

```rust
use std::cmp::Ordering;
use simple_std::{prompt, random_int_range}; 

fn main() {
    let number = random_int_range(0..100);
    loop {
        let input = prompt("Guess: ").parse::<i32>().expect("not a number");
        match input.cmp(&number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```