extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    if rng.gen() {
        for _ in 0..100000 {
            println!("{}", rng.gen::<u32>() % 100)
        }
    }
}
