extern crate directories;
use anyhow::{anyhow, Ok, Result};
use directories::{BaseDirs, ProjectDirs, UserDirs};
use glob::glob;
use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use slugify::slugify;
use crate::entry::Entry;

#[derive(Debug, Serialize, Deserialize)]
pub enum TargetLang {
    Chinese,
    Spanish,
    Japanese,
    Korean,
    German,
    French,
    Portuguese
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollyConfig {
    aws_key: String,
    aws_secret: String,
    voice_id: String
}

pub struct State {
    workspace_path: String,
    openai_token: String,
    target_lang: TargetLang,
    polly_config: Option<PollyConfig>
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    workspace_path: String,
    openai_token: String,
    target_lang: TargetLang,
    polly_config: Option<PollyConfig>
}

fn mkdir_p<P: AsRef<Path>>(path: &P) -> Result<()> {
    if let Err(e) = create_dir_all(path) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(e.into());
        }
    }
    Ok(())
}

impl State {
    pub fn new() -> Self {
        State {
            workspace_path: String::new(),
            openai_token: String::new(),
            target_lang: TargetLang::Chinese,
            polly_config: None
        }
    }

    fn init_db(&self) {
        let workspace_path = Path::new(self.workspace_path.as_str());

        let workspace_vocabulary_path_buf = PathBuf::new().join(workspace_path).join("vocabulary");

        if !workspace_vocabulary_path_buf.exists() {
            mkdir_p(&workspace_vocabulary_path_buf).unwrap();
        }
        let conn = Connection::open(workspace_path.join("cache.db")).unwrap();

        conn.execute("CREATE TABLE IF NOT EXISTS vocabulary ( query TEXT UNIQUE, content TEXT NOT NULL, timestamp INT NOT NULL);", ()).unwrap();

        conn.execute(
            "CREATE INDEX IF NOT EXISTS query_index ON vocabulary (query COLLATE NOCASE);",
            (),
        )
        .unwrap();

        conn.execute(
            "CREATE INDEX IF NOT EXISTS timestamp_index ON vocabulary(timestamp);",
            (),
        )
        .unwrap();
    }

    pub fn delete_word(&self, query: &str)  -> Result<String>  {
        let workspace_path = Path::new(self.workspace_path.as_str());

        let workspace_vocabulary_path_buf = PathBuf::new().join(workspace_path).join("vocabulary");

        let conn = Connection::open(workspace_path.join("cache.db")).unwrap();

        conn.execute("DELETE FROM vocabulary WHERE query = ?1;", &[query]).unwrap();

        let slug = slugify!(query, separator = "_");
        let new_filename = format!("{}.json", slug.as_str());

        let path = workspace_vocabulary_path_buf.join(&new_filename);
        std::fs::remove_file(path.as_path()).unwrap();

        Ok(new_filename)
    }

    pub async fn search(&self, query: &str) -> Result<String>{
        let workspace_path = Path::new(self.workspace_path.as_str());

        let workspace_vocabulary_path_buf = PathBuf::new().join(workspace_path).join("vocabulary");

        if !workspace_vocabulary_path_buf.exists() {
            mkdir_p(&workspace_vocabulary_path_buf).unwrap();
        }

        print!("search in {:?}", &self.target_lang);

        let res = crate::openai::search(query.to_lowercase().as_str(), self.openai_token.as_str(), &self.target_lang).await?;

        let slug = slugify!(query, separator = "_");

        let new_filename = format!("{}.json", slug.as_str());

        let serialized = serde_json::to_string_pretty(&res)?;

        let path = workspace_vocabulary_path_buf.join(&new_filename);

        let mut file = File::create(path.as_path())?;

        file.write_all(serialized.as_bytes())?;

        let conn = Connection::open(workspace_path.join("cache.db"))?;

        let seconds = std::fs::metadata(path.as_path())
            .unwrap()
            .modified()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        conn.execute("INSERT OR REPLACE INTO vocabulary(query, content, timestamp) SELECT ?1, ?2, ?3 WHERE NOT EXISTS (SELECT * FROM vocabulary WHERE query = ?4 AND timestamp >= ?5);", (query.to_lowercase(), serialized.clone(), seconds, query.to_lowercase(), seconds)).unwrap();

        Ok(serialized)
    }

