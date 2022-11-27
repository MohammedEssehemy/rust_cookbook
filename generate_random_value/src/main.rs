use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Random i8: {}", rng.gen::<i8>());
    println!("Random u8: {}", rng.gen::<u8>());
    println!("Random i16: {}", rng.gen::<i16>());
    println!("Random u16: {}", rng.gen::<u16>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random float: {}", rng.gen::<f64>());
}
