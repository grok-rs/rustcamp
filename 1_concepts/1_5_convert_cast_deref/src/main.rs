use rand::Rng;
use std::ops::Deref;

#[derive(Debug)]
struct EmailString(String);

impl TryFrom<&str> for EmailString {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains('@') {
            Ok(EmailString(value.to_string()))
        } else {
            Err("Invalid email address")
        }
    }
}

struct Random<T> {
    values: [T; 3],
}

impl<T> Random<T> {
    fn new(a: T, b: T, c: T) -> Self {
        Random { values: [a, b, c] }
    }
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let index = rand::thread_rng().gen_range(0..3);
        &self.values[index]
    }
}

fn main() {
    let email = EmailString::try_from("example@example.com").unwrap();
    println!("Email is {}", email.0);

    let random = Random::new(1, 2, 3);
    println!("Random value: {}", *random);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emailstring_valid() {
        let email = EmailString::try_from("user@example.com");
        assert!(email.is_ok());
        assert_eq!(email.unwrap().0, "user@example.com");
    }

    #[test]
    fn test_emailstring_invalid() {
        let email = EmailString::try_from("invalid-email");
        assert!(email.is_err());
        assert_eq!(email.unwrap_err(), "Invalid email address");
    }

    // TODO Test should use mocked random generator
    #[test]
    fn test_random_deref() {
        let random = Random::new(10, 20, 30);
        for _ in 0..10 {
            let value = *random;
            assert!(value == 10 || value == 20 || value == 30);
        }
    }

    #[test]
    fn test_random_values() {
        let random = Random::new(10, 20, 30);
        assert_eq!(random.values, [10, 20, 30]);
    }
}
