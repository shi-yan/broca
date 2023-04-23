use anyhow::{anyhow, Ok as AnyhowOk, Result};
use glob::glob;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use std::fs::{create_dir_all, File};
use std::io::{Write,BufReader};
use std::path::{Path, PathBuf};
use std::result::Result::Ok;
use std::time::UNIX_EPOCH;
use rusqlite::{Connection, Result as SqliteResult};

mod entry;
mod openai;

fn mkdir_p<P: AsRef<Path>>(path: &P) -> Result<()> {
    if let Err(e) = create_dir_all(path) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(e.into());
        }
    }
    Ok(())
}

async fn search(query: &str) {
    let workspace_path = Path::new("/Users/shiy/vocab");

    let workspace_vocabulary_path_buf = PathBuf::new().join(workspace_path).join("vocabulary");

    if !workspace_vocabulary_path_buf.exists() {
        mkdir_p(&workspace_vocabulary_path_buf).unwrap();
    }

    let res = openai::search(query.to_lowercase().as_str()).await.unwrap();

    let slug = slugify!(query, separator = "_");

    let mut new_filename = format!("{}.json", slug.as_str());

    let serialized = serde_json::to_string_pretty(&res).unwrap();

    let path = workspace_vocabulary_path_buf.join(&new_filename);

    let mut file = File::create(path.as_path()).unwrap();

    file.write_all(serialized.as_bytes()).unwrap();

    let conn = Connection::open(workspace_path.join("cache.db")).unwrap();

    let seconds = std::fs::metadata(path.as_path())
    .unwrap()
    .modified()
    .unwrap()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

    conn.execute("INSERT OR REPLACE INTO vocabulary(query, content, timestamp) SELECT ?1, ?2, ?3 WHERE NOT EXISTS (SELECT * FROM vocabulary WHERE query = ?4 AND timestamp >= ?5);", (query.to_lowercase(), serialized, seconds, query.to_lowercase(), seconds)).unwrap();

}

fn init_db() {
    let workspace_path = Path::new("/Users/shiy/vocab");
    let conn = Connection::open(workspace_path.join("cache.db")).unwrap();

   /*  conn.execute(
        "CREATE TABLE vocabulary (
            id    INTEGER PRIMARY KEY,
            query TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp INT NOT NULL
        )",
        (), // empty list of parameters.
    )?;*/

    conn.execute("CREATE VIRTUAL TABLE IF NOT EXISTS vocabulary USING fts4(query TEXT UNIQUE, content TEXT NOT NULL, timestamp INT NOT NULL, notindexed=content);",()).unwrap();


   // conn.execute("INSERT INTO pages(title, body) VALUES('Home Page', 'SQLite is a software...');", ()).unwrap();
}

fn query(query:&str) {
    let workspace_path = Path::new("/Users/shiy/vocab");
    let mut conn = Connection::open(workspace_path.join("cache.db")).unwrap();

    let mut stmt = conn.prepare("SELECT content FROM vocabulary WHERE query LIKE :pattern;").unwrap();
    let person_iter = stmt.query_map(&[(":pattern", format!("%{}%",query).as_str())], |row| {
        let raw:String = row.get(0).unwrap();
        
        Ok(raw)
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}

fn query_words(query: &str) {
    let workspace_path = Path::new("/Users/shiy/vocab");
    let mut conn = Connection::open(workspace_path.join("cache.db")).unwrap();

    let mut stmt = conn.prepare("SELECT query FROM vocabulary WHERE query LIKE :pattern;").unwrap();
    let person_iter = stmt.query_map(&[(":pattern", format!("%{}%",query).as_str())], |row| {
        let raw:String = row.get(0).unwrap();
        
        Ok(raw)
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}

fn scan() {
    let workspace_path = Path::new("/Users/shiy/vocab");

    let workspace_vocabulary_path_buf = PathBuf::new().join(workspace_path).join("vocabulary");

    if !workspace_vocabulary_path_buf.exists() {
        mkdir_p(&workspace_vocabulary_path_buf).unwrap();
    }
    let mut conn = Connection::open(workspace_path.join("cache.db")).unwrap();

    /*conn.trace(Some(|query:&str| {
        println!("{}", query);
    }));*/
    //conn.execute("CREATE VIRTUAL TABLE IF NOT EXISTS vocabulary USING fts4(query TEXT UNIQUE, content TEXT NOT NULL, timestamp INT NOT NULL, notindexed=content);",()).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS vocabulary ( query TEXT UNIQUE, content TEXT NOT NULL, timestamp INT NOT NULL);", ()).unwrap();

    conn.execute("CREATE INDEX IF NOT EXISTS query_index ON vocabulary (query COLLATE NOCASE);", ()).unwrap();
    for entry in glob(
        workspace_vocabulary_path_buf
            .join("*.json")
            .to_str()
            .unwrap(),
    )
    .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                let seconds = std::fs::metadata(path.clone())
                    .unwrap()
                    .modified()
                    .unwrap()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                println!("{:?} {}", path.display(), seconds);
                let file = File::open(path)
                .expect("could not open file");
                let mut buffered_reader = BufReader::new(file);

                let e:entry::Entry = serde_json::from_reader(buffered_reader).unwrap();
                println!("{:?}",e);
                conn.execute("INSERT OR REPLACE INTO vocabulary(query, content, timestamp) SELECT ?1, ?2, ?3 WHERE NOT EXISTS (SELECT * FROM vocabulary WHERE query = ?4 AND timestamp >= ?5);", (e.query.clone(), serde_json::to_string(&e).unwrap(), seconds, e.query, seconds)).unwrap();

            }
            Err(e) => println!("{:?}", e),
        }
    }
}

#[tokio::main]
async fn main() {
    //search("whim").await;
    scan();

    query("wh");
    println!("test");
}
