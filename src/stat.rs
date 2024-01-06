use std::collections::HashMap;

use rusqlite::{Connection, Result};

pub fn print_stat(
    seen: HashMap<String, u32>,
    seen_since_reset: HashMap<String, u32>,
    len_seen: (u32, u32),
) {
    let seen_keys = seen
        .keys()
        .cloned()
        .chain(std::iter::once(String::from("total")));
    let alignment = seen_keys.map(|key| key.len()).max().unwrap_or(0);

    println!();
    println!("{} seen {}", "+".repeat(6), "+".repeat(6));

    seen.iter().for_each(|(key, value)| {
        println!(
            "{0:>alignment$}: {1:>3} | {2: <3}",
            key,
            value,
            seen_since_reset.get(&key.to_string()).unwrap(),
            alignment = alignment
        )
    });

    println!("{}", "_".repeat(18));
    println!(
        "{0:>alignment$}: {1:>3} | {2: <3}",
        "total",
        len_seen.0,
        len_seen.1,
        alignment = alignment
    );
    println!("{}", "+".repeat(18));
    println!();
}

pub fn query_metadata(
    conn: &Connection,
) -> Result<(HashMap<String, u32>, HashMap<String, u32>, (u32, u32))> {
    let mut stmt =
        conn.prepare("SELECT source, sum(total_seen) FROM questions group by source;")?;
    let seen_since_reset = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<HashMap<String, u32>>>()?;

    let mut stmt =
        conn.prepare("SELECT source, sum(session_seen) FROM questions group by source;")?;
    let seen = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<HashMap<String, u32>>>()?;

    let mut stmt = conn
        .prepare("SELECT sum(session_seen) as session, sum(total_seen) as total FROM questions;")?;
    let len_seen = stmt.query_row([], |row| Ok((row.get(0)?, row.get(1)?)))?;

    Ok((seen, seen_since_reset, len_seen))
}
