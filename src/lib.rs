//!
//! # Example: guessing game
//! ```
//! use std::cmp::Ordering;
// use simple_std::{prompt, random_int_range};
//
// fn main() {
//     let number = random_int_range(0..100);
//     loop {
//         let input = prompt("Guess: ").parse::<i32>().expect("not a number");
//         match input.cmp(&number) {
//             Ordering::Less => println!("Too Small"),
//             Ordering::Greater => println!("Too Big"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }
//! ```

pub use io::{input, prompt};
pub use random::{random_float, random_int_range};

mod io {
    ///
    /// Reads a single line of input, similar to Pythons `input` function
    ///
    /// # Example
    /// ```
    /// use simple_std::input;
    ///
    /// println!("What is your name?");
    /// let name = input();
    /// println!("Hello {}!", name)
    /// ```
    ///
    /// # Why is this not in std?
    ///
    /// The implementation is fairly simple, just 2 lines, but it has a little complexity to it,
    /// that's why there is the simplified version here.
    pub fn input() -> String {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer
    }

    ///
    /// Reads a single line of input, while providing a message that comes on the same line.
    ///
    /// # Example
    /// ```
    /// use simple_std::prompt;
    ///
    /// let name = prompt("Your name: ");
    /// println!("Hello {}!", name)
    /// ```
    ///
    /// # Why is this not in std?
    ///
    /// see [`input`]
    pub fn prompt(message: &str) -> String {
        use std::io::Write;

        print!("{}", message);
        std::io::stdout().flush().unwrap();
        input()
    }
}

mod random {
    use std::ops::Range;

    ///
    /// Returns a random number from 0 to 1, like Javascript `Math.random`
    ///
    /// # Example
    /// ```
    /// use simple_std::random_float;
    ///
    /// let number = random_float();
    ///
    /// println!("Number between 0 and 1: {}", number);
    ///
    /// assert!(number < 1.0);
    /// assert!(number >= 0.0);
    /// ```
    ///
    /// # Why is this not in std?
    ///
    /// Rust aims to be correct, that's why it's major random number library is cryptographically secure,
    /// meaning it's randomness can't easily be guessed. And cryptographically secure random number generation
    /// is a big task, that's why it has it's own crate.
    pub fn random_float() -> f64 {
        ((random_u64() >> 11) as f64) / ((1u64 << 53) as f64)
    }

    ///
    /// Returns an integer number contained in the range
    ///
    /// # Example
    /// ```
    /// use simple_std::random_int_range;
    ///
    /// let number = random_int_range(0..100);
    ///
    /// println!("Number between 0 and 100: {}", number);
    ///
    /// assert!(number < 100);
    /// assert!(number >= 0);
    /// ```
    ///
    /// # Why is this not in std?
    ///
    /// See [`random_float`]
    ///
    pub fn random_int_range(range: Range<i32>) -> i32 {
        let difference = range.end - range.start;
        range.start + ((random_u64() as i32).abs() % difference)
    }

    /// generates a pseudo-random u32
    fn random_u64() -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};

        static STATE0: AtomicU64 = AtomicU64::new(0);
        static STATE1: AtomicU64 = AtomicU64::new(0);

        if STATE0.load(Ordering::SeqCst) == 0 {
            // more or less random initial state
            STATE0.store((system_time_random()) as u64, Ordering::SeqCst);
            STATE1.store((system_time_random()) as u64, Ordering::SeqCst);
        }

        // use xorshift128+ because it's easy https://v8.dev/blog/math-random

        // not a bug
        let mut s1 = STATE0.load(Ordering::SeqCst);
        let s0 = STATE1.load(Ordering::SeqCst);

        STATE0.store(s0, Ordering::SeqCst);

        s1 ^= s1 << 23;
        s1 ^= s1 >> 17;
        s1 ^= s0;
        s1 ^= s0 >> 26;

        STATE1.store(s1, Ordering::SeqCst);

        s0.wrapping_add(s1)
    }

    fn system_time_random() -> u128 {
        use std::time::SystemTime;

        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros()
            ^ SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
    }

    #[cfg(test)]
    mod test {
        use crate::{random_float, random_int_range};
        use std::iter::repeat_with;

        #[test]
        fn not_equal() {
            repeat_with(random_float)
                .take(100)
                .collect::<Vec<_>>()
                .windows(2)
                .for_each(|win| assert_ne!(win[0], win[1]));
        }

        #[test]
        fn between_0_1() {
            assert!(repeat_with(random_float)
                .take(100000)
                .all(|n| n >= 0.0 && n < 1.0))
        }

        #[test]
        fn distributed() {
            assert!(repeat_with(random_float).take(100000).any(|n| n > 0.999));
            assert!(repeat_with(random_float).take(100000).any(|n| n < 0.001));
        }

        #[test]
        fn range_in_range() {
            [0..10, 5..15, 1000..1004, (-5)..5, (-10)..(-5)]
                .iter()
                .for_each(|range| {
                    assert!(repeat_with(|| random_int_range(range.clone()))
                        .take(10000)
                        .all(|n| n < range.end && n >= range.start));
                })
        }

        #[test]
        fn distributed_range() {
            [0..10, 5..15, 1000..1004, (-5)..5, (-10)..(-5)]
                .iter()
                .for_each(|range| {
                    range.clone().for_each(|expected| {
                        assert!(repeat_with(|| random_int_range(range.clone()))
                            .take(100000)
                            .any(|n| n == expected));
                    });
                })
        }
    }
}
