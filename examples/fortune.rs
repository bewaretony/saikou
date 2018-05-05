extern crate rand;
extern crate saikou;

fn main() {
    use rand::Rng;

    println!("{}", rand::thread_rng().choose(&saikou::MEMES).unwrap());
}
