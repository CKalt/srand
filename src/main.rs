use rand::Rng;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_chacha::ChaCha20Rng;

use rand::{
    distributions::{Distribution, Standard},
}; // 0.8.0

#[derive(Debug)]
enum Spinner {
    One,
    Two,
    Three,
}

impl Distribution<Spinner> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Spinner {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=2) { // rand 0.8
            0 => Spinner::One,
            1 => Spinner::Two,
            _ => Spinner::Three,
        }
    }
}

fn main() {
    let mut srng = ChaCha8Rng::seed_from_u64(2);
    let n1: u8 = srng.gen();
    let n2: u16 = srng.gen();

    println!("ChaCha8 Fixed Seeded Random u8: {}", n1);
    println!("ChaCha8 Fixed Seeded Random u16: {}", n2);
    println!("ChaCha8 Fixed Seeded Random u32: {}", srng.gen::<u32>());
    println!("ChaCha8 Fixed Seeded Random i32: {}", srng.gen::<i32>());
    println!("ChaCha8 Fixed Seeded Random float: {}", srng.gen::<f64>());
    let spinner: Spinner = srng.gen();
    println!("ChaCha8 Fixed Seeded Spinner {:?}\n\n", spinner);

    let mut srng = ChaCha8Rng::from_entropy();
    let n1: u8 = srng.gen();
    let n2: u16 = srng.gen();
    println!("ChaCha8 Seeded From Entropy Random u8: {}", n1);
    println!("ChaCha8 Seeded From Entropy Random u16: {}", n2);
    println!("ChaCha8 Seeded From Entropy Random u32: {}", srng.gen::<u32>());
    println!("ChaCha8 Seeded From Entropy Random i32: {}", srng.gen::<i32>());
    println!("ChaCha8 Seeded From Entropy Random float: {}", srng.gen::<f64>());
    let spinner: Spinner = srng.gen();
    println!("ChaCha8 Seeded From Entropy Spinner {:?}\n\n", spinner);

    let mut srng = ChaCha20Rng::seed_from_u64(2);
    let n1: u8 = srng.gen();
    let n2: u16 = srng.gen();

    println!("ChaCha20 Fixed Seeded Random u8: {}", n1);
    println!("ChaCha20 Fixed Seeded Random u16: {}", n2);
    println!("ChaCha20 Fixed Seeded Random u32: {}", srng.gen::<u32>());
    println!("ChaCha20 Fixed Seeded Random i32: {}", srng.gen::<i32>());
    println!("ChaCha20 Fixed Seeded Random float: {}", srng.gen::<f64>());
    let spinner: Spinner = srng.gen();
    println!("ChaCha20 Fixed Seeded Spinner {:?}\n\n", spinner);

    let mut srng = ChaCha20Rng::from_entropy();
    let n1: u8 = srng.gen();
    let n2: u16 = srng.gen();
    println!("ChaCha20 Seeded From Entropy Random u8: {}", n1);
    println!("ChaCha20 Seeded From Entropy Random u16: {}", n2);
    println!("ChaCha20 Seeded From Entropy Random u32: {}", srng.gen::<u32>());
    println!("ChaCha20 Seeded From Entropy Random i32: {}", srng.gen::<i32>());
    println!("ChaCha20 Seeded From Entropy Random float: {}", srng.gen::<f64>());
    let spinner: Spinner = srng.gen();
    println!("ChaCha20 Seeded From Entropy Spinner {:?}", spinner);

}
