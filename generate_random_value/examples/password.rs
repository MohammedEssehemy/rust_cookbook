use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    let rand_string = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect::<String>();

    println!("{}", rand_string);
}
