use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let mut conn = Connection::open("target/cats.db")?;

    conn.execute("DELETE FROM cats;", [])?;
    conn.execute("DELETE FROM cat_colors", [])?;

    successful_tx(&mut conn)?;

    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());
    eprintln!("{:?}", res);

    Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", [])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;

    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", [])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;

    tx.commit()
}
