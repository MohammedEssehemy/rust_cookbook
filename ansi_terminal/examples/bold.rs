use ansi_term::{Colour, Style};

fn main() {
    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );
    println!(
        "{}, {} and {}",
        Colour::Yellow.paint("This is colored"),
        Style::new().bold().paint("this is bold"),
        Colour::Yellow.bold().paint("this is bold and colored")
    );
}
