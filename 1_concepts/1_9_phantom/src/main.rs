use rand::seq::SliceRandom;
use rand::thread_rng;
use std::marker::PhantomData;

// Define the Fact<T> struct with PhantomData
pub struct Fact<T> {
    _marker: PhantomData<T>,
}

impl<T> Fact<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

// Specialization for Vec<T>
impl<T> Fact<Vec<T>> {
    pub fn fact(&self) -> &'static str {
        let facts = [
            "Vec is heap-allocated.",
            "Vec may re-allocate on growing.",
            "Vec has an amortized O(1) push operation.",
        ];
        let mut rng = thread_rng();
        facts.choose(&mut rng).unwrap_or(&"Unknown fact about Vec.")
    }
}

// Specialization for String
impl Fact<String> {
    pub fn fact(&self) -> &'static str {
        let facts = [
            "String is UTF-8 encoded.",
            "String is dynamically sized.",
            "String is backed by a Vec<u8>.",
        ];
        let mut rng = thread_rng();
        facts
            .choose(&mut rng)
            .unwrap_or(&"Unknown fact about String.")
    }
}

// Specialization for Option<T>
impl<T> Fact<Option<T>> {
    pub fn fact(&self) -> &'static str {
        let facts = [
            "Option represents an optional value.",
            "Option can be None or Some.",
            "Option is commonly used to handle nullable values.",
        ];
        let mut rng = thread_rng();
        facts
            .choose(&mut rng)
            .unwrap_or(&"Unknown fact about Option.")
    }
}

// Specialization for Result<T, E>
impl<T, E> Fact<Result<T, E>> {
    pub fn fact(&self) -> &'static str {
        let facts = [
            "Result represents success or failure.",
            "Result has Ok and Err variants.",
            "Result is used for error handling.",
        ];
        let mut rng = thread_rng();
        facts
            .choose(&mut rng)
            .unwrap_or(&"Unknown fact about Result.")
    }
}

fn main() {
    // Fact about Vec
    let vec_fact: Fact<Vec<i32>> = Fact::new();
    println!("Fact about Vec: {}", vec_fact.fact());

    // Fact about String
    let string_fact: Fact<String> = Fact::new();
    println!("Fact about String: {}", string_fact.fact());

    // Fact about Option
    let option_fact: Fact<Option<i32>> = Fact::new();
    println!("Fact about Option: {}", option_fact.fact());

    // Fact about Result
    let result_fact: Fact<Result<i32, String>> = Fact::new();
    println!("Fact about Result: {}", result_fact.fact());
}
