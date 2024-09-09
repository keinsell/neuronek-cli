use config::Config;
use config::ConfigError;
use directories_next::ProjectDirs;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

lazy_static! {
    static ref XDG_DIRECTORIES: ProjectDirs =
        ProjectDirs::from("com", "neuronek", "cli").expect("failed to get XDG directories");
    pub static ref CONFIG_DIR: PathBuf = XDG_DIRECTORIES.config_dir().to_path_buf();
    pub static ref CONFIG_PATH: PathBuf = CONFIG_DIR.join("config.toml").to_path_buf();
    pub static ref CACHE_DIR: PathBuf = XDG_DIRECTORIES.cache_dir().to_path_buf();
    pub static ref DATA_DIR: PathBuf = XDG_DIRECTORIES.data_dir().to_path_buf();
}

pub fn ensure_xdg_directories() -> io::Result<()>
{
    fs::create_dir_all(&*CONFIG_DIR)?;
    fs::create_dir_all(&*CACHE_DIR)?;
    fs::create_dir_all(&*DATA_DIR)?;
    Ok(())
}

#[derive(Debug, StructOpt, Serialize, Deserialize)]
#[structopt(name = "example", about = "Configuration of app")]
pub struct Settings
{
    #[structopt(short, long, env = "NEURONEK_SQLITE_URI")]
    pub sqlite_uri: String,
}

impl Settings
{
    pub fn new() -> Result<Self, ConfigError>
    {
        let _config = Config::default();

        // TODO: Read sqlite_uri configuration and if not set fallback to defaults
        // defined bellow.

        let sqlite_uri = if cfg!(debug_assertions)
        {
            let development_database_path = CACHE_DIR.join("dev.db");

            rusqlite::Connection::open(development_database_path.clone())
                .expect("Failed to open sqlite database")
                .close()
                .expect("Failed to close sqlite connection");

            format!("sqlite://{}", development_database_path.to_str().unwrap())
        }
        else
        {
            let production_database_path = DATA_DIR.join("journal.db");

            rusqlite::Connection::open(production_database_path.clone())
                .expect("Failed to open sqlite database")
                .close()
                .expect("Failed to close sqlite connection");

            format!("sqlite://{}", production_database_path.to_str().unwrap())
        };

        Ok(Settings { sqlite_uri })
    }
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().unwrap();
}