    pub fn load_config(&mut self) -> Result<String> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "Epiphany", "Broca") {
            let path = proj_dirs.config_dir();

            println!("{:?}", path);

            let path_buf = PathBuf::new();
            let config_file_path = path_buf.join(path).join("broca.conf.json");

            println!("{:?}", config_file_path);

            let rs = config_file_path.exists();

            if rs == true {
                let mut file = File::open(config_file_path)?;
                let mut file_buf: Vec<u8> = Vec::new();
                file.read_to_end(&mut file_buf)?;

                let config: Config = serde_json::from_slice(&file_buf)?;

                println!("exisiting config, {:?}", config);

                self.workspace_path = config.workspace_path;
                self.openai_token = config.openai_token;
                self.target_lang = config.target_lang;

                return Ok(self.workspace_path.clone());
            }

            // Lin: /home/alice/.config/barapp
            // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
            // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        }

        Err(anyhow!("Not configed."))
    }

    pub fn load_word(&self, query: &str) -> Result<String> {
        let workspace_path = Path::new(self.workspace_path.as_str());
        let conn = Connection::open(workspace_path.join("cache.db")).unwrap();

        let content = conn
            .query_row(
                "SELECT content FROM vocabulary WHERE query = ?1 LIMIT 1;",
                &[query],
                |row| row.get(0),
            )
            .unwrap();

        Ok(content)
    }

    pub fn query_words(&self, query: &str) -> Result<Vec<String>> {
        let workspace_path = Path::new(self.workspace_path.as_str());
        let conn = Connection::open(workspace_path.join("cache.db")).unwrap();

        let mut stmt = conn
            .prepare("SELECT query FROM vocabulary WHERE query LIKE :pattern ORDER BY timestamp DESC;")
            .unwrap();
        let word_iter = stmt
            .query_map(&[(":pattern", format!("%{}%", query).as_str())], |row| {
                row.get(0)
            })
            .unwrap();

        let mut result = Vec::<String>::new();
        for word in word_iter {
            result.push(word.unwrap());
        }

        Ok(result)
    }

    pub fn fetch_all_words(&self) -> Result<Vec<String>> {
        let workspace_path = Path::new(self.workspace_path.as_str());
        let conn = Connection::open(workspace_path.join("cache.db")).unwrap();

        let mut stmt = conn
            .prepare("SELECT query FROM vocabulary ORDER BY timestamp DESC;")
            .unwrap();
        let word_iter = stmt
            .query_map((), |row| {
                row.get(0)
            })
            .unwrap();

        let mut result = Vec::<String>::new();
        for word in word_iter {
            result.push(word.unwrap());
        }

        Ok(result)
    }

    pub fn scan_vocabulary(&self) -> Result<Vec<String>> {
        let workspace_path = Path::new(self.workspace_path.as_str());

        let workspace_vocabulary_path_buf = PathBuf::new().join(workspace_path).join("vocabulary");

        if !workspace_vocabulary_path_buf.exists() {
            mkdir_p(&workspace_vocabulary_path_buf).unwrap();
        }

        let conn = Connection::open(workspace_path.join("cache.db")).unwrap();


        for entry in glob(
            workspace_vocabulary_path_buf
                .join("*.json")
                .to_str()
                .unwrap(),
        )
        .expect("Failed to read glob pattern")
        {
            match entry {
                std::result::Result::Ok(path) => {
                    let seconds = std::fs::metadata(path.clone())
                        .unwrap()
                        .modified()
                        .unwrap()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    println!("{:?} {}", path.display(), seconds);
                    let file = File::open(path).expect("could not open file");
                    let buffered_reader = BufReader::new(file);
                    let e: Entry = serde_json::from_reader(buffered_reader).unwrap();
                    conn.execute("INSERT OR REPLACE INTO vocabulary(query, content, timestamp) SELECT ?1, ?2, ?3 WHERE NOT EXISTS (SELECT * FROM vocabulary WHERE query = ?4 AND timestamp >= ?5);", (e.query.clone(), serde_json::to_string(&e).unwrap(), seconds, e.query.clone(), seconds)).unwrap();
                    //results.push(e.query);
                }
                Err(e) => println!("{:?}", e),
            }
        }
        let  results = self.fetch_all_words();
        results
    }

    pub fn first_time_setup(
        &mut self,
        workspace_path_str: &str,
        openai_token: &str,
        target_lang: &str
    ) -> Result<String> {
        println!("first time setup");
        if let Some(proj_dirs) = ProjectDirs::from("com", "Epiphany", "Broca") {
            let config_dir_path = proj_dirs.config_dir();

            let config_filename = "broca.conf.json";
            let config_file_path = PathBuf::new().join(config_dir_path).join(config_filename);

            let config = Config {
                workspace_path: String::from(workspace_path_str),
                openai_token: String::from(openai_token),
                target_lang: match target_lang {
                    "Chinese" => { TargetLang::Chinese },
                    "Spanish" => {TargetLang::Spanish},
                    "Japanese" => {TargetLang::Japanese},
                    "Korean" => {TargetLang::Korean},
                    "German" => {TargetLang::German},
                    "French" => {TargetLang::French},
                    "Portuguese" => {TargetLang::Portuguese},
                    &_ => return Err(anyhow!("Unknown Target Language."))
                },
                polly_config:None
            };

            let serialized_config = serde_json::to_vec_pretty(&config)?;

            if !config_dir_path.exists() {
                mkdir_p(&config_dir_path).unwrap();
            }
            let mut file = File::create(config_file_path)?;
            file.write_all(&serialized_config)?;

            self.workspace_path = config.workspace_path;
            self.openai_token = config.openai_token;
            self.target_lang = config.target_lang;

            let workspace_path = Path::new(workspace_path_str);

            if !workspace_path.exists() {
                mkdir_p(&workspace_path).unwrap();
            }

            let gitignore_path = workspace_path.join(".gitignore");

            if !gitignore_path.exists() {
                let mut gitignore_file = File::create(gitignore_path)?;
                gitignore_file.write_all(String::from("cache.db").as_bytes())?;
            }

            let workspace_vocabulary_path_buf =
                PathBuf::new().join(workspace_path).join("vocabulary");

            if !workspace_vocabulary_path_buf.exists() {
                mkdir_p(&workspace_vocabulary_path_buf).unwrap();
            }

            self.init_db();

            return Ok(self.workspace_path.clone());
        }

        Err(anyhow!("No config directory found."))
    }
}
