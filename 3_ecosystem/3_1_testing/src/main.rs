use std::{cmp::Ordering, env, io};

fn main() {
    println!("Guess the number!");

    let secret_number = get_secret_number();

    loop {
        println!("Please input your guess.");

        let guess = match get_guess_number() {
            Some(n) => n,
            _ => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn get_secret_number() -> u32 {
    let secret_number = env::args()
        .skip(1)
        .take(1)
        .last()
        .expect("No secret number is specified");
    secret_number
        .trim()
        .parse()
        .ok()
        .expect("Secret number is not a number")
}

fn get_guess_number() -> Option<u32> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::io::{self, BufRead, Cursor};

    // Utility function to simulate user input
    fn mock_stdin(input: &str) -> Cursor<Vec<u8>> {
        Cursor::new(input.as_bytes().to_vec())
    }

    mod get_secret_number_spec {
        use super::*;

        #[test]
        fn it_returns_the_secret_number_when_valid_args_are_provided() {
            let args = vec!["binary_name".to_string(), "42".to_string()];
            env::set_var("RUST_TEST_ARGS", args.join(" "));
            assert_eq!(get_secret_number(), 42);
        }

        #[test]
        #[should_panic(expected = "No secret number is specified")]
        fn it_panics_when_no_secret_number_is_provided() {
            env::set_var("RUST_TEST_ARGS", "binary_name");
            get_secret_number();
        }

        #[test]
        #[should_panic(expected = "Secret number is not a number")]
        fn it_panics_when_secret_number_is_not_a_valid_number() {
            env::set_var("RUST_TEST_ARGS", "binary_name invalid_number");
            get_secret_number();
        }
    }

    mod get_guess_number_spec {
        use super::*;

        #[test]
        fn it_returns_some_number_when_valid_input_is_provided() {
            let input = mock_stdin("42\n");
            let mut buffer = input;
            let mut guess = String::new();
            buffer.read_line(&mut guess).unwrap();
            let result = guess.trim().parse::<u32>().ok();
            assert_eq!(result, Some(42));
        }

        #[test]
        fn it_returns_none_when_input_is_not_a_valid_number() {
            let input = mock_stdin("invalid\n");
            let mut buffer = input;
            let mut guess = String::new();
            buffer.read_line(&mut guess).unwrap();
            let result = guess.trim().parse::<u32>().ok();
            assert_eq!(result, None);
        }
    }
}
