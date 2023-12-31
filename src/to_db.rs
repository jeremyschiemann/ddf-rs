use rusqlite::{Connection, Result};
use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::io::Read;

fn deserialize_as_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

    match value {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        serde_json::Value::Bool(b) => {
            if b {
                Ok("Wahr".to_string())
            } else {
                Ok("Falsch".to_string())
            }
        }
        _ => Err(serde::de::Error::custom(format!(
            "Expected a string, found {}",
            value
        ))),
    }
}

#[derive(Deserialize)]
struct QuestionJson {
    #[serde(rename = "antwort", deserialize_with = "deserialize_as_string")]
    answer: String,

    #[serde(rename = "frage")]
    question: String,
}

struct QuestionWithSource {
    answer: String,
    question: String,
    source: String,
}

pub fn load_from_files(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS questions (
              question        TEXT NOT NULL,
              answer          TEXT NOT NULL,
              source          TEXT NOT NULL,
              session_seen    INTEGER DEFAULT 0 NOT NULL,
              total_seen      INTEGER DEFAULT 0 NOT NULL
              )",
        (),
    )?;

    let files = ["ddf", "chatgpt", "bard"];

    let mut questions: Vec<QuestionWithSource> = Vec::new();

    for filepath in files {
        let mut file = File::open(format!("./{}.json", filepath)).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");
        let json_content: Vec<QuestionJson> =
            serde_json::from_str(&contents).expect("Unable to deserialize JSON");
        let mapped: Vec<QuestionWithSource> = json_content
            .iter()
            .map(|q| QuestionWithSource {
                answer: q.answer.to_string(),
                question: q.question.to_string(),
                source: filepath.to_string(),
            })
            .collect();
        questions.extend(mapped);
    }

    // Now, you can use 'your_struct' in your program
    for question in questions {
        let mut stmt = conn.prepare(
            "INSERT INTO questions (question, answer, source, session_seen, total_seen)
                      VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;

        stmt.execute((&question.question, &question.answer, &question.source, 0, 0))?;
    }

    Ok(())
}

pub fn reload_from_files(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE IF EXISTS questions", ())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS questions (
              question        TEXT NOT NULL,
              answer          TEXT NOT NULL,
              source          TEXT NOT NULL,
              session_seen    INTEGER DEFAULT 0 NOT NULL,
              total_seen      INTEGER DEFAULT 0 NOT NULL
              )",
        (),
    )?;

    load_from_files(conn)?;
    Ok(())
}
