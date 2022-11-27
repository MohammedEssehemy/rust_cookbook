use clap::{Arg, Command};

fn main() {
    let matches = Command::new("My Test Program")
        .version("0.1.0")
        .author("Mohammed Essehemy <mohammedessehemy@gmail.com>")
        .about("Teaches argument parsing")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("A cool file"),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .value_name("NUMBER")
                .help("Five less than your favorite number"),
        )
        .get_matches();

    let default_file = "input.txt".to_owned();

    let my_file = matches.get_one::<String>("file").unwrap_or(&default_file);
    println!("The file passed is: {}", my_file);

    let num_str: Option<&String> = matches.get_one("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!("Your favorite number must be {}.", n + 5),
            Err(_) => println!("That's not a number! {}", s),
        },
    }
}
