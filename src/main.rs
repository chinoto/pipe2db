use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::io::{prelude::*, BufReader};
use rusqlite::{Connection, NO_PARAMS, params};

fn main() -> rusqlite::Result<()> {
    let fifo=File::open("fifo").expect("./fifo must exist and be readable");

    let perm=fifo.metadata().unwrap().permissions().mode();
    assert!(0o0_010_000 & perm !=0, "./fifo must be FIFO, try `mkfifo fifo`");

    let mut fifo_buf=BufReader::new(&fifo);
    let mut buf=String::new();

    let conn=Connection::open("lines.sqlite3")
        .expect("./lines.sqlite3 must be writable");

    conn.execute(
        "CREATE TABLE lines (
            id   INTEGER PRIMARY KEY,
            line TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    // Attempt to read forever. Wait for more lines on EOF.
    while let Ok(len)=fifo_buf.read_line(&mut buf) {
        if len==0 {
            std::thread::sleep(std::time::Duration::from_millis(1));
            continue;
        }

        conn.execute(
            "INSERT INTO lines (line) VALUES (?1)",
            params![buf.trim_end()],
        )?;

        buf.clear();
    }
    Ok(())
}
