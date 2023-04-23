extern crate directories;
use anyhow::{anyhow, Ok, Result};
use directories::{BaseDirs, ProjectDirs, UserDirs};
use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct State {
    workspace_path: String,
    openai_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    workspace_path: String,
    openai_token: String,
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
        }
    }

    fn init_db(&self) {
        let workspace_path = Path::new(self.workspace_path.as_str());

        let workspace_vocabulary_path_buf = PathBuf::new().join(workspace_path).join("vocabulary");
    
        if !workspace_vocabulary_path_buf.exists() {
            mkdir_p(&workspace_vocabulary_path_buf).unwrap();
        }
        let mut conn = Connection::open(workspace_path.join("cache.db")).unwrap();
    
        conn.execute("CREATE TABLE IF NOT EXISTS vocabulary ( query TEXT UNIQUE, content TEXT NOT NULL, timestamp INT NOT NULL);", ()).unwrap();
    
        conn.execute("CREATE INDEX IF NOT EXISTS query_index ON vocabulary (query COLLATE NOCASE);", ()).unwrap();
        
    }

    pub fn load_config(&mut self) -> Result<String> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "Epiphany", "Broca") {
            let path = proj_dirs.config_dir();

            let path_buf = PathBuf::new();
            let config_file_path = path_buf.join(path).join("broca.conf.json");

            println!("{:?}", config_file_path);

            let rs = config_file_path.exists();

            if rs == true {
                let mut file = File::open(config_file_path).unwrap();
                let mut file_buf: Vec<u8> = Vec::new();
                file.read_to_end(&mut file_buf).unwrap();

                let config: Config = serde_json::from_slice(&file_buf).unwrap();

                println!("exisiting config, {:?}", config);

                self.workspace_path = config.workspace_path;
                self.openai_token = config.openai_token;

                return Ok(self.workspace_path);
            }

            // Lin: /home/alice/.config/barapp
            // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
            // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        }

        Err(anyhow!("Not configed."))
    }

    fn config_database(&self) {}

    pub fn first_time_setup(&mut self, workspace_path_str: &str, openai_token: &str) -> Result<()> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "Epiphany", "Broca") {
            let config_dir_path = proj_dirs.config_dir();

            let config_filename = "broca.conf.json";
            let config_file_path = PathBuf::new().join(config_dir_path).join(config_filename);

            let config = Config {
                workspace_path: String::from(workspace_path_str),
                openai_token: String::from(openai_token),
            };

            let serialized_config = serde_json::to_vec_pretty(&config)?;

            if !config_dir_path.exists() {
                mkdir_p(&config_dir_path).unwrap();
            }
            let mut file = File::create(config_file_path)?;
            file.write_all(&serialized_config)?;

            self.workspace_path = config.workspace_path;
            self.openai_token = config.openai_token;

            let workspace_path = Path::new(workspace_path_str);

            if !workspace_path.exists() {
                mkdir_p(&workspace_path).unwrap();
            }

            let workspace_vocabulary_path_buf =
                PathBuf::new().join(workspace_path).join("vocabulary");

            if !workspace_vocabulary_path_buf.exists() {
                mkdir_p(&workspace_vocabulary_path_buf).unwrap();
            }

            self.init_db();

            return Ok(());
        }

        Err(anyhow!("No config directory found."))
    }
}
