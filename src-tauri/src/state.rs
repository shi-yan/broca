extern crate directories;
use crate::entry::Entry;
use anyhow::{anyhow, Ok, Result};
use aws_sdk_polly::config::Config as AWSConfig;
use aws_sdk_polly::config::Credentials;
use aws_sdk_polly::Client;
use aws_types::region::Region;
use directories::{BaseDirs, ProjectDirs, UserDirs};
use glob::glob;
use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use slugify::slugify;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

#[derive(Debug, Serialize, Deserialize)]
pub enum TargetLang {
    Chinese,
    Spanish,
    Japanese,
    Korean,
    German,
    French,
    Portuguese,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollyConfig {
    aws_key: String,
    aws_secret: String,
    voice_id: String,
}

pub struct State {
    workspace_path: String,
    openai_token: String,
    target_lang: TargetLang,
    polly_config: Option<PollyConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    workspace_path: String,
    openai_token: String,
    target_lang: TargetLang,
    polly_config: Option<PollyConfig>,
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
            polly_config: None,
        }
    }

    fn init_db(&self) -> Result<()> {
        let workspace_path = Path::new(self.workspace_path.as_str());

        let workspace_vocabulary_path_buf = PathBuf::new().join(workspace_path).join("vocabulary");

        if !workspace_vocabulary_path_buf.exists() {
            mkdir_p(&workspace_vocabulary_path_buf)?;
        }
        let conn = Connection::open(workspace_path.join("cache.db"))?;

        conn.execute("CREATE TABLE IF NOT EXISTS vocabulary ( query TEXT UNIQUE, content TEXT NOT NULL, timestamp INT NOT NULL);", ())?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS query_index ON vocabulary (query COLLATE NOCASE);",
            (),
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS timestamp_index ON vocabulary(timestamp);",
            (),
        )?;

        Ok(())
    }

    pub fn delete_word(&self, query: &str) -> Result<String> {
        let workspace_path = Path::new(self.workspace_path.as_str());

        let workspace_vocabulary_path_buf = PathBuf::new().join(workspace_path).join("vocabulary");

        let conn = Connection::open(workspace_path.join("cache.db"))?;

        conn.execute("DELETE FROM vocabulary WHERE query = ?1;", &[query])?;

        let slug = slugify!(query, separator = "_");
        let new_filename = format!("{}.json", slug.as_str());

        let path = workspace_vocabulary_path_buf.join(&new_filename);
        std::fs::remove_file(path.as_path())?;

        Ok(new_filename)
    }

    pub async fn say(&self, content: &str) -> Result<String> {
        let slug = slugify!(content, separator = "_");
        let new_filename = format!("{}.mp3", slug.as_str());

        let workspace_path = Path::new(self.workspace_path.as_str());

        let workspace_audio_path_buf = PathBuf::new().join(workspace_path).join("audio");

        let path = workspace_audio_path_buf.join(new_filename.as_str());

        if path.exists() {
            return Ok(String::from(path.to_str().unwrap()));
        }

        if let Some(polly_config) = &self.polly_config {
            let creds = Credentials::new(&polly_config.aws_key, &polly_config.aws_secret, None, None, "self");

            let conf = aws_sdk_polly::config::Config::builder()
                .credentials_provider(creds)
                .region(Region::new("us-west-2"))
                .build();

            let client = Client::from_conf(conf);

            let audio = client
                .synthesize_speech()
                .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
                .text(content)
                .language_code(aws_sdk_polly::types::LanguageCode::EnUs)
                .voice_id(aws_sdk_polly::types::VoiceId::Amy)
                .send()
                .await?;
            let buf = audio.audio_stream.collect().await?;
            let mut file = std::fs::File::create(path.to_str().unwrap())?;
            file.write_all(&buf.to_vec())?;
            file.flush()?;
            println!("Generated audio {}", path.to_str().unwrap());

            return Ok(String::from(path.to_str().unwrap()));
        } else {
            return Err(anyhow!("Polly is not configured!".to_string()));
        }
    }

    pub async fn search(&self, query: &str) -> Result<String> {
        let workspace_path = Path::new(self.workspace_path.as_str());

        let workspace_vocabulary_path_buf = PathBuf::new().join(workspace_path).join("vocabulary");

        if !workspace_vocabulary_path_buf.exists() {
            mkdir_p(&workspace_vocabulary_path_buf)?;
        }

        print!("search in {:?}", &self.target_lang);

        let res = crate::openai::search(
            query.to_lowercase().as_str(),
            self.openai_token.as_str(),
            &self.target_lang,
        )
        .await?;

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
                self.polly_config = config.polly_config;

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
        let conn = Connection::open(workspace_path.join("cache.db"))?;

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
        let conn = Connection::open(workspace_path.join("cache.db"))?;

        let mut stmt = conn
            .prepare(
                "SELECT query FROM vocabulary WHERE query LIKE :pattern ORDER BY timestamp DESC;",
            )
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

    pub fn to_asset_absolute_path(&mut self, image_filename: &str) -> Result<String> {
        let workspace_assets_path_buf = PathBuf::new()
            .join(self.workspace_path.as_str())
            .join("assets");

        Ok(String::from(
            workspace_assets_path_buf
                .join(image_filename)
                .to_str()
                .unwrap(),
        ))
    }

    pub fn fetch_all_words(&self) -> Result<Vec<String>> {
        let workspace_path = Path::new(self.workspace_path.as_str());
        let conn = Connection::open(workspace_path.join("cache.db"))?;

        let mut stmt = conn.prepare("SELECT query FROM vocabulary ORDER BY timestamp DESC;")?;
        let word_iter = stmt.query_map((), |row| row.get(0))?;

        let mut result = Vec::<String>::new();
        for word in word_iter {
            result.push(word?);
        }

        Ok(result)
    }

    pub fn scan_vocabulary(&self) -> Result<Vec<String>> {
        let workspace_path = Path::new(self.workspace_path.as_str());

        let workspace_vocabulary_path_buf = PathBuf::new().join(workspace_path).join("vocabulary");

        if !workspace_vocabulary_path_buf.exists() {
            mkdir_p(&workspace_vocabulary_path_buf)?;
        }

        let workspace_audio_path_buf = PathBuf::new().join(workspace_path).join("audio");

        if !workspace_audio_path_buf.exists() {
            mkdir_p(&workspace_audio_path_buf)?;
        }

        let conn = Connection::open(workspace_path.join("cache.db"))?;

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
                    let seconds = std::fs::metadata(path.clone())?
                        .modified()?
                        .duration_since(UNIX_EPOCH)?
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
        let results = self.fetch_all_words();
        results
    }

    pub fn first_time_setup(
        &mut self,
        workspace_path_str: &str,
        openai_token: &str,
        target_lang: &str,
        aws_key: Option<&str>,
        aws_secret: Option<&str>
    ) -> Result<String> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "Epiphany", "Broca") {
            let config_dir_path = proj_dirs.config_dir();

            let config_filename = "broca.conf.json";
            let config_file_path = PathBuf::new().join(config_dir_path).join(config_filename);

            let config = Config {
                workspace_path: String::from(workspace_path_str),
                openai_token: String::from(openai_token),
                target_lang: match target_lang {
                    "Chinese" => TargetLang::Chinese,
                    "Spanish" => TargetLang::Spanish,
                    "Japanese" => TargetLang::Japanese,
                    "Korean" => TargetLang::Korean,
                    "German" => TargetLang::German,
                    "French" => TargetLang::French,
                    "Portuguese" => TargetLang::Portuguese,
                    &_ => return Err(anyhow!("Unknown Target Language.")),
                },
                polly_config: if aws_key.is_some() && aws_secret.is_some() {
                    Some( PollyConfig{
                        aws_key: aws_key.unwrap().to_string(),
                        aws_secret: aws_secret.unwrap().to_string(),
                        voice_id: "Olivia".to_string()
                    })
                }else {
                    None
                },
            };

            let serialized_config = serde_json::to_vec_pretty(&config)?;

            if !config_dir_path.exists() {
                mkdir_p(&config_dir_path)?;
            }
            let mut file = File::create(config_file_path)?;
            file.write_all(&serialized_config)?;

            self.workspace_path = config.workspace_path;
            self.openai_token = config.openai_token;
            self.target_lang = config.target_lang;
            self.polly_config = config.polly_config;

            let workspace_path = Path::new(workspace_path_str);

            if !workspace_path.exists() {
                mkdir_p(&workspace_path)?;
            }

            let gitignore_path = workspace_path.join(".gitignore");

            if !gitignore_path.exists() {
                let mut gitignore_file = File::create(gitignore_path)?;
                gitignore_file.write_all(String::from("cache.db").as_bytes())?;
            }

            let workspace_vocabulary_path_buf =
                PathBuf::new().join(workspace_path).join("vocabulary");

            if !workspace_vocabulary_path_buf.exists() {
                mkdir_p(&workspace_vocabulary_path_buf)?;
            }

            let workspace_audio_path_buf = PathBuf::new().join(workspace_path).join("audio");

            if !workspace_audio_path_buf.exists() {
                mkdir_p(&workspace_audio_path_buf)?;
            }

            self.init_db()?;


            return Ok(self.workspace_path.clone());
        }

        Err(anyhow!("No config directory found."))
    }
}
