use rusqlite::{Connection, Result};

pub(crate) struct Question {
    answer: String,
    question: String,
    source: String,
    session_seen: u32,
    total_seen: u32,
}

pub fn print_question(question: &Question) {
    println!();
    println!("{}", "+".repeat(18));
    println!("Frage: {}", question.question);
    println!("Antwort: {}", question.answer);
    println!("{}", "-".repeat(18));
    println!(
        "source: {} | session_seen: {} | total_seen: {}",
        question.source, question.session_seen, question.total_seen
    );
    println!("{}", "+".repeat(18));
    println!();
}

pub fn query_next_question(conn: &Connection) -> rusqlite::Result<Question> {
    let mut stmt = conn.prepare(
        "select *
    from questions
    where total_seen = (select min(total_seen) from questions)
    order by random() limit 1;",
    )?;

    let question = stmt.query_row([], |row| {
        Ok(Question {
            answer: row.get("answer")?,
            question: row.get("question")?,
            source: row.get("source")?,
            session_seen: row.get("session_seen")?,
            total_seen: row.get("total_seen")?,
        })
    })?;

    Ok(question)
}

pub fn update_question_meta(connection: &Connection, question: &Question) -> Result<()> {
    connection.execute(
        "UPDATE questions
        SET session_seen = session_seen + 1, total_seen = total_seen + 1
        WHERE question = ? AND answer = ? AND source = ?;",
        [&question.question, &question.answer, &question.source],
    )?;
    Ok(())
}

pub fn reset_seen(conn: &Connection) -> Result<()> {
    conn.execute(
        "UPDATE questions
        SET session_seen = 0;",
        [],
    )?;
    Ok(())
}

pub fn show_all(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("select * from questions;")?;
    let rows = stmt.query_map([], |row| {
        Ok(Question {
            answer: row.get("answer")?,
            question: row.get("question")?,
            source: row.get("source")?,
            session_seen: row.get("session_seen")?,
            total_seen: row.get("total_seen")?,
        })
    })?;

    let questions = rows.collect::<Result<Vec<Question>>>()?;

    for question in questions {
        println!(
            "{} | {} | {} | {} | {}",
            question.question,
            question.answer,
            question.source,
            question.session_seen,
            question.total_seen
        );
    }

    Ok(())
}
