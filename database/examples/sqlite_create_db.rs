use rusqlite::{Connection, Result};
use std::fs::remove_file;

fn main() -> Result<()> {
    let path = "target/cats.db";
    if let Some(err) = remove_file(path).err() {
        println!("error in removing file {err:?}");
    };
    let conn = Connection::open(path)?;

    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        [],
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        [],
    )?;

    Ok(())
}
