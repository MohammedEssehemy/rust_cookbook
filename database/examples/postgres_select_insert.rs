use postgres::{Client, Error, NoTls};
use std::collections::HashMap;

struct Author {
    _id: i32,
    name: String,
    country: String,
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;

    client.batch_execute("DELETE FROM book")?;
    client.batch_execute("DELETE FROM author")?;

    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    for (author_name, country) in &authors {
        client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author_name, &country],
        )?;
    }

    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    Ok(())
}
