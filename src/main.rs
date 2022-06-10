use rand::Rng;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_chacha::ChaCha20Rng;

fn main() {
    let mut srng = ChaCha8Rng::seed_from_u64(2);
    let n1: u8 = srng.gen();
    let n2: u16 = srng.gen();

    println!("Fixed Seeded Random u8: {}", n1);
    println!("Fixed Seeded Random u16: {}", n2);
    println!("Fixed Seeded Random u32: {}", srng.gen::<u32>());
    println!("Fixed Seeded Random i32: {}", srng.gen::<i32>());
    println!("Fixed Seeded Random float: {}\n\n", srng.gen::<f64>());

    let mut srng = ChaCha20Rng::from_entropy();
    let n1: u8 = srng.gen();
    let n2: u16 = srng.gen();
    println!("Seeded From Entropy Random u8: {}", n1);
    println!("Seeded From Entropy Random u16: {}", n2);
    println!("Seeded From Entropy Random u32: {}", srng.gen::<u32>());
    println!("Seeded From Entropy Random i32: {}", srng.gen::<i32>());
    println!("Seeded From Entropy Random float: {}", srng.gen::<f64>());
}
